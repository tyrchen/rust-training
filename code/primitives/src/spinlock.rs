use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Lock<T> {
    locked: AtomicBool, // ***
    data: UnsafeCell<T>,
}

unsafe impl<T> Sync for Lock<T> where T: Send {}

impl<T> Lock<T> {
    pub fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock<R>(&self, op: impl FnOnce(&mut T) -> R) -> R {
        // spin if we can't get lock
        while self
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            while self.locked.load(Ordering::Relaxed) == true {
                // we may yield thread now
                std::thread::yield_now();
            }
        }
        // ha, we can lock and do our job
        // execute the op as we got lock
        let ret = op(unsafe { &mut *self.data.get() }); // **3
                                                        // unlock
        self.locked.store(false, Ordering::Release); // **4
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::spawn;

    #[test]
    fn it_works() {
        let l: &'static _ = Box::leak(Box::new(Lock::new(0)));
        let nthreads = 2;
        let nloops = 1000;
        let handles: Vec<_> = (0..nthreads)
            .map(|_| {
                spawn(move || {
                    for _ in 0..nloops {
                        l.lock(|v| {
                            *v += 1;
                        })
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(l.lock(|v| *v), nthreads * nloops);
    }
}
