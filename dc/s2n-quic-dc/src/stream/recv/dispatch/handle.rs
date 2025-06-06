// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::{
    descriptor::Descriptor,
    probes,
    queue::{Error, Half},
};
use crate::{stream::Actor, sync::ring_deque};
use core::{
    fmt,
    task::{Context, Poll},
};
use s2n_quic_core::varint::VarInt;
use std::collections::VecDeque;

macro_rules! impl_recv {
    ($name:ident, $field:ident, $half:expr) => {
        pub struct $name<T: 'static, Key: 'static> {
            descriptor: Descriptor<T, Key>,
        }

        impl<T: 'static, Key: 'static> $name<T, Key> {
            #[inline]
            pub(super) fn new(descriptor: Descriptor<T, Key>) -> Self {
                Self { descriptor }
            }

            /// Returns the associated `queue_id` for the channel
            ///
            /// This can be sent to a peer, which can be used to route packets back to the channel.
            #[inline]
            pub fn queue_id(&self) -> VarInt {
                unsafe { self.descriptor.queue_id() }
            }

            #[inline]
            pub fn push(&self, item: T) -> Option<T> {
                unsafe {
                    let prev = self.descriptor.$field().force_push(item);
                    probes::on_send(self.descriptor.queue_id(), $half, prev.is_some());
                    prev
                }
            }

            #[inline]
            pub fn try_recv(&self) -> Result<Option<T>, ring_deque::Closed> {
                unsafe {
                    let value = self.descriptor.$field().pop()?;
                    probes::on_recv(self.descriptor.queue_id(), $half, value.is_some().into());
                    Ok(value)
                }
            }

            #[inline]
            pub async fn recv(&self, actor: Actor) -> Result<T, ring_deque::Closed> {
                core::future::poll_fn(|cx| self.poll_recv(cx, actor)).await
            }

            #[inline]
            pub fn poll_recv(
                &self,
                cx: &mut Context,
                actor: Actor,
            ) -> Poll<Result<T, ring_deque::Closed>> {
                unsafe {
                    match self.descriptor.$field().poll_pop(cx, actor) {
                        Poll::Ready(Ok(value)) => {
                            probes::on_recv(self.descriptor.queue_id(), $half, 1);
                            Poll::Ready(Ok(value))
                        }
                        Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
                        Poll::Pending => {
                            probes::on_recv(self.descriptor.queue_id(), $half, 0);
                            Poll::Pending
                        }
                    }
                }
            }

            #[inline]
            pub fn poll_swap(
                &self,
                cx: &mut Context,
                actor: Actor,
                out: &mut VecDeque<T>,
            ) -> Poll<Result<(), ring_deque::Closed>> {
                unsafe {
                    match self.descriptor.$field().poll_swap(cx, actor, out) {
                        Poll::Ready(Ok(_)) => {
                            probes::on_recv(self.descriptor.queue_id(), $half, out.len());
                            Poll::Ready(Ok(()))
                        }
                        Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
                        Poll::Pending => {
                            probes::on_recv(self.descriptor.queue_id(), $half, 0);
                            Poll::Pending
                        }
                    }
                }
            }
        }

        impl<T: 'static, Key: 'static> fmt::Debug for $name<T, Key> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("queue_id", &self.queue_id())
                    .finish()
            }
        }

        impl<T: 'static, Key: 'static> Drop for $name<T, Key> {
            #[inline]
            fn drop(&mut self) {
                unsafe {
                    self.descriptor.drop_receiver($half);
                }
            }
        }
    };
}

impl_recv!(Control, control_queue, Half::Control);
impl_recv!(Stream, stream_queue, Half::Stream);

pub struct Sender<T: 'static, Key: 'static> {
    descriptor: Descriptor<T, Key>,
}

impl<T: 'static, Key: 'static> Clone for Sender<T, Key> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            Self {
                descriptor: self.descriptor.clone_for_sender(),
            }
        }
    }
}

impl<T: 'static, Key: 'static> Sender<T, Key> {
    #[inline]
    pub(super) fn new(descriptor: Descriptor<T, Key>) -> Self {
        Self { descriptor }
    }

    #[inline]
    pub fn send_stream(&self, item: T) -> Result<Option<T>, Error> {
        unsafe {
            let prev = self.descriptor.stream_queue().push(item)?;
            probes::on_send(self.descriptor.queue_id(), Half::Stream, prev.is_some());
            Ok(prev)
        }
    }

    #[inline]
    pub fn send_control(&self, item: T) -> Result<Option<T>, Error> {
        unsafe {
            let prev = self.descriptor.control_queue().push(item)?;
            probes::on_send(self.descriptor.queue_id(), Half::Control, prev.is_some());
            Ok(prev)
        }
    }
}

impl<T: 'static, Key: 'static> Drop for Sender<T, Key> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self.descriptor.drop_sender();
        }
    }
}
