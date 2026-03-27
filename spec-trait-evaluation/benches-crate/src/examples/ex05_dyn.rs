// Ex05 -- dyn dispatch: Distinct count
pub trait DynDistinct {
    fn distinct_dyn(&self) -> usize;
}

pub struct DynDistinctU8<'a> {
    pub data: &'a [u8],
}

impl DynDistinct for DynDistinctU8<'_> {
    #[inline(never)]
    fn distinct_dyn(&self) -> usize {
        let mut seen = [false; 256];
        for &b in self.data {
            seen[b as usize] = true;
        }
        seen.iter().filter(|&&v| v).count()
    }
}

pub fn distinct_dyn(d: &dyn DynDistinct) -> usize {
    d.distinct_dyn()
}
