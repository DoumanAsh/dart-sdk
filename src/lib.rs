//!High level wrapper over Dart runtime

#![no_std]

use core::fmt;
use core::sync::atomic::{AtomicI64, Ordering};

use dart_sdk_sys as sys;

mod object;
mod callback;
pub use object::Object;
pub use callback::Callback;

#[repr(transparent)]
///Representation of Dart Port.
pub struct Port {
    inner: AtomicI64
}

impl Port {
    #[inline]
    ///Creates new instance with provided port.
    pub const fn new(port: sys::Dart_Port) -> Self {
        Self {
            inner: AtomicI64::new(port)
        }
    }

    #[inline]
    ///Sets new port value
    pub fn set_raw(&self, port: sys::Dart_Port) {
        self.inner.store(port, Ordering::Release)
    }


    #[inline]
    ///Gets raw port value
    pub fn get_raw(&self) -> sys::Dart_Port {
        self.inner.load(Ordering::Acquire)
    }

    ///Returns whether data has been successfully posted.
    pub fn send_data(&self, data: Object) -> bool {
        unsafe {
            sys::Dart_Post(self.get_raw(), *data)
        }
    }
}

impl Clone for Port {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: AtomicI64::new(self.get_raw()),
        }
    }
}

impl fmt::Debug for Port {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.get_raw(), fmt)
    }
}


