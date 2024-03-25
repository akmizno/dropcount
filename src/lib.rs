//! # dropcount
//! Count destructor calls.
//!
//! Rust is memory safe, but memory bugs may occur in unsafe blocks.
//! The unsafe programming is often required when implementing our own data structures like
//! containers or smart pointers.
//! This crate provides a way to test memory leaks or multiple destruction by counting destructor calls.
//!
//! If this crate is used only for tests, use 'dev-dependencies' section in Cargo.toml as follows;
//! ```toml
//! # Cargo.toml
//! [dev-dependencies]
//! dropcount = "0.1"
//! ```
//!
//! ## Usage
//! ```
//! // Create a pair of counter and viewer.
//! // They share an internal count value.
//! let (counter, viewer) = dropcount::new();
//!
//! // The viewer returns 0.
//! // The counter has not been destructed.
//! assert_eq!(viewer.get(), 0);
//!
//! // Destruct the counter.
//! drop(counter);
//!
//! // The viewer returns 1 after destructing the counter.
//! assert_eq!(viewer.get(), 1);
//! ```
//!
//! ## Example
//! ### Testing smart pointers
//! An example for testing a smart pointer destructs its value exactly once.
//! ```
//! use std::rc::Rc;
//!
//! fn test_rc() {
//!     let (counter, viewer) = dropcount::new();
//!
//!     // rc1 and rc2 shares the counter object.
//!     let rc1 = Rc::new(counter);
//!     let rc2 = rc1.clone();
//!
//!     // The counter is not destructed.
//!     assert_eq!(viewer.get(), 0);
//!
//!     drop(rc1);
//!
//!     // The counter is not destructed.
//!     assert_eq!(viewer.get(), 0);
//!
//!     drop(rc2);
//!
//!     // The counter is destructed.
//!     assert_eq!(viewer.get(), 1);
//! }
//!
//! # fn main() {
//! #     test_rc();
//! # }
//! ```
//!
//! ### Testing collections
//! An example for testing a container destructs each value exactly once.
//! ```
//! use std::collections::HashMap;
//!
//! fn test_hashmap() {
//!     let (counters, viewers) = dropcount::new_vec(5);
//!
//!     let mut map: HashMap<usize, dropcount::Counter> =
//!         counters.into_iter().enumerate().collect();
//!
//!     assert_eq!(viewers[0].get(), 0);
//!     assert_eq!(viewers[1].get(), 0);
//!     assert_eq!(viewers[2].get(), 0);
//!     assert_eq!(viewers[3].get(), 0);
//!     assert_eq!(viewers[4].get(), 0);
//!
//!     // Remove an element.
//!     map.remove(&2);
//!
//!     // The viewer paired with the removed element returns 1.
//!     assert_eq!(viewers[0].get(), 0);
//!     assert_eq!(viewers[1].get(), 0);
//!     assert_eq!(viewers[2].get(), 1);
//!     assert_eq!(viewers[3].get(), 0);
//!     assert_eq!(viewers[4].get(), 0);
//!
//!     // Drop the container.
//!     drop(map);
//!
//!     // All viewers returns 1 after the container is dropped.
//!     assert_eq!(viewers[0].get(), 1);
//!     assert_eq!(viewers[1].get(), 1);
//!     assert_eq!(viewers[2].get(), 1);
//!     assert_eq!(viewers[3].get(), 1);
//!     assert_eq!(viewers[4].get(), 1);
//! }
//!
//! # fn main() {
//! #     test_hashmap();
//! # }
//! ```
//!
//! ### Multi-threading support
//! This crate supports multi-threading.
//! Atomic integers are used as the internal coutner values.
//! Therefore, it is possible to capture the number of destructions with multi-threading.
//! ```
//! use std::thread;
//!
//! fn test_multi_thread() {
//!     let (counter, viewer) = dropcount::new();
//!
//!     let handle = thread::spawn(move || {
//!         drop(counter);
//!     });
//!
//!     handle.join().expect("Error in thread.");
//!
//!     assert_eq!(viewer.get(), 1);
//! }
//!
//! # fn main() {
//! #     test_multi_thread();
//! # }
//! ```
pub mod dropcount;

pub use dropcount::{new, new_vec, Counter, Viewer};

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::thread;

    use crate::dropcount;

    #[test]
    fn example_rc() {
        let (counter, viewer) = dropcount::new();

        // rc1 and rc2 shares the counter object.
        let rc1 = Rc::new(counter);
        let rc2 = rc1.clone();

        // The counter is not destructed.
        assert_eq!(viewer.get(), 0);

        drop(rc1);

        // The counter is not destructed.
        assert_eq!(viewer.get(), 0);

        drop(rc2);

        // The counter is destructed.
        assert_eq!(viewer.get(), 1);
    }

    #[test]
    fn example_hashmap() {
        let (counters, viewers) = dropcount::new_vec(5);

        let mut map: HashMap<usize, dropcount::Counter> =
            counters.into_iter().enumerate().collect();

        assert_eq!(viewers[0].get(), 0);
        assert_eq!(viewers[1].get(), 0);
        assert_eq!(viewers[2].get(), 0);
        assert_eq!(viewers[3].get(), 0);
        assert_eq!(viewers[4].get(), 0);

        // Remove an element.
        map.remove(&2);

        // The viewer paired with the removed element returns 1.
        assert_eq!(viewers[0].get(), 0);
        assert_eq!(viewers[1].get(), 0);
        assert_eq!(viewers[2].get(), 1);
        assert_eq!(viewers[3].get(), 0);
        assert_eq!(viewers[4].get(), 0);

        // Drop the container.
        drop(map);

        // All viewers returns 1 after the container is dropped.
        assert_eq!(viewers[0].get(), 1);
        assert_eq!(viewers[1].get(), 1);
        assert_eq!(viewers[2].get(), 1);
        assert_eq!(viewers[3].get(), 1);
        assert_eq!(viewers[4].get(), 1);
    }

    #[test]
    fn example_multi_thread() {
        let (counter, viewer) = dropcount::new();

        let handle = thread::spawn(move || {
            drop(counter);
        });

        handle.join().expect("Error in thread.");

        assert_eq!(viewer.get(), 1);
    }
}
