//! Rust bindings to librandomx, a library for computing RandomX hashes.
//!
//! # Examples
//!
//! ## Light mode hash
//!
//! Requires 256M of shared memory.
//!
//! ```no_run
//! use randomx4r::{RandomxCache, RandomxError, RandomxFlags, RandomxVm};
//!
//! // Get flags supported by this system.
//! let flags = RandomxFlags::default();
//! let cache = RandomxCache::new(flags, b"key")?;
//! let vm = RandomxVm::new(flags, &cache)?;
//! let hash = vm.hash(b"input"); // is a [u8; 32]
//! # Ok::<(), RandomxError>(())
//! ```
//!
//! ## Fast mode hash
//!
//! Requires 2080M of shared memory.
//!
//! ```no_run
//! use randomx4r::{RandomxDataset, RandomxError, RandomxFlags, RandomxVm};
//!
//! // OR the default flags with FULLMEM (aka. fast mode)
//! let flags = RandomxFlags::default() | RandomxFlags::FULLMEM;
//! let dataset = RandomxDataset::new(flags, b"key")?;
//! let vm = RandomxVm::new_fast(flags, &dataset)?;
//! let hash = vm.hash(b"input");
//! # Ok::<(), RandomxError>(())
//! ```
//!
//! # Errors
//!
//! Some operations (e.g. allocating a VM or dataset) can fail if the
//! system doesn't have enough free memory, or if you tried to force a
//! feature like large pages or AVX2 on a system that does not support
//! it.

#[macro_use]
extern crate bitflags;
extern crate randomx4r_sys;

pub mod cache;
pub mod dataset;
pub mod error;
pub mod flags;
pub mod vm;

pub use crate::cache::*;
pub use crate::dataset::*;
pub use crate::error::*;
pub use crate::flags::*;
pub use crate::vm::*;
