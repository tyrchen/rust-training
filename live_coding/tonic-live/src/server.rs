use crate::pb::chat_server::{Chat, ChatServer};
use crate::pb::*;
use futures::prelude::*;
use std::pin::Pin;
use tokio::sync::{broadcast, mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::transport::Server;
use tonic::{Extensions, Request, Response, Status};
use tracing::{info, warn};

const MAX_MESSAGES: usize = 1024;

pub struct ChatService {
    tx: broadcast::Sender<ChatMessage>,
}

pub type ChatResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl Chat for ChatService {
    async fn login(&self, request: Request<LoginRequest>) -> ChatResult<Token> {
        let info = request.into_inner();
        info!("login: {info:?}");
        let token = info.into_token();
        Ok(Response::new(token))
    }

    async fn send_message(
        &self,
        request: Request<NewChatMessage>,
    ) -> ChatResult<SendMessageResponse> {
        // how to get sender from request?
        let sender = get_username(request.extensions())?;
        let info = request.into_inner();
        info!("send_message: {info:?}");
        let msg = info.into_chat_message(sender);
        // store it to the server storage
        // publish message to everyone who interested in it
        self.tx.send(msg).unwrap();
        Ok(Response::new(SendMessageResponse {}))
    }

    type GetMessagesStream = Pin<Box<dyn Stream<Item = Result<ChatMessage, tonic::Status>> + Send>>;

    async fn get_messages(
        &self,
        request: Request<GetMessagesRequest>,
    ) -> ChatResult<Self::GetMessagesStream> {
        let info = request.into_inner();
        info!("subscribe to messages {info:?}");
        let mut rx = self.tx.subscribe();
        let (sender, receiver) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            // TODO: filter out messages I'm not interested in
            while let Ok(msg) = rx.recv().await {
                if let Err(_) = sender.send(Ok(msg)) {
                    warn!("Failed to send. Sender might be closed.");
                    return;
                }
            }
        });

        let stream = UnboundedReceiverStream::new(receiver);
        Ok(Response::new(Box::pin(stream)))
    }
}

impl Default for ChatService {
    fn default() -> Self {
        let (tx, _rx) = broadcast::channel(MAX_MESSAGES);
        Self { tx }
    }
}

pub async fn start() {
    let svc = ChatServer::with_interceptor(ChatService::default(), check_auth);
    let addr = "0.0.0.0:8080".parse().unwrap();
    info!("listening on http://{}", addr);
    Server::builder()
        .add_service(svc)
        .serve(addr)
        .await
        .unwrap();
}

fn check_auth(mut req: Request<()>) -> Result<Request<()>, Status> {
    let token = match req.metadata().get("authorization") {
        Some(v) => {
            let data = v
                .to_str()
                .map_err(|_| Status::new(tonic::Code::Unauthenticated, "Invalid token format"))?;
            Token::new(data.strip_prefix("Bearer ").unwrap())
        }
        None => Token::default(),
    };
    req.extensions_mut().insert(token);
    Ok(req)
}

fn get_username(ext: &Extensions) -> Result<String, Status> {
    let token = ext
        .get::<Token>()
        .ok_or(Status::unauthenticated("No token"))?;
    if token.is_valid() {
        Ok(token.into_username())
    } else {
        Err(Status::unauthenticated("Invalid token"))
    }
}
