// RustyRosetta
// Copyright (C) 2025 Oscar Alvarez Gonzalez

#[cfg(feature = "authenticated_type")]
pub mod authenticated_type;

#[cfg(feature = "checksum")]
pub mod checksum;

#[cfg(feature = "codec")]
pub mod codec;

#[cfg(feature = "compress")]
pub mod compress;

#[cfg(feature = "crypt")]
pub mod crypt;

#[cfg(feature = "codec")]
use codec::*;

use std::{cell::RefCell, marker::PhantomData, ops::Deref};

use anyhow::{bail, ensure, Context, Result};
use arrayvec::ArrayString;
use derive_more::Display;
use garde::rules::AsStr;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use smallvec::{SmallVec, ToSmallVec};
use tracing::instrument;

const CHEAP_VEC_INLINE_ELEMENTS: usize = 16;
const ENCODING_BUFFER_SIZE: usize = 2048;
const MAX_STACK_ALLOC_SIZE: usize = 8 * 1024;

pub type CheapVec<T> = SmallVec<[T; CHEAP_VEC_INLINE_ELEMENTS]>;

pub type Bytes = CheapVec<u8>;

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Display, Debug)]
pub struct FixedString<const T: usize>(pub ArrayString<T>);

impl<const T: usize> AsStr for FixedString<T> {
    fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl<const T: usize> Deref for FixedString<T> {
    type Target = ArrayString<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
