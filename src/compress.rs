// RustyRosetta
// Copyright (C) 2025 Oscar Alvarez Gonzalez

use crate::*;

use lz4_flex::{compress_into, decompress_into};

/// Contains compressed data
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Compressed<T: Clone + PartialEq + Encode>(#[serde(skip)] PhantomData<T>, Bytes);

impl<T: Clone + PartialEq + Encode<Output = Bytes> + Decode<Input = [u8]>> Compressed<T> {
    /// Serializes and compresses data
    #[instrument(level = "trace", skip_all, err)]
    pub fn new(data: &T) -> Result<Self> {
        let mut buff = Bytes::new();

        compress_into(T::encode(data)?.as_slice(), &mut buff)?;

        Ok(Self(PhantomData, buff))
    }

    /// Decompresses and deserializes data
    #[instrument(level = "trace", skip_all, err)]
    pub fn take(self) -> Result<T> {
        let mut buff = Bytes::new();

        decompress_into(&self.1, &mut buff)?;

        Ok(T::decode(&buff)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compress() -> Result<()> {
        // Generate some random data
        let random_bytes = CheapVec::<u8, 32>::from_slice(&rand::random::<[u8; 32]>());

        // Compress the random bytes
        let compressed = Compressed::new(&random_bytes)?;

        // Check that the random bytes and the decompressed data are the same
        assert_eq!(compressed.take()?, random_bytes);

        Ok(())
    }
}
