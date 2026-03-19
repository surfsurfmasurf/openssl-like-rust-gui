use crate::crypto::symmetric::{self, CipherMode, SymAlgorithm};
use crate::theme;
use iced::widget::{button, column, container, pick_list, row, text, text_input, Column};
use iced::{Background, Border, Element, Length, Theme};

#[derive(Debug, Clone)]
pub enum Msg {
    AlgorithmSelected(SymAlgorithm),
    ModeSelected(CipherMode),
    KeyChanged(String),
    IvChanged(String),
    PlaintextChanged(String),
    CiphertextChanged(String),
    Encrypt,
    Decrypt,
    GenerateKey,
    GenerateIv,
    CopyResult,
}

pub struct State {
    pub algorithm: Option<SymAlgorithm>,
    pub mode: Option<CipherMode>,
    pub key_hex: String,
    pub iv_hex: String,
    pub plaintext: String,
    pub ciphertext: String,
    pub error: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            algorithm: Some(SymAlgorithm::Aes256),
            mode: Some(CipherMode::Gcm),
            key_hex: String::new(),
            iv_hex: String::new(),
            plaintext: String::new(),
            ciphertext: String::new(),
            error: String::new(),
        }
    }
}

pub fn update(state: &mut State, msg: Msg) -> Option<String> {
    state.error.clear();
    match msg {
        Msg::AlgorithmSelected(a) => { state.algorithm = Some(a); None }
        Msg::ModeSelected(m) => { state.mode = Some(m); None }
        Msg::KeyChanged(v) => { state.key_hex = v; None }
        Msg::IvChanged(v) => { state.iv_hex = v; None }
        Msg::PlaintextChanged(v) => { state.plaintext = v; None }
        Msg::CiphertextChanged(v) => { state.ciphertext = v; None }
        Msg::GenerateKey => {
            if let Some(algo) = state.algorithm {
                let key = symmetric::generate_key(algo);
                state.key_hex = hex::encode(&key);
            }
            None
        }
        Msg::GenerateIv => {
            if let (Some(algo), Some(mode)) = (state.algorithm, state.mode) {
                let iv = symmetric::generate_iv(algo, mode);
                state.iv_hex = hex::encode(&iv);
            }
            None
        }
        Msg::Encrypt => {
            let (algo, mode) = match (state.algorithm, state.mode) {
                (Some(a), Some(m)) => (a, m),
                _ => return None,
            };
            match (hex::decode(&state.key_hex), hex::decode(&state.iv_hex)) {
                (Ok(key), Ok(iv)) => {
                    match symmetric::encrypt(algo, mode, &key, &iv, state.plaintext.as_bytes()) {
                        Ok(ct) => {
                            use base64::{engine::general_purpose::STANDARD, Engine};
                            state.ciphertext = STANDARD.encode(&ct);
                            Some("Encrypted successfully".into())
                        }
                        Err(e) => { state.error = e.to_string(); None }
                    }
                }
                _ => { state.error = "Invalid hex in key or IV".into(); None }
            }
        }
        Msg::Decrypt => {
            let (algo, mode) = match (state.algorithm, state.mode) {
                (Some(a), Some(m)) => (a, m),
                _ => return None,
            };
            match (hex::decode(&state.key_hex), hex::decode(&state.iv_hex)) {
                (Ok(key), Ok(iv)) => {
                    use base64::{engine::general_purpose::STANDARD, Engine};
                    match STANDARD.decode(state.ciphertext.trim()) {
                        Ok(ct) => match symmetric::decrypt(algo, mode, &key, &iv, &ct) {
                            Ok(pt) => {
                                state.plaintext = String::from_utf8_lossy(&pt).into_owned();
                                Some("Decrypted successfully".into())
                            }
                            Err(e) => { state.error = e.to_string(); None }
                        },
                        Err(e) => { state.error = format!("Invalid base64: {}", e); None }
                    }
                }
                _ => { state.error = "Invalid hex in key or IV".into(); None }
            }
        }
        Msg::CopyResult => Some("copy".into()),
    }
}

