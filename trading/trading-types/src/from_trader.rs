use serde::{Deserialize, Serialize};

use crate::common::{LobbyId, Order, RequestId, TraderId};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TraderMessage {
    InitConnection(RequestId, LobbyId, TraderId),
    PlaceOrder(RequestId, Order),
    // Persist connectivity
    TraderTime(chrono::DateTime<chrono::Utc>),
    TraderTimeAck(chrono::DateTime<chrono::Utc>),
}
