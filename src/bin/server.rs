use futures_lite::stream::StreamExt;
use std::str::from_utf8;
use lapin::{
    options::*, types::FieldTable, Connection,
    ConnectionProperties, Result,
};
use tracing::{error,info};

fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::init();

    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());

    async_global_executor::block_on(async {
        let conn = Connection::connect(
            &addr,
            ConnectionProperties::default(),
        )
        .await?;

        info!("Connected to {}", &addr);

        let channel = conn.create_channel().await?;

        let queue = channel
            .queue_declare(
                "hello",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        info!(?queue, "Declared queue");

        let mut consumer = channel
            .basic_consume(
                "hello",
                "my_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        info!("will consume");

        while let Some(delivery) = consumer.next().await {
            let delivery = delivery.expect("error in consumer");

            match from_utf8(&delivery.data) {
                Ok(str) => {
                    info!("Received: {:?}", str);
                },
                Err(err) => {
                    error!("Error decoding delivery data: {}", err);
                }
            }

            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("ack");

                info!("Acknowledged {}", delivery.delivery_tag);
        }

        Ok(())
    })
}
