use crate::snowflake::Snowflake;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Different levels that an user can be
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Level {
    /// Anonymous user
    Guess,
    /// Registered user
    User,
    /// System user [ app/serve itself ]
    System,
}

/// User Authentication
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LoginUser {
    pub username: Option<String>,
    pub password: Option<String>,
}

/// User information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// User's unique snowflake
    id: Snowflake,
    /// user's name
    name: String,
    /// user's level
    level: Level,
}

impl User {
    pub const SYSTEM_NAME: &str = "System";
    pub const SYSTEM_ID: Snowflake = Snowflake(1);

    pub fn new(name: String, guess: bool) -> Self {
        Self {
            id: Snowflake(0),
            name,
            level: if guess { Level::Guess } else { Level::User },
        }
    }

    pub fn with_id(id: Snowflake, name: String, guess: bool) -> Self {
        Self {
            id,
            name,
            level: if guess { Level::Guess } else { Level::User },
        }
    }

    pub fn new_system() -> Self {
        Self {
            id: User::SYSTEM_ID,
            name: User::SYSTEM_NAME.to_owned(),
            level: Level::System,
        }
    }

    pub fn id(&self) -> Snowflake {
        self.id
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

impl AsRef<Snowflake> for User {
    fn as_ref(&self) -> &Snowflake {
        &self.id
    }
}
