use num_traits::WrappingSub;

/// Construct a delta-encoder
#[derive(Debug, Default, Copy, Clone)]
pub struct DeltaEncoder<T> {
    current: T,
}

impl<T: WrappingSub + Copy> DeltaEncoder<T> {
    /// Encode a value, and return the delta between the current value and the encoded value.
    pub fn encode(&mut self, value: T) -> T {
        let delta = value.wrapping_sub(&self.current);
        self.current = value;
        delta
    }
}

/// A utility struct to construct a delta-encoded sequence from an iterator.
#[derive(Clone, Debug)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct DeltaEncoderIter<I>
where
    I: Iterator,
    <I as Iterator>::Item: WrappingSub + Copy,
{
    iter: I,
    encoder: DeltaEncoder<I::Item>,
}

impl<I> Iterator for DeltaEncoderIter<I>
where
    I: Iterator,
    <I as Iterator>::Item: WrappingSub + Copy,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.encoder.encode(self.iter.next()?))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<I> ExactSizeIterator for DeltaEncoderIter<I>
where
    I: ExactSizeIterator,
    <I as Iterator>::Item: WrappingSub + Copy,
{
}

pub trait DeltaEncoderExt: Iterator
where
    <Self as Iterator>::Item: Default + Copy + WrappingSub,
{
    /// Construct a delta-encoded iterator from an iterator.
    /// The first element of the iterator is used as the starting point for the delta-encoding.
    /// Note that unlike the [`DeltaEncoder.encode`] method, this method will panic if the delta is too large.
    ///
    /// ## Example
    /// ```
    /// use delta_encoding::DeltaEncoderExt;
    ///
    /// // Consuming original data into a delta-encoded iterator.
    /// let mut encoded: Vec<i64> = vec![1, 2, 5, 4, 2].into_iter().deltas().collect();
    /// assert_eq!(encoded, vec![1, 1, 3, -1, -2]);
    ///
    /// // Non-consuming original data, but avoiding the allocation of a new vector.
    /// let mut encoded: Vec<i64> = vec![1, 2, 5, 4, 2].iter().copied().deltas().collect();
    /// assert_eq!(encoded, vec![1, 1, 3, -1, -2]);
    /// ```
    fn deltas(self) -> DeltaEncoderIter<Self>
    where
        Self: Sized,
    {
        DeltaEncoderIter {
            iter: self,
            encoder: Default::default(),
        }
    }
}

impl<I> DeltaEncoderExt for I
where
    I: Iterator,
    <I as Iterator>::Item: Default + Copy + WrappingSub,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::TEST_DATA;

    fn run(original: &[i64], encoded: &[i64]) {
        let mut enc = DeltaEncoder::default();
        let result: Vec<i64> = original.iter().map(|&v| enc.encode(v)).collect();
        assert_eq!(result, encoded, "encoded from: {original:?}");

        let result: Vec<i64> = original.iter().copied().deltas().collect();
        assert_eq!(result, encoded, "iter().copied() original: {original:?}");
    }

    #[test]
    fn test() {
        for &(original, encoded) in TEST_DATA {
            run(original, encoded);
        }
    }
}
