#!/usr/bin/env python3
"""Generate all 32 benchmark example files for spec-trait-evaluation."""
import os

BASE = os.path.join(os.path.dirname(__file__), "benches-crate", "src", "examples")

FILES = {}

# ============================================================================
# Ex01: extend_from_slice — #[when(T: Copy)] — memcpy vs clone loop
# ============================================================================

FILES["ex01_spec.rs"] = r'''// Ex01 -- spec: extend_from_slice — Copy memcpy vs Clone loop
use spec_trait_macro::{spec, when};

pub struct Extender;

pub trait ExtendTrait<T> {
    fn extend_slice(&self, dst: &mut Vec<T>, src: &[T]);
}

impl<T: Clone> ExtendTrait<T> for Extender {
    #[inline(never)]
    fn extend_slice(&self, dst: &mut Vec<T>, src: &[T]) {
        for item in src {
            dst.push(item.clone());
        }
    }
}

#[when(T: Copy)]
impl<T> ExtendTrait<T> for Extender {
    #[inline(never)]
    fn extend_slice(&self, dst: &mut Vec<T>, src: &[T]) {
        let len = src.len();
        dst.reserve(len);
        unsafe {
            std::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr().add(dst.len()), len);
            dst.set_len(dst.len() + len);
        }
    }
}

pub fn extend_u8(e: &Extender, dst: &mut Vec<u8>, src: &[u8]) {
    spec! { e.extend_slice(dst, src); Extender; [&mut Vec<u8>, &[u8]]; u8: Copy }
}
'''

FILES["ex01_naive.rs"] = r'''// Ex01 -- naive: extend_from_slice — clone loop (no specialization)

#[inline(never)]
pub fn extend_naive<T: Clone>(dst: &mut Vec<T>, src: &[T]) {
    for item in src {
        dst.push(item.clone());
    }
}
'''

FILES["ex01_enum.rs"] = r'''// Ex01 -- enum dispatch: extend_from_slice

pub enum ExtendEnum<'a> {
    U8(&'a [u8]),
}

#[inline(never)]
pub fn extend_enum(e: &ExtendEnum<'_>, dst: &mut Vec<u8>) {
    match e {
        ExtendEnum::U8(src) => {
            let len = src.len();
            dst.reserve(len);
            unsafe {
                std::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr().add(dst.len()), len);
                dst.set_len(dst.len() + len);
            }
        }
    }
}
'''

FILES["ex01_runtime_typeid.rs"] = r'''// Ex01 -- runtime TypeId dispatch: extend_from_slice
use std::any::TypeId;

#[inline(never)]
pub fn extend_runtime<T: Clone + Copy + 'static>(dst: &mut Vec<T>, src: &[T]) {
    if TypeId::of::<T>() == TypeId::of::<u8>() {
        let len = src.len();
        dst.reserve(len);
        unsafe {
            std::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr().add(dst.len()), len);
            dst.set_len(dst.len() + len);
        }
    } else {
        for item in src {
            dst.push(item.clone());
        }
    }
}
'''

# ============================================================================
# Ex02: to_bytes — #[when(T: Copy)] — raw transmute vs Display formatting
# ============================================================================

FILES["ex02_spec.rs"] = r'''// Ex02 -- spec: to_bytes — Copy raw transmute vs Display formatting
use spec_trait_macro::{spec, when};
use std::fmt::Display;

pub struct Serializer;

pub trait ToByteTrait<T> {
    fn to_bytes(&self, data: &[T]) -> Vec<u8>;
}

impl<T: Display> ToByteTrait<T> for Serializer {
    #[inline(never)]
    fn to_bytes(&self, data: &[T]) -> Vec<u8> {
        let mut result = Vec::new();
        for item in data {
            result.extend_from_slice(format!("{}", item).as_bytes());
        }
        result
    }
}

#[when(T: Copy)]
impl<T> ToByteTrait<T> for Serializer {
    #[inline(never)]
    fn to_bytes(&self, data: &[T]) -> Vec<u8> {
        unsafe {
            std::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * std::mem::size_of::<T>(),
            )
            .to_vec()
        }
    }
}

pub fn to_bytes_u8(s: &Serializer, data: &[u8]) -> Vec<u8> {
    spec! { s.to_bytes(data); Serializer; [&[u8]]; u8: Copy }
}
'''

