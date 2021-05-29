use anyhow::Result;
use tokio::sync::{mpsc, oneshot};

pub struct Actor<State, Request, Reply> {
    // receiver side mpsc
    receiver: mpsc::Receiver<ActorMessage<Request, Reply>>,
    state: State,
}

impl<State, Request, Reply> Actor<State, Request, Reply>
where
    State: Default + Send + 'static,
    Request: HandlCall<Reply = Reply, State = State> + Send + 'static,
    Reply: Send + 'static,
{
    pub fn spawn(mailbox: usize) -> Pid<Request, Reply> {
        let (sender, receiver) = mpsc::channel(mailbox);
        let mut actor: Actor<State, Request, Reply> = Actor {
            receiver,
            state: State::default(),
        };

        tokio::spawn(async move {
            while let Some(msg) = actor.receiver.recv().await {
                let reply = msg.data.handle_call(&mut actor.state).unwrap();
                let _ = msg.sender.send(reply);
            }
        });

        Pid { sender }
    }
}

pub struct ActorMessage<Request, Reply> {
    data: Request,
    sender: oneshot::Sender<Reply>,
}

pub trait HandlCall {
    type State;
    type Reply;
    fn handle_call(&self, state: &mut Self::State) -> Result<Self::Reply>;
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

    impl HandlCall for usize {
        type State = usize;
        type Reply = usize;

        fn handle_call(&self, state: &mut Self::State) -> Result<Self::Reply> {
            *state += 1;
            println!("state: {:?}", *state);
            Ok(self + 1)
        }
    }

    #[tokio::test]
    async fn it_works() {
        let pid: Pid<usize, usize> = Actor::spawn(20);
        let result = pid.send(42).await.unwrap();
        assert_eq!(result, 43);

        let pid1 = pid.clone();
        let result = pid1.send(100).await.unwrap();
        assert_eq!(result, 101);
    }
}
