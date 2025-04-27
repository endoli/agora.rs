// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Access control is provided by guard objects which can be
//! composed in a tree structure, like an AST. Specicalized
//! implementations can be provided by implementing the
//! `Guard` trait.
//!
//! XXX: Can we use macaroons instead?

/// Used as an error in a [`Guard`] result.
pub struct AccessDenied;

/// The result type for testing a `Guard`.
///
/// XXX: This will clearly evolve to be more specific in the types.
pub type GuardResult = Result<(), AccessDenied>;

/// Base trait for nodes in an ACL AST.
///
/// Type Parameters:
///
/// * `S`: Subject. The object carrying out an operation.
/// * `O`: Operation: The operation that this guard is being tested for.
/// * `T`: Target. The target of the operation.
pub trait Guard<S, O, T> {
    /// Test whether or not the guard's requirements have been satisfied.
    fn attempt(&self, subject: &S, operation: O, target: &T) -> GuardResult;
}

/// Guard that is always valid.
pub struct True {}

impl<S, O, T> Guard<S, O, T> for True {
    fn attempt(&self, _subject: &S, _operation: O, _target: &T) -> GuardResult {
        Ok(())
    }
}

/// Guard that is always invalid.
pub struct False {}

impl<S, O, T> Guard<S, O, T> for False {
    fn attempt(&self, _subject: &S, _operation: O, _target: &T) -> GuardResult {
        Err(AccessDenied)
    }
}

/// Guard that is valid if all of the constituent guards are also valid.
pub struct And<S, O, T> {
    /// Constituent guards that must be valid for this to be valid.
    pub guards: Vec<Box<dyn Guard<S, O, T>>>,
}

impl<S, O: Copy, T> Guard<S, O, T> for And<S, O, T> {
    fn attempt(&self, subject: &S, operation: O, target: &T) -> GuardResult {
        if self
            .guards
            .iter()
            .all(|guard| guard.attempt(subject, operation, target).is_ok())
        {
            Ok(())
        } else {
            Err(AccessDenied)
        }
    }
}