FILES["ex02_naive.rs"] = r'''// Ex02 -- naive: to_bytes — Display formatting (no specialization)
use std::fmt::Display;

#[inline(never)]
pub fn to_bytes_naive<T: Display>(data: &[T]) -> Vec<u8> {
    let mut result = Vec::new();
    for item in data {
        result.extend_from_slice(format!("{}", item).as_bytes());
    }
    result
}
'''

FILES["ex02_enum.rs"] = r'''// Ex02 -- enum dispatch: to_bytes

pub enum ToBytesEnum<'a> {
    U8(&'a [u8]),
}

#[inline(never)]
pub fn to_bytes_enum(e: &ToBytesEnum<'_>) -> Vec<u8> {
    match e {
        ToBytesEnum::U8(data) => {
            data.to_vec()
        }
    }
}
'''

FILES["ex02_runtime_typeid.rs"] = r'''// Ex02 -- runtime TypeId dispatch: to_bytes
use std::any::TypeId;
use std::fmt::Display;

#[inline(never)]
pub fn to_bytes_runtime<T: Display + Copy + 'static>(data: &[T]) -> Vec<u8> {
    if TypeId::of::<T>() == TypeId::of::<u8>() {
        unsafe {
            std::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * std::mem::size_of::<T>(),
            )
            .to_vec()
        }
    } else {
        let mut result = Vec::new();
        for item in data {
            result.extend_from_slice(format!("{}", item).as_bytes());
        }
        result
    }
}
'''

# ============================================================================
# Ex03: contains — #[when(T = u8)] — memchr SIMD vs linear scan
# ============================================================================

FILES["ex03_spec.rs"] = r'''// Ex03 -- spec: contains — memchr SIMD vs linear scan
use spec_trait_macro::{spec, when};

pub struct Searcher;

pub trait ContainsTrait<T> {
    fn contains(&self, data: &[T], needle: &T) -> bool;
}

impl<T: PartialEq> ContainsTrait<T> for Searcher {
    #[inline(never)]
    fn contains(&self, data: &[T], needle: &T) -> bool {
        data.iter().any(|x| x == needle)
    }
}

#[when(T = u8)]
impl<T> ContainsTrait<T> for Searcher {
    #[inline(never)]
    fn contains(&self, data: &[T], needle: &T) -> bool {
        let byte_slice = unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len())
        };
        let n = unsafe { *(needle as *const T as *const u8) };
        memchr::memchr(n, byte_slice).is_some()
    }
}

pub fn contains_u8(s: &Searcher, data: &[u8], needle: &u8) -> bool {
    spec! { s.contains(data, needle); Searcher; [&[u8], &u8] }
}
'''

FILES["ex03_naive.rs"] = r'''// Ex03 -- naive: contains — linear scan (no specialization)

#[inline(never)]
pub fn contains_naive<T: PartialEq>(data: &[T], needle: &T) -> bool {
    data.iter().any(|x| x == needle)
}
'''

FILES["ex03_enum.rs"] = r'''// Ex03 -- enum dispatch: contains

pub enum ContainsEnum<'a> {
    U8(&'a [u8], u8),
}

#[inline(never)]
pub fn contains_enum(e: &ContainsEnum<'_>) -> bool {
    match e {
        ContainsEnum::U8(data, needle) => {
            memchr::memchr(*needle, data).is_some()
        }
    }
}
'''

FILES["ex03_runtime_typeid.rs"] = r'''// Ex03 -- runtime TypeId dispatch: contains
use std::any::TypeId;

#[inline(never)]
pub fn contains_runtime<T: PartialEq + 'static>(data: &[T], needle: &T) -> bool {
    if TypeId::of::<T>() == TypeId::of::<u8>() {
        let byte_slice = unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len())
        };
        let n = unsafe { *(needle as *const T as *const u8) };
        memchr::memchr(n, byte_slice).is_some()
    } else {
        data.iter().any(|x| x == needle)
    }
}
'''

