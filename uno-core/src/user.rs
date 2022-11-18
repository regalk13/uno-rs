use std::fmt::Display;

/// Different levels that an user can be
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Level {
    /// Anonymous user
    Guess,
    /// Registered user
    User,
    /// System user [ app/serve itself ]
    System,
}

/// User information
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    /// user's name
    name: String,
    /// user's level
    level: Level,
}

impl User {
    pub const SYSTEM_NAME: &str = "System";

    pub fn new(name: String, guess: bool) -> Self {
        Self {
            name,
            level: if guess { Level::Guess } else { Level::User },
        }
    }

    pub fn new_system() -> Self {
        Self {
            name: User::SYSTEM_NAME.to_owned(),
            level: Level::System,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn level(&self) -> Level {
        self.level
    }
}

impl Display for User {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.name)
    }
}
