use num_traits::WrappingAdd;

/// Construct a delta-decoder
#[derive(Debug, Default, Copy, Clone)]
pub struct DeltaDecoder<T> {
    current: T,
}

impl<T: WrappingAdd + Copy> DeltaDecoder<T> {
    pub fn decode(&mut self, value: T) -> T {
        self.current = self.current.wrapping_add(&value);
        self.current
    }
}

/// A utility struct to construct a delta-decoder sequence from an iterator.
#[derive(Clone, Debug)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct DeltaDecoderIter<I>
where
    I: Iterator,
    <I as Iterator>::Item: WrappingAdd + Copy,
{
    iter: I,
    decoder: DeltaDecoder<I::Item>,
}

impl<I> Iterator for DeltaDecoderIter<I>
where
    I: Iterator,
    <I as Iterator>::Item: WrappingAdd + Copy,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.decoder.decode(self.iter.next()?))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<I> ExactSizeIterator for DeltaDecoderIter<I>
where
    I: ExactSizeIterator,
    <I as Iterator>::Item: WrappingAdd + Copy,
{
}

pub trait DeltaDecoderExt: Iterator
where
    <Self as Iterator>::Item: Default + Copy + WrappingAdd,
{
    /// Construct a delta-decoded iterator from an iterator.
    /// The first element of the iterator is used as the starting point for the delta-encoding.
    /// Note that unlike the [`DeltaDecoder.decode`] method, this method will panic if the delta is too large.
    ///
    /// ## Example
    /// ```
    /// use delta_encoding::DeltaDecoderExt;
    ///
    /// // Consuming original data into a delta-decoded iterator.
    /// let mut decoded: Vec<i64> = vec![1, 1, 3, -1, -2].into_iter().original().collect();
    /// assert_eq!(decoded, vec![1, 2, 5, 4, 2]);
    ///
    /// // Non-consuming original data, but avoiding the allocation of a new vector.
    /// let mut decoded: Vec<i64> = vec![1, 1, 3, -1, -2].iter().copied().original().collect();
    /// assert_eq!(decoded, vec![1, 2, 5, 4, 2]);
    /// ```
    fn original(self) -> DeltaDecoderIter<Self>
    where
        Self: Sized,
    {
        DeltaDecoderIter {
            iter: self,
            decoder: Default::default(),
        }
    }
}

impl<I> DeltaDecoderExt for I
where
    I: Iterator,
    <I as Iterator>::Item: Default + Copy + WrappingAdd,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::TEST_DATA;

    fn run(original: &[i64], encoded: &[i64]) {
        let mut dec = DeltaDecoder::default();
        let result: Vec<i64> = encoded.iter().map(|&v| dec.decode(v)).collect();
        assert_eq!(result, original, "decoded from: {encoded:?}");

        let result: Vec<i64> = encoded.iter().copied().original().collect();
        assert_eq!(result, original, "iter().copied() encoded: {encoded:?}");
    }

    #[test]
    fn test() {
        for &(original, encoded) in TEST_DATA {
            run(original, encoded);
        }
    }
}
