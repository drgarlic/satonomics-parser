use super::SATOSHIS_PER_BITCOIN;

#[allow(unused)]
#[inline(always)]
pub fn sats_to_btc(sats: u64) -> f32 {
    sats as f32 / SATOSHIS_PER_BITCOIN as f32
}

#[allow(unused)]
#[inline(always)]
pub fn btc_to_sats(btc: f32) -> u64 {
    (btc * SATOSHIS_PER_BITCOIN as f32) as u64
}
