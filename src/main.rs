use rdkafka::{
    config::ClientConfig,
    consumer::{StreamConsumer},
};

fn main() {
    let client_config = ClientConfig::new();

    let _consumer: StreamConsumer = client_config.create().unwrap();
}
