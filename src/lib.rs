#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
use core as gstd;

#[cfg(feature = "std")]
use std as gstd;
