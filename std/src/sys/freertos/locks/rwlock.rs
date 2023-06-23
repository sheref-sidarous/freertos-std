use crate::sync::{Mutex, Condvar};

struct RwCounters {
    readers : usize,
    writers : usize
}
pub struct RwLock {
    counters: Mutex<RwCounters>,
    waiting : Condvar
}

unsafe impl Send for RwLock {}
unsafe impl Sync for RwLock {}

impl RwLock {
    #[inline]
    #[rustc_const_stable(feature = "const_locks", since = "1.63.0")]
    pub const fn new() -> RwLock {
        RwLock {
            counters : Mutex::new(RwCounters { readers :0, writers :0}),
            waiting : Condvar::new(),
        }
    }

    #[inline]
    pub fn read(&self) {
        let mut counters = self.counters.lock().unwrap();
        while counters.writers > 0 {
            // there is a pending writer
            counters = self.waiting.wait(counters).unwrap();
        }
        counters.readers += 1;
        return;
    }

    #[inline]
    pub fn try_read(&self) -> bool {
        let mut counters = self.counters.lock().unwrap();
        if counters.writers > 0 {
            // there is a pending writer
            return false;
        } else {
            counters.readers += 1;
            return true;
        }
    }

    #[inline]
    pub fn write(&self) {
        let mut counters = self.counters.lock().unwrap();
        while counters.writers > 0 && counters.readers > 0{
            // there is a pending writer or reader
            counters = self.waiting.wait(counters).unwrap();
        }
        counters.writers += 1;
        return;
    }

    #[inline]
    pub fn try_write(&self) -> bool {
        let mut counters = self.counters.lock().unwrap();
        if counters.writers > 0 && counters.readers > 0{
            // there is a pending writer or reader
            return false;
        } else {
            counters.writers += 1;
            return true;
        }
    }

    #[inline]
    pub unsafe fn read_unlock(&self) {
        let mut counters = self.counters.lock().unwrap();
        //assert_gte!(counters.readers, 1);
        counters.readers -= 1;
        self.waiting.notify_all();
    }

    #[inline]
    pub unsafe fn write_unlock(&self) {
        let mut counters = self.counters.lock().unwrap();
        //assert_gte!(counters.writers, 1);
        counters.writers -= 1;
        self.waiting.notify_all();
    }
}
