use tokio::sync::{mpsc, oneshot};

pub struct Actor<Request, Reply, State> {
    receiver: mpsc::Receiver<ActorMessage<Request, Reply>>,
    state: State,
}

pub struct ActorMessage<Request, Reply> {
    sender: oneshot::Sender<Reply>,
    data: Request,
}

pub trait HandleCall {
    type State;
    type Reply;
    fn handle_call(&self, state: &mut Self::State) -> Result<Self::Reply, std::io::Error>;
}

#[derive(Clone)]
pub struct Pid<Request, Reply> {
    sender: mpsc::Sender<ActorMessage<Request, Reply>>,
}

impl<Request, Reply, State> Actor<Request, Reply, State>
where
    Request: HandleCall<Reply = Reply, State = State>,
    State: Default + Send + Sync + 'static,
    Request: Send + Sync + 'static,
    Reply: Send + Sync + 'static,
{
    pub fn spawn(mailbox: usize) -> Pid<Request, Reply> {
        let (sender, receiver) = mpsc::channel(mailbox);
        let mut actor: Actor<Request, Reply, State> = Actor {
            receiver,
            state: Default::default(),
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

impl<Request, Reply> Pid<Request, Reply> {
    pub async fn send(
        &self,
        data: Request,
    ) -> Result<Reply, tokio::sync::oneshot::error::RecvError> {
        let (sender, receiver) = oneshot::channel();
        let msg = ActorMessage { sender, data };
        let _ = self.sender.send(msg).await;
        receiver.await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default, PartialEq, Debug)]
    struct Req(u32);

    #[derive(Default, PartialEq, Debug)]
    struct Res(u32);

    #[derive(Default, PartialEq, Debug)]
    struct ActorState(u32);

    impl HandleCall for Req {
        type State = ActorState;

        type Reply = Res;

        fn handle_call(&self, state: &mut Self::State) -> Result<Self::Reply, std::io::Error> {
            state.0 += 1;
            println!("state: {:?}", state);
            Ok(Res(self.0))
        }
    }

    #[tokio::test]
    async fn it_works() {
        let pid: Pid<Req, Res> = Actor::spawn(10);

        let result = pid.send(Req(10)).await.unwrap();
        assert_eq!(result, Res(10));

        let result = pid.send(Req(100)).await.unwrap();
        assert_eq!(result, Res(100));
    }
}
