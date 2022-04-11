//! Deserialization.
use std::collections::TryReserveError;
use std::io;

use cbor4ii::core::enc::{self, Encode};
use cbor4ii::core::types;
pub use cbor4ii::core::utils::BufWriter;
use core::fmt;
use serde::{ser, Serialize};
//use cbor4ii::core::{types, enc::{ self, Encode }};
//use cbor4ii::serde::ser::{BoundedCollect, Collect};
use crate::CBOR_TAGS_CID;
use cbor4ii::serde::Serializer as Cbor4iiSerializer;
use cid::serde::CID_SERDE_PRIVATE_IDENTIFIER;

use delegate_attr::delegate;

/// Serializes a value to a vector.
pub fn to_vec<T>(value: &T)
    -> Result<Vec<u8>, enc::Error<TryReserveError>>
where T: Serialize + ?Sized
{
    let writer = BufWriter::new(Vec::new());
    let mut serializer = Serializer::new(writer);
    value.serialize(&mut serializer)?;
    Ok(serializer.into_inner().into_inner())
}
//pub fn to_vec<T>(buf: Vec<u8>, value: &T)
//    -> Result<Vec<u8>, enc::Error<TryReserveError>>
//where T: Serialize + ?Sized
//{
//    let writer = BufWriter::new(buf);
//    let mut writer = Serializer::new(writer);
//    value.serialize(&mut writer)?;
//    Ok(writer.into_inner().into_inner())
//}

struct IoWrite<W>(W);

impl<W: io::Write> enc::Write for IoWrite<W> {
    type Error = io::Error;

    #[inline]
    fn push(&mut self, input: &[u8]) -> Result<(), Self::Error> {
        self.0.write_all(input)
    }
}

/// Serializes a value to a writer.
pub fn to_writer<W, T>(writer: W, value: &T)
    -> Result<(), enc::Error<io::Error>>
where
    W: io::Write,
    T: Serialize
{
    let mut serializer = Serializer::new(IoWrite(writer));
    value.serialize(&mut serializer)
}



pub struct Serializer<W>(Cbor4iiSerializer<W>);

impl<W> Serializer<W> {
    pub fn new(writer: W) -> Serializer<W> {
        Serializer(Cbor4iiSerializer::new(writer))
    }

    #[delegate(self.0)]
    pub fn into_inner(self) -> W;
}

impl<'a, W: enc::Write> ser::Serializer for &'a mut Serializer<W> {
    type Ok = <&'a mut Cbor4iiSerializer<W> as ser::Serializer>::Ok;
    type Error = <&'a mut Cbor4iiSerializer<W> as ser::Serializer>::Error;

    type SerializeSeq = <&'a mut Cbor4iiSerializer<W> as ser::Serializer>::SerializeSeq;
    type SerializeTuple = <&'a mut Cbor4iiSerializer<W> as ser::Serializer>::SerializeTuple;
    type SerializeTupleStruct =
        <&'a mut Cbor4iiSerializer<W> as ser::Serializer>::SerializeTupleStruct;
    type SerializeTupleVariant =
        <&'a mut Cbor4iiSerializer<W> as ser::Serializer>::SerializeTupleVariant;
    type SerializeMap = <&'a mut Cbor4iiSerializer<W> as ser::Serializer>::SerializeMap;
    type SerializeStruct = <&'a mut Cbor4iiSerializer<W> as ser::Serializer>::SerializeStruct;
    type SerializeStructVariant =
        <&'a mut Cbor4iiSerializer<W> as ser::Serializer>::SerializeStructVariant;

    #[delegate(self.0)]
    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error>;

    #[delegate(self.0)]
    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error>;

    #[delegate(self.0)]
    #[inline]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error>;

    #[delegate(self.0)]
    #[inline]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error>;

    #[delegate(self.0)]
    #[inline]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error>;

    #[delegate(self.0)]
    #[inline]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error>;

    #[delegate(self.0)]
    #[inline]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error>;

    #[delegate(self.0)]
    #[inline]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error>;

