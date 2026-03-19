use crate::crypto::asymmetric::{self, AsymAlgorithm};
use crate::theme;
use iced::widget::{button, column, container, pick_list, row, scrollable, text, text_input, Column};
use iced::{Background, Border, Element, Length, Theme};

#[derive(Debug, Clone)]
pub enum Msg {
    AlgorithmSelected(AsymAlgorithm),
    Generate,
    Generated(Result<(String, String), String>),
    CopyPrivate,
    CopyPublic,
}

pub struct State {
    pub algorithm: Option<AsymAlgorithm>,
    pub private_pem: String,
    pub public_pem: String,
    pub generating: bool,
    pub error: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            algorithm: Some(AsymAlgorithm::Rsa2048),
            private_pem: String::new(),
            public_pem: String::new(),
            generating: false,
            error: String::new(),
        }
    }
}

pub fn update(state: &mut State, msg: Msg) -> (Option<String>, Option<iced::Task<Msg>>) {
    state.error.clear();
    match msg {
        Msg::AlgorithmSelected(a) => { state.algorithm = Some(a); (None, None) }
        Msg::Generate => {
            if let Some(algo) = state.algorithm {
                state.generating = true;
                state.private_pem.clear();
                state.public_pem.clear();
                let task = iced::Task::perform(
                    async move {
                        tokio::task::spawn_blocking(move || {
                            asymmetric::generate_keypair(algo)
                                .map(|kp| (kp.private_pem, kp.public_pem))
                                .map_err(|e| e.to_string())
                        })
                        .await
                        .unwrap_or_else(|e| Err(e.to_string()))
                    },
                    Msg::Generated,
                );
                (None, Some(task))
            } else {
                (None, None)
            }
        }
        Msg::Generated(result) => {
            state.generating = false;
            match result {
                Ok((priv_pem, pub_pem)) => {
                    state.private_pem = priv_pem;
                    state.public_pem = pub_pem;
                    (Some("Key pair generated".into()), None)
                }
                Err(e) => { state.error = e; (None, None) }
            }
        }
        Msg::CopyPrivate => (Some("copy_private".into()), None),
        Msg::CopyPublic => (Some("copy_public".into()), None),
    }
}

pub fn view(state: &State) -> Element<'_, Msg> {
    let title = text("Asymmetric Key Generation").size(24).color(theme::TEXT_DARK);
    let desc = text("Generate RSA and ECDSA key pairs")
        .size(13)
        .color(iced::Color::from_rgb(0.5, 0.5, 0.55));

    let algo_row = row![
        column![
            text("Algorithm / Key Size").size(13),
            pick_list(AsymAlgorithm::ALL, state.algorithm, Msg::AlgorithmSelected).width(200).padding(8),
        ].spacing(4),
        if state.generating {
            column![
                text("").size(13),
                text("Generating...").size(14).color(theme::ACCENT),
            ].spacing(4)
        } else {
            column![
                text("").size(13),
                styled_btn("Generate Key Pair", Msg::Generate),
            ].spacing(4)
        },
    ]
    .spacing(15);

    let keys_section = if !state.private_pem.is_empty() {
        let priv_section = column![
            row![
                text("Private Key").size(13).color(theme::TEXT_DARK),
                styled_btn("Copy", Msg::CopyPrivate),
            ].spacing(10),
            pem_box(&state.private_pem),
        ].spacing(5);

        let pub_section = column![
            row![
                text("Public Key").size(13).color(theme::TEXT_DARK),
                styled_btn("Copy", Msg::CopyPublic),
            ].spacing(10),
            pem_box(&state.public_pem),
        ].spacing(5);

        column![priv_section, pub_section].spacing(15)
    } else {
        column![]
    };

    let error_el: Element<'_, Msg> = if !state.error.is_empty() {
        text(&state.error).size(13).color(theme::ERROR).into()
    } else {
        column![].into()
    };

    let content = Column::new()
        .push(title)
        .push(desc)
        .push(iced::widget::vertical_space().height(15))
        .push(algo_row)
        .push(iced::widget::vertical_space().height(15))
        .push(keys_section)
        .push(error_el)
        .spacing(5)
        .width(Length::Fill);

    container(scrollable(content.padding(25)))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(theme::CARD_BG)),
            border: Border { radius: 8.0.into(), ..Border::default() },
            ..container::Style::default()
        })
        .into()
}

fn pem_box(pem: &str) -> Element<'static, Msg> {
    let content = text(pem.to_string())
        .size(11)
        .font(iced::Font::MONOSPACE)
        .color(theme::TEXT_DARK);

    container(content)
        .padding(10)
        .width(Length::Fill)
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(iced::Color::from_rgb(0.93, 0.95, 0.98))),
            border: Border { radius: 4.0.into(), width: 1.0, color: theme::BORDER },
            ..container::Style::default()
        })
        .into()
}

fn styled_btn(label: &str, msg: Msg) -> Element<'static, Msg> {
    button(text(label.to_string()).size(13).color(iced::Color::WHITE).center())
        .on_press(msg)
        .padding([6, 14])
        .style(move |_: &Theme, status| {
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
