use std::ffi::{c_ulong, c_short, c_uint};

pub mod bind;

pub fn variant_d(idx: u32, shuffle_id: u32, list_size: u32) -> u32 {
    unsafe { bind::MillerShuffleAlgo_d(idx as c_uint, shuffle_id as c_uint, list_size as c_uint) }
}

pub fn variant_e(idx: u32, shuffle_id: u32, list_size: u32) -> u32 {
    unsafe { bind::MillerShuffleAlgo_e(idx as c_uint, shuffle_id as c_uint, list_size as c_uint) }
}

pub fn variant_xlite(idx: i16, mix_id: u64, nlim: i16) -> i16 {
    unsafe { bind::MillerShuffle_xlite(idx as c_short, mix_id as c_ulong, nlim as c_short) }
}

pub fn variant_lite(idx: i16, mix_id: u64, nlim: i16) -> i16 {
    unsafe { bind::MillerShuffle_lite(idx as c_short, mix_id as c_ulong, nlim as c_short) }
}
