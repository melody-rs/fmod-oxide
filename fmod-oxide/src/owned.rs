use crate::{FmodResultExt, Result};
use std::ptr::NonNull;

pub struct Owned<T: Resource> {
    pub(crate) raw: NonNull<T::Raw>,
}

unsafe impl<T: Send + Resource> Send for Owned<T> {}
unsafe impl<T: Sync + Resource> Sync for Owned<T> {}

pub trait Resource: std::fmt::Debug {
    type Raw;

    fn release(this: NonNull<Self::Raw>) -> Result<()>;

    fn from_raw<'a>(raw: NonNull<Self::Raw>) -> &'a Self;
}

pub trait HasRelease: Resource {}

impl<T: Resource> Owned<T> {
    pub(crate) fn new(raw: *mut T::Raw) -> Self {
        let raw = NonNull::new(raw).unwrap();
        Self { raw }
    }

    pub fn as_resource(&self) -> &T {
        T::from_raw(self.raw)
    }
}

impl<T: HasRelease> Owned<T> {
    pub fn release(self) -> Result<()> {
        T::release(self.raw)
    }
}

impl Owned<crate::studio::System> {
    ///This function will free the memory used by the Studio System object and everything created under it.
    ///
    /// # Safety
    ///
    /// Calling either of this function concurrently with any FMOD Studio API function (including this function) may cause undefined behavior.
    /// External synchronization must be used if calls to [`SystemBuilder::new`] or [`System::release`] could overlap other FMOD Studio API calls.
    /// All other FMOD Studio API functions are thread safe and may be called freely from any thread unless otherwise documented.
    ///
    /// All handles or pointers to objects associated with a Studio System object become invalid when the Studio System object is released.
    /// The FMOD Studio API attempts to protect against stale handles and pointers being used with a different Studio System object but this protection cannot be guaranteed and attempting to use stale handles or pointers may cause undefined behavior.
    ///
    /// This function is not safe to be called at the same time across multiple threads.
    pub unsafe fn release(self) -> Result<()> {
        unsafe { fmod_sys::FMOD_Studio_System_Release(self.raw.as_ptr()).to_result() }
    }
}

impl<T: Resource> Drop for Owned<T> {
    fn drop(&mut self) {
        T::release(self.raw).expect("failed to release an Owned handle");
    }
}

impl<T: Resource> std::ops::Deref for Owned<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.as_resource()
    }
}

impl<T: Resource> std::fmt::Debug for Owned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (**self).fmt(f)
    }
}
