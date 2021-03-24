use dart_sdk_sys as sys;

#[repr(transparent)]
///Wrapper over `Dart_CObject``
pub struct Object {
    inner: sys::Dart_Handle,
}

impl core::ops::Deref for Object {
    type Target = sys::Dart_Handle;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl From<bool> for Object {
    #[inline]
    fn from(data: bool) -> Self {
        Self {
            inner: unsafe {
                sys::Dart_NewBoolean(data)
            }
        }
    }
}

impl From<u64> for Object {
    #[inline]
    fn from(data: u64) -> Self {
        Self {
            inner: unsafe {
                sys::Dart_NewIntegerFromUint64(data)
            }
        }
    }
}

impl From<i64> for Object {
    #[inline]
    fn from(data: i64) -> Self {
        Self {
            inner: unsafe {
                sys::Dart_NewInteger(data)
            }
        }
    }
}

impl From<()> for Object {
    #[inline]
    fn from(_: ()) -> Self {
        Self {
            inner: unsafe {
                sys::Dart_Null()
            }
        }
    }
}

impl<'a> From<&'a str> for Object {
    fn from(data: &'a str) -> Self {
        Self {
            inner: unsafe {
                sys::Dart_NewStringFromUTF8(data.as_ptr(), data.len() as isize)
            }
        }
    }
}
