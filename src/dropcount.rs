use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// A view for the count of destruction.
///
/// An instance of this type views how many times are called the [Counter]'s destructor.
#[derive(Debug, Clone, Default)]
pub struct Viewer(Arc<AtomicUsize>);

impl Viewer {
    /// Gets destruction count.
    ///
    /// This should return 0 or 1.
    /// The 1 means the [Counter] instance paired with this has been destructed, and 0 not.
    /// If any other values are returned, it must be a bug of resource management.
    pub fn get(&self) -> usize {
        self.0.load(Ordering::Relaxed)
    }

    fn inc(&self) {
        let prev = self.0.fetch_add(1, Ordering::Relaxed);
        assert!(prev < usize::MAX);
    }
}

/// A type for counting destruction.
///
/// An instance of this type increments the internal value when its destructor is called.
/// The value can be observed through a [Viewer] instance paired with this.
#[derive(Debug)]
pub struct Counter(Viewer);

impl Counter {
    /// Gets destruction count.
    ///
    /// Normally, this should always return 0 since this method can be called only for living
    /// instances.
    pub fn get(&self) -> usize {
        self.0.get()
    }

    /// Create a new individual counter.
    ///
    /// The returned instance does not share internal value with a [Viewer].
    pub fn new() -> Self {
        new().0
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Counter {
    fn drop(&mut self) {
        self.0.inc();
    }
}

/// Create a pair of [Counter] and [Viewer].
///
/// The returned objects share an internal value.
pub fn new() -> (Counter, Viewer) {
    let arc = Arc::new(AtomicUsize::new(0));
    let viewer = Viewer(arc.clone());
    let counter = Counter(viewer.clone());
    (counter, viewer)
}

/// Create a pair of multiple [Counter]s and [Viewer]s.
///
/// Elements of the returned arrays share each internal value respectively.
pub fn new_vec(size: usize) -> (Vec<Counter>, Vec<Viewer>) {
    (0..size).map(|_| new()).unzip()
}

#[cfg(test)]
mod tests {
    use crate::dropcount;

    #[test]
    fn new() {
        let (counter, viewer) = dropcount::new();

        assert_eq!(viewer.get(), 0);
        assert_eq!(counter.get(), 0);

        drop(counter);
        assert_eq!(viewer.get(), 1);
    }

    #[test]
    fn new_vec() {
        let (counters, viewers) = dropcount::new_vec(5);

        assert_eq!(viewers.len(), 5);
        assert_eq!(counters.len(), 5);

        assert_eq!(viewers[0].get(), 0);
        assert_eq!(viewers[1].get(), 0);
        assert_eq!(viewers[2].get(), 0);
        assert_eq!(viewers[3].get(), 0);
        assert_eq!(viewers[4].get(), 0);

        assert_eq!(counters[0].get(), 0);
        assert_eq!(counters[1].get(), 0);
        assert_eq!(counters[2].get(), 0);
        assert_eq!(counters[3].get(), 0);
        assert_eq!(counters[4].get(), 0);

        drop(counters);

        assert_eq!(viewers[0].get(), 1);
        assert_eq!(viewers[1].get(), 1);
        assert_eq!(viewers[2].get(), 1);
        assert_eq!(viewers[3].get(), 1);
        assert_eq!(viewers[4].get(), 1);
    }

    #[test]
    fn vec_pop() {
        let (mut counters, viewers) = dropcount::new_vec(5);

        assert_eq!(viewers.len(), 5);
        assert_eq!(counters.len(), 5);

        assert_eq!(viewers[0].get(), 0);
        assert_eq!(viewers[1].get(), 0);
        assert_eq!(viewers[2].get(), 0);
        assert_eq!(viewers[3].get(), 0);
        assert_eq!(viewers[4].get(), 0);

        assert_eq!(counters[0].get(), 0);
        assert_eq!(counters[1].get(), 0);
        assert_eq!(counters[2].get(), 0);
        assert_eq!(counters[3].get(), 0);
        assert_eq!(counters[4].get(), 0);

        counters.pop();

        assert_eq!(viewers[0].get(), 0);
        assert_eq!(viewers[1].get(), 0);
        assert_eq!(viewers[2].get(), 0);
        assert_eq!(viewers[3].get(), 0);
        assert_eq!(viewers[4].get(), 1);

        assert_eq!(counters[0].get(), 0);
        assert_eq!(counters[1].get(), 0);
        assert_eq!(counters[2].get(), 0);
        assert_eq!(counters[3].get(), 0);

        counters.pop();

        assert_eq!(viewers[0].get(), 0);
        assert_eq!(viewers[1].get(), 0);
        assert_eq!(viewers[2].get(), 0);
        assert_eq!(viewers[3].get(), 1);
        assert_eq!(viewers[4].get(), 1);

        assert_eq!(counters[0].get(), 0);
        assert_eq!(counters[1].get(), 0);
        assert_eq!(counters[2].get(), 0);
    }

    #[test]
    fn vec_drain() {
        let (mut counters, viewers) = dropcount::new_vec(5);

        assert_eq!(viewers.len(), 5);
        assert_eq!(counters.len(), 5);

        assert_eq!(viewers[0].get(), 0);
        assert_eq!(viewers[1].get(), 0);
        assert_eq!(viewers[2].get(), 0);
        assert_eq!(viewers[3].get(), 0);
        assert_eq!(viewers[4].get(), 0);

        assert_eq!(counters[0].get(), 0);
        assert_eq!(counters[1].get(), 0);
        assert_eq!(counters[2].get(), 0);
        assert_eq!(counters[3].get(), 0);
        assert_eq!(counters[4].get(), 0);

        counters.drain(1..3);

        assert_eq!(viewers[0].get(), 0);
        assert_eq!(viewers[1].get(), 1);
        assert_eq!(viewers[2].get(), 1);
        assert_eq!(viewers[3].get(), 0);
        assert_eq!(viewers[4].get(), 0);

        assert_eq!(counters[0].get(), 0);
        assert_eq!(counters[1].get(), 0);
        assert_eq!(counters[2].get(), 0);

        counters.drain(..);

        assert_eq!(viewers[0].get(), 1);
        assert_eq!(viewers[1].get(), 1);
        assert_eq!(viewers[2].get(), 1);
        assert_eq!(viewers[3].get(), 1);
        assert_eq!(viewers[4].get(), 1);
    }

    #[test]
    fn vec_push() {
        let (mut counters, viewers) = dropcount::new_vec(5);

        assert_eq!(viewers.len(), 5);
        assert_eq!(counters.len(), 5);

        assert_eq!(viewers[0].get(), 0);
        assert_eq!(viewers[1].get(), 0);
        assert_eq!(viewers[2].get(), 0);
        assert_eq!(viewers[3].get(), 0);
        assert_eq!(viewers[4].get(), 0);

        assert_eq!(counters[0].get(), 0);
        assert_eq!(counters[1].get(), 0);
        assert_eq!(counters[2].get(), 0);
        assert_eq!(counters[3].get(), 0);
        assert_eq!(counters[4].get(), 0);

        counters.push(dropcount::Counter::new());
        counters.push(dropcount::Counter::new());
        counters.push(dropcount::Counter::new());

        assert_eq!(viewers[0].get(), 0);
        assert_eq!(viewers[1].get(), 0);
        assert_eq!(viewers[2].get(), 0);
        assert_eq!(viewers[3].get(), 0);
        assert_eq!(viewers[4].get(), 0);

        assert_eq!(counters[0].get(), 0);
        assert_eq!(counters[1].get(), 0);
        assert_eq!(counters[2].get(), 0);
        assert_eq!(counters[3].get(), 0);
        assert_eq!(counters[4].get(), 0);
        assert_eq!(counters[5].get(), 0);
        assert_eq!(counters[6].get(), 0);
        assert_eq!(counters[7].get(), 0);

        counters.drain(3..7);

        assert_eq!(viewers[0].get(), 0);
        assert_eq!(viewers[1].get(), 0);
        assert_eq!(viewers[2].get(), 0);
        assert_eq!(viewers[3].get(), 1);
        assert_eq!(viewers[4].get(), 1);

        assert_eq!(counters[0].get(), 0);
        assert_eq!(counters[1].get(), 0);
        assert_eq!(counters[2].get(), 0);
        assert_eq!(counters[3].get(), 0);
    }

    #[test]
    fn new_counter() {
        let counter = dropcount::Counter::new();

        assert_eq!(counter.get(), 0);
    }

    #[test]
    fn default_counter() {
        let counter = dropcount::Counter::default();

        assert_eq!(counter.get(), 0);
    }

    #[test]
    fn do_drop() {
        let (counter, viewer) = dropcount::new();

        assert_eq!(viewer.get(), 0);
        assert_eq!(counter.get(), 0);

        drop(counter);

        assert_eq!(viewer.get(), 1);
    }

    #[test]
    fn drop_in_place() {
        let (mut counter, viewer) = dropcount::new();

        assert_eq!(viewer.get(), 0);
        assert_eq!(counter.get(), 0);

        unsafe {
            std::ptr::drop_in_place(&mut counter as *mut _);
        }

        assert_eq!(viewer.get(), 1);
        assert_eq!(counter.get(), 1);

        std::mem::forget(counter);
    }

    #[test]
    fn drop_after_forget() {
        let ptr: *mut crate::dropcount::Counter;
        {
            let (mut counter, viewer) = dropcount::new();
            ptr = &mut counter as *mut _;

            assert_eq!(viewer.get(), 0);
            assert_eq!(counter.get(), 0);

            std::mem::forget(counter);
        }

        unsafe {
            std::ptr::drop_in_place(ptr);
            assert_eq!(ptr.as_ref().unwrap().get(), 1);
        }
    }
}
