use bincode::{
    de::{BorrowDecoder, Decoder},
    enc::Encoder,
    error::{DecodeError, EncodeError},
    BorrowDecode, Decode, Encode,
};
use bitcoin::Txid;
use bitcoin_hashes::{sha256d::Hash, Hash as HashTrait};
use derive_deref::{Deref, DerefMut};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deref, DerefMut)]
pub struct WTxid(Txid);

impl WTxid {
    pub fn wrap(txid: Txid) -> Self {
        Self(txid)
    }

    fn from_vec(slice: Vec<u8>) -> Self {
        Self(Txid::from_raw_hash(Hash::from_slice(&slice).unwrap()))
    }

    fn from_slice(slice: &[u8]) -> Self {
        Self(Txid::from_raw_hash(Hash::from_slice(slice).unwrap()))
    }
}

impl Encode for WTxid {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Encode::encode(&self.to_byte_array(), encoder)
    }
}

impl Decode for WTxid {
    fn decode<D: Decoder>(decoder: &mut D) -> core::result::Result<Self, DecodeError> {
        Ok(Self::from_vec(Decode::decode(decoder)?))
    }
}

impl<'de> BorrowDecode<'de> for WTxid {
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        Ok(Self::from_slice(BorrowDecode::borrow_decode(decoder)?))
    }
}
