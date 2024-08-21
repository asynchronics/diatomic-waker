#[cfg(feature = "diatomic-waker-loom")]
#[allow(unused_imports)]
pub(crate) mod sync {
    pub(crate) mod atomic {
        pub(crate) use loom::sync::atomic::AtomicUsize;
    }
}
#[cfg(not(feature = "diatomic-waker-loom"))]
#[allow(unused_imports)]
pub(crate) mod sync {
    pub(crate) mod atomic {
        pub(crate) use core::sync::atomic::AtomicUsize;
    }
}

#[cfg(feature = "diatomic-waker-loom")]
pub(crate) mod cell {
    pub(crate) use loom::cell::UnsafeCell;
}
#[cfg(not(feature = "diatomic-waker-loom"))]
pub(crate) mod cell {
    #[derive(Debug)]
    pub(crate) struct UnsafeCell<T>(core::cell::UnsafeCell<T>);

    impl<T> UnsafeCell<T> {
        pub(crate) const fn new(data: T) -> UnsafeCell<T> {
            UnsafeCell(core::cell::UnsafeCell::new(data))
        }
        pub(crate) fn with<R>(&self, f: impl FnOnce(*const T) -> R) -> R {
            f(self.0.get())
        }
        pub(crate) fn with_mut<R>(&self, f: impl FnOnce(*mut T) -> R) -> R {
            f(self.0.get())
        }
    }
}