# ============================================================================
# Ex04: sort — #[when(T = u8)] — counting sort vs comparison sort
# ============================================================================

FILES["ex04_spec.rs"] = r'''// Ex04 -- spec: sort — counting sort for u8 vs generic sort_unstable
use spec_trait_macro::{spec, when};

pub struct Sorter;

pub trait SortTrait<T> {
    fn sort(&self, data: &mut [T]);
}

impl<T: Ord> SortTrait<T> for Sorter {
    #[inline(never)]
    fn sort(&self, data: &mut [T]) {
        data.sort_unstable();
    }
}

#[when(T = u8)]
impl<T> SortTrait<T> for Sorter {
    #[inline(never)]
    fn sort(&self, data: &mut [T]) {
        let slice = unsafe {
            std::slice::from_raw_parts_mut(data.as_mut_ptr() as *mut u8, data.len())
        };
        let mut counts = [0u32; 256];
        for &b in slice.iter() {
            counts[b as usize] += 1;
        }
        let mut idx = 0;
        for (val, &cnt) in counts.iter().enumerate() {
            for _ in 0..cnt {
                slice[idx] = val as u8;
                idx += 1;
            }
        }
    }
}

pub fn sort_u8(s: &Sorter, data: &mut [u8]) {
    spec! { s.sort(data); Sorter; [&mut [u8]] }
}
'''

FILES["ex04_naive.rs"] = r'''// Ex04 -- naive: sort — sort_unstable (no specialization)

#[inline(never)]
pub fn sort_naive<T: Ord>(data: &mut [T]) {
    data.sort_unstable();
}
'''

FILES["ex04_enum.rs"] = r'''// Ex04 -- enum dispatch: sort

pub enum SortEnum<'a> {
    U8(&'a mut [u8]),
}

#[inline(never)]
pub fn sort_enum(e: &mut SortEnum<'_>) {
    match e {
        SortEnum::U8(data) => {
            let mut counts = [0u32; 256];
            for &b in data.iter() {
                counts[b as usize] += 1;
            }
            let mut idx = 0;
            for (val, &cnt) in counts.iter().enumerate() {
                for _ in 0..cnt {
                    data[idx] = val as u8;
                    idx += 1;
                }
            }
        }
    }
}
'''

FILES["ex04_runtime_typeid.rs"] = r'''// Ex04 -- runtime TypeId dispatch: sort
use std::any::TypeId;

#[inline(never)]
pub fn sort_runtime<T: Ord + 'static>(data: &mut [T]) {
    if TypeId::of::<T>() == TypeId::of::<u8>() {
        let slice = unsafe {
            std::slice::from_raw_parts_mut(data.as_mut_ptr() as *mut u8, data.len())
        };
        let mut counts = [0u32; 256];
        for &b in slice.iter() {
            counts[b as usize] += 1;
        }
        let mut idx = 0;
        for (val, &cnt) in counts.iter().enumerate() {
            for _ in 0..cnt {
                slice[idx] = val as u8;
                idx += 1;
            }
        }
    } else {
        data.sort_unstable();
    }
}
'''

# ============================================================================
# Ex05: histogram — #[when(T = u8)] — array[256] vs HashMap
# ============================================================================

