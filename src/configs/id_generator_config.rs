#[derive(Debug)]
pub struct IdGeneratorConfig {
    pub worker_id: u8,
    pub data_center_id: u8,
    pub timestamp_offset: u128,
}

impl IdGeneratorConfig {
    pub fn new() -> Self {
        let worker_id = dotenv::var("WORKER_ID")
            .expect("Can't read worker id from env.")
            .parse::<u8>()
            .expect("Can't parse the worker id to u8.");

        let data_center_id = dotenv::var("DATA_CENTER_ID")
            .expect("Can't read data center id from env.")
            .parse::<u8>()
            .expect("Can't parse the data center id to u8.");

        let timestamp_offset = dotenv::var("TIMESTAMP_OFFSET")
            .expect("Can't read timestamp offset from env.")
            .parse::<u128>()
            .expect("Can't parse the timestamp offset to u128.");

        Self {
            worker_id,
            data_center_id,
            timestamp_offset,
        }
    }
}
