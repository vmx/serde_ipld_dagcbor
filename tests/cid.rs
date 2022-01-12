#![feature(core_intrinsics)]
use std::convert::TryFrom;
use std::str::FromStr;

use cid::Cid;
use libipld_core::ipld::Ipld;
use serde_cbor::{from_slice, to_vec};
use serde_derive::{Deserialize, Serialize};

#[test]
fn test_cid_struct() {
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct MyStruct {
        cid: Cid,
        data: bool,
    }

    let cid = Cid::from_str("bafkreibme22gw2h7y2h7tg2fhqotaqjucnbc24deqo72b6mkl2egezxhvy").unwrap();
    let cid_encoded = to_vec(&cid).unwrap();
    println!("vmx: cid_encoded: {:02X?}", cid_encoded);
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
    println!("vmx: cid decoded as ipld: {:#?}", cid_decoded_as_ipld);
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
}

/// Test whether a binary CID could be serialized if it isn't prefixed by tag 42. It should fail.
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

#[test]
fn test_cid_in_untagged_union_with_newtype() {
    //#[derive(Debug, Deserialize, PartialEq, Serialize)]
    //pub struct Foo(#[serde(with = "serde_bytes")] Vec<u8>);
    pub struct Foo(Vec<u8>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Foo {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Foo(ref __self_0_0) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Foo");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Foo {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Foo>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Foo;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "tuple struct Foo")
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: Vec<u8> = match serde_bytes::deserialize(__e) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(Foo(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match {
                            struct __DeserializeWith<'de> {
                                value: Vec<u8>,
                                phantom: _serde::__private::PhantomData<Foo>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::__private::Ok(__DeserializeWith {
                                        value: match serde_bytes::deserialize(__deserializer) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                        phantom: _serde::__private::PhantomData,
                                        lifetime: _serde::__private::PhantomData,
                                    })
                                }
                            }
                            _serde::__private::Option::map(
                                match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                },
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"tuple struct Foo with 1 element",
                                ));
                            }
                        };
                        _serde::__private::Ok(Foo(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "Foo",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Foo>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Foo {
        #[inline]
        fn eq(&self, other: &Foo) -> bool {
            match *other {
                Foo(ref __self_1_0) => match *self {
                    Foo(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Foo) -> bool {
            match *other {
                Foo(ref __self_1_0) => match *self {
                    Foo(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Foo {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(__serializer, "Foo", {
                    struct __SerializeWith<'__a> {
                        values: (&'__a Vec<u8>,),
                        phantom: _serde::__private::PhantomData<Foo>,
                    }
                    impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                        fn serialize<__S>(
                            &self,
                            __s: __S,
                        ) -> _serde::__private::Result<__S::Ok, __S::Error>
                        where
                            __S: _serde::Serializer,
                        {
                            serde_bytes::serialize(self.values.0, __s)
                        }
                    }
                    &__SerializeWith {
                        values: (&self.0,),
                        phantom: _serde::__private::PhantomData::<Foo>,
                    }
                })
            }
        }
    };



    //#[derive(Debug, Deserialize, PartialEq, Serialize)]
    //#[serde(untagged)]
    pub enum Untagged {
        MyBytes(Foo),
        Link(Cid),
    }

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Untagged {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Untagged::MyBytes(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "MyBytes");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Untagged::Link(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Link");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Untagged {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                let __content =
                    match <_serde::__private::de::Content as _serde::Deserialize>::deserialize(
                        __deserializer,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                //if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
                //    <Foo as _serde::Deserialize>::deserialize(
                //        _serde::__private::de::ContentRefDeserializer::<__D::Error>::new(
                //            &__content,
                //        ),
                //    ),
                //    Untagged::MyBytes,
                //) {
                //    return _serde::__private::Ok(__ok);
                //}
                //if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
                //    <Cid as _serde::Deserialize>::deserialize(
                //        _serde::__private::de::ContentRefDeserializer::<__D::Error>::new(
                //            &__content,
                //        ),
                //    ),
                //    Untagged::Link,
                //) {
                //    return _serde::__private::Ok(__ok);
                //}
                let deserializer =  _serde::__private::de::ContentRefDeserializer::<__D::Error>::new(
                            &__content,
                );
                let deserialized = <Foo as _serde::Deserialize>::deserialize(deserializer);
                if let Ok(bla) = deserialized.map(Untagged::MyBytes) {
                    println!("vmx: tests cid: interesting");
                }

                if let Ok(__ok) = Result::map(
                    <Foo as _serde::Deserialize>::deserialize(
                        _serde::__private::de::ContentRefDeserializer::<__D::Error>::new(
                            &__content,
                        ),
                    ),
                    Untagged::MyBytes,
                ) {
                    return _serde::__private::Ok(__ok);
                }
                if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
                    <Cid as _serde::Deserialize>::deserialize(
                        _serde::__private::de::ContentRefDeserializer::<__D::Error>::new(
                            &__content,
                        ),
                    ),
                    Untagged::Link,
                ) {
                    return _serde::__private::Ok(__ok);
                }
                _serde::__private::Err(_serde::de::Error::custom(
                    "data did not match any variant of untagged enum Untagged",
                ))
            }
        }
    };
    //impl core::marker::StructuralPartialEq for Untagged {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Untagged {
        #[inline]
        fn eq(&self, other: &Untagged) -> bool {
            {
                let __self_vi = core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Untagged::MyBytes(ref __self_0), &Untagged::MyBytes(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Untagged::Link(ref __self_0), &Untagged::Link(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
                } else {
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, other: &Untagged) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Untagged::MyBytes(ref __self_0), &Untagged::MyBytes(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Untagged::Link(ref __self_0), &Untagged::Link(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
                } else {
                    true
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Untagged {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Untagged::MyBytes(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                    Untagged::Link(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                }
            }
        }
    };


    let cbor_cid = [
        0xd8, 0x2a, 0x58, 0x25, 0x00, 0x01, 0x55, 0x12, 0x20, 0x2c, 0x26, 0xb4, 0x6b, 0x68, 0xff,
        0xc6, 0x8f, 0xf9, 0x9b, 0x45, 0x3c, 0x1d, 0x30, 0x41, 0x34, 0x13, 0x42, 0x2d, 0x70, 0x64,
        0x83, 0xbf, 0xa0, 0xf9, 0x8a, 0x5e, 0x88, 0x62, 0x66, 0xe7, 0xae,
    ];

    let decoded_cid: Untagged = from_slice(&cbor_cid).unwrap();
    let cid = Cid::try_from(&cbor_cid[5..]).unwrap();
    assert_eq!(decoded_cid, Untagged::Link(cid));

    //// The CID without the tag 42 prefix
    //let cbor_bytes = &cbor_cid[2..];
    //let decoded_bytes: Untagged = from_slice(&cbor_bytes).unwrap();
    //// The CBOR decoded bytes don't contain the prefix with the bytes type identifier and the
    //// length.
    //let bytes = cbor_bytes[2..].to_vec();
    //assert_eq!(decoded_bytes, Untagged::Bytes(Foo(bytes)));
}
