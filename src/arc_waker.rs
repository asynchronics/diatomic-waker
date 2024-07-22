use alloc::sync::Arc;
use core::task::Waker;

use crate::primitives::DiatomicWaker;
use crate::primitives::WaitUntil;

/// An object that can await a notification from one or several
/// [`WakeSource`](WakeSource)s.
///
/// See the [crate-level documentation](crate) for usage.
#[derive(Debug, Default)]
pub struct WakeSink {
    /// The shared data.
    inner: Arc<DiatomicWaker>,
}

impl WakeSink {
    /// Creates a new sink.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(DiatomicWaker::new()),
        }
    }

    /// Creates a new source.
    #[inline]
    pub fn source(&self) -> WakeSource {
        WakeSource {
            inner: self.inner.clone(),
        }
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

/// An object that can send a notification to a [`WakeSink`](WakeSink).
///
/// See the [crate-level documentation](crate) for usage.
#[derive(Clone, Debug)]
pub struct WakeSource {
    /// The shared data.
    inner: Arc<DiatomicWaker>,
}

impl WakeSource {
    /// Notifies the sink if a waker is registered.
    #[inline]
    pub fn notify(&self) {
        self.inner.notify();
    }
}
