use crate::crypto::tls;
use crate::theme;
use iced::widget::{button, column, container, row, scrollable, text, text_input, Column};
use iced::{Background, Border, Element, Length, Theme};

#[derive(Debug, Clone)]
pub enum Msg {
    HostChanged(String),
    PortChanged(String),
    Connect,
    Connected(Result<String, String>),
}

pub struct State {
    pub host: String,
    pub port: String,
    pub result: String,
    pub cert_pem: String,
    pub connecting: bool,
    pub error: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            host: "www.google.com".into(),
            port: "443".into(),
            result: String::new(),
            cert_pem: String::new(),
            connecting: false,
            error: String::new(),
        }
    }
}

pub fn update(state: &mut State, msg: Msg) -> (Option<String>, Option<iced::Task<Msg>>) {
    state.error.clear();
    match msg {
        Msg::HostChanged(v) => { state.host = v; (None, None) }
        Msg::PortChanged(v) => { state.port = v; (None, None) }
        Msg::Connect => {
            state.connecting = true;
            state.result.clear();
            state.cert_pem.clear();
            let host = state.host.clone();
            let port: u16 = state.port.parse().unwrap_or(443);

            let task = iced::Task::perform(
                async move {
                    tokio::task::spawn_blocking(move || {
                        match tls::connect_tls(&host, port) {
                            Ok(info) => {
                                let display = format!("{}", info);
                                let pem = info.server_cert_pem.clone();
                                Ok(format!("{}\n---PEM---\n{}", display, pem))
                            }
                            Err(e) => Err(e.to_string()),
                        }
                    })
                    .await
                    .unwrap_or_else(|e| Err(e.to_string()))
                },
                Msg::Connected,
            );
            (None, Some(task))
        }
        Msg::Connected(result) => {
            state.connecting = false;
            match result {
                Ok(data) => {
                    if let Some(idx) = data.find("\n---PEM---\n") {
                        state.result = data[..idx].to_string();
                        state.cert_pem = data[idx + 11..].to_string();
                    } else {
                        state.result = data;
                    }
                    (Some("TLS connection successful".into()), None)
                }
                Err(e) => { state.error = e; (None, None) }
            }
        }
    }
}

pub fn view(state: &State) -> Element<'_, Msg> {
    let title = text("TLS Connection Test").size(24).color(theme::TEXT_DARK);
    let desc = text("Connect to a server and inspect TLS handshake details (like openssl s_client)")
        .size(13).color(iced::Color::from_rgb(0.5, 0.5, 0.55));

    let input_row = row![
        column![
            text("Hostname").size(13),
            text_input("www.google.com", &state.host).on_input(Msg::HostChanged).padding(8).size(13),
        ].spacing(4).width(Length::Fill),
        column![
            text("Port").size(13),
            text_input("443", &state.port).on_input(Msg::PortChanged).padding(8).size(13).width(100),
        ].spacing(4).width(Length::Shrink),
    ].spacing(10);

    let btn: Element<'_, Msg> = if state.connecting {
        text("Connecting...").size(14).color(theme::ACCENT).into()
    } else {
        styled_btn("Connect")
    };

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

        let cert_section = if !state.cert_pem.is_empty() {
            column![
                text("Server Certificate PEM").size(13).color(theme::TEXT_DARK),
                container(
                    text(&state.cert_pem).size(10).font(iced::Font::MONOSPACE).color(theme::TEXT_DARK)
                )
                .padding(10)
                .width(Length::Fill)
                .style(|_: &Theme| container::Style {
                    background: Some(Background::Color(iced::Color::from_rgb(0.96, 0.96, 0.98))),
                    border: Border { radius: 4.0.into(), width: 1.0, color: theme::BORDER },
                    ..container::Style::default()
                }),
            ].spacing(5)
        } else {
            column![]
        };

        column![
            text("Connection Details").size(14).color(theme::TEXT_DARK),
            result_box,
            cert_section,
        ].spacing(10)
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
        .push(iced::widget::vertical_space().height(15))
        .push(input_row)
        .push(iced::widget::vertical_space().height(10))
        .push(btn)
        .push(iced::widget::vertical_space().height(15))
        .push(result_section)
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
        .on_press(Msg::Connect)
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
