// Ex01 -- dyn dispatch: Histogram
pub trait DynHistogram {
    fn max_freq_dyn(&self) -> u32;
}

pub struct DynHistU8<'a> {
    pub data: &'a [u8],
}

impl DynHistogram for DynHistU8<'_> {
    #[inline(never)]
    fn max_freq_dyn(&self) -> u32 {
        let mut counts = [0u32; 256];
        for &b in self.data {
            counts[b as usize] += 1;
        }
        counts.iter().copied().max().unwrap_or(0)
    }
}

pub fn max_freq_dyn(d: &dyn DynHistogram) -> u32 {
    d.max_freq_dyn()
}
