// Ex02 -- dyn dispatch: Set intersection
pub trait DynIntersect {
    fn intersection_dyn(&self) -> usize;
}

pub struct DynIntersectU8<'a> {
    pub a: &'a [u8],
    pub b: &'a [u8],
}

impl DynIntersect for DynIntersectU8<'_> {
    #[inline(never)]
    fn intersection_dyn(&self) -> usize {
        let mut present = [false; 256];
        for &v in self.a {
            present[v as usize] = true;
        }
        self.b.iter().filter(|&&v| present[v as usize]).count()
    }
}

pub fn intersect_dyn(d: &dyn DynIntersect) -> usize {
    d.intersection_dyn()
}
