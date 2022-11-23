use crate::user::User;
use chrono::Utc;

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    sender: User,
    content: String,
    /// UTC Timestamp of non-leap-milliseconds since January 1, 1970 UTC
    create: i64,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Chat {
    /// history of chat, the last messages are more recent
    /// than first messages
    history: Vec<Message>,
}

impl Message {
    pub fn new(sender: User, content: String) -> Self {
        Self {
            sender,
            content,
            create: Utc::now().timestamp_millis(),
        }
    }

    pub fn sender(&self) -> &User {
        &self.sender
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn create_timestamp(&self) -> i64 {
        self.create
    }
}

impl Chat {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.history.is_empty()
    }

    pub fn at(&self, index: usize) -> &Message {
        &self.history[index]
    }

    pub fn push(&mut self, message: Message) {
        self.history.push(message)
    }

    pub fn delete(&mut self, index: usize) {
        self.history.remove(index);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Message> {
        self.history.iter()
    }
}

impl AsRef<[Message]> for Chat {
    fn as_ref(&self) -> &[Message] {
        &self.history
    }
}

impl IntoIterator for Chat {
    type Item = Message;
    type IntoIter = std::vec::IntoIter<Message>;

    fn into_iter(self) -> Self::IntoIter {
        self.history.into_iter()
    }
}