FILES["ex05_spec.rs"] = r'''// Ex05 -- spec: histogram — array[256] indexing vs HashMap
use spec_trait_macro::{spec, when};
use std::collections::HashMap;
use std::hash::Hash;

pub struct HistogramOp;

pub trait HistogramTrait<T> {
    fn max_freq(&self, data: &[T]) -> u32;
}

impl<T: Eq + Hash> HistogramTrait<T> for HistogramOp {
    #[inline(never)]
    fn max_freq(&self, data: &[T]) -> u32 {
        let mut map = HashMap::<&T, u32>::new();
        for item in data {
            *map.entry(item).or_insert(0) += 1;
        }
        map.values().copied().max().unwrap_or(0)
    }
}

#[when(T = u8)]
impl<T> HistogramTrait<T> for HistogramOp {
    #[inline(never)]
    fn max_freq(&self, data: &[T]) -> u32 {
        let slice = unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len())
        };
        let mut counts = [0u32; 256];
        for &b in slice {
            counts[b as usize] += 1;
        }
        counts.iter().copied().max().unwrap_or(0)
    }
}

pub fn max_freq_u8(h: &HistogramOp, data: &[u8]) -> u32 {
    spec! { h.max_freq(data); HistogramOp; [&[u8]] }
}
'''

FILES["ex05_naive.rs"] = r'''// Ex05 -- naive: histogram — HashMap (no specialization)
use std::collections::HashMap;
use std::hash::Hash;

#[inline(never)]
pub fn max_freq_naive<T: Eq + Hash>(data: &[T]) -> u32 {
    let mut map = HashMap::<&T, u32>::new();
    for item in data {
        *map.entry(item).or_insert(0) += 1;
    }
    map.values().copied().max().unwrap_or(0)
}
'''

FILES["ex05_enum.rs"] = r'''// Ex05 -- enum dispatch: histogram

pub enum HistEnum<'a> {
    U8(&'a [u8]),
}

#[inline(never)]
pub fn histogram_enum(e: &HistEnum<'_>) -> u32 {
    match e {
        HistEnum::U8(data) => {
            let mut counts = [0u32; 256];
            for &b in *data {
                counts[b as usize] += 1;
            }
            counts.iter().copied().max().unwrap_or(0)
        }
    }
}
'''

FILES["ex05_runtime_typeid.rs"] = r'''// Ex05 -- runtime TypeId dispatch: histogram
use std::any::TypeId;
use std::collections::HashMap;
use std::hash::Hash;

#[inline(never)]
pub fn max_freq_runtime<T: Eq + Hash + 'static>(data: &[T]) -> u32 {
    if TypeId::of::<T>() == TypeId::of::<u8>() {
        let slice = unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len())
        };
        let mut counts = [0u32; 256];
        for &b in slice {
            counts[b as usize] += 1;
        }
        counts.iter().copied().max().unwrap_or(0)
    } else {
        let mut map = HashMap::<&T, u32>::new();
        for item in data {
            *map.entry(item).or_insert(0) += 1;
        }
        map.values().copied().max().unwrap_or(0)
    }
}
'''

# ============================================================================
# Ex06: dedup — #[when(T = u8)] — bitmap vs HashSet
# ============================================================================

FILES["ex06_spec.rs"] = r'''// Ex06 -- spec: dedup (keep first occurrence) — bitmap vs HashSet
use spec_trait_macro::{spec, when};
use std::collections::HashSet;
use std::hash::Hash;

pub struct Deduplicator;

pub trait DedupTrait<T> {
    fn dedup(&self, data: &[T]) -> Vec<T>;
}

impl<T: Clone + Eq + Hash> DedupTrait<T> for Deduplicator {
    #[inline(never)]
    fn dedup(&self, data: &[T]) -> Vec<T> {
        let mut seen = HashSet::new();
        let mut out = Vec::new();
        for item in data {
            if seen.insert(item) {
                out.push(item.clone());
            }
        }
        out
    }
}

#[when(T = u8)]
impl<T> DedupTrait<T> for Deduplicator {
    #[inline(never)]
    fn dedup(&self, data: &[T]) -> Vec<T> {
        let slice = unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len())
        };
        let mut seen = [false; 256];
        let mut out_bytes = Vec::new();
        for &b in slice {
            if !seen[b as usize] {
                seen[b as usize] = true;
                out_bytes.push(b);
            }
        }
        let mut out = Vec::with_capacity(out_bytes.len());
        unsafe {
            std::ptr::copy_nonoverlapping(out_bytes.as_ptr(), out.as_mut_ptr() as *mut u8, out_bytes.len());
            out.set_len(out_bytes.len());
        }
        out
    }
}

pub fn dedup_u8(d: &Deduplicator, data: &[u8]) -> Vec<u8> {
    spec! { d.dedup(data); Deduplicator; [&[u8]] }
}
'''

