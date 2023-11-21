use crate::cell::{RefCell, RefMut, Ref};
use crate::ptr::null_mut;
use crate::sys::freertos::locks::Mutex;
use crate::vec::Vec;
use crate::collections::HashMap;
use crate::sys::freertos::freertos_api;

pub type Key = usize;

struct DestructorsWrapper {
    inner: RefCell<Vec<Option<unsafe extern "C" fn(*mut u8)>>>,
    lock: Mutex,
}

unsafe impl Sync for DestructorsWrapper {} // will use primitive locking to guard it

impl DestructorsWrapper {
    const fn new() -> Self {
        DestructorsWrapper {
            inner: RefCell::new(Vec::new()),
            lock: Mutex::new()
        }
    }

    fn lock(&self) {
        self.lock.lock();
    }

    fn unlock(&self) {
        unsafe { self.lock.unlock(); }
    }

    fn borrow_inner_mut(&self) -> RefMut<'_, Vec<Option<unsafe extern "C" fn(*mut u8)>>> {
        self.inner.borrow_mut()
    }

    fn borrow_inner(&self) -> Ref<'_, Vec<Option<unsafe extern "C" fn(*mut u8)>>> {
        self.inner.borrow()

    }
}
static DESTRUCTORS : DestructorsWrapper = DestructorsWrapper::new();

#[inline]
pub unsafe fn create(dtor: Option<unsafe extern "C" fn(*mut u8)>) -> Key {
    DESTRUCTORS.lock();
    let mut destructors = DESTRUCTORS.borrow_inner_mut();
    destructors.push(dtor);
    let ret =  destructors.len();
    DESTRUCTORS.unlock();

    // returning an index would'v been simpler, but 0 has a special meaning as posix's KEY_SENTVAL
    // destructors.len() -1

    ret


}

#[inline]
pub unsafe fn set(key: Key, value: *mut u8) {
    // get the thread-specific map
    let list_raw_ptr;
    let list : &mut Vec<*mut u8> = unsafe {
        list_raw_ptr = freertos_api::rust_std_pvTaskGetThreadLocalStoragePointer (
            null_mut(),
            0) as *mut Vec<*mut u8>;

        &mut *list_raw_ptr
    };

    // remember, index is actually off-by-one to avoid the key value of Zero
    let index = key -1;

    if index >=  list.len() {
        // need to expand the Vector
        list.resize(index + 1, null_mut())
    }
    list[index] = value;

    // As the vector is resized, can it change its base address ?
    // I hear that Rust can do that, and I probably need a Pin<..>
    assert_eq!(list_raw_ptr, list as *mut Vec<*mut u8>);
}

#[inline]
pub unsafe fn get(key: Key) -> *mut u8 {
    let list = unsafe {
        let list_raw_ptr = freertos_api::rust_std_pvTaskGetThreadLocalStoragePointer (
            null_mut(),
            0) as *mut Vec<*mut u8>;

         assert_ne!(list_raw_ptr, null_mut());

        &*list_raw_ptr
    };

    // remember, index is actually off-by-one to avoid the key value of Zero
    let index = key -1;

    if index >= list.len() {
        null_mut()
    } else {
        list[index]
    }
}

#[inline]
pub unsafe fn destroy(key: Key) {

    DESTRUCTORS.lock();
    let mut destructors = DESTRUCTORS.borrow_inner_mut();
    let dtor = destructors.get(key).unwrap();
    if let Some(_function) = dtor {
        // TODO: this should actually loop on all local threads and call function for any non-null key value.
        //function(null_mut());
    }
    DESTRUCTORS.unlock();
}
