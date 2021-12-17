use std::str::FromStr;

use cid::Cid;
use serde_cbor::value::Value;
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

    let cid_decoded_as_value: Value = from_slice(&cid_encoded).unwrap();
    assert_eq!(cid_decoded_as_value, Value::Cid(cid));

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

    let mystruct_decoded_as_value: Value = from_slice(&mystruct_encoded).unwrap();
    let mut expected_map = std::collections::BTreeMap::new();
    expected_map.insert(Value::Text("cid".to_string()), Value::Cid(cid));
    expected_map.insert(Value::Text("data".to_string()), Value::Bool(true));
    assert_eq!(mystruct_decoded_as_value, Value::Map(expected_map));
}

/// Test that arbitrary bytes are not interpreted as CID.
#[test]
fn test_binary_not_as_cid() {
    // h'affe'
    // 42      # bytes(2)
    //    AFFE # "\xAF\xFE"
    let bytes = [0x42, 0xaf, 0xfe];
    let bytes_as_value: Value = from_slice(&bytes).unwrap();
    assert_eq!(bytes_as_value, Value::Bytes(vec![0xaf, 0xfe]));
}
