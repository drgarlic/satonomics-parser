use bincode::{
    de::{BorrowDecoder, Decoder},
    enc::Encoder,
    error::{DecodeError, EncodeError},
    BorrowDecode, Decode, Encode,
};
use derive_deref::{Deref, DerefMut};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Deref, DerefMut, Debug, Serialize, Deserialize)]
pub struct WMutex<T>(Mutex<T>);

impl<T> WMutex<T> {
    pub fn new(value: T) -> Self {
        Self(Mutex::new(value))
    }
}

impl<T> Encode for WMutex<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.lock().encode(encoder)
    }
}

impl<T> Decode for WMutex<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> core::result::Result<Self, DecodeError> {
        let value: T = Decode::decode(decoder)?;

        Ok(Self(Mutex::new(value)))
    }
}

impl<'de, T> BorrowDecode<'de> for WMutex<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let value: T = BorrowDecode::borrow_decode(decoder)?;

        Ok(Self(Mutex::new(value)))
    }
}
