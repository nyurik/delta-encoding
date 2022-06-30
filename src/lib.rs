#![cfg_attr(all(feature = "nightly", test), feature(test))]

#[cfg(all(feature = "nightly", test))]
extern crate test;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

use num_traits::{CheckedAdd, CheckedSub};

/// Construct a delta-encoder
#[derive(Debug, Default, Copy, Clone)]
pub struct DeltaEncoder<T> {
    current: T,
}

impl<T: CheckedSub + Copy> DeltaEncoder<T> {
    pub fn encode(&mut self, value: T) -> T {
        let delta = value.checked_sub(&self.current).unwrap();
        self.current = value;
        delta
    }
}

/// Construct a delta-decoder
#[derive(Debug, Default, Copy, Clone)]
pub struct DeltaDecoder<T> {
    current: T,
}

impl<T: CheckedAdd + Copy> DeltaDecoder<T> {
    pub fn decode(&mut self, value: T) -> T {
        self.current = self.current.checked_add(&value).unwrap();
        self.current
    }
}

/// A utility struct to construct a delta-encoded sequence from an iterator.
#[derive(Debug)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct DeltaEncoderIter<I>
where
    I: Iterator,
    <I as Iterator>::Item: CheckedSub + Copy,
{
    iter: I,
    encoder: DeltaEncoder<I::Item>,
}

impl<I> Iterator for DeltaEncoderIter<I>
where
    I: Iterator,
    <I as Iterator>::Item: CheckedSub + Copy,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(v) => Some(self.encoder.encode(v)),
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub trait DeltaEncoderExt<T>: Iterator<Item = T>
where
    T: Default + Copy + CheckedSub,
{
    /// Construct a delta-encoded iterator from an iterator.
    /// The first element of the iterator is used as the starting point for the delta-encoding.
    ///
    /// ## Example
    /// ```
    /// use delta_encoding::DeltaEncoderExt;
    ///
    /// let mut encoded: Vec<i64> = vec![1, 2, 5, 4, 2].into_iter().to_deltas().collect();
    /// assert_eq!(encoded, vec![1, 1, 3, -1, -2]);
    /// ```
    fn to_deltas(self) -> DeltaEncoderIter<Self>
    where
        Self: Sized,
    {
        DeltaEncoderIter {
            iter: self,
            encoder: Default::default(),
        }
    }
}

impl<I> DeltaEncoderExt<I::Item> for I
where
    I: Iterator,
    <I as Iterator>::Item: Default + Copy + CheckedSub,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(original: &[i64], encoded: &[i64]) {
        let data = original.to_vec();
        let mut dlt = DeltaEncoder::default();
        let actual: Vec<i64> = data.iter().map(|&v| dlt.encode(v)).collect();
        assert_eq!(actual, encoded, "encoded from: {original:?}");

        let data = encoded.to_vec();
        let mut dlt = DeltaDecoder::default();
        let actual: Vec<i64> = data.iter().map(|&v| dlt.decode(v)).collect();
        assert_eq!(actual, original, "decoded from: {encoded:?}");

        let data = original.to_vec();
        let encoded: Vec<i64> = data.into_iter().to_deltas().collect();
        assert_eq!(encoded, encoded, "into_iter() original: {original:?}");

        // TODO: implement non-consuming iterator
        // let data = original.to_vec();
        // let encoded: Vec<i64> = data.iter().to_deltas().collect();
        // assert_eq!(encoded, encoded, "iter() original: {original:?}");
    }

    #[test]
    fn test() {
        // Delta encoding cannot support deltas bigger than i64::MAX (half the size of u64)
        let min = i64::MIN / 2;
        let max = i64::MAX / 2;

        run(&[], &[]);
        run(&[0], &[0]);
        run(&[1], &[1]);
        run(&[1, 2], &[1, 1]);
        run(&[1, -2], &[1, -3]);
        run(&[1, 3, 5], &[1, 2, 2]);
        run(&[1, 3, 10], &[1, 2, 7]);
        run(&[min], &[min]);
        run(&[max], &[max]);
        run(&[max, min], &[max, (min - max)]);
        run(&[0, max], &[0, max]);
        run(&[0, max, min, max], &[0, max, (min - max), max + max + 1]);
    }
}

/// The benchmarks require nightly to run:
///   $ cargo +nightly bench
#[cfg(all(feature = "nightly", test))]
mod bench {
    use super::test::Bencher;
    use super::*;

    #[bench]
    fn bench_empty_t(bench: &mut Bencher) {
        bench.iter(|| {
            let mut enc = DeltaEncoder::default();
            let _: Vec<i64> = (0..1).map(|v| enc.encode(v)).collect();
        });
    }

    #[bench]
    fn bench_short_t(bench: &mut Bencher) {
        bench.iter(|| {
            let mut enc = DeltaEncoder::default();
            let _: Vec<i64> = (0..1000).map(|v| enc.encode(v)).collect();
        });
    }

    #[bench]
    fn bench_long_t(bench: &mut Bencher) {
        bench.iter(|| {
            let mut enc = DeltaEncoder::default();
            let _: Vec<i64> = (0..100000).map(|v| enc.encode(v)).collect();
        });
    }
}
