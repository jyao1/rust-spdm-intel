// Copyright (c) 2023 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0

#![no_main]

use libfuzzer_sys::fuzz_target;

include!("../../../fuzz-target/requester/psk_finish_req/src/main.rs");

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    fuzz_send_receive_spdm_psk_finish(data);
});
