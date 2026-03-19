use crate::crypto::signatures::{self, SigAlgorithm};
use crate::theme;
use iced::widget::{button, column, container, pick_list, row, scrollable, text, text_input, Column};
use iced::{Background, Border, Element, Length, Theme};

#[derive(Debug, Clone)]
pub enum Msg {
    AlgorithmSelected(SigAlgorithm),
    PrivateKeyChanged(String),
    PublicKeyChanged(String),
    DataChanged(String),
    SignatureChanged(String),
    Sign,
    Verify,
}

pub struct State {
    pub algorithm: Option<SigAlgorithm>,
    pub private_key: String,
    pub public_key: String,
    pub data: String,
    pub signature_hex: String,
    pub verify_result: Option<bool>,
    pub error: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            algorithm: Some(SigAlgorithm::RsaSha256),
            private_key: String::new(),
            public_key: String::new(),
            data: String::new(),
            signature_hex: String::new(),
            verify_result: None,
            error: String::new(),
        }
    }
}

pub fn update(state: &mut State, msg: Msg) -> Option<String> {
    state.error.clear();
    state.verify_result = None;
    match msg {
        Msg::AlgorithmSelected(a) => { state.algorithm = Some(a); None }
        Msg::PrivateKeyChanged(v) => { state.private_key = v; None }
        Msg::PublicKeyChanged(v) => { state.public_key = v; None }
        Msg::DataChanged(v) => { state.data = v; None }
        Msg::SignatureChanged(v) => { state.signature_hex = v; None }
        Msg::Sign => {
            if let Some(algo) = state.algorithm {
                match signatures::sign(algo, &state.private_key, state.data.as_bytes()) {
                    Ok(sig) => {
                        state.signature_hex = hex::encode(&sig);
                        Some("Data signed".into())
                    }
                    Err(e) => { state.error = e.to_string(); None }
                }
            } else { None }
        }
        Msg::Verify => {
            if let Some(algo) = state.algorithm {
                match hex::decode(&state.signature_hex) {
                    Ok(sig) => {
                        match signatures::verify(algo, &state.public_key, state.data.as_bytes(), &sig) {
                            Ok(valid) => {
                                state.verify_result = Some(valid);
                                Some(if valid { "Signature VALID".into() } else { "Signature INVALID".into() })
                            }
                            Err(e) => { state.error = e.to_string(); None }
                        }
                    }
                    Err(e) => { state.error = format!("Invalid hex signature: {}", e); None }
                }
            } else { None }
        }
    }
}

pub fn view(state: &State) -> Element<'_, Msg> {
    let title = text("Digital Signatures").size(24).color(theme::TEXT_DARK);
    let desc = text("Sign and verify data with RSA and ECDSA")
        .size(13).color(iced::Color::from_rgb(0.5, 0.5, 0.55));

    let algo_row = column![
        text("Algorithm").size(13),
        pick_list(SigAlgorithm::ALL, state.algorithm, Msg::AlgorithmSelected).width(250).padding(8),
    ].spacing(4);

    let key_section = row![
        column![
            text("Private Key (PEM) - for signing").size(13),
            text_input("-----BEGIN PRIVATE KEY-----...", &state.private_key)
                .on_input(Msg::PrivateKeyChanged).padding(8).size(11),
        ].spacing(4).width(Length::Fill),
        column![
            text("Public Key (PEM) - for verifying").size(13),
            text_input("-----BEGIN PUBLIC KEY-----...", &state.public_key)
                .on_input(Msg::PublicKeyChanged).padding(8).size(11),
        ].spacing(4).width(Length::Fill),
    ].spacing(10);

    let data_section = column![
        text("Data to sign/verify").size(13),
        text_input("Enter data...", &state.data).on_input(Msg::DataChanged).padding(8).size(13),
    ].spacing(4);

    let sig_section = column![
        text("Signature (hex)").size(13),
        text_input("Signature will appear here...", &state.signature_hex)
            .on_input(Msg::SignatureChanged).padding(8).size(11).font(iced::Font::MONOSPACE),
    ].spacing(4);

    let btn_row = row![
        styled_btn("Sign", Msg::Sign),
        styled_btn("Verify", Msg::Verify),
    ].spacing(10);

    let result: Element<'_, Msg> = match state.verify_result {
        Some(true) => container(
            text("VALID - Signature verification succeeded").size(14).color(theme::SUCCESS)
        ).padding(10).width(Length::Fill)
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(iced::Color::from_rgb(0.93, 0.98, 0.93))),
            border: Border { radius: 4.0.into(), width: 1.0, color: theme::SUCCESS },
            ..container::Style::default()
        }).into(),
        Some(false) => container(
            text("INVALID - Signature verification failed").size(14).color(theme::ERROR)
        ).padding(10).width(Length::Fill)
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(iced::Color::from_rgb(1.0, 0.95, 0.95))),
            border: Border { radius: 4.0.into(), width: 1.0, color: theme::ERROR },
            ..container::Style::default()
        }).into(),
        None => column![].into(),
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
        .push(iced::widget::vertical_space().height(10))
        .push(key_section)
        .push(iced::widget::vertical_space().height(10))
        .push(data_section)
        .push(iced::widget::vertical_space().height(10))
        .push(sig_section)
        .push(iced::widget::vertical_space().height(10))
        .push(btn_row)
        .push(iced::widget::vertical_space().height(10))
        .push(result)
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
