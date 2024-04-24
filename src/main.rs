use borsh_derive::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, HandleError, MessageHandler, QueueProperties};
use dotenv::dotenv;
use std::env;
use std::time;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        let ten_millis = time::Duration::from_millis(1000);
        let now = time::Instant::now();

        //thread::sleep(ten_millis);
        println!(
            "In Haekal's computer [129500004y] message received: {:?}",
            message
        );

        Ok(())
    }
}

fn main() {
    dotenv().ok();
    let rabbitmq_url = env::var("RABBITMQ_URL").unwrap();

    let listener = CrosstownBus::new_subscriber(rabbitmq_url).unwrap();

    _ = listener.subscribe(
        String::from("user_created"),
        UserCreatedHandler {},
        QueueProperties {
            auto_delete: false,
            durable: false,
            use_dead_letter: true,
            consume_queue_name: Some("user".to_string()),
        },
    );
}
