// Copyright (c) 2020 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0

use fuzzlib::{spdmlib::protocol::SpdmVersion, *};
use spdmlib::common::SpdmConnectionState;

fn fuzz_handle_spdm_algorithm(data: &[u8]) {
    let (config_info, provision_info) = rsp_create_info();
    let pcidoe_transport_encap = &mut PciDoeTransportEncap {};

    spdmlib::secret::asym_sign::register(SECRET_ASYM_IMPL_INSTANCE.clone());

    let shared_buffer = SharedBuffer::new();
    let mut socket_io_transport = FakeSpdmDeviceIoReceve::new(&shared_buffer);

    let mut context = responder::ResponderContext::new(
        &mut socket_io_transport,
        pcidoe_transport_encap,
        config_info,
        provision_info,
    );
    context.common.negotiate_info.spdm_version_sel = SpdmVersion::SpdmVersion12;
    context
        .common
        .runtime_info
        .set_connection_state(SpdmConnectionState::SpdmConnectionAfterCapabilities);

    context.handle_spdm_algorithm(data);
}

#[cfg(not(feature = "use_libfuzzer"))]
fn main() {
    #[cfg(all(feature = "fuzzlogfile", feature = "fuzz"))]
    flexi_logger::Logger::try_with_str("info")
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .directory("traces")
                .basename("foo")
                .discriminant("Sample4711A")
                .suffix("trc"),
        )
        .print_message()
        .create_symlink("current_run")
        .start()
        .unwrap();

    #[cfg(not(feature = "fuzz"))]
    {
        let args: Vec<String> = std::env::args().collect();
        if args.len() < 2 {
            // Here you can replace the single-step debugging value in the fuzzdata array.
            let fuzzdata = [
                17, 227, 4, 0, 48, 0, 1, 0, 128, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 2, 32, 16, 0, 3, 32, 2, 0, 4, 32, 2, 0, 5, 32, 1, 0,
            ];
            fuzz_handle_spdm_algorithm(&fuzzdata);
        } else {
            let path = &args[1];
            let data = std::fs::read(path).expect("read crash file fail");
            fuzz_handle_spdm_algorithm(data.as_slice());
        }
    }
    #[cfg(feature = "fuzz")]
    afl::fuzz!(|data: &[u8]| {
        fuzz_handle_spdm_algorithm(data);
    });
}
