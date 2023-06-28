use serde::{Serialize, Deserialize};
use strum_macros::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display)]
pub enum AuthType {
    #[strum(to_string = "user")]
    User,
    #[strum(to_string = "bot")]
    Bot,
    #[strum(to_string = "server")]
    Server,
}
