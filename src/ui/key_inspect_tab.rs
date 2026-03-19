use crate::crypto::key_inspect;
use crate::theme;
use iced::widget::{button, column, container, scrollable, text, text_input, Column};
use iced::{Background, Border, Element, Length, Theme};

#[derive(Debug, Clone)]
pub enum Msg {
    PemChanged(String),
    Inspect,
}

pub struct State {
    pub pem_input: String,
    pub result: String,
    pub error: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            pem_input: String::new(),
            result: String::new(),
            error: String::new(),
        }
    }
}

pub fn update(state: &mut State, msg: Msg) -> Option<String> {
    state.error.clear();
    match msg {
        Msg::PemChanged(v) => { state.pem_input = v; None }
        Msg::Inspect => {
            match key_inspect::inspect_pem(&state.pem_input) {
                Ok(info) => {
                    let mut lines = vec![
                        format!("Type:       {}", info.key_type),
                        format!("Key Size:   {}", info.bit_length),
                        String::new(),
                    ];
                    for (label, value) in &info.details {
                        lines.push(format!("{}: {}", label, value));
                    }
                    state.result = lines.join("\n");
                    Some(format!("Key inspected: {}", info.key_type))
                }
                Err(e) => { state.error = e.to_string(); None }
            }
        }
    }
}

pub fn view(state: &State) -> Element<'_, Msg> {
    let title = text("Key Inspector").size(24).color(theme::TEXT_DARK);
    let desc = text("Inspect PEM-encoded keys and certificates (like openssl pkey -text, openssl rsa -text)")
        .size(13).color(iced::Color::from_rgb(0.5, 0.5, 0.55));

    let supported = text("Supported: RSA Private/Public (PKCS#1, PKCS#8), ECDSA P-256/P-384, X.509 Certificates")
        .size(12).color(iced::Color::from_rgb(0.5, 0.5, 0.6));

    let input_section = column![
        text("Paste PEM data").size(13),
        text_input("-----BEGIN PRIVATE KEY----- ...", &state.pem_input)
            .on_input(Msg::PemChanged).padding(10).size(12),
        styled_btn("Inspect Key"),
    ].spacing(6);

    let result_section = if !state.result.is_empty() {
        column![
            text("Key Details").size(14).color(theme::TEXT_DARK),
            container(
                text(&state.result).size(12).font(iced::Font::MONOSPACE).color(theme::TEXT_DARK)
            )
            .padding(12)
            .width(Length::Fill)
            .style(|_: &Theme| container::Style {
                background: Some(Background::Color(iced::Color::from_rgb(0.93, 0.95, 0.98))),
                border: Border { radius: 4.0.into(), width: 1.0, color: theme::BORDER },
                ..container::Style::default()
            }),
        ].spacing(8)
    } else {
        column![]
    };

    let error_el: Element<'_, Msg> = if !state.error.is_empty() {
        container(text(&state.error).size(13).color(theme::ERROR))
            .padding(8).width(Length::Fill)
            .style(|_: &Theme| container::Style {
                background: Some(Background::Color(iced::Color::from_rgb(1.0, 0.95, 0.95))),
                border: Border { radius: 4.0.into(), width: 1.0, color: theme::ERROR },
                ..container::Style::default()
            }).into()
    } else {
        column![].into()
    };

    let content = Column::new()
        .push(title)
        .push(desc)
        .push(supported)
        .push(iced::widget::vertical_space().height(15))
        .push(input_section)
        .push(iced::widget::vertical_space().height(15))
        .push(result_section)
        .push(iced::widget::vertical_space().height(10))
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

fn styled_btn(label: &str) -> Element<'static, Msg> {
    button(text(label.to_string()).size(13).color(iced::Color::WHITE).center())
        .on_press(Msg::Inspect)
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
