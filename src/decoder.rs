use num_traits::CheckedAdd;

/// Construct a delta-decoder
#[derive(Debug, Default, Copy, Clone)]
pub struct DeltaDecoder<T> {
    current: T,
}

impl<T: CheckedAdd + Copy> DeltaDecoder<T> {
    pub fn decode(&mut self, value: T) -> Option<T> {
        match self.current.checked_add(&value) {
            Some(value) => {
                self.current = value;
                Some(value)
            }
            None => None,
        }
    }
}
