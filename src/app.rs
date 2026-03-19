use crate::theme;
use crate::ui::{
    asymmetric_tab, certificate_tab, ciphers_tab, encoding_tab, file_encrypt_tab, hash_tab,
    key_inspect_tab, random_tab, sidebar, signature_tab, symmetric_tab, tls_tab,
};
use iced::widget::{column, container, row, text};
use iced::{Background, Border, Element, Length, Task, Theme};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Hashing,
    SymmetricEncryption,
    AsymmetricKeys,
    Certificates,
    Signatures,
    Encoding,
    RandomData,
    FileEncryption,
    TlsConnect,
    KeyInspect,
    Ciphers,
}

#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(Tab),
    Hash(hash_tab::Msg),
    Symmetric(symmetric_tab::Msg),
    Asymmetric(asymmetric_tab::Msg),
    Certificate(certificate_tab::Msg),
    Signature(signature_tab::Msg),
    Encoding(encoding_tab::Msg),
    Random(random_tab::Msg),
    FileEncrypt(file_encrypt_tab::Msg),
    Tls(tls_tab::Msg),
    KeyInspect(key_inspect_tab::Msg),
    Ciphers(ciphers_tab::Msg),
}

pub struct App {
    active_tab: Tab,
    status: String,
    hash_state: hash_tab::State,
    symmetric_state: symmetric_tab::State,
    asymmetric_state: asymmetric_tab::State,
    certificate_state: certificate_tab::State,
    signature_state: signature_tab::State,
    encoding_state: encoding_tab::State,
    random_state: random_tab::State,
    file_encrypt_state: file_encrypt_tab::State,
    tls_state: tls_tab::State,
    key_inspect_state: key_inspect_tab::State,
    ciphers_state: ciphers_tab::State,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                active_tab: Tab::Hashing,
                status: "Ready".into(),
                hash_state: hash_tab::State::new(),
                symmetric_state: symmetric_tab::State::default(),
                asymmetric_state: asymmetric_tab::State::default(),
                certificate_state: certificate_tab::State::default(),
                signature_state: signature_tab::State::default(),
                encoding_state: encoding_tab::State::default(),
                random_state: random_tab::State::default(),
                file_encrypt_state: file_encrypt_tab::State::default(),
                tls_state: tls_tab::State::default(),
                key_inspect_state: key_inspect_tab::State::default(),
                ciphers_state: ciphers_tab::State::default(),
            },
            Task::none(),
        )
    }

    pub fn theme(&self) -> Theme {
        Theme::Light
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TabSelected(tab) => {
                self.active_tab = tab;
                Task::none()
            }
            Message::Hash(msg) => {
                if let Some(status) = hash_tab::update(&mut self.hash_state, msg) {
                    self.status = status;
                }
                Task::none()
            }
            Message::Symmetric(msg) => {
                if let Some(status) = symmetric_tab::update(&mut self.symmetric_state, msg) {
                    self.status = status;
                }
                Task::none()
            }
            Message::Asymmetric(msg) => {
                let (status, task) =
                    asymmetric_tab::update(&mut self.asymmetric_state, msg);
                if let Some(s) = status {
                    self.status = s;
                }
                task.map(|t| t.map(Message::Asymmetric)).unwrap_or(Task::none())
            }
            Message::Certificate(msg) => {
                if let Some(status) = certificate_tab::update(&mut self.certificate_state, msg) {
                    self.status = status;
                }
                Task::none()
            }
            Message::Signature(msg) => {
                if let Some(status) = signature_tab::update(&mut self.signature_state, msg) {
                    self.status = status;
                }
                Task::none()
            }
            Message::Encoding(msg) => {
                if let Some(status) = encoding_tab::update(&mut self.encoding_state, msg) {
                    self.status = status;
                }
                Task::none()
            }
            Message::Random(msg) => {
                if let Some(status) = random_tab::update(&mut self.random_state, msg) {
                    self.status = status;
                }
                Task::none()
            }
            Message::FileEncrypt(msg) => {
                if let Some(status) =
                    file_encrypt_tab::update(&mut self.file_encrypt_state, msg)
                {
                    self.status = status;
                }
                Task::none()
            }
            Message::Tls(msg) => {
                let (status, task) = tls_tab::update(&mut self.tls_state, msg);
                if let Some(s) = status {
                    self.status = s;
                }
                task.map(|t| t.map(Message::Tls)).unwrap_or(Task::none())
            }
            Message::KeyInspect(msg) => {
                if let Some(status) = key_inspect_tab::update(&mut self.key_inspect_state, msg) {
                    self.status = status;
                }
                Task::none()
            }
            Message::Ciphers(_msg) => {
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let sidebar = sidebar::view(&self.active_tab).map(|m| m);

        let content: Element<Message> = match self.active_tab {
            Tab::Hashing => hash_tab::view(&self.hash_state).map(Message::Hash),
            Tab::SymmetricEncryption => {
                symmetric_tab::view(&self.symmetric_state).map(Message::Symmetric)
            }
            Tab::AsymmetricKeys => {
                asymmetric_tab::view(&self.asymmetric_state).map(Message::Asymmetric)
            }
            Tab::Certificates => {
                certificate_tab::view(&self.certificate_state).map(Message::Certificate)
            }
            Tab::Signatures => {
                signature_tab::view(&self.signature_state).map(Message::Signature)
            }
            Tab::Encoding => encoding_tab::view(&self.encoding_state).map(Message::Encoding),
            Tab::RandomData => random_tab::view(&self.random_state).map(Message::Random),
            Tab::FileEncryption => {
                file_encrypt_tab::view(&self.file_encrypt_state).map(Message::FileEncrypt)
            }
            Tab::TlsConnect => tls_tab::view(&self.tls_state).map(Message::Tls),
            Tab::KeyInspect => {
                key_inspect_tab::view(&self.key_inspect_state).map(Message::KeyInspect)
            }
            Tab::Ciphers => ciphers_tab::view(&self.ciphers_state).map(Message::Ciphers),
        };

        let content_area = container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(15)
            .style(|_: &Theme| container::Style {
                background: Some(Background::Color(theme::CONTENT_BG)),
                ..container::Style::default()
            });

        let status_bar = container(
            text(&self.status)
                .size(12)
                .color(iced::Color::from_rgb(0.5, 0.5, 0.55)),
        )
        .padding([6, 15])
        .width(Length::Fill)
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(iced::Color::from_rgb(0.92, 0.92, 0.94))),
            border: Border {
                width: 1.0,
                color: theme::BORDER,
                ..Border::default()
            },
            ..container::Style::default()
        });

        let main_row = row![sidebar, content_area];

        column![main_row, status_bar].into()
    }
}
