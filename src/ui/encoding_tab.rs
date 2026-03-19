use crate::crypto::encoding;
use crate::theme;
use iced::widget::{button, column, container, row, text, text_input, Column};
use iced::{Background, Border, Element, Length, Theme};

#[derive(Debug, Clone)]
pub enum Msg {
    InputChanged(String),
    OutputChanged(String),
    Encode,
    Decode,
    CopyOutput,
}

#[derive(Default)]
pub struct State {
    pub input: String,
    pub output: String,
    pub error: String,
}

pub fn update(state: &mut State, msg: Msg) -> Option<String> {
    state.error.clear();
    match msg {
        Msg::InputChanged(v) => { state.input = v; None }
        Msg::OutputChanged(v) => { state.output = v; None }
        Msg::Encode => {
            state.output = encoding::base64_encode(state.input.as_bytes());
            Some("Base64 encoded".into())
        }
        Msg::Decode => {
            match encoding::base64_decode(&state.output) {
                Ok(bytes) => {
                    state.input = String::from_utf8_lossy(&bytes).into_owned();
                    Some("Base64 decoded".into())
                }
                Err(e) => { state.error = e.to_string(); None }
            }
        }
        Msg::CopyOutput => Some("copy".into()),
    }
}

pub fn view(state: &State) -> Element<'_, Msg> {
    let title = text("Base64 Encoding").size(24).color(theme::TEXT_DARK);
    let desc = text("Encode and decode data in Base64 format")
        .size(13).color(iced::Color::from_rgb(0.5, 0.5, 0.55));

    let data_row = row![
        column![
            text("Plain Text").size(13),
            text_input("Enter text to encode...", &state.input)
                .on_input(Msg::InputChanged).padding(10).size(13),
        ].spacing(4).width(Length::Fill),
        column![
            styled_btn("Encode >>", Msg::Encode),
            styled_btn("<< Decode", Msg::Decode),
        ].spacing(8).width(Length::Shrink),
        column![
            text("Base64 Output").size(13),
            text_input("Base64 output...", &state.output)
                .on_input(Msg::OutputChanged).padding(10).size(13).font(iced::Font::MONOSPACE),
        ].spacing(4).width(Length::Fill),
    ].spacing(10);

    let error_el: Element<'_, Msg> = if !state.error.is_empty() {
        text(&state.error).size(13).color(theme::ERROR).into()
    } else {
        column![].into()
    };

    let content = Column::new()
        .push(title)
        .push(desc)
        .push(iced::widget::vertical_space().height(20))
        .push(data_row)
        .push(iced::widget::vertical_space().height(10))
        .push(error_el)
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
        .padding([8, 16])
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
