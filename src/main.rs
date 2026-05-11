use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, MessageHandler, HandleError};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        println!("Message received on handler 1: {:?}", message);
        Ok(())
    }

    fn get_handler_action(&self) -> String {
        "user_created_handler".to_string()
    }
}

fn main() {
    dotenv::dotenv().ok();
    let amqp_url = std::env::var("AMQP_URL").unwrap_or_else(|_| "amqp://guest:guest@localhost:5672".into());
    println!("Connecting to RabbitMQ Broker...");
    let mut p = CrosstownBus::new_queue_publisher(amqp_url).unwrap();
    println!("Connected successfully! Publishing 5 messages...");

    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage { user_id: "1".to_owned(), user_name: "2406423055-Amir".to_owned() });
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage { user_id: "2".to_owned(), user_name: "2406423055-Budi".to_owned() });
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage { user_id: "3".to_owned(), user_name: "2406423055-Cica".to_owned() });
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage { user_id: "4".to_owned(), user_name: "2406423055-Dira".to_owned() });
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage { user_id: "5".to_owned(), user_name: "2406423055-Emir".to_owned() });
}