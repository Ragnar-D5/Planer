use iced::Sandbox, Element;

struct Planer;

struct Message;

impl Sandbox for Planer {
    type Message = Message

    fn new() -> Self {
        Planer
    }

    fn title(&self) -> String {
        "Planer"
    }

    fn update (&mut self, Message) {

    }

    fn view(&self) -> Element<Message> {
        
    }
}