use std::convert::TryFrom;
use std::str::FromStr;

use cid::Cid;
use libipld_core::ipld::Ipld;
use serde_cbor::{from_slice, to_vec};
use serde_derive::{Deserialize, Serialize};

#[test]
fn test_cid() {
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct MyStruct {
        cid: Cid,
        data: bool,
    }

    let cid = Cid::from_str("bafkreibme22gw2h7y2h7tg2fhqotaqjucnbc24deqo72b6mkl2egezxhvy").unwrap();
    let cid_encoded = to_vec(&cid).unwrap();
    assert_eq!(
        cid_encoded,
        [
            0xd8, 0x2a, 0x58, 0x25, 0x00, 0x01, 0x55, 0x12, 0x20, 0x2c, 0x26, 0xb4, 0x6b, 0x68,
            0xff, 0xc6, 0x8f, 0xf9, 0x9b, 0x45, 0x3c, 0x1d, 0x30, 0x41, 0x34, 0x13, 0x42, 0x2d,
            0x70, 0x64, 0x83, 0xbf, 0xa0, 0xf9, 0x8a, 0x5e, 0x88, 0x62, 0x66, 0xe7, 0xae,
        ]
    );

    let cid_decoded_as_cid: Cid = from_slice(&cid_encoded).unwrap();
    assert_eq!(cid_decoded_as_cid, cid);

    let cid_decoded_as_ipld: Ipld = from_slice(&cid_encoded).unwrap();
    assert_eq!(cid_decoded_as_ipld, Ipld::Link(cid));

    // Tests with the Type nested in a struct

    let mystruct = MyStruct { cid, data: true };
    let mystruct_encoded = to_vec(&mystruct).unwrap();
    assert_eq!(
        mystruct_encoded,
        [
            0xa2, 0x63, 0x63, 0x69, 0x64, 0xd8, 0x2a, 0x58, 0x25, 0x00, 0x01, 0x55, 0x12, 0x20,
            0x2c, 0x26, 0xb4, 0x6b, 0x68, 0xff, 0xc6, 0x8f, 0xf9, 0x9b, 0x45, 0x3c, 0x1d, 0x30,
            0x41, 0x34, 0x13, 0x42, 0x2d, 0x70, 0x64, 0x83, 0xbf, 0xa0, 0xf9, 0x8a, 0x5e, 0x88,
            0x62, 0x66, 0xe7, 0xae, 0x64, 0x64, 0x61, 0x74, 0x61, 0xf5
        ]
    );

    let mystruct_decoded_as_mystruct: MyStruct = from_slice(&mystruct_encoded).unwrap();
    assert_eq!(mystruct_decoded_as_mystruct, mystruct);

    let mystruct_decoded_as_ipld: Ipld = from_slice(&mystruct_encoded).unwrap();
    let mut expected_map = std::collections::BTreeMap::new();
    expected_map.insert("cid".to_string(), Ipld::Link(cid));
    expected_map.insert("data".to_string(), Ipld::Bool(true));
    assert_eq!(mystruct_decoded_as_ipld, Ipld::Map(expected_map));
}

/// Test that arbitrary bytes are not interpreted as CID.
#[test]
fn test_binary_not_as_cid() {
    // h'affe'
    // 42      # bytes(2)
    //    AFFE # "\xAF\xFE"
    let bytes = [0x42, 0xaf, 0xfe];
    let bytes_as_ipld: Ipld = from_slice(&bytes).unwrap();
    assert_eq!(bytes_as_ipld, Ipld::Bytes(vec![0xaf, 0xfe]));
}

