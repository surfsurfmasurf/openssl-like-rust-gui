use crate::crypto::hashing::{self, HashAlgorithm};
use crate::theme;
use iced::widget::{button, column, container, pick_list, row, text, text_input, Column};
use iced::{Background, Border, Element, Length, Theme};

#[derive(Debug, Clone)]
pub enum Msg {
    AlgorithmSelected(HashAlgorithm),
    InputChanged(String),
    Compute,
    CopyResult,
}

#[derive(Default)]
pub struct State {
    pub algorithm: Option<HashAlgorithm>,
    pub input: String,
    pub result: String,
}

impl State {
    pub fn new() -> Self {
        Self {
            algorithm: Some(HashAlgorithm::Sha256),
            ..Default::default()
        }
    }
}

pub fn update(state: &mut State, msg: Msg) -> Option<String> {
    match msg {
        Msg::AlgorithmSelected(algo) => {
            state.algorithm = Some(algo);
            None
        }
        Msg::InputChanged(val) => {
            state.input = val;
            None
        }
        Msg::Compute => {
            if let Some(algo) = state.algorithm {
                state.result = hashing::compute_hash(algo, state.input.as_bytes());
                Some(format!("{} hash computed", algo))
            } else {
                None
            }
        }
        Msg::CopyResult => Some("copy".into()),
    }
}

pub fn view(state: &State) -> Element<'_, Msg> {
    let title = text("Hash Functions").size(24).color(theme::TEXT_DARK);
    let desc = text("Compute cryptographic hash digests (MD5, SHA-1, SHA-256, SHA-384, SHA-512)")
        .size(13)
        .color(iced::Color::from_rgb(0.5, 0.5, 0.55));

    let algo_label = text("Algorithm").size(13).color(theme::TEXT_DARK);
    let algo_picker = pick_list(
        HashAlgorithm::ALL,
        state.algorithm,
        Msg::AlgorithmSelected,
    )
    .width(200)
    .padding(8);

    let input_label = text("Input Text").size(13).color(theme::TEXT_DARK);
    let input_field = text_input("Enter text to hash...", &state.input)
        .on_input(Msg::InputChanged)
        .padding(10)
        .size(14);

    let compute_btn = styled_button("Compute Hash", Some(Msg::Compute), theme::ACCENT);

    let result_section = if !state.result.is_empty() {
        let result_label = text("Result").size(13).color(theme::TEXT_DARK);
        let result_box = container(
            text(&state.result)
                .size(13)
                .font(iced::Font::MONOSPACE)
                .color(theme::TEXT_DARK),
        )
        .padding(12)
        .width(Length::Fill)
        .style(|_theme: &Theme| container::Style {
            background: Some(Background::Color(iced::Color::from_rgb(0.93, 0.95, 0.98))),
            border: Border {
                radius: 4.0.into(),
                width: 1.0,
                color: theme::BORDER,
            },
            ..container::Style::default()
        });

        let copy_btn = styled_button("Copy", Some(Msg::CopyResult), theme::ACCENT);
        let bit_len = state.result.len() * 4;
        let info = text(format!("{} bits ({} hex chars)", bit_len, state.result.len()))
            .size(12)
            .color(iced::Color::from_rgb(0.5, 0.5, 0.55));

        column![result_label, result_box, row![copy_btn, info].spacing(10)].spacing(8)
    } else {
        column![]
    };

    let content = Column::new()
        .push(title)
        .push(desc)
        .push(iced::widget::vertical_space().height(15))
        .push(algo_label)
        .push(algo_picker)
        .push(iced::widget::vertical_space().height(10))
        .push(input_label)
        .push(input_field)
        .push(iced::widget::vertical_space().height(10))
        .push(compute_btn)
        .push(iced::widget::vertical_space().height(15))
        .push(result_section)
        .spacing(5)
        .width(Length::Fill);

    card(content).into()
}

fn styled_button<'a>(label: &str, msg: Option<Msg>, color: iced::Color) -> Element<'a, Msg> {
    let btn = button(
        text(label.to_string())
            .size(14)
            .color(iced::Color::WHITE)
            .center(),
    )
    .padding([8, 20])
    .style(move |_theme: &Theme, status| {
        let bg = match status {
            button::Status::Hovered => iced::Color {
                a: 0.85,
                ..color
            },
            _ => color,
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: iced::Color::WHITE,
            border: Border {
                radius: 4.0.into(),
                ..Border::default()
            },
            ..button::Style::default()
        }
    });

    match msg {
        Some(m) => btn.on_press(m).into(),
        None => btn.into(),
    }
}

fn card<'a>(content: Column<'a, Msg>) -> Element<'a, Msg> {
    container(content.padding(25))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme: &Theme| container::Style {
            background: Some(Background::Color(theme::CARD_BG)),
            border: Border {
                radius: 8.0.into(),
                ..Border::default()
            },
            ..container::Style::default()
        })
        .into()
}
