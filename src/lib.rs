// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A communication platform.
//!
//! Agora provides a foundation for communicating in a variety of
//! ways among a set of people.
//!
//! This is a collection of sketches right now that will likely
//! see many of them evolve into standalone crates of their own.

#![warn(missing_docs)]
#![deny(
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

pub mod access_control;
pub mod accounts;
pub mod caucus;