    #[delegate(self.0)]
    #[inline]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error>;

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        // In DAG-CBOR floats are always encoded as f64.
        self.0.serialize_f64(f64::from(v))
    }

    #[delegate(self.0)]
    #[inline]
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error>;

    #[delegate(self.0)]
    #[inline]
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error>;

    #[delegate(self.0)]
    #[inline]
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error>;

    #[delegate(self.0)]
    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error>;

    #[delegate(self.0)]
    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error>;

    #[delegate(self.0)]
    #[inline]
    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>;

    #[delegate(self.0)]
    #[inline]
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        // The cbor4ii Serde implementation encodes unit as an empty array, for DAG-CBOR we encode
        // it as `NULL`.
        types::Null.encode(&mut self.writer)?;
        Ok(())
    }

    #[delegate(self.0)]
    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error>;

    #[delegate(self.0)]
    #[inline]
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error>;

    #[inline]
    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        if name == CID_SERDE_PRIVATE_IDENTIFIER {
            value.serialize(&mut CidSerializer(self))
        } else {
            value.serialize(self)
        }
    }

    #[delegate(self.0)]
    #[inline]
    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>;

    #[delegate(self.0)]
    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error>;

    #[delegate(self.0)]
    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error>;

    #[delegate(self.0)]
    #[inline]
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error>;

    #[delegate(self.0)]
    #[inline]
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error>;

    #[delegate(self.0)]
    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error>;

    #[delegate(self.0)]
    #[inline]
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error>;

    #[delegate(self.0)]
    #[inline]
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error>;

    #[delegate(self.0)]
    #[inline]
    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error>;

    #[delegate(self.0)]
    #[inline]
    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error>;

    // forward to Cbor4iiSerializer

    fn collect_map<K, V, I>(self, iter: I) -> Result<(), Self::Error>
    where
       K: ser::Serialize,
       V: ser::Serialize,
       I: IntoIterator<Item = (K, V)>,
    {
       use serde::ser::SerializeMap;

       // CBOR RFC-7049 specifies a canonical sort order, where keys are sorted by length first.
       // This was later revised with RFC-8949, but we need to stick to the original order to stay
       // compatible with existing data.
       // We first serialize each map entry into a buffer and then sort those buffers. Byte-wise
       // comparison gives us the right order as keys in DAG-CBOR are always strings and prefixed
       // with the length. Once sorted they are written to the actual output.
       let mut buffer = BufWriter::new(Vec::new());
       let mut mem_serializer = Serializer::new(&mut buffer);
       let mut serializer = Self::SerializeMap {
          bounded: true,
          ser: &mut mem_serializer.0,
       };
       //let mut entries = Vec::new();
       //for (key, value) in iter {
       //   serializer.serialize_entry(&key, &value)
       //      .map_err(|_| enc::Error::Msg("Map entry cannot be serialized.".into()))?;
       //   entries.push(serializer.0.ser.writer.buffer().to_vec());
       //   serializer.0.ser.writer.clear();
       //}
       //
       //enc::MapStartBounded(entries.len()).encode(&mut self.writer)?;
       //entries.sort_unstable();
       //for entry in entries {
       //   self.writer.push(&entry)?;
       //}

       Ok(())
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        false
    }
}

/// Serializing a CID correctly as DAG-CBOR.
struct CidSerializer<'a, W>(&'a mut Serializer<W>);

