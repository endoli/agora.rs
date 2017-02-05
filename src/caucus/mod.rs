// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The Caucus framework provides a way of coordinating communication
//! between a set of actors.
//!
//! The subject of the communication may be a chat channel, a mailing
//! list, a discussion around an article, collaboration on a
//! document, etc.

use access_control::Lock;

/// Caucus operations, used for access control.
pub enum Operation {
    /// Joining a caucus.
    Join,
    /// Sending to a caucus.
    Send,
    /// Receiving from a caucus.
    Receive,
}

/// Type alias for the locks on caucuses.
///
/// XXX: This should probably have a different set of type
/// specifiers for the various operations.
pub type CaucusLock<A, M> = Box<Lock<A, Operation, Caucus<A, M>>>;

/// A channel of communication between the actors in the caucus.
///
/// Type Parameters:
///
/// * `A`: The type of actor participating in the caucus.
/// * `M`: The type of messages exchanged over the caucus.
pub struct Caucus<A, M> {
    /// Control who can join the caucus.
    pub join_lock: CaucusLock<A, M>,
    /// Control who can send to the caucus.
    pub send_lock: CaucusLock<A, M>,
    /// Control who will receive messages from the caucus.
    pub receive_lock: CaucusLock<A, M>,
    /// Actors in the caucus.
    pub actors: Vec<A>,
}

impl<A: PartialEq, M> Caucus<A, M> {
    /// Add a actor to the caucus.
    pub fn add_actor(&mut self, actor: A) -> Result<(), ()> {
        self.join_lock
            .try(&actor, Operation::Join, self)
            .map(|_| self.actors.push(actor))
    }

    /// Remove a actor from the caucus.
    pub fn remove_actor(&mut self, actor: A) -> Result<(), ()> {
        self.actors
            .iter()
            .position(|m| m == &actor)
            .map(|pos| self.actors.remove(pos));
        Ok(())
    }

    /// Broadcast a message to the actors in the caucus.
    pub fn broadcast(&mut self, _sender: A, _message: M) -> Result<(), ()> {
        // How do we actually send a message to the actors?
        Ok(())
    }
}