/// Test that CIDs don't decode into byte buffers, lists, etc.
#[test]
fn test_cid_not_as_bytes() {
    let cbor_cid = [
        0xd8, 0x2a, 0x58, 0x25, 0x00, 0x01, 0x55, 0x12, 0x20, 0x2c, 0x26, 0xb4, 0x6b, 0x68, 0xff,
        0xc6, 0x8f, 0xf9, 0x9b, 0x45, 0x3c, 0x1d, 0x30, 0x41, 0x34, 0x13, 0x42, 0x2d, 0x70, 0x64,
        0x83, 0xbf, 0xa0, 0xf9, 0x8a, 0x5e, 0x88, 0x62, 0x66, 0xe7, 0xae,
    ];
    from_slice::<Vec<u8>>(&cbor_cid).expect_err("shouldn't have parsed a tagged CID as a sequence");
    from_slice::<serde_bytes::ByteBuf>(&cbor_cid)
        .expect_err("shouldn't have parsed a tagged CID as a byte array");
    from_slice::<serde_bytes::ByteBuf>(&cbor_cid[2..])
        .expect("should have parsed an untagged CID as a byte array");

    #[derive(Deserialize, Serialize, Debug)]
    struct WrappedVec(Vec<u8>);
    from_slice::<WrappedVec>(&cbor_cid)
        .expect_err("shouldn't have parsed a tagged CID as a newtype sequence");
}

/// Test whether a binary CID could be deserialized if it isn't prefixed by tag 42. It should fail.
#[test]
fn test_cid_bytes_without_tag() {
    let cbor_cid = [
        0xd8, 0x2a, 0x58, 0x25, 0x00, 0x01, 0x55, 0x12, 0x20, 0x2c, 0x26, 0xb4, 0x6b, 0x68, 0xff,
        0xc6, 0x8f, 0xf9, 0x9b, 0x45, 0x3c, 0x1d, 0x30, 0x41, 0x34, 0x13, 0x42, 0x2d, 0x70, 0x64,
        0x83, 0xbf, 0xa0, 0xf9, 0x8a, 0x5e, 0x88, 0x62, 0x66, 0xe7, 0xae,
    ];
    let decoded_cbor_cid: Cid = from_slice(&cbor_cid).unwrap();
    assert_eq!(decoded_cbor_cid.to_bytes(), &cbor_cid[5..]);

    // The CID without the tag 42 prefix
    let cbor_bytes = &cbor_cid[2..];
    from_slice::<Cid>(&cbor_bytes).expect_err("should have failed to decode bytes as cid");

    // And it won't work with a newtype CID.

    #[derive(Deserialize, Serialize, Debug)]
    struct WrappedCid(Cid);
    from_slice::<WrappedCid>(&cbor_bytes)
        .expect_err("should have failed to decode bytes as wrapped cid");
}

#[test]
fn test_cid_in_untagged_union() {
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum Untagged {
        Bytes(#[serde(with = "serde_bytes")] Vec<u8>),
        Link(Cid),
    }

    let cbor_cid = [
        0xd8, 0x2a, 0x58, 0x25, 0x00, 0x01, 0x55, 0x12, 0x20, 0x2c, 0x26, 0xb4, 0x6b, 0x68, 0xff,
        0xc6, 0x8f, 0xf9, 0x9b, 0x45, 0x3c, 0x1d, 0x30, 0x41, 0x34, 0x13, 0x42, 0x2d, 0x70, 0x64,
        0x83, 0xbf, 0xa0, 0xf9, 0x8a, 0x5e, 0x88, 0x62, 0x66, 0xe7, 0xae,
    ];

    let decoded_cid: Untagged = from_slice(&cbor_cid).unwrap();
    let cid = Cid::try_from(&cbor_cid[5..]).unwrap();
    assert_eq!(decoded_cid, Untagged::Link(cid));

    // The CID without the tag 42 prefix
    let cbor_bytes = &cbor_cid[2..];
    let decoded_bytes: Untagged = from_slice(&cbor_bytes).unwrap();
    // The CBOR decoded bytes don't contain the prefix with the bytes type identifier and the
    // length.
    let bytes = cbor_bytes[2..].to_vec();
    assert_eq!(decoded_bytes, Untagged::Bytes(bytes));
}