FILES["ex06_naive.rs"] = r'''// Ex06 -- naive: dedup — HashSet (no specialization)
use std::collections::HashSet;
use std::hash::Hash;

#[inline(never)]
pub fn dedup_naive<T: Clone + Eq + Hash>(data: &[T]) -> Vec<T> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for item in data {
        if seen.insert(item) {
            out.push(item.clone());
        }
    }
    out
}
'''

FILES["ex06_enum.rs"] = r'''// Ex06 -- enum dispatch: dedup

pub enum DedupEnum<'a> {
    U8(&'a [u8]),
}

#[inline(never)]
pub fn dedup_enum(e: &DedupEnum<'_>) -> Vec<u8> {
    match e {
        DedupEnum::U8(data) => {
            let mut seen = [false; 256];
            let mut out = Vec::new();
            for &b in *data {
                if !seen[b as usize] {
                    seen[b as usize] = true;
                    out.push(b);
                }
            }
            out
        }
    }
}
'''

FILES["ex06_runtime_typeid.rs"] = r'''// Ex06 -- runtime TypeId dispatch: dedup
use std::any::TypeId;
use std::collections::HashSet;
use std::hash::Hash;

#[inline(never)]
pub fn dedup_runtime<T: Clone + Eq + Hash + 'static>(data: &[T]) -> Vec<T> {
    if TypeId::of::<T>() == TypeId::of::<u8>() {
        let slice = unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len())
        };
        let mut seen = [false; 256];
        let mut out_bytes = Vec::new();
        for &b in slice {
            if !seen[b as usize] {
                seen[b as usize] = true;
                out_bytes.push(b);
            }
        }
        let mut out = Vec::with_capacity(out_bytes.len());
        unsafe {
            std::ptr::copy_nonoverlapping(out_bytes.as_ptr(), out.as_mut_ptr() as *mut u8, out_bytes.len());
            out.set_len(out_bytes.len());
        }
        out
    } else {
        let mut seen = HashSet::new();
        let mut out = Vec::new();
        for item in data {
            if seen.insert(item) {
                out.push(item.clone());
            }
        }
        out
    }
}
'''

# ============================================================================
# Ex07: hash_key — #[when(K = u64)] — identity hash vs SipHash
# ============================================================================

FILES["ex07_spec.rs"] = r'''// Ex07 -- spec: hash_key — identity hash for u64 vs SipHash (DefaultHasher)
use spec_trait_macro::{spec, when};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct HashOp;

pub trait HashKeyTrait<K> {
    fn hash_all(&self, keys: &[K]) -> u64;
}

impl<K: Hash> HashKeyTrait<K> for HashOp {
    #[inline(never)]
    fn hash_all(&self, keys: &[K]) -> u64 {
        let mut acc: u64 = 0;
        for key in keys {
            let mut h = DefaultHasher::new();
            key.hash(&mut h);
            acc ^= h.finish();
        }
        acc
    }
}

#[when(K = u64)]
impl<K> HashKeyTrait<K> for HashOp {
    #[inline(never)]
    fn hash_all(&self, keys: &[K]) -> u64 {
        let slice = unsafe {
            std::slice::from_raw_parts(keys.as_ptr() as *const u64, keys.len())
        };
        let mut acc: u64 = 0;
        for &k in slice {
            acc ^= k;
        }
        acc
    }
}

pub fn hash_all_u64(h: &HashOp, keys: &[u64]) -> u64 {
    spec! { h.hash_all(keys); HashOp; [&[u64]] }
}
'''

