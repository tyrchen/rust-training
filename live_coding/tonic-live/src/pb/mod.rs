mod abi;

pub use abi::*;

impl Token {
    pub fn new(data: impl Into<String>) -> Self {
        Self { data: data.into() }
    }

    /// TODO: use jwt for decode token
    pub fn into_username(&self) -> String {
        self.data.clone()
    }

    pub fn is_valid(&self) -> bool {
        self.data.len() > 0
    }
}
impl LoginRequest {
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }

    /// TODO: use jwt token instead
    pub fn into_token(&self) -> Token {
        Token::new(&self.username)
    }
}

impl NewChatMessage {
    pub fn new(room: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            room: room.into(),
            content: content.into(),
        }
    }

    pub fn into_chat_message(self, sender: impl Into<String>) -> ChatMessage {
        ChatMessage::new(sender, self.room, self.content)
    }
}

impl ChatMessage {
    pub fn new(
        sender: impl Into<String>,
        room: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        let timestamp = chrono::Utc::now().timestamp();
        Self {
            sender: sender.into(),
            room: room.into(),
            content: content.into(),
            timestamp,
        }
    }
}

impl GetMessagesRequest {
    pub fn new() -> Self {
        Self {}
    }
}
