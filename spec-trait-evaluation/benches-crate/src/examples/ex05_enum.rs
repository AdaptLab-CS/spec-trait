// Ex05 -- enum dispatch: histogram
use std::collections::HashMap;
use std::hash::Hash;

pub enum HistEnum<'a, T> {
    U8(&'a [u8]),
    Generic(&'a [T]),
}

pub trait EnumHistogramTrait<T> {
    fn max_freq(&self) -> u32;
}

impl<T: Eq + Hash> EnumHistogramTrait<T> for HistEnum<'_, T> {
    #[inline(never)]
    fn max_freq(&self) -> u32 {
        match self {
            HistEnum::U8(data) => {
                let mut counts = [0u32; 256];
                for &b in *data {
                    counts[b as usize] += 1;
                }
                counts.iter().copied().max().unwrap_or(0)
            }
            HistEnum::Generic(data) => {
                let mut map = HashMap::<&T, u32>::new();
                for item in *data {
                    *map.entry(item).or_insert(0) += 1;
                }
                map.values().copied().max().unwrap_or(0)
            }
        }
    }
}

pub fn histogram_enum(e: &HistEnum<u8>) -> u32 {
    e.max_freq()
}
