#[derive(Default, Debug)]
pub struct AddressRealizedData {
    pub received: u64,
    pub sent: u64,
    pub profit: f32,
    pub loss: f32,
}
