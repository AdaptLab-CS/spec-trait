// Ex07 -- enum dispatch: hash_key

pub enum HashEnum<'a> {
    U64(&'a [u64]),
}

#[inline(never)]
pub fn hash_all_enum(e: &HashEnum<'_>) -> u64 {
    match e {
        HashEnum::U64(keys) => {
            let mut acc: u64 = 0;
            for &k in *keys {
                acc ^= k;
            }
            acc
        }
    }
}
