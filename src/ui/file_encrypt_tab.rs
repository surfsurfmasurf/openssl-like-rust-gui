use crate::crypto::file_ops;
use crate::theme;
use iced::widget::{button, column, container, row, text, text_input, Column};
use iced::{Background, Border, Element, Length, Theme};

#[derive(Debug, Clone)]
pub enum Msg {
    InputPathChanged(String),
    OutputPathChanged(String),
    PasswordChanged(String),
    Encrypt,
    Decrypt,
}

pub struct State {
    pub input_path: String,
    pub output_path: String,
    pub password: String,
    pub status: String,
    pub error: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            input_path: String::new(),
            output_path: String::new(),
            password: String::new(),
            status: String::new(),
            error: String::new(),
        }
    }
}

pub fn update(state: &mut State, msg: Msg) -> Option<String> {
    state.error.clear();
    state.status.clear();
    match msg {
        Msg::InputPathChanged(v) => { state.input_path = v; None }
        Msg::OutputPathChanged(v) => { state.output_path = v; None }
        Msg::PasswordChanged(v) => { state.password = v; None }
        Msg::Encrypt => {
            let input = std::path::Path::new(&state.input_path);
            let output = std::path::Path::new(&state.output_path);
            match file_ops::encrypt_file(input, output, &state.password) {
                Ok(()) => {
                    state.status = "File encrypted successfully!".into();
                    Some("File encrypted".into())
                }
                Err(e) => { state.error = e.to_string(); None }
            }
        }
        Msg::Decrypt => {
            let input = std::path::Path::new(&state.input_path);
            let output = std::path::Path::new(&state.output_path);
            match file_ops::decrypt_file(input, output, &state.password) {
                Ok(()) => {
                    state.status = "File decrypted successfully!".into();
                    Some("File decrypted".into())
                }
                Err(e) => { state.error = e.to_string(); None }
            }
        }
    }
}

pub fn view(state: &State) -> Element<'_, Msg> {
    let title = text("File Encryption").size(24).color(theme::TEXT_DARK);
    let desc = text("Encrypt and decrypt files using AES-256-GCM with password-based key derivation")
        .size(13).color(iced::Color::from_rgb(0.5, 0.5, 0.55));

    let form = column![
        text("Input File Path").size(13),
        text_input("C:\\path\\to\\input\\file", &state.input_path)
            .on_input(Msg::InputPathChanged).padding(8).size(13),
        text("Output File Path").size(13),
        text_input("C:\\path\\to\\output\\file", &state.output_path)
            .on_input(Msg::OutputPathChanged).padding(8).size(13),
        text("Password").size(13),
        text_input("Enter password...", &state.password)
            .on_input(Msg::PasswordChanged).padding(8).size(13).secure(true),
    ].spacing(6);

    let btn_row = row![
        styled_btn("Encrypt File", Msg::Encrypt),
        styled_btn("Decrypt File", Msg::Decrypt),
    ].spacing(10);

    let status: Element<'_, Msg> = if !state.status.is_empty() {
        container(text(&state.status).size(14).color(theme::SUCCESS))
            .padding(10).width(Length::Fill)
            .style(|_: &Theme| container::Style {
                background: Some(Background::Color(iced::Color::from_rgb(0.93, 0.98, 0.93))),
                border: Border { radius: 4.0.into(), width: 1.0, color: theme::SUCCESS },
                ..container::Style::default()
            }).into()
    } else if !state.error.is_empty() {
        container(text(&state.error).size(14).color(theme::ERROR))
            .padding(10).width(Length::Fill)
            .style(|_: &Theme| container::Style {
                background: Some(Background::Color(iced::Color::from_rgb(1.0, 0.95, 0.95))),
                border: Border { radius: 4.0.into(), width: 1.0, color: theme::ERROR },
                ..container::Style::default()
            }).into()
    } else {
        column![].into()
    };

    let info = container(
        column![
            text("File Format Info").size(13).color(theme::TEXT_DARK),
            text("Encrypted files: [16-byte salt][12-byte nonce][AES-256-GCM ciphertext + tag]")
                .size(12).font(iced::Font::MONOSPACE).color(iced::Color::from_rgb(0.5, 0.5, 0.55)),
            text("Key derivation: SHA-256 iterated 10,000 times with random salt")
                .size(12).font(iced::Font::MONOSPACE).color(iced::Color::from_rgb(0.5, 0.5, 0.55)),
        ].spacing(4)
    ).padding(12).width(Length::Fill)
    .style(|_: &Theme| container::Style {
        background: Some(Background::Color(iced::Color::from_rgb(0.96, 0.96, 0.98))),
        border: Border { radius: 4.0.into(), ..Border::default() },
        ..container::Style::default()
    });

    let content = Column::new()
        .push(title)
        .push(desc)
        .push(iced::widget::vertical_space().height(20))
        .push(form)
        .push(iced::widget::vertical_space().height(15))
        .push(btn_row)
        .push(iced::widget::vertical_space().height(10))
        .push(status)
        .push(iced::widget::vertical_space().height(15))
        .push(info)
        .spacing(5)
        .width(Length::Fill);

    container(content.padding(25))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(theme::CARD_BG)),
            border: Border { radius: 8.0.into(), ..Border::default() },
            ..container::Style::default()
        })
        .into()
}

fn styled_btn(label: &str, msg: Msg) -> Element<'static, Msg> {
    button(text(label.to_string()).size(13).color(iced::Color::WHITE).center())
        .on_press(msg)
        .padding([8, 20])
        .style(|_: &Theme, status| {
            let bg = match status {
                button::Status::Hovered => iced::Color { a: 0.85, ..theme::ACCENT },
                _ => theme::ACCENT,
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: iced::Color::WHITE,
                border: Border { radius: 4.0.into(), ..Border::default() },
                ..button::Style::default()
            }
        })
        .into()
}
