mod file_operations;

use file_operations::*;
use iced::widget::{
    TextInput, button, center, checkbox, column, container, horizontal_rule, pick_list,
    progress_bar, row, scrollable, slider, text, text_input, toggler, vertical_rule,
    vertical_space,
};
use iced::{Center, Element, Fill, Subscription, Theme};
use iced::{Font, Task, keyboard};
use rascii_art::RenderOptions;
use rascii_art::charsets::{Charset, MINIMAL};
use rustii::ascii_image_options::AsciiImageOptions;
use rustii::convert_image_to_ascii_png;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub fn main() -> iced::Result {
    iced::application("RustiiGUI", RustiiGui::update, RustiiGui::view)
        .subscription(RustiiGui::subscription)
        .theme(RustiiGui::theme)
        // since the default font is broken on linux, we will use monospace
        .default_font(Font::MONOSPACE)
        .run()
}

#[derive(Default)]
struct RustiiGui {
    theme: Theme,
    input_value: String,
    slider_value: f32,
    checkbox_value: bool,
    toggler_value: bool,
    // whether we're loading a file or not (waiting for the user to finish opening a file)
    is_loading: bool,
    input_file: Option<PathBuf>,
    output_file: Option<PathBuf>,
}

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(Theme),
    InputChanged(String),
    InputFile,
    OutputFile,
    Convert,
    OpenInputFile,
    OpenOutputFile,
    InputFileOpened(Result<PathBuf, Error>),
    OutputFileOpened(Result<PathBuf, Error>),
    SliderChanged(f32),
    CheckboxToggled(bool),
    TogglerToggled(bool),
    PreviousTheme,
    NextTheme,
}

impl RustiiGui {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = theme;

                Task::none()
            }
            Message::InputChanged(value) => {
                self.input_value = value;

                Task::none()
            }
            Message::InputFile => {
                println!("input file");

                Task::none()
            }
            Message::OutputFile => {
                println!("output file");

                Task::none()
            }
            Message::Convert => {
                // don't convert if we're already loading a file
                if self.is_loading {
                    Task::none()
                } else {
                    let rascii_options = RenderOptions {
                        width: Some(100),
                        height: None,
                        colored: true,
                        escape_each_colored_char: true,
                        invert: false,
                        charset: MINIMAL,
                    };

                    let rustii_options = AsciiImageOptions::new(None, false);

                    // FIXME: this is ugly
                    if let Some(input_file) = &self.input_file {
                        if let Some(output_file) = &self.output_file {
                            if let Some(input_file_str) = input_file.to_str() {
                                if let Some(output_file_str) = output_file.to_str() {
                                    match convert_image_to_ascii_png(
                                        input_file_str,
                                        output_file_str,
                                        &rascii_options,
                                        &rustii_options,
                                    ) {
                                        Ok(_) => {
                                            println!("Saved PNG {}", output_file_str);
                                        }
                                        Err(_) => {
                                            eprintln!("Could not save PNG {}", output_file_str);
                                        }
                                    }
                                }
                            }
                        }
                    }

                    Task::none()
                }
            }
            Message::OpenInputFile => {
                if self.is_loading {
                    Task::none()
                } else {
                    self.is_loading = true;

                    Task::perform(open_file(), Message::InputFileOpened)
                }
            }
            Message::OpenOutputFile => {
                if self.is_loading {
                    Task::none()
                } else {
                    self.is_loading = true;
                    Task::perform(save_file(None), Message::OutputFileOpened)
                }
            }
            Message::InputFileOpened(result) => {
                self.is_loading = false;

                if let Ok(path) = result {
                    self.input_file = Some(path);
                }

                Task::none()
            }
            Message::OutputFileOpened(result) => {
                self.is_loading = false;

                if let Ok(path) = result {
                    self.output_file = Some(path);
                }

                Task::none()
            }
            Message::SliderChanged(value) => {
                self.slider_value = value;
                Task::none()
            }
            Message::CheckboxToggled(value) => {
                self.checkbox_value = value;
                Task::none()
            }
            Message::TogglerToggled(value) => {
                self.toggler_value = value;
                Task::none()
            }
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

                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let choose_theme = column![
            text("Theme:"),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged).width(Fill),
        ]
        .spacing(10);

        let rustii_text_input = text_input("Type something...", &self.input_value)
            .on_input(Message::InputChanged)
            .padding(10)
            .size(20);

        let styled_button = |label| button(text(label).width(Fill).center()).padding(10);

        let input_file_name_str = match &self.input_file {
            Some(input_file_name) => match input_file_name.to_str() {
                Some(the_name) => Some(the_name),
                None => None,
            },
            None => None,
        };

        let output_file_name_str = match &self.output_file {
            Some(output_file_name) => match output_file_name.to_str() {
                Some(the_name) => Some(the_name),
                None => None,
            },
            None => None,
        };

        let input_file_header = container(column![text("Input File:").size(36)]);

        let input_file_container = container(scrollable(column![text(
            input_file_name_str.unwrap_or("No Input File.")
        ),]))
        .style(container::rounded_box);

        let output_file_header = container(column![text("Output File:").size(36)]);

        let output_file_container = container(scrollable(column![text(
            output_file_name_str.unwrap_or("No Output File.")
        )]))
        .style(container::rounded_box);

        let open_input_button = styled_button("Select Input File").on_press(Message::OpenInputFile);
        let open_output_button =
            styled_button("Select Output File").on_press(Message::OpenOutputFile);
        let convert_button = styled_button("Convert to ASCII")
            .style(button::danger)
            .on_press(Message::Convert);

        let slider = || slider(0.0..=100.0, self.slider_value, Message::SliderChanged);

        let progress_bar = || progress_bar(0.0..=100.0, self.slider_value);

        // TODO: instead of doing println when we have saved an image, should we keep output somewhere to put here?
        let my_scrollable = scrollable(column![
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

        let content = container(scrollable(
            column![
                choose_theme,
                row![input_file_header, output_file_header].spacing(10),
                row![input_file_container, output_file_container].spacing(10),
                horizontal_rule(38),
                rustii_text_input,
                row![open_input_button, open_output_button, convert_button]
                    .spacing(10)
                    .align_y(Center),
                slider(),
                progress_bar(),
                row![
                    my_scrollable,
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
            .max_width(600),
        ));

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
