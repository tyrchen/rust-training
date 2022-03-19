use crate::pb::chat_client::ChatClient;
use crate::pb::*;
use anyhow::Result;
use arc_swap::ArcSwap;
use dashmap::DashMap;
use lazy_static::lazy_static;
use std::ops::Deref;
use std::sync::Arc;
use tonic::codegen::InterceptedService;
use tonic::metadata::AsciiMetadataValue;
use tonic::service::Interceptor;
use tonic::transport::Channel;
use tracing::info;

lazy_static! {
    static ref TOKEN: ArcSwap<Token> = ArcSwap::from(Arc::new(Token {
        data: "".to_string(),
    }));
}

#[derive(Default, Clone)]
struct Rooms(Arc<DashMap<String, Vec<ChatMessage>>>);

pub struct Client {
    username: String,
    conn: ChatClient<InterceptedService<Channel, AuthInterceptor>>,
    rooms: Rooms,
}

impl Deref for Rooms {
    type Target = DashMap<String, Vec<ChatMessage>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Rooms {
    fn insert_message(&self, msg: ChatMessage) {
        let room = msg.room.clone();
        let mut room_messages = self.entry(room).or_insert_with(Vec::new);
        room_messages.push(msg);
    }
}

impl Client {
    pub async fn new(username: impl Into<String>) -> Self {
        let channel = Channel::from_static("http://127.0.0.1:8080")
            .connect()
            .await
            .unwrap();
        let conn = ChatClient::with_interceptor(channel, AuthInterceptor);

        Self {
            username: username.into(),
            conn,
            rooms: Default::default(),
        }
    }

    pub async fn login(&mut self) -> Result<()> {
        let login = LoginRequest::new(&self.username, "password");
        let token = self.conn.login(login).await?.into_inner();
        TOKEN.store(Arc::new(token));
        Ok(())
    }

    pub async fn send_message(
        &mut self,
        room: impl Into<String>,
        content: impl Into<String>,
    ) -> Result<()> {
        let msg = NewChatMessage::new(room, content);
        self.conn.send_message(msg).await?;
        Ok(())
    }

    pub async fn get_messages(&mut self) -> Result<()> {
        let req = GetMessagesRequest::new();
        let mut stream = self.conn.get_messages(req).await?.into_inner();
        let rooms = self.rooms.clone();
        tokio::spawn(async move {
            while let Some(msg) = stream.message().await? {
                info!("got message: {msg:?}");
                rooms.insert_message(msg);
            }
            Ok::<_, tonic::Status>(())
        });

        Ok(())
    }
}

struct AuthInterceptor;

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut req: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
        let token = TOKEN.load();

        if token.is_valid() {
            let value = AsciiMetadataValue::from_str(&format!("Bearer {}", token.data)).unwrap();
            req.metadata_mut().insert("authorization", value);
        }

        Ok(req)
    }
}
