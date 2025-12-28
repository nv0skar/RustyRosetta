// RustyRosetta
// Copyright (C) 2025 Oscar Alvarez Gonzalez

use crate::*;

thread_local! {
    static ENCODING_BUFFER: RefCell<[u8; ENCODING_BUFFER_SIZE]> = RefCell::new([0; ENCODING_BUFFER_SIZE]);
}

pub trait Encode {
    type Output;

    /// Encodes data from `T` into bytes
    fn encode(&self) -> Result<Self::Output>;
}

pub trait Decode {
    type Input: ?Sized;

    /// Decodes from bytes into `T`
    fn decode<'a>(data: &'a Self::Input) -> Result<Self>
    where
        Self: Sized;
}

impl<T: Serialize + Send + Sync> Encode for T {
    type Output = Bytes;

    /// Serializes data from `T` into bytes
    #[instrument(level = "trace", skip_all, err)]
    fn encode(&self) -> Result<Self::Output> {
        let _required_size = postcard::experimental::serialized_size(self)?;

        #[cfg(feature = "stack_allocated_encoding")]
        if _required_size <= MAX_STACK_ALLOC_SIZE {
            return Ok(stackalloc::stackalloc(
                _required_size,
                u8::default(),
                |buffer: &mut [u8]| -> Result<Bytes> {
                    Ok(postcard::to_slice(self, buffer)
                        .context("Failed to serialize payload.")?
                        .to_smallvec())
                },
            )
            .context("Cannot acquire a varible length stack.")?);
        }

        #[cfg(feature = "preallocated_stack_encoding")]
        if _required_size <= ENCODING_BUFFER_SIZE {
            return ENCODING_BUFFER.with(|buffer| -> Result<Bytes> {
                let mut borrowed = buffer.borrow_mut();
                let target_slice = &mut borrowed[.._required_size];
                Ok(postcard::to_slice(self, target_slice)
                    .context("Failed to serialize payload.")?
                    .to_smallvec())
            });
        }

        tracing::debug!(
            "Large payload ({} bytes), using heap allocation.",
            _required_size
        );
        Ok(postcard::to_allocvec(self)
            .context("Failed to serialize payload.")?
            .to_smallvec())
    }
}

impl<T: DeserializeOwned + Send + Sync> Decode for T {
    type Input = [u8];

    #[instrument(level = "trace", skip_all, err)]
    fn decode<'a>(data: &'a Self::Input) -> Result<Self> {
        Ok(postcard::from_bytes(&data).context("Unable to deserialize payload")?)
    }
}
