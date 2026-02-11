//! Example demonstrating integration with the `iced` crate.
use iced::{
    application,
    daemon::Appearance,
    widget::{button, column, container, text},
    Alignment::Center,
    Element,
    Length::Fill,
    Result,
};

const COLORS: catppuccin::FlavorColors = catppuccin::PALETTE.latte.colors;

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Clone, Copy, Debug)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    const fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let green: iced::Color = COLORS.green.into();
        let red: iced::Color = COLORS.red.into();
        container(
            column![
                button(text("+").size(50).center())
                    .style(move |_, _| button::Style {
                        background: Some(green.into()),
                        text_color: COLORS.crust.into(),
                        ..Default::default()
                    })
                    .width(60)
                    .on_press(Message::Increment),
                text(self.value).size(50),
                button(text("-").size(50).center())
                    .style(move |_, _| button::Style {
                        background: Some(red.into()),
                        text_color: COLORS.crust.into(),
                        ..Default::default()
                    })
                    .width(60)
                    .on_press(Message::Decrement),
            ]
            .align_x(Center)
            .spacing(10),
        )
        .padding(20)
        .center_x(Fill)
        .center_y(Fill)
        .into()
    }
}

fn main() -> Result {
    application(|| Counter { value: 0 }, Counter::update, Counter::view)
        .style(move |_, _| Appearance {
            background_color: COLORS.base.into(),
            text_color: COLORS.text.into(),
        })
        .run()
}
