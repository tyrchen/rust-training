// This code is from: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=50b7e4783d908b6fdcf4edede8ce0881
// great thanks to @tony612: https://github.com/tony612
// The API is much cleaner, and feels more like `gen_server`.
// recommended for this version (compared with the live coding version).
use anyhow::Result;
use tokio::sync::{mpsc, mpsc::Receiver, oneshot};

pub trait Actor {
    type Request;
    type Reply;
    fn handle_call(&mut self, msg: Self::Request) -> Result<Self::Reply>;
}

pub struct ActorMessage<Request, Reply> {
    data: Request,
    sender: oneshot::Sender<Reply>,
}

pub fn spawn<A: Actor>(mut actor: A, mailbox: usize) -> Pid<A::Request, A::Reply>
where
    A::Request: Send + 'static,
    A::Reply: Send + 'static,
    A: Send + 'static,
{
    // compiler needs this explicit type
    let (sender, mut receiver): (_, Receiver<ActorMessage<A::Request, A::Reply>>) =
        mpsc::channel(mailbox);
    tokio::spawn(async move {
        while let Some(msg) = receiver.recv().await {
            let reply = actor.handle_call(msg.data).unwrap();
            let _ = msg.sender.send(reply);
        }
    });

    Pid { sender }
}

#[derive(Debug, Clone)]
pub struct Pid<Request, Reply> {
    sender: mpsc::Sender<ActorMessage<Request, Reply>>,
}

impl<Request, Reply> Pid<Request, Reply> {
    pub async fn send(&self, data: Request) -> Result<Reply> {
        let (sender, receiver) = oneshot::channel();
        let msg = ActorMessage { sender, data };
        let _ = self.sender.send(msg).await;
        Ok(receiver.await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MyActor {
        state: usize,
    }

    impl Actor for MyActor {
        type Request = usize;
        type Reply = usize;

        fn handle_call(&mut self, req: Self::Request) -> Result<Self::Reply> {
            self.state += 1;
            println!("state: {}", self.state);
            Ok(req + 1)
        }
    }

    #[tokio::test]
    async fn it_works() {
        let my_actor = MyActor { state: 0 };
        let pid = spawn(my_actor, 20);
        let result = pid.send(42).await.unwrap();
        assert_eq!(result, 43);

        let pid1 = pid.clone();
        let result = pid1.send(100).await.unwrap();
        assert_eq!(result, 101);
    }
}
