#[cfg(test)]
mod change_cipher_spec_test;

use std::io::{Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};

use util::Error;

use super::content::*;
use super::errors::*;

// The change cipher spec protocol exists to signal transitions in
// ciphering strategies.  The protocol consists of a single message,
// which is encrypted and compressed under the current (not the pending)
// connection state.  The message consists of a single byte of value 1.
// https://tools.ietf.org/html/rfc5246#section-7.1
#[derive(Clone, PartialEq, Debug)]
pub struct ChangeCipherSpec;

impl ChangeCipherSpec {
    pub fn content_type(&self) -> ContentType {
        ContentType::ChangeCipherSpec
    }

    pub fn marshal<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_u8(0x01)?;

        Ok(())
    }

    pub fn unmarshal<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let data = reader.read_u8()?;
        if data != 0x01 {
            return Err(ERR_INVALID_CIPHER_SPEC.clone());
        }

        Ok(ChangeCipherSpec {})
    }
}
