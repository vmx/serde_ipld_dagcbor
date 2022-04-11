//! This module contains code that was just copied from cbor4ii.
//!
//! Some things needed for the Serde implementation are not public in the cbor4ii crate. Those are
//! copied into this file.

use core::ops::{Deref, DerefMut};

use cbor4ii::core::dec;

use crate::error::DecodeError;

// Copy from cbor4ii/core.rs.
#[allow(dead_code)]
pub(crate) mod marker {
    pub const START: u8 = 0x1f;
    pub const FALSE: u8 = 0xf4; // simple(20)
    pub const TRUE: u8 = 0xf5; // simple(21)
    pub const NULL: u8 = 0xf6; // simple(22)
    pub const UNDEFINED: u8 = 0xf7; // simple(23)
    pub const F16: u8 = 0xf9;
    pub const F32: u8 = 0xfa;
    pub const F64: u8 = 0xfb;
    pub const BREAK: u8 = 0xff;
}

// Copy from cbor4ii/core/dec.rs.
#[inline]
pub(crate) fn peek_one<'a, R: dec::Read<'a>>(reader: &mut R) -> Result<u8, DecodeError<R::Error>> {
    let buf = match reader.fill(1)? {
        dec::Reference::Long(buf) => buf,
        dec::Reference::Short(buf) => buf,
    };
    let byte = buf.get(0).copied().ok_or(DecodeError::Eof)?;
    Ok(byte)
}

// Copy from cbor4ii/core/dec.rs.
#[inline]
pub(crate) fn pull_one<'a, R: dec::Read<'a>>(reader: &mut R) -> Result<u8, DecodeError<R::Error>> {
    let byte = peek_one(reader)?;
    reader.advance(1);
    Ok(byte)
}

// Copy from cbor4ii/util.rs.
/// Executes the given function when the variables moves out of scope.
pub(crate) struct ScopeGuard<'a, T>(pub &'a mut T, pub fn(&mut T));

impl<T> Deref for ScopeGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<T> DerefMut for ScopeGuard<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl<T> Drop for ScopeGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        (self.1)(self.0);
    }
}
