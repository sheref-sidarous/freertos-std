use crate::cell::RefCell;
use crate::ptr::null_mut;
use crate::sync::Mutex;
use crate::vec::Vec;
use crate::collections::HashMap;
use crate::sys::freertos::freertos_api;

pub type Key = usize;


static DESTRUCTORS : Mutex<RefCell<
                        Vec<
                            Option<unsafe extern "C" fn(*mut u8)>
                        >
                      >>  = Mutex::new(RefCell::new(Vec::new()));

#[inline]
pub unsafe fn create(dtor: Option<unsafe extern "C" fn(*mut u8)>) -> Key {
    let binding = DESTRUCTORS.lock().expect("err");
    let mut destructors = binding.borrow_mut();
    destructors.push(dtor);

    // returning an index would'v been simpler, but 0 has a special meaning as posix's KEY_SENTVAL
    // destructors.len() -1

    destructors.len()
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
    // I head that Rust can do that, and I probably need a Pin<..>
    assert_eq!(list_raw_ptr, list as *mut Vec<*mut u8>);
}

#[inline]
pub unsafe fn get(key: Key) -> *mut u8 {
    let list = unsafe {
        let list_raw_ptr = freertos_api::rust_std_pvTaskGetThreadLocalStoragePointer (
            null_mut(),
            0) as *mut Vec<*mut u8>;

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

    let binding = DESTRUCTORS.lock().expect("err");
    let destructors = binding.borrow();
    let dtor = destructors.get(key).unwrap();
    if let Some(_function) = dtor {
        // TODO: this should actually loop on all local threads and call function for any non-null key value.
        //function(null_mut());
    }
}
