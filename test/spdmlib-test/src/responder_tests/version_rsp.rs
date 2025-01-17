// Copyright (c) 2020 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0

use crate::common::device_io::{FakeSpdmDeviceIoReceve, SharedBuffer};
use crate::common::secret_callback::SECRET_ASYM_IMPL_INSTANCE;
use crate::common::transport::PciDoeTransportEncap;
use crate::common::util::create_info;
use codec::{Codec, Reader, Writer};
use spdmlib::common::*;
use spdmlib::message::*;
use spdmlib::protocol::*;
use spdmlib::{responder, secret};

#[test]
fn test_case0_handle_spdm_version() {
    let (config_info, provision_info) = create_info();
    let pcidoe_transport_encap = &mut PciDoeTransportEncap {};

    secret::asym_sign::register(SECRET_ASYM_IMPL_INSTANCE.clone());

    let shared_buffer = SharedBuffer::new();
    let mut socket_io_transport = FakeSpdmDeviceIoReceve::new(&shared_buffer);

    let mut context = responder::ResponderContext::new(
        &mut socket_io_transport,
        pcidoe_transport_encap,
        config_info,
        provision_info,
    );

    let bytes = &mut [0u8; 1024];
    let mut writer = Writer::init(bytes);
    let value = SpdmMessageHeader {
        version: SpdmVersion::SpdmVersion10,
        request_response_code: SpdmRequestResponseCode::SpdmRequestChallenge,
    };
    assert!(value.encode(&mut writer).is_ok());

    context.handle_spdm_version(bytes);

    let data = context.common.runtime_info.message_a.as_ref();
    let u8_slice = &mut [0u8; 1024];
    for (i, data) in data.iter().enumerate() {
        u8_slice[i] = *data;
    }

    let mut reader = Reader::init(u8_slice);
    let spdm_message_header = SpdmMessageHeader::read(&mut reader).unwrap();
    assert_eq!(spdm_message_header.version, SpdmVersion::SpdmVersion10);
    assert_eq!(
        spdm_message_header.request_response_code,
        SpdmRequestResponseCode::SpdmRequestChallenge
    );

    let u8_slice = &u8_slice[4..];
    let mut reader = Reader::init(u8_slice);
    let spdm_message: SpdmMessage =
        SpdmMessage::spdm_read(&mut context.common, &mut reader).unwrap();

    assert_eq!(spdm_message.header.version, SpdmVersion::SpdmVersion10);
    assert_eq!(
        spdm_message.header.request_response_code,
        SpdmRequestResponseCode::SpdmResponseVersion
    );
    if let SpdmMessagePayload::SpdmVersionResponse(payload) = &spdm_message.payload {
        assert_eq!(payload.version_number_entry_count, 0x03);
        assert_eq!(payload.versions[0].update, 0);
        assert_eq!(payload.versions[0].version, SpdmVersion::SpdmVersion10);
        assert_eq!(payload.versions[1].update, 0);
        assert_eq!(payload.versions[1].version, SpdmVersion::SpdmVersion11);
        assert_eq!(payload.versions[2].update, 0);
        assert_eq!(payload.versions[2].version, SpdmVersion::SpdmVersion12);
    }
}
