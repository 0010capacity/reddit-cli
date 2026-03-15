use serde::{Deserialize, Serialize};

/// Reddit thing type prefix
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThingType {
    Comment,   // t1_
    Account,   // t2_
    Link,      // t3_
    Message,   // t4_
    Subreddit, // t5_
    Award,     // t6_
}

impl std::fmt::Display for ThingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThingType::Comment => write!(f, "t1"),
            ThingType::Account => write!(f, "t2"),
            ThingType::Link => write!(f, "t3"),
            ThingType::Message => write!(f, "t4"),
            ThingType::Subreddit => write!(f, "t5"),
            ThingType::Award => write!(f, "t6"),
        }
    }
}

/// Generic Reddit thing wrapper
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Thing<T> {
    pub kind: String,
    pub data: T,
}

/// Listing response (pagination)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Listing<T> {
    #[serde(rename = "before")]
    pub before: Option<String>,
    #[serde(rename = "after")]
    pub after: Option<String>,
    #[serde(rename = "children")]
    pub children: Vec<Thing<T>>,
}

/// Generic listing response wrapper
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ListingResponse<T> {
    #[serde(rename = "data")]
    pub data: Listing<T>,
}

/// Time period for top/controversial
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimePeriod {
    Hour,
    Day,
    Week,
    Month,
    Year,
    All,
}

impl std::fmt::Display for TimePeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimePeriod::Hour => write!(f, "hour"),
            TimePeriod::Day => write!(f, "day"),
            TimePeriod::Week => write!(f, "week"),
            TimePeriod::Month => write!(f, "month"),
            TimePeriod::Year => write!(f, "year"),
            TimePeriod::All => write!(f, "all"),
        }
    }
}

impl std::str::FromStr for TimePeriod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "hour" => Ok(TimePeriod::Hour),
            "day" => Ok(TimePeriod::Day),
            "week" => Ok(TimePeriod::Week),
            "month" => Ok(TimePeriod::Month),
            "year" => Ok(TimePeriod::Year),
            "all" => Ok(TimePeriod::All),
            _ => Err(format!("Invalid time period: {}", s)),
        }
    }
}

/// Sort method for listings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortMethod {
    Hot,
    New,
    Top,
    Rising,
    Controversial,
    Best,
}

impl std::fmt::Display for SortMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortMethod::Hot => write!(f, "hot"),
            SortMethod::New => write!(f, "new"),
            SortMethod::Top => write!(f, "top"),
            SortMethod::Rising => write!(f, "rising"),
            SortMethod::Controversial => write!(f, "controversial"),
            SortMethod::Best => write!(f, "best"),
        }
    }
}
