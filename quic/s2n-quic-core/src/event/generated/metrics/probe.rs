// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

// DO NOT MODIFY THIS FILE
// This file was generated with the `s2n-quic-events` crate and any required
// changes should be made there.

use crate::{
    event::metrics::aggregate::{
        self, info, BoolRecorder, Info, NominalRecorder, Recorder as MetricRecorder,
    },
    probe::define,
};
mod counter {
    #![allow(non_snake_case)]
    use super::*;
    use crate::event::metrics::aggregate::Metric;
    pub struct Recorder(fn(u64));
    impl Recorder {
        pub(crate) fn new(info: &'static Info) -> Self {
            match info.id {
                0usize => Self(application_protocol_information),
                1usize => Self(server_name_information),
                2usize => Self(packet_skipped),
                3usize => Self(packet_sent),
                5usize => Self(packet_sent__bytes__total),
                7usize => Self(packet_received),
                9usize => Self(active_path_updated),
                10usize => Self(path_created),
                11usize => Self(frame_sent),
                14usize => Self(frame_received),
                17usize => Self(connection_close_frame_received),
                19usize => Self(packet_lost),
                21usize => Self(packet_lost__bytes__total),
                24usize => Self(recovery_metrics),
                34usize => Self(congestion),
                36usize => Self(rx_ack_range_dropped),
                37usize => Self(ack_range_received),
                39usize => Self(ack_range_sent),
                41usize => Self(packet_dropped),
                43usize => Self(key_update),
                46usize => Self(key_space_discarded),
                51usize => Self(connection_started),
                52usize => Self(duplicate_packet),
                55usize => Self(transport_parameters_received),
                57usize => Self(datagram_sent),
                58usize => Self(datagram_sent__bytes__total),
                61usize => Self(datagram_received),
                62usize => Self(datagram_received__bytes__total),
                64usize => Self(datagram_dropped),
                65usize => Self(datagram_dropped__bytes__total),
                68usize => Self(handshake_remote_address_change_observed),
                69usize => Self(connection_id_updated),
                70usize => Self(ecn_state_changed),
                72usize => Self(connection_migration_denied),
                74usize => Self(handshake_status_updated),
                79usize => Self(tls_exporter_ready),
                80usize => Self(path_challenge_updated),
                82usize => Self(tls_client_hello),
                84usize => Self(tls_server_hello),
                86usize => Self(rx_stream_progress),
                87usize => Self(rx_stream_progress__bytes__total),
                89usize => Self(tx_stream_progress),
                90usize => Self(tx_stream_progress__bytes__total),
                92usize => Self(keep_alive_timer_expired),
                93usize => Self(mtu_updated),
                97usize => Self(slow_start_exited),
                101usize => Self(delivery_rate_sampled),
                102usize => Self(pacing_rate_updated),
                106usize => Self(bbr_state_changed),
                108usize => Self(dc_state_changed),
                114usize => Self(dc_path_created),
                115usize => Self(connection_closed),
                118usize => Self(version_information),
                119usize => Self(endpoint_packet_sent),
                120usize => Self(endpoint_packet_received),
                121usize => Self(endpoint_datagram_sent),
                125usize => Self(endpoint_datagram_received),
                128usize => Self(endpoint_datagram_dropped),
                132usize => Self(endpoint_connection_attempt_failed),
                134usize => Self(platform_tx),
                135usize => Self(platform_tx__packets__total),
                137usize => Self(platform_tx__syscalls__total),
                139usize => Self(platform_tx__syscalls__blocked__total),
                141usize => Self(platform_tx__errors__total),
                143usize => Self(platform_tx__errors__dropped__total),
                145usize => Self(platform_tx_error),
                146usize => Self(platform_rx),
                147usize => Self(platform_rx__packets__total),
                149usize => Self(platform_rx__syscalls__total),
                151usize => Self(platform_rx__syscalls__blocked__total),
                153usize => Self(platform_rx__errors__total),
                155usize => Self(platform_rx__errors__dropped__total),
                157usize => Self(platform_rx_error),
                158usize => Self(platform_feature_configured),
                159usize => Self(platform_event_loop_wakeup),
                160usize => Self(platform_event_loop_sleep),
                162usize => Self(platform_event_loop_started),
                _ => unreachable!("invalid info: {info:?}"),
            }
        }
    }
    impl MetricRecorder for Recorder {
        fn record<T: Metric>(&self, _info: &'static Info, value: T) {
            (self.0)(value.as_u64());
        }
    }
    define!(
        extern "probe" {
            # [link_name = s2n_quic__event__counter__application_protocol_information]
            fn application_protocol_information(value: u64);
            # [link_name = s2n_quic__event__counter__server_name_information]
            fn server_name_information(value: u64);
            # [link_name = s2n_quic__event__counter__packet_skipped]
            fn packet_skipped(value: u64);
            # [link_name = s2n_quic__event__counter__packet_sent]
            fn packet_sent(value: u64);
            # [link_name = s2n_quic__event__counter__packet_sent__bytes__total]
            fn packet_sent__bytes__total(value: u64);
            # [link_name = s2n_quic__event__counter__packet_received]
            fn packet_received(value: u64);
            # [link_name = s2n_quic__event__counter__active_path_updated]
            fn active_path_updated(value: u64);
            # [link_name = s2n_quic__event__counter__path_created]
            fn path_created(value: u64);
            # [link_name = s2n_quic__event__counter__frame_sent]
            fn frame_sent(value: u64);
            # [link_name = s2n_quic__event__counter__frame_received]
            fn frame_received(value: u64);
            # [link_name = s2n_quic__event__counter__connection_close_frame_received]
            fn connection_close_frame_received(value: u64);
            # [link_name = s2n_quic__event__counter__packet_lost]
            fn packet_lost(value: u64);
            # [link_name = s2n_quic__event__counter__packet_lost__bytes__total]
            fn packet_lost__bytes__total(value: u64);
            # [link_name = s2n_quic__event__counter__recovery_metrics]
            fn recovery_metrics(value: u64);
            # [link_name = s2n_quic__event__counter__congestion]
            fn congestion(value: u64);
            # [link_name = s2n_quic__event__counter__rx_ack_range_dropped]
            fn rx_ack_range_dropped(value: u64);
            # [link_name = s2n_quic__event__counter__ack_range_received]
            fn ack_range_received(value: u64);
            # [link_name = s2n_quic__event__counter__ack_range_sent]
            fn ack_range_sent(value: u64);
            # [link_name = s2n_quic__event__counter__packet_dropped]
            fn packet_dropped(value: u64);
            # [link_name = s2n_quic__event__counter__key_update]
            fn key_update(value: u64);
            # [link_name = s2n_quic__event__counter__key_space_discarded]
            fn key_space_discarded(value: u64);
            # [link_name = s2n_quic__event__counter__connection_started]
            fn connection_started(value: u64);
            # [link_name = s2n_quic__event__counter__duplicate_packet]
            fn duplicate_packet(value: u64);
            # [link_name = s2n_quic__event__counter__transport_parameters_received]
            fn transport_parameters_received(value: u64);
            # [link_name = s2n_quic__event__counter__datagram_sent]
            fn datagram_sent(value: u64);
            # [link_name = s2n_quic__event__counter__datagram_sent__bytes__total]
            fn datagram_sent__bytes__total(value: u64);
            # [link_name = s2n_quic__event__counter__datagram_received]
            fn datagram_received(value: u64);
            # [link_name = s2n_quic__event__counter__datagram_received__bytes__total]
            fn datagram_received__bytes__total(value: u64);
            # [link_name = s2n_quic__event__counter__datagram_dropped]
            fn datagram_dropped(value: u64);
            # [link_name = s2n_quic__event__counter__datagram_dropped__bytes__total]
            fn datagram_dropped__bytes__total(value: u64);
            # [link_name = s2n_quic__event__counter__handshake_remote_address_change_observed]
            fn handshake_remote_address_change_observed(value: u64);
            # [link_name = s2n_quic__event__counter__connection_id_updated]
            fn connection_id_updated(value: u64);
            # [link_name = s2n_quic__event__counter__ecn_state_changed]
            fn ecn_state_changed(value: u64);
            # [link_name = s2n_quic__event__counter__connection_migration_denied]
            fn connection_migration_denied(value: u64);
            # [link_name = s2n_quic__event__counter__handshake_status_updated]
            fn handshake_status_updated(value: u64);
            # [link_name = s2n_quic__event__counter__tls_exporter_ready]
            fn tls_exporter_ready(value: u64);
            # [link_name = s2n_quic__event__counter__path_challenge_updated]
            fn path_challenge_updated(value: u64);
            # [link_name = s2n_quic__event__counter__tls_client_hello]
            fn tls_client_hello(value: u64);
            # [link_name = s2n_quic__event__counter__tls_server_hello]
            fn tls_server_hello(value: u64);
            # [link_name = s2n_quic__event__counter__rx_stream_progress]
            fn rx_stream_progress(value: u64);
            # [link_name = s2n_quic__event__counter__rx_stream_progress__bytes__total]
            fn rx_stream_progress__bytes__total(value: u64);
            # [link_name = s2n_quic__event__counter__tx_stream_progress]
            fn tx_stream_progress(value: u64);
            # [link_name = s2n_quic__event__counter__tx_stream_progress__bytes__total]
            fn tx_stream_progress__bytes__total(value: u64);
            # [link_name = s2n_quic__event__counter__keep_alive_timer_expired]
            fn keep_alive_timer_expired(value: u64);
            # [link_name = s2n_quic__event__counter__mtu_updated]
            fn mtu_updated(value: u64);
            # [link_name = s2n_quic__event__counter__slow_start_exited]
            fn slow_start_exited(value: u64);
            # [link_name = s2n_quic__event__counter__delivery_rate_sampled]
            fn delivery_rate_sampled(value: u64);
            # [link_name = s2n_quic__event__counter__pacing_rate_updated]
            fn pacing_rate_updated(value: u64);
            # [link_name = s2n_quic__event__counter__bbr_state_changed]
            fn bbr_state_changed(value: u64);
            # [link_name = s2n_quic__event__counter__dc_state_changed]
            fn dc_state_changed(value: u64);
            # [link_name = s2n_quic__event__counter__dc_path_created]
            fn dc_path_created(value: u64);
            # [link_name = s2n_quic__event__counter__connection_closed]
            fn connection_closed(value: u64);
            # [link_name = s2n_quic__event__counter__version_information]
            fn version_information(value: u64);
            # [link_name = s2n_quic__event__counter__endpoint_packet_sent]
            fn endpoint_packet_sent(value: u64);
            # [link_name = s2n_quic__event__counter__endpoint_packet_received]
            fn endpoint_packet_received(value: u64);
            # [link_name = s2n_quic__event__counter__endpoint_datagram_sent]
            fn endpoint_datagram_sent(value: u64);
            # [link_name = s2n_quic__event__counter__endpoint_datagram_received]
            fn endpoint_datagram_received(value: u64);
            # [link_name = s2n_quic__event__counter__endpoint_datagram_dropped]
            fn endpoint_datagram_dropped(value: u64);
            # [link_name = s2n_quic__event__counter__endpoint_connection_attempt_failed]
            fn endpoint_connection_attempt_failed(value: u64);
            # [link_name = s2n_quic__event__counter__platform_tx]
            fn platform_tx(value: u64);
            # [link_name = s2n_quic__event__counter__platform_tx__packets__total]
            fn platform_tx__packets__total(value: u64);
            # [link_name = s2n_quic__event__counter__platform_tx__syscalls__total]
            fn platform_tx__syscalls__total(value: u64);
            # [link_name = s2n_quic__event__counter__platform_tx__syscalls__blocked__total]
            fn platform_tx__syscalls__blocked__total(value: u64);
            # [link_name = s2n_quic__event__counter__platform_tx__errors__total]
            fn platform_tx__errors__total(value: u64);
            # [link_name = s2n_quic__event__counter__platform_tx__errors__dropped__total]
            fn platform_tx__errors__dropped__total(value: u64);
            # [link_name = s2n_quic__event__counter__platform_tx_error]
            fn platform_tx_error(value: u64);
            # [link_name = s2n_quic__event__counter__platform_rx]
            fn platform_rx(value: u64);
            # [link_name = s2n_quic__event__counter__platform_rx__packets__total]
            fn platform_rx__packets__total(value: u64);
            # [link_name = s2n_quic__event__counter__platform_rx__syscalls__total]
            fn platform_rx__syscalls__total(value: u64);
            # [link_name = s2n_quic__event__counter__platform_rx__syscalls__blocked__total]
            fn platform_rx__syscalls__blocked__total(value: u64);
            # [link_name = s2n_quic__event__counter__platform_rx__errors__total]
            fn platform_rx__errors__total(value: u64);
            # [link_name = s2n_quic__event__counter__platform_rx__errors__dropped__total]
            fn platform_rx__errors__dropped__total(value: u64);
            # [link_name = s2n_quic__event__counter__platform_rx_error]
            fn platform_rx_error(value: u64);
            # [link_name = s2n_quic__event__counter__platform_feature_configured]
            fn platform_feature_configured(value: u64);
            # [link_name = s2n_quic__event__counter__platform_event_loop_wakeup]
            fn platform_event_loop_wakeup(value: u64);
            # [link_name = s2n_quic__event__counter__platform_event_loop_sleep]
            fn platform_event_loop_sleep(value: u64);
            # [link_name = s2n_quic__event__counter__platform_event_loop_started]
            fn platform_event_loop_started(value: u64);
        }
    );
    pub mod bool {
        #![allow(non_snake_case)]
        use super::*;
        pub struct Recorder(fn(bool));
        impl Recorder {
            pub(crate) fn new(info: &'static Info) -> Self {
                match info.id {
                    23usize => Self(packet_lost__is_mtu_probe),
                    33usize => Self(recovery_metrics__congestion_limited),
                    96usize => Self(mtu_updated__search_complete),
                    _ => unreachable!("invalid info: {info:?}"),
                }
            }
        }
        impl BoolRecorder for Recorder {
            fn record(&self, _info: &'static Info, value: bool) {
                (self.0)(value);
            }
        }
        define!(
            extern "probe" {
                # [link_name = s2n_quic__event__counter__bool__packet_lost__is_mtu_probe]
                fn packet_lost__is_mtu_probe(value: bool);
                # [link_name = s2n_quic__event__counter__bool__recovery_metrics__congestion_limited]
                fn recovery_metrics__congestion_limited(value: bool);
                # [link_name = s2n_quic__event__counter__bool__mtu_updated__search_complete]
                fn mtu_updated__search_complete(value: bool);
            }
        );
    }
    pub mod nominal {
        #![allow(non_snake_case)]
        use super::*;
        use crate::event::metrics::aggregate::Metric;
        pub struct Recorder(fn(u64, u64, &info::Str));
        impl Recorder {
            pub(crate) fn new(info: &'static Info, _variant: &'static info::Variant) -> Self {
                match info.id {
                    4usize => Self(packet_sent__kind),
                    8usize => Self(packet_received__kind),
                    12usize => Self(frame_sent__packet),
                    13usize => Self(frame_sent__frame),
                    15usize => Self(frame_received__packet),
                    16usize => Self(frame_received__frame),
                    18usize => Self(connection_close_frame_received__packet),
                    20usize => Self(packet_lost__kind),
                    35usize => Self(congestion__source),
                    38usize => Self(ack_range_received__packet),
                    40usize => Self(ack_range_sent__packet),
                    42usize => Self(packet_dropped__reason),
                    44usize => Self(key_update__key_type),
                    45usize => Self(key_update__cipher_suite),
                    50usize => Self(key_space_discarded__space),
                    53usize => Self(duplicate_packet__kind),
                    54usize => Self(duplicate_packet__error),
                    67usize => Self(datagram_dropped__reason),
                    71usize => Self(ecn_state_changed__state),
                    73usize => Self(connection_migration_denied__reason),
                    78usize => Self(handshake_status_updated__status),
                    81usize => Self(path_challenge_updated__status),
                    95usize => Self(mtu_updated__cause),
                    98usize => Self(slow_start_exited__cause),
                    107usize => Self(bbr_state_changed__state),
                    113usize => Self(dc_state_changed__state),
                    117usize => Self(connection_closed__error),
                    131usize => Self(endpoint_datagram_dropped__reason),
                    133usize => Self(endpoint_connection_attempt_failed__error),
                    _ => unreachable!("invalid info: {info:?}"),
                }
            }
        }
        impl NominalRecorder for Recorder {
            fn record<T: Metric>(
                &self,
                _info: &'static Info,
                variant: &'static info::Variant,
                value: T,
            ) {
                (self.0)(value.as_u64(), variant.id as _, variant.name);
            }
        }
        define!(
            extern "probe" {
                # [link_name = s2n_quic__event__counter__nominal__packet_sent__kind]
                fn packet_sent__kind(value: u64, variant: u64, variant_name: &info::Str);
                # [link_name = s2n_quic__event__counter__nominal__packet_received__kind]
                fn packet_received__kind(value: u64, variant: u64, variant_name: &info::Str);
                # [link_name = s2n_quic__event__counter__nominal__frame_sent__packet]
                fn frame_sent__packet(value: u64, variant: u64, variant_name: &info::Str);
                # [link_name = s2n_quic__event__counter__nominal__frame_sent__frame]
                fn frame_sent__frame(value: u64, variant: u64, variant_name: &info::Str);
                # [link_name = s2n_quic__event__counter__nominal__frame_received__packet]
                fn frame_received__packet(value: u64, variant: u64, variant_name: &info::Str);
                # [link_name = s2n_quic__event__counter__nominal__frame_received__frame]
                fn frame_received__frame(value: u64, variant: u64, variant_name: &info::Str);
                # [link_name = s2n_quic__event__counter__nominal__connection_close_frame_received__packet]
                fn connection_close_frame_received__packet(
                    value: u64,
                    variant: u64,
                    variant_name: &info::Str,
                );
                # [link_name = s2n_quic__event__counter__nominal__packet_lost__kind]
                fn packet_lost__kind(value: u64, variant: u64, variant_name: &info::Str);
                # [link_name = s2n_quic__event__counter__nominal__congestion__source]
                fn congestion__source(value: u64, variant: u64, variant_name: &info::Str);
                # [link_name = s2n_quic__event__counter__nominal__ack_range_received__packet]
                fn ack_range_received__packet(value: u64, variant: u64, variant_name: &info::Str);
                # [link_name = s2n_quic__event__counter__nominal__ack_range_sent__packet]
                fn ack_range_sent__packet(value: u64, variant: u64, variant_name: &info::Str);
                # [link_name = s2n_quic__event__counter__nominal__packet_dropped__reason]
                fn packet_dropped__reason(value: u64, variant: u64, variant_name: &info::Str);
                # [link_name = s2n_quic__event__counter__nominal__key_update__key_type]
                fn key_update__key_type(value: u64, variant: u64, variant_name: &info::Str);
                # [link_name = s2n_quic__event__counter__nominal__key_update__cipher_suite]
                fn key_update__cipher_suite(value: u64, variant: u64, variant_name: &info::Str);
                # [link_name = s2n_quic__event__counter__nominal__key_space_discarded__space]
                fn key_space_discarded__space(value: u64, variant: u64, variant_name: &info::Str);
                # [link_name = s2n_quic__event__counter__nominal__duplicate_packet__kind]
                fn duplicate_packet__kind(value: u64, variant: u64, variant_name: &info::Str);
                # [link_name = s2n_quic__event__counter__nominal__duplicate_packet__error]
                fn duplicate_packet__error(value: u64, variant: u64, variant_name: &info::Str);
                # [link_name = s2n_quic__event__counter__nominal__datagram_dropped__reason]
                fn datagram_dropped__reason(value: u64, variant: u64, variant_name: &info::Str);
                # [link_name = s2n_quic__event__counter__nominal__ecn_state_changed__state]
                fn ecn_state_changed__state(value: u64, variant: u64, variant_name: &info::Str);
                # [link_name = s2n_quic__event__counter__nominal__connection_migration_denied__reason]
                fn connection_migration_denied__reason(
                    value: u64,
                    variant: u64,
                    variant_name: &info::Str,
                );
                # [link_name = s2n_quic__event__counter__nominal__handshake_status_updated__status]
                fn handshake_status_updated__status(
                    value: u64,
                    variant: u64,
                    variant_name: &info::Str,
                );
                # [link_name = s2n_quic__event__counter__nominal__path_challenge_updated__status]
                fn path_challenge_updated__status(
                    value: u64,
                    variant: u64,
                    variant_name: &info::Str,
                );
                # [link_name = s2n_quic__event__counter__nominal__mtu_updated__cause]
                fn mtu_updated__cause(value: u64, variant: u64, variant_name: &info::Str);
                # [link_name = s2n_quic__event__counter__nominal__slow_start_exited__cause]
                fn slow_start_exited__cause(value: u64, variant: u64, variant_name: &info::Str);
                # [link_name = s2n_quic__event__counter__nominal__bbr_state_changed__state]
                fn bbr_state_changed__state(value: u64, variant: u64, variant_name: &info::Str);
                # [link_name = s2n_quic__event__counter__nominal__dc_state_changed__state]
                fn dc_state_changed__state(value: u64, variant: u64, variant_name: &info::Str);
                # [link_name = s2n_quic__event__counter__nominal__connection_closed__error]
                fn connection_closed__error(value: u64, variant: u64, variant_name: &info::Str);
                # [link_name = s2n_quic__event__counter__nominal__endpoint_datagram_dropped__reason]
                fn endpoint_datagram_dropped__reason(
                    value: u64,
                    variant: u64,
                    variant_name: &info::Str,
                );
                # [link_name = s2n_quic__event__counter__nominal__endpoint_connection_attempt_failed__error]
                fn endpoint_connection_attempt_failed__error(
                    value: u64,
                    variant: u64,
                    variant_name: &info::Str,
                );
            }
        );
    }
}
mod measure {
    #![allow(non_snake_case)]
    use super::*;
    use crate::event::metrics::aggregate::Metric;
    pub struct Recorder(fn(u64));
    impl Recorder {
        pub(crate) fn new(info: &'static Info) -> Self {
            match info.id {
                6usize => Self(packet_sent__bytes),
                22usize => Self(packet_lost__bytes),
                25usize => Self(recovery_metrics__min_rtt),
                26usize => Self(recovery_metrics__smoothed_rtt),
                27usize => Self(recovery_metrics__latest_rtt),
                28usize => Self(recovery_metrics__rtt_variance),
                29usize => Self(recovery_metrics__max_ack_delay),
                30usize => Self(recovery_metrics__pto_count),
                31usize => Self(recovery_metrics__congestion_window),
                32usize => Self(recovery_metrics__bytes_in_flight),
                59usize => Self(datagram_sent__bytes),
                60usize => Self(datagram_sent__gso_offset),
                63usize => Self(datagram_received__bytes),
                66usize => Self(datagram_dropped__bytes),
                88usize => Self(rx_stream_progress__bytes),
                91usize => Self(tx_stream_progress__bytes),
                94usize => Self(mtu_updated__mtu),
                100usize => Self(slow_start_exited__congestion_window),
                103usize => Self(pacing_rate_updated__bytes_per_second),
                104usize => Self(pacing_rate_updated__burst_size),
                105usize => Self(pacing_rate_updated__pacing_gain),
                122usize => Self(endpoint_datagram_sent__bytes),
                123usize => Self(endpoint_datagram_sent__bytes__total),
                124usize => Self(endpoint_datagram_sent__gso_offset),
                126usize => Self(endpoint_datagram_received__bytes),
                127usize => Self(endpoint_datagram_received__bytes__total),
                129usize => Self(endpoint_datagram_dropped__bytes),
                130usize => Self(endpoint_datagram_dropped__bytes__total),
                136usize => Self(platform_tx__packets),
                138usize => Self(platform_tx__syscalls),
                140usize => Self(platform_tx__syscalls__blocked),
                142usize => Self(platform_tx__errors),
                144usize => Self(platform_tx__errors__dropped),
                148usize => Self(platform_rx__packets),
                150usize => Self(platform_rx__syscalls),
                152usize => Self(platform_rx__syscalls__blocked),
                154usize => Self(platform_rx__errors),
                156usize => Self(platform_rx__errors__dropped),
                _ => unreachable!("invalid info: {info:?}"),
            }
        }
    }
    impl MetricRecorder for Recorder {
        fn record<T: Metric>(&self, _info: &'static Info, value: T) {
            (self.0)(value.as_u64());
        }
    }
    define!(
        extern "probe" {
            # [link_name = s2n_quic__event__measure__packet_sent__bytes]
            fn packet_sent__bytes(value: u64);
            # [link_name = s2n_quic__event__measure__packet_lost__bytes]
            fn packet_lost__bytes(value: u64);
            # [link_name = s2n_quic__event__measure__recovery_metrics__min_rtt]
            fn recovery_metrics__min_rtt(value: u64);
            # [link_name = s2n_quic__event__measure__recovery_metrics__smoothed_rtt]
            fn recovery_metrics__smoothed_rtt(value: u64);
            # [link_name = s2n_quic__event__measure__recovery_metrics__latest_rtt]
            fn recovery_metrics__latest_rtt(value: u64);
            # [link_name = s2n_quic__event__measure__recovery_metrics__rtt_variance]
            fn recovery_metrics__rtt_variance(value: u64);
            # [link_name = s2n_quic__event__measure__recovery_metrics__max_ack_delay]
            fn recovery_metrics__max_ack_delay(value: u64);
            # [link_name = s2n_quic__event__measure__recovery_metrics__pto_count]
            fn recovery_metrics__pto_count(value: u64);
            # [link_name = s2n_quic__event__measure__recovery_metrics__congestion_window]
            fn recovery_metrics__congestion_window(value: u64);
            # [link_name = s2n_quic__event__measure__recovery_metrics__bytes_in_flight]
            fn recovery_metrics__bytes_in_flight(value: u64);
            # [link_name = s2n_quic__event__measure__datagram_sent__bytes]
            fn datagram_sent__bytes(value: u64);
            # [link_name = s2n_quic__event__measure__datagram_sent__gso_offset]
            fn datagram_sent__gso_offset(value: u64);
            # [link_name = s2n_quic__event__measure__datagram_received__bytes]
            fn datagram_received__bytes(value: u64);
            # [link_name = s2n_quic__event__measure__datagram_dropped__bytes]
            fn datagram_dropped__bytes(value: u64);
            # [link_name = s2n_quic__event__measure__rx_stream_progress__bytes]
            fn rx_stream_progress__bytes(value: u64);
            # [link_name = s2n_quic__event__measure__tx_stream_progress__bytes]
            fn tx_stream_progress__bytes(value: u64);
            # [link_name = s2n_quic__event__measure__mtu_updated__mtu]
            fn mtu_updated__mtu(value: u64);
            # [link_name = s2n_quic__event__measure__slow_start_exited__congestion_window]
            fn slow_start_exited__congestion_window(value: u64);
            # [link_name = s2n_quic__event__measure__pacing_rate_updated__bytes_per_second]
            fn pacing_rate_updated__bytes_per_second(value: u64);
            # [link_name = s2n_quic__event__measure__pacing_rate_updated__burst_size]
            fn pacing_rate_updated__burst_size(value: u64);
            # [link_name = s2n_quic__event__measure__pacing_rate_updated__pacing_gain]
            fn pacing_rate_updated__pacing_gain(value: u64);
            # [link_name = s2n_quic__event__measure__endpoint_datagram_sent__bytes]
            fn endpoint_datagram_sent__bytes(value: u64);
            # [link_name = s2n_quic__event__measure__endpoint_datagram_sent__bytes__total]
            fn endpoint_datagram_sent__bytes__total(value: u64);
            # [link_name = s2n_quic__event__measure__endpoint_datagram_sent__gso_offset]
            fn endpoint_datagram_sent__gso_offset(value: u64);
            # [link_name = s2n_quic__event__measure__endpoint_datagram_received__bytes]
            fn endpoint_datagram_received__bytes(value: u64);
            # [link_name = s2n_quic__event__measure__endpoint_datagram_received__bytes__total]
            fn endpoint_datagram_received__bytes__total(value: u64);
            # [link_name = s2n_quic__event__measure__endpoint_datagram_dropped__bytes]
            fn endpoint_datagram_dropped__bytes(value: u64);
            # [link_name = s2n_quic__event__measure__endpoint_datagram_dropped__bytes__total]
            fn endpoint_datagram_dropped__bytes__total(value: u64);
            # [link_name = s2n_quic__event__measure__platform_tx__packets]
            fn platform_tx__packets(value: u64);
            # [link_name = s2n_quic__event__measure__platform_tx__syscalls]
            fn platform_tx__syscalls(value: u64);
            # [link_name = s2n_quic__event__measure__platform_tx__syscalls__blocked]
            fn platform_tx__syscalls__blocked(value: u64);
            # [link_name = s2n_quic__event__measure__platform_tx__errors]
            fn platform_tx__errors(value: u64);
            # [link_name = s2n_quic__event__measure__platform_tx__errors__dropped]
            fn platform_tx__errors__dropped(value: u64);
            # [link_name = s2n_quic__event__measure__platform_rx__packets]
            fn platform_rx__packets(value: u64);
            # [link_name = s2n_quic__event__measure__platform_rx__syscalls]
            fn platform_rx__syscalls(value: u64);
            # [link_name = s2n_quic__event__measure__platform_rx__syscalls__blocked]
            fn platform_rx__syscalls__blocked(value: u64);
            # [link_name = s2n_quic__event__measure__platform_rx__errors]
            fn platform_rx__errors(value: u64);
            # [link_name = s2n_quic__event__measure__platform_rx__errors__dropped]
            fn platform_rx__errors__dropped(value: u64);
        }
    );
}
mod gauge {
    #![allow(non_snake_case)]
    use super::*;
    use crate::event::metrics::aggregate::Metric;
    pub struct Recorder(fn(u64));
    impl Recorder {
        pub(crate) fn new(info: &'static Info) -> Self {
            unreachable!("invalid info: {info:?}")
        }
    }
    impl MetricRecorder for Recorder {
        fn record<T: Metric>(&self, _info: &'static Info, value: T) {
            (self.0)(value.as_u64());
        }
    }
}
mod timer {
    #![allow(non_snake_case)]
    use super::*;
    use crate::event::metrics::aggregate::Metric;
    pub struct Recorder(fn(core::time::Duration));
    impl Recorder {
        pub(crate) fn new(info: &'static Info) -> Self {
            match info.id {
                47usize => Self(key_space_discarded__initial__latency),
                48usize => Self(key_space_discarded__handshake__latency),
                49usize => Self(key_space_discarded__one_rtt__latency),
                56usize => Self(transport_parameters_received__latency),
                75usize => Self(handshake_status_updated__complete__latency),
                76usize => Self(handshake_status_updated__confirmed__latency),
                77usize => Self(handshake_status_updated__handshake_done_acked__latency),
                83usize => Self(tls_client_hello__latency),
                85usize => Self(tls_server_hello__latency),
                109usize => Self(dc_state_changed__version_negotiated__latency),
                110usize => Self(dc_state_changed__no_version_negotiated__latency),
                111usize => Self(dc_state_changed__path_secrets__latency),
                112usize => Self(dc_state_changed__complete__latency),
                116usize => Self(connection_closed__latency),
                161usize => Self(platform_event_loop_sleep__processing_duration),
                _ => unreachable!("invalid info: {info:?}"),
            }
        }
    }
    impl MetricRecorder for Recorder {
        fn record<T: Metric>(&self, _info: &'static Info, value: T) {
            (self.0)(value.as_duration());
        }
    }
    define!(
        extern "probe" {
            # [link_name = s2n_quic__event__timer__key_space_discarded__initial__latency]
            fn key_space_discarded__initial__latency(value: core::time::Duration);
            # [link_name = s2n_quic__event__timer__key_space_discarded__handshake__latency]
            fn key_space_discarded__handshake__latency(value: core::time::Duration);
            # [link_name = s2n_quic__event__timer__key_space_discarded__one_rtt__latency]
            fn key_space_discarded__one_rtt__latency(value: core::time::Duration);
            # [link_name = s2n_quic__event__timer__transport_parameters_received__latency]
            fn transport_parameters_received__latency(value: core::time::Duration);
            # [link_name = s2n_quic__event__timer__handshake_status_updated__complete__latency]
            fn handshake_status_updated__complete__latency(value: core::time::Duration);
            # [link_name = s2n_quic__event__timer__handshake_status_updated__confirmed__latency]
            fn handshake_status_updated__confirmed__latency(value: core::time::Duration);
            # [link_name = s2n_quic__event__timer__handshake_status_updated__handshake_done_acked__latency]
            fn handshake_status_updated__handshake_done_acked__latency(value: core::time::Duration);
            # [link_name = s2n_quic__event__timer__tls_client_hello__latency]
            fn tls_client_hello__latency(value: core::time::Duration);
            # [link_name = s2n_quic__event__timer__tls_server_hello__latency]
            fn tls_server_hello__latency(value: core::time::Duration);
            # [link_name = s2n_quic__event__timer__dc_state_changed__version_negotiated__latency]
            fn dc_state_changed__version_negotiated__latency(value: core::time::Duration);
            # [link_name = s2n_quic__event__timer__dc_state_changed__no_version_negotiated__latency]
            fn dc_state_changed__no_version_negotiated__latency(value: core::time::Duration);
            # [link_name = s2n_quic__event__timer__dc_state_changed__path_secrets__latency]
            fn dc_state_changed__path_secrets__latency(value: core::time::Duration);
            # [link_name = s2n_quic__event__timer__dc_state_changed__complete__latency]
            fn dc_state_changed__complete__latency(value: core::time::Duration);
            # [link_name = s2n_quic__event__timer__connection_closed__latency]
            fn connection_closed__latency(value: core::time::Duration);
            # [link_name = s2n_quic__event__timer__platform_event_loop_sleep__processing_duration]
            fn platform_event_loop_sleep__processing_duration(value: core::time::Duration);
        }
    );
    pub mod nominal {
        #![allow(non_snake_case)]
        use super::*;
        use crate::event::metrics::aggregate::Metric;
        pub struct Recorder(fn(core::time::Duration, u64, &info::Str));
        impl Recorder {
            pub(crate) fn new(info: &'static Info, _variant: &'static info::Variant) -> Self {
                match info.id {
                    99usize => Self(slow_start_exited__latency),
                    _ => unreachable!("invalid info: {info:?}"),
                }
            }
        }
        impl NominalRecorder for Recorder {
            fn record<T: Metric>(
                &self,
                _info: &'static Info,
                variant: &'static info::Variant,
                value: T,
            ) {
                (self.0)(value.as_duration(), variant.id as _, variant.name);
            }
        }
        define!(
            extern "probe" {
                # [link_name = s2n_quic__event__timer__nominal__slow_start_exited__latency]
                fn slow_start_exited__latency(
                    value: core::time::Duration,
                    variant: u64,
                    variant_name: &info::Str,
                );
            }
        );
    }
}
#[derive(Default)]
pub struct Registry(());
impl aggregate::Registry for Registry {
    type Counter = counter::Recorder;
    type BoolCounter = counter::bool::Recorder;
    type NominalCounter = counter::nominal::Recorder;
    type Measure = measure::Recorder;
    type Gauge = gauge::Recorder;
    type Timer = timer::Recorder;
    type NominalTimer = timer::nominal::Recorder;
    #[inline]
    fn register_counter(&self, info: &'static Info) -> Self::Counter {
        counter::Recorder::new(info)
    }
    #[inline]
    fn register_bool_counter(&self, info: &'static Info) -> Self::BoolCounter {
        counter::bool::Recorder::new(info)
    }
    #[inline]
    fn register_nominal_counter(
        &self,
        info: &'static Info,
        variant: &'static info::Variant,
    ) -> Self::NominalCounter {
        counter::nominal::Recorder::new(info, variant)
    }
    #[inline]
    fn register_measure(&self, info: &'static Info) -> Self::Measure {
        measure::Recorder::new(info)
    }
    #[inline]
    fn register_gauge(&self, info: &'static Info) -> Self::Gauge {
        gauge::Recorder::new(info)
    }
    #[inline]
    fn register_timer(&self, info: &'static Info) -> Self::Timer {
        timer::Recorder::new(info)
    }
    #[inline]
    fn register_nominal_timer(
        &self,
        info: &'static Info,
        variant: &'static info::Variant,
    ) -> Self::NominalTimer {
        timer::nominal::Recorder::new(info, variant)
    }
}
