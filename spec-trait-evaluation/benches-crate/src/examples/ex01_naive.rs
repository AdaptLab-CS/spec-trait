// Ex01 -- naive: extend_from_slice — clone loop (no specialization)

pub struct Extender;

pub trait NaiveExtendTrait<T> {
    fn extend_slice(&self, dst: &mut Vec<T>, src: &[T]);
}

impl<T: Clone> NaiveExtendTrait<T> for Extender {
    #[inline(never)]
    fn extend_slice(&self, dst: &mut Vec<T>, src: &[T]) {
        for item in src {
            dst.push(item.clone());
        }
    }
}

pub fn extend_naive(e: &Extender, dst: &mut Vec<u8>, src: &[u8]) {
    e.extend_slice(dst, src);
}
