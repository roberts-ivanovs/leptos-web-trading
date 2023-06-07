use serde::{Deserialize, Serialize};

use crate::common::{LobbyId, Order, RequestId, Size, Tick, TraderId};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ServerMessage {
    ConnectionInfo(LobbyId, TraderId, TraderInfo, Latency),
    MatchInfo(LobbyId, TraderInfo),
    TickSetWhole(Vec<TickData>),
    TickUpdate(TickData),
    OrderAccepted(RequestId),
    OrderRejected(RequestId, String),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct TickData {
    pub back: Size,
    pub lay: Size,
    pub tick: Tick,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Latency {
    pub ms: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct TraderInfo {
    pub balance: Size,
    pub exposure: Size,
    pub orders: Vec<Order>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct MatchInfo {
    pub total_matched: Size,
    pub total_available: Size,
    pub end_date: chrono::DateTime<chrono::Utc>,
}
