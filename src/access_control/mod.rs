// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Access control is provided by lock objects which can be
//! composed in a tree structure, like an AST. Specicalized
//! implementations can be provided by implementing the
//! `Lock` trait.
//!
//! XXX: Can we use macaroons instead?

/// The result type for testing a `Lock`.
///
/// XXX: This will clearly evolve to be more specific in the types.
pub type LockResult = Result<(), ()>;

/// Base trait for nodes in an ACL AST.
///
/// Type Parameters:
///
/// * `S`: Subject. The object carrying out an operation.
/// * `O`: Operation: The operation that this lock is being tested for.
/// * `T`: Target. The target of the operation.
pub trait Lock<S, O, T> {
    /// Test whether or not the lock's requirements have been satisfied.
    fn try(&self, subject: &S, operation: O, target: &T) -> LockResult;
}

/// Lock that is always valid.
pub struct True {}

impl<S, O, T> Lock<S, O, T> for True {
    fn try(&self, _subject: &S, _operation: O, _target: &T) -> LockResult {
        Ok(())
    }
}

/// Lock that is always invalid.
pub struct False {}

impl<S, O, T> Lock<S, O, T> for False {
    fn try(&self, _subject: &S, _operation: O, _target: &T) -> LockResult {
        Err(())
    }
}

/// Lock that is valid if all of the constituent locks are also valid.
pub struct And<S, O, T> {
    /// Constituent locks that must be valid for this to be valid.
    pub locks: Vec<Box<Lock<S, O, T>>>,
}

impl<S, O: Copy, T> Lock<S, O, T> for And<S, O, T> {
    fn try(&self, subject: &S, operation: O, target: &T) -> LockResult {
        if self
            .locks
            .iter()
            .all(|lock| lock.try(subject, operation, target).is_ok())
        {
            Ok(())
        } else {
            Err(())
        }
    }
}