FILES["ex07_naive.rs"] = r'''// Ex07 -- naive: hash_key — SipHash for all keys (no specialization)
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[inline(never)]
pub fn hash_all_naive<K: Hash>(keys: &[K]) -> u64 {
    let mut acc: u64 = 0;
    for key in keys {
        let mut h = DefaultHasher::new();
        key.hash(&mut h);
        acc ^= h.finish();
    }
    acc
}
'''

FILES["ex07_enum.rs"] = r'''// Ex07 -- enum dispatch: hash_key

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
'''

FILES["ex07_runtime_typeid.rs"] = r'''// Ex07 -- runtime TypeId dispatch: hash_key
use std::any::TypeId;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[inline(never)]
pub fn hash_all_runtime<K: Hash + 'static>(keys: &[K]) -> u64 {
    if TypeId::of::<K>() == TypeId::of::<u64>() {
        let slice = unsafe {
            std::slice::from_raw_parts(keys.as_ptr() as *const u64, keys.len())
        };
        let mut acc: u64 = 0;
        for &k in slice {
            acc ^= k;
        }
        acc
    } else {
        let mut acc: u64 = 0;
        for key in keys {
            let mut h = DefaultHasher::new();
            key.hash(&mut h);
            acc ^= h.finish();
        }
        acc
    }
}
'''

# ============================================================================
# Ex08: count_value — #[when(T = u8)] — memchr SIMD vs filter
# ============================================================================

FILES["ex08_spec.rs"] = r'''// Ex08 -- spec: count_value — memchr SIMD vs filter().count()
use spec_trait_macro::{spec, when};

pub struct CountValOp;

pub trait CountValTrait<T> {
    fn count_val(&self, data: &[T], needle: &T) -> usize;
}

impl<T: PartialEq> CountValTrait<T> for CountValOp {
    #[inline(never)]
    fn count_val(&self, data: &[T], needle: &T) -> usize {
        data.iter().filter(|x| *x == needle).count()
    }
}

#[when(T = u8)]
impl<T> CountValTrait<T> for CountValOp {
    #[inline(never)]
    fn count_val(&self, data: &[T], needle: &T) -> usize {
        let byte_slice = unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len())
        };
        let n = unsafe { *(needle as *const T as *const u8) };
        memchr::memchr_iter(n, byte_slice).count()
    }
}

pub fn count_val_u8(op: &CountValOp, data: &[u8], needle: &u8) -> usize {
    spec! { op.count_val(data, needle); CountValOp; [&[u8], &u8] }
}
'''

FILES["ex08_naive.rs"] = r'''// Ex08 -- naive: count_value — filter (no specialization)

#[inline(never)]
pub fn count_val_naive<T: PartialEq>(data: &[T], needle: &T) -> usize {
    data.iter().filter(|x| *x == needle).count()
}
'''

FILES["ex08_enum.rs"] = r'''// Ex08 -- enum dispatch: count_value

pub enum CountValEnum<'a> {
    U8(&'a [u8], u8),
}

#[inline(never)]
pub fn count_val_enum(e: &CountValEnum<'_>) -> usize {
    match e {
        CountValEnum::U8(data, needle) => {
            memchr::memchr_iter(*needle, data).count()
        }
    }
}
'''

FILES["ex08_runtime_typeid.rs"] = r'''// Ex08 -- runtime TypeId dispatch: count_value
use std::any::TypeId;

#[inline(never)]
pub fn count_val_runtime<T: PartialEq + 'static>(data: &[T], needle: &T) -> usize {
    if TypeId::of::<T>() == TypeId::of::<u8>() {
        let byte_slice = unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len())
        };
        let n = unsafe { *(needle as *const T as *const u8) };
        memchr::memchr_iter(n, byte_slice).count()
    } else {
        data.iter().filter(|x| *x == needle).count()
    }
}
'''


def main():
    os.makedirs(BASE, exist_ok=True)
    for name, content in sorted(FILES.items()):
        path = os.path.join(BASE, name)
        with open(path, "w") as f:
            f.write(content.lstrip("\n"))
        print(f"  wrote {name}")
    print(f"\n[OK] Wrote {len(FILES)} files to {BASE}")


if __name__ == "__main__":
    main()
