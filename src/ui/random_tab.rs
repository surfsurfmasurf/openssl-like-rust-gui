use crate::crypto::random::{self, RandomFormat};
use crate::theme;
use iced::widget::{button, column, container, pick_list, row, text, text_input, Column};
use iced::{Background, Border, Element, Length, Theme};

#[derive(Debug, Clone)]
pub enum Msg {
    ByteCountChanged(String),
    FormatSelected(RandomFormat),
    Generate,
    CopyResult,
}

pub struct State {
    pub byte_count: String,
    pub format: Option<RandomFormat>,
    pub result: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            byte_count: "32".into(),
            format: Some(RandomFormat::Hex),
            result: String::new(),
        }
    }
}

pub fn update(state: &mut State, msg: Msg) -> Option<String> {
    match msg {
        Msg::ByteCountChanged(v) => { state.byte_count = v; None }
        Msg::FormatSelected(f) => { state.format = Some(f); None }
        Msg::Generate => {
            let count: usize = state.byte_count.parse().unwrap_or(32);
            let fmt = state.format.unwrap_or(RandomFormat::Hex);
            state.result = random::generate_random(count, fmt);
            Some(format!("Generated {} random bytes", count))
        }
        Msg::CopyResult => Some("copy".into()),
    }
}

pub fn view(state: &State) -> Element<'_, Msg> {
    let title = text("Random Data Generator").size(24).color(theme::TEXT_DARK);
    let desc = text("Generate cryptographically secure random data")
        .size(13).color(iced::Color::from_rgb(0.5, 0.5, 0.55));

    let config_row = row![
        column![
            text("Byte Count").size(13),
            text_input("32", &state.byte_count).on_input(Msg::ByteCountChanged).padding(8).size(13).width(120),
        ].spacing(4),
        column![
            text("Output Format").size(13),
            pick_list(RandomFormat::ALL, state.format, Msg::FormatSelected).width(150).padding(8),
        ].spacing(4),
        column![
            text("").size(13),
            styled_btn("Generate", Msg::Generate),
        ].spacing(4),
    ].spacing(15);

    let result_section = if !state.result.is_empty() {
        let result_box = container(
            text(&state.result).size(12).font(iced::Font::MONOSPACE).color(theme::TEXT_DARK)
        )
        .padding(12)
        .width(Length::Fill)
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(iced::Color::from_rgb(0.93, 0.95, 0.98))),
            border: Border { radius: 4.0.into(), width: 1.0, color: theme::BORDER },
            ..container::Style::default()
        });

        column![
            text("Result").size(13).color(theme::TEXT_DARK),
            result_box,
            styled_btn("Copy", Msg::CopyResult),
        ].spacing(8)
    } else {
        column![]
    };

    let content = Column::new()
        .push(title)
        .push(desc)
        .push(iced::widget::vertical_space().height(20))
        .push(config_row)
        .push(iced::widget::vertical_space().height(15))
        .push(result_section)
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