pub fn view(state: &State) -> Element<'_, Msg> {
    let title = text("Symmetric Encryption").size(24).color(theme::TEXT_DARK);
    let desc = text("AES (CBC/GCM), DES, 3DES encryption and decryption")
        .size(13)
        .color(iced::Color::from_rgb(0.5, 0.5, 0.55));

    let algo_row = row![
        column![
            text("Algorithm").size(13),
            pick_list(SymAlgorithm::ALL, state.algorithm, Msg::AlgorithmSelected).width(150).padding(8),
        ].spacing(4),
        column![
            text("Mode").size(13),
            pick_list(CipherMode::ALL, state.mode, Msg::ModeSelected).width(120).padding(8),
        ].spacing(4),
    ]
    .spacing(15);

    let key_info = state.algorithm.map(|a| format!("({} bytes hex)", a.key_size())).unwrap_or_default();
    let key_row = row![
        text_input(&format!("Key {}", key_info), &state.key_hex)
            .on_input(Msg::KeyChanged)
            .padding(8)
            .size(13)
            .font(iced::Font::MONOSPACE)
            .width(Length::Fill),
        styled_btn("Gen Key", Msg::GenerateKey),
    ]
    .spacing(8);

    let iv_info = if let (Some(a), Some(m)) = (state.algorithm, state.mode) {
        format!("({} bytes hex)", a.iv_size(m))
    } else { String::new() };
    let iv_row = row![
        text_input(&format!("IV/Nonce {}", iv_info), &state.iv_hex)
            .on_input(Msg::IvChanged)
            .padding(8)
            .size(13)
            .font(iced::Font::MONOSPACE)
            .width(Length::Fill),
        styled_btn("Gen IV", Msg::GenerateIv),
    ]
    .spacing(8);

    let data_row = row![
        column![
            text("Plaintext").size(13),
            text_input("Enter plaintext...", &state.plaintext)
                .on_input(Msg::PlaintextChanged)
                .padding(10)
                .size(13),
        ]
        .spacing(4)
        .width(Length::Fill),
        column![
            styled_btn("Encrypt >>", Msg::Encrypt),
            styled_btn("<< Decrypt", Msg::Decrypt),
        ]
        .spacing(5)
        .width(Length::Shrink),
        column![
            text("Ciphertext (Base64)").size(13),
            text_input("Ciphertext...", &state.ciphertext)
                .on_input(Msg::CiphertextChanged)
                .padding(10)
                .size(13)
                .font(iced::Font::MONOSPACE),
        ]
        .spacing(4)
        .width(Length::Fill),
    ]
    .spacing(10);

    let error_el: Element<'_, Msg> = if !state.error.is_empty() {
        container(
            text(&state.error).size(13).color(theme::ERROR),
        )
        .padding(8)
        .width(Length::Fill)
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(iced::Color::from_rgb(1.0, 0.95, 0.95))),
            border: Border { radius: 4.0.into(), width: 1.0, color: theme::ERROR },
            ..container::Style::default()
        })
        .into()
    } else {
        column![].into()
    };

    let content = Column::new()
        .push(title)
        .push(desc)
        .push(iced::widget::vertical_space().height(15))
        .push(algo_row)
        .push(iced::widget::vertical_space().height(10))
        .push(text("Key (hex)").size(13))
        .push(key_row)
        .push(text("IV / Nonce (hex)").size(13))
        .push(iv_row)
        .push(iced::widget::vertical_space().height(15))
        .push(data_row)
        .push(iced::widget::vertical_space().height(10))
        .push(error_el)
        .spacing(5)
        .width(Length::Fill);

    card(content)
}

fn styled_btn(label: &str, msg: Msg) -> Element<'static, Msg> {
    button(
        text(label.to_string()).size(13).color(iced::Color::WHITE).center(),
    )
    .on_press(msg)
    .padding([8, 16])
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

fn card<'a>(content: Column<'a, Msg>) -> Element<'a, Msg> {
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
