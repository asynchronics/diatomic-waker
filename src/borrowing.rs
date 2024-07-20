//! Non-allcating, safe, statically allocated wrapper for `DiatomicWaker`.
//!
//! See the [crate-level documentation](crate) for usage.

use std::task::Waker;

use crate::primitives::DiatomicWaker;
use crate::primitives::WaitUntil;

/// A safe, statically allocated `DiatomicWaker`
///
/// See the [crate-level documentation](crate) for usage.
#[derive(Default, Debug)]
pub struct BorrowingDiatomicWaker {
    inner: DiatomicWaker,
}

/// An object that can await a notification from one or several
/// [`WakeSource`](WakeSource)s.
///
/// See the [crate-level documentation](crate) for usage.
#[derive(Debug)]
pub struct WakeSink<'a> {
    /// The shared data.
    inner: &'a DiatomicWaker,
}
/// An object that can send a notification to a [`WakeSink`](WakeSink).
///
/// See the [crate-level documentation](crate) for usage.
#[derive(Clone, Debug)]
pub struct WakeSource<'a> {
    /// The shared data.
    inner: &'a DiatomicWaker,
}

impl BorrowingDiatomicWaker {
    /// Creates [`BorrowingDiatomicWaker`]
    pub const fn new() -> Self {
        Self {
            inner: DiatomicWaker::new(),
        }
    }
    /// Splits to [`WakeSink`] and [`WakeSource`]. Mutably borrows `self`,
    /// ensuring that multiple [`WakeSink`] to the same [`DiatomicWaker`] are
    /// impossible
    pub fn split(&mut self) -> (WakeSink, WakeSource) {
        let inner = &self.inner;
        (WakeSink { inner }, WakeSource { inner })
    }
}

impl WakeSource<'_> {
    /// Notifies the sink if a waker is registered.
    #[inline]
    pub fn notify(&self) {
        self.inner.notify();
    }
}

impl WakeSink<'_> {
    /// Creates a new source.
    #[inline]
    pub fn source(&self) -> WakeSource {
        WakeSource { inner: self.inner }
    }

    /// Registers a new waker.
    ///
    /// Registration is lazy: the waker is cloned only if it differs from the
    /// last registered waker (note that the last registered waker is cached
    /// even if it was unregistered).
    #[inline]
    pub fn register(&mut self, waker: &Waker) {
        // Safety: `WakePrimitive::register`, `WakePrimitive::unregister` and
        // `WakePrimitive::wait_until` cannot be used concurrently from multiple
        // thread since `WakeSink` does not implement `Clone` and the wrappers
        // of the above methods require exclusive ownership to `WakeSink`.
        unsafe { self.inner.register(waker) };
    }

    /// Unregisters the waker.
    ///
    /// After the waker is unregistered, subsequent calls to
    /// `WakeSource::notify` will be ignored.
    #[inline]
    pub fn unregister(&mut self) {
        // Safety: `WakePrimitive::register`, `WakePrimitive::unregister` and
        // `WakePrimitive::wait_until` cannot be used concurrently from multiple
        // thread since `WakeSink` does not implement `Clone` and the wrappers
        // of the above methods require exclusive ownership to `WakeSink`.
        unsafe { self.inner.unregister() };
    }

    /// Returns a future that can be `await`ed until the provided predicate
    /// returns a value.
    ///
    /// The predicate is checked each time a notification is received.
    #[inline]
    pub fn wait_until<P, T>(&mut self, predicate: P) -> WaitUntil<'_, P, T>
    where
        P: FnMut() -> Option<T> + Unpin,
    {
        // Safety: `WakePrimitive::register`, `WakePrimitive::unregister` and
        // `WakePrimitive::wait_until` cannot be used concurrently from multiple
        // thread since `WakeSink` does not implement `Clone` and the wrappers
        // of the above methods require exclusive ownership to `WakeSink`.
        unsafe { self.inner.wait_until(predicate) }
    }
}
