//! A font toolbox.

pub extern crate font;

#[cfg(feature = "draw")]
pub extern crate svg;

extern crate colored;

#[cfg(feature = "draw")]
pub mod drawing;

pub mod support;
