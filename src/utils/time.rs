use chrono::{DateTime, NaiveDateTime, Utc};

pub fn timestamp2datetime(timestamp: u64) -> DateTime<Utc> {
    let secs = (timestamp / 1000) as i64;
    let nsecs = (timestamp % 1000) as u32;

    let native_datetime = NaiveDateTime::from_timestamp(secs, nsecs);

    DateTime::<Utc>::from_utc(native_datetime, Utc)
}
