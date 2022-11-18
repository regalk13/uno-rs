use crate::user::User;

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    sender: User,
    content: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Chat {
    /// history of chat, the last messages are more recent
    /// than first messages
    history: Vec<Message>,
}

impl Message {
    pub fn new(sender: User, content: String) -> Self {
        Self { sender, content }
    }

    pub fn sender(&self) -> &User {
        &self.sender
    }

    pub fn content(&self) -> &str {
        &self.content
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
