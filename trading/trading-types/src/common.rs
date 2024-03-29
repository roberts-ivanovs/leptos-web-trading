use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct LobbyId(pub u32);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct TraderId(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct RequestId(pub String);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Tick(pub rust_decimal::Decimal);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Size(pub rust_decimal::Decimal);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Order {
    pub tick: Tick,
    pub size: Size,
    pub side: Side,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Side {
    Back,
    Lay,
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::Back => write!(f, "Back"),
            Side::Lay => write!(f, "Lay"),
        }
    }
}

impl Tick {
    pub fn all() -> Vec<Tick> {
        // Generate ticks from 1.01 to 2.0 with a step 0.01
        let mut ticks = Vec::new();
        let mut tick = rust_decimal::Decimal::new(145, 2);
        while tick < rust_decimal::Decimal::new(155, 2) {
            ticks.push(Tick(tick));
            tick += rust_decimal::Decimal::new(1, 2);
        }
        ticks
    }
}

impl std::ops::Add<&Size> for Size {
    type Output = Size;

    fn add(self, rhs: &Size) -> Self::Output {
        Size(self.0 + rhs.0)
    }
}
