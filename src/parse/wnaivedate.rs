use std::str::FromStr;

use chrono::NaiveDate;
use derive_deref::{Deref, DerefMut};
use savefile::IsReprC;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Deref,
    DerefMut,
    Default,
    Serialize,
    Deserialize,
)]
pub struct WNaiveDate(NaiveDate);

impl WNaiveDate {
    pub fn wrap(date: NaiveDate) -> Self {
        Self(date)
    }
}

impl savefile::ReprC for WNaiveDate {
    unsafe fn repr_c_optimization_safe(_version: u32) -> savefile::prelude::IsReprC {
        IsReprC::yes()
    }
}

impl savefile::Introspect for WNaiveDate {
    fn introspect_value(&self) -> String {
        self.to_string()
    }

    fn introspect_child(&self, _index: usize) -> Option<Box<dyn savefile::IntrospectItem + '_>> {
        None
    }
}

impl savefile::WithSchema for WNaiveDate {
    fn schema(_: u32) -> savefile::prelude::Schema {
        savefile::Schema::Primitive(savefile::SchemaPrimitive::schema_string)
    }
}

impl savefile::Serialize for WNaiveDate {
    fn serialize(
        &self,
        serializer: &mut savefile::prelude::Serializer<impl std::io::prelude::Write>,
    ) -> Result<(), savefile::prelude::SavefileError> {
        serializer.write_string(&self.to_string())
    }
}

impl savefile::Deserialize for WNaiveDate {
    fn deserialize(
        deserializer: &mut savefile::prelude::Deserializer<impl std::io::prelude::Read>,
    ) -> Result<Self, savefile::prelude::SavefileError> {
        let str = deserializer.read_string()?;

        let date = NaiveDate::from_str(&str).unwrap();

        Ok(WNaiveDate::wrap(date))
    }
}

// impl Encode for WNaiveDate {
//     fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
//         Encode::encode(&self.to_string(), encoder)
//     }
// }

// impl Decode for WNaiveDate {
//     fn decode<D: Decoder>(decoder: &mut D) -> core::result::Result<Self, DecodeError> {
//         let str: String = Decode::decode(decoder)?;

//         Ok(Self(NaiveDate::from_str(&str).unwrap()))
//     }
// }

// impl<'de> BorrowDecode<'de> for WNaiveDate {
//     fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
//         let str: String = BorrowDecode::borrow_decode(decoder)?;

//         Ok(Self(NaiveDate::from_str(&str).unwrap()))
//     }
// }