impl<'a, W: enc::Write> ser::Serializer for &'a mut CidSerializer<'a, W>
where
    W::Error: core::fmt::Debug,
{
    type Ok = ();
    type Error = enc::Error<W::Error>;

    type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeMap = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _value: bool) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_i8(self, _value: i8) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_i16(self, _value: i16) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_i32(self, _value: i32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_i64(self, _value: i64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_u8(self, _value: u8) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_u16(self, _value: u16) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_u32(self, _value: u32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_u64(self, _value: u64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_f32(self, _value: f32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_f64(self, _value: f64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_char(self, _value: char) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_str(self, _value: &str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        //// The bytes of the CID is prefixed with a null byte when encoded as CBOR.
        //let prefixed = [&[0x00], value].concat();
        //// CIDs are serialized with CBOR tag 42.
        ////types::Tag(CBOR_TAGS_CID.into(), types::Bytes(&prefixed[..])).encode(&mut self.0.0.writer)?;
        //types::Tag(CBOR_TAGS_CID.into(), types::Bytes(&prefixed[..])).encode(self.0 .0.writer())?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_some<T: ?Sized + ser::Serialize>(
        self,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_unit_struct(self, _name: &str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_unit_variant(
        self,
        _name: &str,
        _variant_index: u32,
        _variant: &str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }

    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(
        self,
        _name: &str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(
        self,
        _name: &str,
        _variant_index: u32,
        _variant: &str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_tuple_struct(
        self,
        _name: &str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_tuple_variant(
        self,
        _name: &str,
        _variant_index: u32,
        _variant: &str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_struct(
        self,
        _name: &str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_struct_variant(
        self,
        _name: &str,
        _variant_index: u32,
        _variant: &str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
}

//pub struct Serializer<W> {
//    writer: W
//}
//
//impl<W> Serializer<W> {
//    pub fn new(writer: W) -> Serializer<W> {
//        Serializer { writer }
//    }
//
//    pub fn into_inner(self) -> W {
//        self.writer
//    }
//}

//impl<'a, W: enc::Write> serde::Serializer for &'a mut Serializer<W> {
//    type Ok = ();
//    type Error = enc::Error<W::Error>;
//
//    type SerializeSeq = Collect<'a, W>;
//    type SerializeTuple = BoundedCollect<'a, W>;
//    type SerializeTupleStruct = BoundedCollect<'a, W>;
//    type SerializeTupleVariant = BoundedCollect<'a, W>;
//    type SerializeMap = Collect<'a, W>;
//    type SerializeStruct = BoundedCollect<'a, W>;
//    type SerializeStructVariant = BoundedCollect<'a, W>;
//
//    #[inline]
//    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
//        v.encode(&mut self.writer)?;
//        Ok(())
//    }
//
//    #[inline]
//    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
//        v.encode(&mut self.writer)?;
//        Ok(())
//    }
//
//    #[inline]
//    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
//        v.encode(&mut self.writer)?;
//        Ok(())
//    }
//
//    #[inline]
//    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
//        v.encode(&mut self.writer)?;
//        Ok(())
//    }
//
//    #[inline]
//    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
//        v.encode(&mut self.writer)?;
//        Ok(())
//    }
//
//    #[inline]
//    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
//        v.encode(&mut self.writer)?;
//        Ok(())
//    }
//
//    #[inline]
//    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
//        v.encode(&mut self.writer)?;
//        Ok(())
//    }
//
//    #[inline]
//    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
//        v.encode(&mut self.writer)?;
//        Ok(())
//    }
//
//    #[inline]
//    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
//        v.encode(&mut self.writer)?;
//        Ok(())
//    }
//
//    #[inline]
//    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
//        // In DAG-CBOR floats are always encoded as f64.
//        f64::from(v).encode(&mut self.writer)?;
//        Ok(())
//    }
//
//    #[inline]
//    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
//        v.encode(&mut self.writer)?;
//        Ok(())
//    }
//
//    #[inline]
//    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
//        let mut buf = [0; 4];
//        self.serialize_str(v.encode_utf8(&mut buf))
//    }
//
//    #[inline]
//    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
//        v.encode(&mut self.writer)?;
//        Ok(())
//    }
//
//    #[inline]
//    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
//        types::Bytes(v).encode(&mut self.writer)?;
//        Ok(())
//    }
//
//    #[inline]
//    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
//        types::Null.encode(&mut self.writer)?;
//        Ok(())
//    }
//
//    #[inline]
//    fn serialize_some<T: Serialize + ?Sized>(self, value: &T)
//        -> Result<Self::Ok, Self::Error>
//    {
//        value.serialize(self)
//    }
//
//    #[inline]
//    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
//        // The cbor4ii Serde implementation encodes unit as an empty array, for DAG-CBOR we encode
//        // it as `NULL`.
//        types::Null.encode(&mut self.writer)?;
//        Ok(())
//    }
//
//    #[inline]
//    fn serialize_unit_struct(self, _name: &'static str)
//        -> Result<Self::Ok, Self::Error>
//    {
//        self.serialize_unit()
//    }
//
//    #[inline]
//    fn serialize_unit_variant(
//        self,
//        _name: &'static str,
//        _variant_index: u32,
//        variant: &'static str
//    ) -> Result<Self::Ok, Self::Error> {
//        self.serialize_str(variant)
//    }
//
//    #[inline]
//    fn serialize_newtype_struct<T: Serialize + ?Sized>(
//        self,
//        name: &'static str,
//        value: &T
//    ) -> Result<Self::Ok, Self::Error> {
//        if name == CID_SERDE_PRIVATE_IDENTIFIER {
//            value.serialize(&mut CidSerializer(self))
//        } else {
//            value.serialize(self)
//        }
//    }
//
//    #[inline]
//    fn serialize_newtype_variant<T: Serialize + ?Sized>(
//        self,
//        _name: &'static str,
//        _variant_index: u32,
//        variant: &'static str,
//        value: &T
//    ) -> Result<Self::Ok, Self::Error> {
//        enc::MapStartBounded(1).encode(&mut self.writer)?;
//        variant.encode(&mut self.writer)?;
//        value.serialize(self)
//    }
//
//    #[inline]
//    fn serialize_seq(self, len: Option<usize>)
//        -> Result<Self::SerializeSeq, Self::Error>
//    {
//        if let Some(len) = len {
//            enc::ArrayStartBounded(len).encode(&mut self.writer)?;
//        } else {
//            enc::ArrayStartUnbounded.encode(&mut self.writer)?;
//        }
//        Ok(Collect {
//            bounded: len.is_some(),
//            ser: self
//        })
//    }
//
//    #[inline]
//    fn serialize_tuple(self, len: usize)
//        -> Result<Self::SerializeTuple, Self::Error>
//    {
//        enc::ArrayStartBounded(len).encode(&mut self.writer)?;
//        Ok(BoundedCollect { ser: self })
//    }
//
//    #[inline]
//    fn serialize_tuple_struct(self, _name: &'static str, len: usize)
//        -> Result<Self::SerializeTupleStruct, Self::Error>
//    {
//        self.serialize_tuple(len)
//    }
//
//    #[inline]
//    fn serialize_tuple_variant(
//        self,
//        _name: &'static str,
//        _variant_index: u32,
//        variant: &'static str,
//        len: usize
//    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
//        enc::MapStartBounded(1).encode(&mut self.writer)?;
//        variant.encode(&mut self.writer)?;
//        enc::ArrayStartBounded(len).encode(&mut self.writer)?;
//        Ok(BoundedCollect { ser: self })
//    }
//
//    #[inline]
//    fn serialize_map(self, len: Option<usize>)
//        -> Result<Self::SerializeMap, Self::Error>
//    {
//        if let Some(len) = len {
//            enc::MapStartBounded(len).encode(&mut self.writer)?;
//        } else {
//            enc::MapStartUnbounded.encode(&mut self.writer)?;
//        }
//        Ok(Collect {
//            bounded: len.is_some(),
//            ser: self
//        })
//    }
//
//    #[inline]
//    fn serialize_struct(self, _name: &'static str, len: usize)
//        -> Result<Self::SerializeStruct, Self::Error>
//    {
//        enc::MapStartBounded(len).encode(&mut self.writer)?;
//        Ok(BoundedCollect { ser: self })
//    }
//
//    #[inline]
//    fn serialize_struct_variant(
//        self,
//        _name: &'static str,
//        _variant_index: u32,
//        variant: &'static str,
//        len: usize
//    ) -> Result<Self::SerializeStructVariant, Self::Error> {
//        enc::MapStartBounded(1).encode(&mut self.writer)?;
//        variant.encode(&mut self.writer)?;
//        enc::MapStartBounded(len).encode(&mut self.writer)?;
//        Ok(BoundedCollect { ser: self })
//    }
//
//    #[inline]
//    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
//        v.encode(&mut self.writer)?;
//        Ok(())
//    }
//
//    #[inline]
//    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
//        v.encode(&mut self.writer)?;
//        Ok(())
//    }
//
//    #[inline]
//    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
//    where
//        T: fmt::Display,
//    {
//        use core::fmt::Write;
//        use serde::ser::Error;
//
//        let mut writer = FmtWriter::new(&mut self.writer);
//
//        if let Err(err) = write!(&mut writer, "{}", value) {
//            if !writer.is_error() {
//                return Err(enc::Error::custom(err));
//            }
//        }
//
//        writer.flush()
//    }
//
//    fn collect_map<K, V, I>(self, iter: I) -> Result<(), Self::Error>
//    where
//        K: ser::Serialize,
//        V: ser::Serialize,
//        I: IntoIterator<Item = (K, V)>,
//    {
//        use serde::ser::SerializeMap;
//        #[cfg(not(feature = "use_std"))]
//        use crate::alloc::vec::Vec;
//        use crate::core::utils::BufWriter;
//
//        // CBOR RFC-7049 specifies a canonical sort order, where keys are sorted by length first.
//        // This was later revised with RFC-8949, but we need to stick to the original order to stay
//        // compatible with existing data.
//        // We first serialize each map entry into a buffer and then sort those buffers. Byte-wise
//        // comparison gives us the right order as keys in DAG-CBOR are always strings and prefixed
//        // with the length. Once sorted they are written to the actual output.
//        let mut buffer = BufWriter::new(Vec::new());
//        let mut mem_serializer = Serializer::new(&mut buffer);
//        let mut serializer = Collect {
//            bounded: true,
//            ser: &mut mem_serializer,
//        };
//        let mut entries = Vec::new();
//        for (key, value) in iter {
//            serializer.serialize_entry(&key, &value)
//               .map_err(|_| enc::Error::Msg("Map entry cannot be serialized.".into()))?;
//            entries.push(serializer.ser.writer.buffer().to_vec());
//            serializer.ser.writer.clear();
//        }
//
//        enc::MapStartBounded(entries.len()).encode(&mut self.writer)?;
//        entries.sort_unstable();
//        for entry in entries {
//            self.writer.push(&entry)?;
//        }
//
//        Ok(())
//    }
//
//    #[inline]
//    fn is_human_readable(&self) -> bool {
//        false
//    }
//}
//
