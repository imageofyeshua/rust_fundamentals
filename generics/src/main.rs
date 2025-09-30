#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Watching the {:?}", self.content)
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

fn main() {
    let message = ChatMessage {
        content: "Hi, lol",
        time: String::from("2025-10-01")
    };
    println!("message: {}", message.retrieve_time());

    let notification = ChatMessage {
        content: String::from("What's your favorite cake topping"),
        time: String::from("2025-05-20")
    };
    println!("notification: {}", notification.retrieve_time());

    let audio = ChatMessage {
        content: DigitalContent::AudioFile,
        time: String::from("2025-06-23")
    };

    audio.consume_entertainment();
}
