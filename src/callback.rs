use dart_sdk_sys as sys;

use core::ptr;

///Describes Callback to be invoked from Dart
pub struct Callback<FN> {
    inner: FN,
}

impl<T> Callback<T> {
    ///Creates new instance
    pub const fn new(inner: T) -> Self {
        Self {
            inner,
        }
    }
}

impl<FN: FnMut(&'_ str)> Callback<FN> {
    ///Gets pointer to extern function to be invoked by dart
    fn get_dart_callback(&'static self) -> unsafe extern "C" fn(sys::Dart_Handle) {
        rust_dart_sdk_string_handler
    }
}

pub unsafe extern "C" fn rust_dart_sdk_string_handler(data: sys::Dart_Handle) {
    let mut ptr = ptr::null_mut();
    let mut length = 0;

    sys::Dart_EnterScope();
    let result = sys::Dart_StringToUTF8(data, &mut ptr, &mut length);

    if sys::Dart_IsError(result) {
        //Scope is auto-cleaned on propagate
        sys::Dart_PropagateError(result);
        return;
    }

    let data = core::str::from_utf8_unchecked(core::slice::from_raw_parts(ptr, length as usize));

    sys::Dart_ExitScope();
}
