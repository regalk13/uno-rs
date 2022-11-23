use coarsetime::Clock;
use std::error::Error;
use std::sync::{Mutex, RwLock};

static PROCESS: RwLock<Option<u64>> = RwLock::new(None);
// (last time, counter)
static COUNTER: Mutex<(u64, u64)> = Mutex::new((0, 0));

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// An unique Snowflake ID
pub struct Snowflake(u64);

impl Snowflake {
    /// Epoch since November 23, 2022 0:00:00 UTC
    pub const UNO_EPOCH: u64 = 1669161600000;

    /// Mask for timestamp
    pub const TIMESTAMP_MASK: u64 = 0xFFFFFFFFFFC00000;
    /// Mask for process id
    pub const PROCESS_MASK: u64 = 0x3FF00;
    /// Mask for internal counter
    pub const COUNTER_MASK: u64 = 0xFFF;

    /// Length of bits shift for timestamp
    pub const TIMESTAMP_SHIFT: u64 = 42;
    /// Length of bits shift for process id
    pub const PROCESS_SHIFT: u64 = 12;
    /// Length of bits shift for internal counter
    pub const COUNTER_SHIFT: u64 = 0;

    /// Maximum length of internal counter
    pub const MAX_COUNTER: u64 = 4096;

    /// Create a Snowflake from a given id
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Generate an unique Snowflake along
    /// the whole process
    pub fn generate() -> Result<Self, Box<dyn Error>> {
        Clock::update();

        let timestamp = Clock::recent_since_epoch().as_millis() - Snowflake::UNO_EPOCH;
        let process = {
            let lock = PROCESS.read()?;
            if let Some(id) = *lock {
                id
            } else {
                // ReadGuard has to be dropped
                // before that WriteGuard is created
                std::mem::drop(lock);

                let id = std::process::id() as u64;
                *PROCESS.write()? = Some(id);
                id
            }
        };
        let counter = {
            let (ref mut last, ref mut counter) = *COUNTER.lock()?;
            let now = Clock::recent_since_epoch().as_millis();

            if now != *last {
                *last = now;
                *counter = 0;
            } else if *counter >= Snowflake::MAX_COUNTER {
                // wait for a milli-second to reset counter
                // to avoid collision
                std::thread::sleep(std::time::Duration::from_millis(1));

                *last = now;
                *counter = 0;
            }

            *counter += 1;
            *counter - 1
        };

        Ok(Self(
            timestamp << Snowflake::TIMESTAMP_SHIFT & Snowflake::TIMESTAMP_MASK
                | process << Snowflake::PROCESS_SHIFT & Snowflake::PROCESS_MASK
                | counter << Snowflake::COUNTER_SHIFT & Snowflake::COUNTER_MASK,
        ))
    }

    /// Return raw Snowflake ID
    pub fn id(&self) -> u64 {
        self.0
    }
}
