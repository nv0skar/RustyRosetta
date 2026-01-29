// RustyRosetta
// Copyright (C) 2025 Oscar Alvarez Gonzalez

use crate::*;

use lz4_flex::{compress_prepend_size, decompress_size_prepended};

/// Contains compressed data
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Compressed<T: Clone + PartialEq + Encode>(#[serde(skip)] PhantomData<T>, Bytes);

impl<T: Clone + PartialEq + Encode<Output = Bytes> + Decode<Input = [u8]>> Compressed<T> {
    /// Serializes and compresses data
    #[instrument(level = "trace", skip_all, err)]
    pub fn new(data: &T) -> Result<Self> {
        Ok(Self(
            PhantomData,
            compress_prepend_size(T::encode(data)?.as_slice()).to_smallvec(),
        ))
    }

    /// Decompresses and deserializes data
    #[instrument(level = "trace", skip_all, err)]
    pub fn take(self) -> Result<T> {
        Ok(T::decode(
            decompress_size_prepended(self.1.as_slice())
                .ok()
                .context("Decompression failed")?
                .as_slice(),
        )?)
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
