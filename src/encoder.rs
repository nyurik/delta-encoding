use num_traits::CheckedSub;

impl<T: CheckedSub + Copy> DeltaEncoder<T> {
    pub fn encode(&mut self, value: T) -> T {
        let delta = value.checked_sub(&self.current).unwrap();
        self.current = value;
        delta
    }
}

/// Construct a delta-encoder
#[derive(Debug, Default, Copy, Clone)]
pub struct DeltaEncoder<T> {
    current: T,
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
