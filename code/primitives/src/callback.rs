use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::{channel, Receiver, Sender},
    },
    thread,
    time::Duration,
};

thread_local! {
    static RT: Runtime = Runtime::new();
}

pub struct Runtime {
    callbacks: RefCell<HashMap<usize, Box<dyn FnOnce() -> ()>>>,
    next_id: AtomicUsize,
    evt_sender: Sender<usize>,
    evt_receiver: Receiver<usize>,
}

impl Runtime {
    pub fn new() -> Self {
        let (s, r) = channel();
        Runtime {
            callbacks: RefCell::new(HashMap::new()),
            next_id: AtomicUsize::new(1),
            evt_sender: s,
            evt_receiver: r,
        }
    }

    pub fn run(&self, program: fn()) {
        program();
        for id in &self.evt_receiver {
            let cb = self.callbacks.borrow_mut().remove(&id).unwrap();
            cb();
            if self.callbacks.borrow().is_empty() {
                break;
            }
        }
    }
}

pub fn set_timeout(ms: u64, cb: impl FnOnce() + 'static) {
    RT.with(|rt| {
        let id = rt.next_id.fetch_add(1, Ordering::Relaxed);
        rt.callbacks.borrow_mut().insert(id, Box::new(cb));
        let sender = rt.evt_sender.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(ms));
            sender.send(id).unwrap();
        });
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn program() {
        println!("So we start the program here!");
        set_timeout(200, || {
            println!("We create tasks with a callback that runs once the task finished!");
        });
        set_timeout(100, || {
            println!("We can even chain sub-tasks...");
            set_timeout(50, || {
                println!("...like this!");
            })
        });
        println!("While our tasks are executing we can do other stuff instead of waiting.");
    }
    #[test]
    fn it_works() {
        RT.with(|rt| rt.run(program));
    }
}
