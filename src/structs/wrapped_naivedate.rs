use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use bincode::{
    de::{BorrowDecoder, Decoder},
    enc::Encoder,
    error::{DecodeError, EncodeError},
    BorrowDecode, Decode, Encode,
};
use chrono::NaiveDate;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct WNaiveDate(NaiveDate);

impl WNaiveDate {
    pub fn wrap(date: NaiveDate) -> Self {
        Self(date)
    }
}

impl Encode for WNaiveDate {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Encode::encode(&self.to_string(), encoder)
    }
}

impl Decode for WNaiveDate {
    fn decode<D: Decoder>(decoder: &mut D) -> core::result::Result<Self, DecodeError> {
        let str: String = Decode::decode(decoder)?;

        Ok(Self(NaiveDate::from_str(&str).unwrap()))
    }
}

impl<'de> BorrowDecode<'de> for WNaiveDate {
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        Ok(Self(
            NaiveDate::from_str(BorrowDecode::borrow_decode(decoder)?).unwrap(),
        ))
    }
}

impl Deref for WNaiveDate {
    type Target = NaiveDate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WNaiveDate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
