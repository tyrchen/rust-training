use anyhow::{anyhow, Result};
use std::collections::VecDeque;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Condvar, Mutex,
};

// Flavours:
// - sync: sender can block, limited capacity
//      - Mutex + Condvar + VecDeque
//      - Atomic VecDeque (atomic queue) + thread::park + thread::notify
// - async: sender cannot block, unbounded.
//      - Mutex + Condvar + VecDeque
//      - Mutex + Condvar + DoubleLinkedList
//      - Atomic LinkedList/Atomic queue of T
//      - crossbeam (Atomic Block linked list, linked list of atomic VecDeque<T>)
// - rendezvous: sync with capacity = 0. Used for thread sync.
//      - Mutex + Condvar
// - oneshot: any capapacity, in practice, only one call to send(). e.g. Ctrl+C to stop all threads
//      - atomic swap
// async/await
//      - waker is different
// crossbeam: high contention environment, flume: low contention environment (it uses Mutex in a clever way)

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let old = self.shared.senders.fetch_add(1, Ordering::AcqRel);
        println!("clone - senders: {}", old);
        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let old = self.shared.senders.fetch_sub(1, Ordering::AcqRel);
        println!("drop - senders: {}", old);
        if old <= 1 {
            println!("notify thread to wake up since no senders left");
            self.shared.available.notify_one();
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) -> Result<()> {
        if self.shared.receivers.load(Ordering::SeqCst) == 0 {
            return Err(anyhow!("no receiver left"));
        }
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.push_back(t);
        drop(inner);
        self.shared.available.notify_one();
        Ok(())
    }
}
pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T> {
        if let Some(t) = self.buffer.pop_front() {
            return Ok(t);
        }

        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                Some(t) => {
                    if !inner.queue.is_empty() {
                        std::mem::swap(&mut self.buffer, &mut inner.queue);
                    }
                    return Ok(t);
                }
                None if self.shared.senders.load(Ordering::SeqCst) == 0 => {
                    return Err(anyhow!("no sender left"))
                }
                None => {
                    inner = self.shared.available.wait(inner).unwrap();
                }
            }
        }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.shared.receivers.fetch_sub(1, Ordering::AcqRel);
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.recv().ok()
    }
}

struct Inner<T> {
    queue: VecDeque<T>,
}

impl<T> Default for Inner<T> {
    fn default() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
    senders: AtomicUsize,
    receivers: AtomicUsize,
}

impl<T> Default for Shared<T> {
    fn default() -> Self {
        Self {
            senders: AtomicUsize::new(1),
            receivers: AtomicUsize::new(1),
            inner: Default::default(),
            available: Condvar::new(),
        }
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Shared::default();
    let shared = Arc::new(shared);
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared,
            buffer: VecDeque::new(),
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(42).unwrap();
        assert_eq!(rx.recv().unwrap(), 42);
    }

    #[test]
    fn closed_tx() {
        let (tx, mut rx) = channel::<()>();
        drop(tx);
        assert!(rx.recv().is_err());
    }

    #[test]
    fn closed_rx() {
        let (mut tx, rx) = channel();
        drop(rx);
        assert!(tx.send(42).is_err());
    }
}
