use iced::Settings;
use iced::Sandbox;
use iced::Element;
use iced::widget::Button;
use iced::widget::Column;
use iced::widget::Container;

fn main() -> iced::Result {
    Counter::run(Settings::default())
}

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonChecker {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = ButtonChecker;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn update(&mut self, message: ButtonChecker) {
        match message {
            ButtonChecker::IncrementPressed => {
                self.value += 1;
                println!("{}", self.value);
            }
            ButtonChecker::DecrementPressed => {
                self.value -= 1;
                println!("{}", self.value);
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let increment = Button::new("increment")
            .on_press(ButtonChecker::IncrementPressed);

        let decriment = Button::new("decriment")
            .on_press(ButtonChecker::DecrementPressed);

        let column = Column::new()
            .padding(20)
            .spacing(5)
            .push(increment)
            .push(decriment);

        Container::new(column)
            .into()
    }
}
