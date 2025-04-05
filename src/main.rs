use iced::widget::{
    button, center, checkbox, column, container, horizontal_rule, pick_list, progress_bar, row,
    scrollable, slider, text, text_input, toggler, vertical_rule, vertical_space,
};
use iced::{Center, Element, Fill, Subscription, Theme};
use iced::{Font, keyboard};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub fn main() -> iced::Result {
    iced::application("Styling - Iced", RustiiGui::update, RustiiGui::view)
        .subscription(RustiiGui::subscription)
        .theme(RustiiGui::theme)
        .default_font(Font::MONOSPACE)
        .run()
}

// TODO: check this code

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IoError(io::ErrorKind),
}

async fn open_file() -> Result<(PathBuf, Arc<String>), Error> {
    let picked_file = rfd::AsyncFileDialog::new()
        .set_title("Open a text file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(picked_file).await
}

async fn load_file(path: impl Into<PathBuf>) -> Result<(PathBuf, Arc<String>), Error> {
    let path = path.into();

    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok((path, contents))
}

async fn save_file(path: Option<PathBuf>, contents: String) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .save_file()
            .await
            .as_ref()
            .map(rfd::FileHandle::path)
            .map(Path::to_owned)
            .ok_or(Error::DialogClosed)?
    };

    tokio::fs::write(&path, contents)
        .await
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok(path)
}
// TODO: end of todo

#[derive(Default)]
struct RustiiGui {
    theme: Theme,
    input_value: String,
    slider_value: f32,
    checkbox_value: bool,
    toggler_value: bool,
}

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(Theme),
    InputChanged(String),
    ButtonPressed,
    SliderChanged(f32),
    CheckboxToggled(bool),
    TogglerToggled(bool),
    PreviousTheme,
    NextTheme,
}

impl RustiiGui {
    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            }
            Message::InputChanged(value) => self.input_value = value,
            Message::ButtonPressed => {}
            Message::SliderChanged(value) => self.slider_value = value,
            Message::CheckboxToggled(value) => self.checkbox_value = value,
            Message::TogglerToggled(value) => self.toggler_value = value,
            Message::PreviousTheme | Message::NextTheme => {
                if let Some(current) = Theme::ALL
                    .iter()
                    .position(|candidate| &self.theme == candidate)
                {
                    self.theme = if matches!(message, Message::NextTheme) {
                        Theme::ALL[(current + 1) % Theme::ALL.len()].clone()
                    } else if current == 0 {
                        Theme::ALL
                            .last()
                            .expect("Theme::ALL must not be empty")
                            .clone()
                    } else {
                        Theme::ALL[current - 1].clone()
                    };
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let choose_theme = column![
            text("Theme:"),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged).width(Fill),
        ]
        .spacing(10);

        let text_input = text_input("Type something...", &self.input_value)
            .on_input(Message::InputChanged)
            .padding(10)
            .size(20);

        let styled_button = |label| {
            button(text(label).width(Fill).center())
                .padding(10)
                .on_press(Message::ButtonPressed)
        };

        let primary = styled_button("Primary");
        let success = styled_button("Success").style(button::success);
        let danger = styled_button("Danger").style(button::danger);

        let slider = || slider(0.0..=100.0, self.slider_value, Message::SliderChanged);

        let progress_bar = || progress_bar(0.0..=100.0, self.slider_value);

        let scrollable = scrollable(column![
            "Scroll me!",
            vertical_space().height(800),
            "You did it!"
        ])
        .width(Fill)
        .height(100);

        let checkbox =
            checkbox("Check me!", self.checkbox_value).on_toggle(Message::CheckboxToggled);

        let toggler = toggler(self.toggler_value)
            .label("Toggle me!")
            .on_toggle(Message::TogglerToggled)
            .spacing(10);

        let card = {
            container(column![text("Card Example").size(24), slider(), progress_bar(),].spacing(20))
                .width(Fill)
                .padding(20)
                .style(container::bordered_box)
        };

        let content = column![
            choose_theme,
            horizontal_rule(38),
            text_input,
            row![primary, success, danger].spacing(10).align_y(Center),
            slider(),
            progress_bar(),
            row![
                scrollable,
                vertical_rule(38),
                column![checkbox, toggler].spacing(20)
            ]
            .spacing(10)
            .height(100)
            .align_y(Center),
            card
        ]
        .spacing(20)
        .padding(20)
        .max_width(600);

        center(content).into()
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key, _modifiers| match key {
            keyboard::Key::Named(
                keyboard::key::Named::ArrowUp | keyboard::key::Named::ArrowLeft,
            ) => Some(Message::PreviousTheme),
            keyboard::Key::Named(
                keyboard::key::Named::ArrowDown | keyboard::key::Named::ArrowRight,
            ) => Some(Message::NextTheme),
            _ => None,
        })
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
