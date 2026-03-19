use crate::crypto::certificates;
use crate::theme;
use iced::widget::{button, checkbox, column, container, row, scrollable, text, text_input, Column};
use iced::{Background, Border, Element, Length, Theme};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubTab {
    Generate,
    ViewCert,
    Csr,
}

#[derive(Debug, Clone)]
pub enum Msg {
    SubTabSelected(SubTab),
    CnChanged(String),
    OrgChanged(String),
    DaysChanged(String),
    SanChanged(String),
    CaToggled(bool),
    GenerateCert,
    GenerateCsr,
    CertPemChanged(String),
    ParseCert,
    CopyResult,
}

pub struct State {
    pub sub_tab: SubTab,
    pub cn: String,
    pub org: String,
    pub days: String,
    pub san: String,
    pub is_ca: bool,
    pub cert_pem: String,
    pub key_pem: String,
    pub csr_pem: String,
    pub view_input: String,
    pub cert_info: String,
    pub error: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            sub_tab: SubTab::Generate,
            cn: "example.com".into(),
            org: "My Organization".into(),
            days: "365".into(),
            san: "example.com".into(),
            is_ca: false,
            cert_pem: String::new(),
            key_pem: String::new(),
            csr_pem: String::new(),
            view_input: String::new(),
            cert_info: String::new(),
            error: String::new(),
        }
    }
}

pub fn update(state: &mut State, msg: Msg) -> Option<String> {
    state.error.clear();
    match msg {
        Msg::SubTabSelected(t) => { state.sub_tab = t; None }
        Msg::CnChanged(v) => { state.cn = v; None }
        Msg::OrgChanged(v) => { state.org = v; None }
        Msg::DaysChanged(v) => { state.days = v; None }
        Msg::SanChanged(v) => { state.san = v; None }
        Msg::CaToggled(v) => { state.is_ca = v; None }
        Msg::CertPemChanged(v) => { state.view_input = v; None }
        Msg::GenerateCert => {
            let days: u32 = state.days.parse().unwrap_or(365);
            let sans: Vec<String> = state.san.split(',').map(|s| s.trim().to_string()).collect();
            match certificates::generate_self_signed(&state.cn, &state.org, days, &sans, state.is_ca) {
                Ok(result) => {
                    state.cert_pem = result.cert_pem;
                    state.key_pem = result.key_pem;
                    Some("Self-signed certificate generated".into())
                }
                Err(e) => { state.error = e.to_string(); None }
            }
        }
        Msg::GenerateCsr => {
            let sans: Vec<String> = state.san.split(',').map(|s| s.trim().to_string()).collect();
            match certificates::generate_csr(&state.cn, &state.org, &sans) {
                Ok(csr) => {
                    state.csr_pem = csr;
                    Some("CSR generated".into())
                }
                Err(e) => { state.error = e.to_string(); None }
            }
        }
        Msg::ParseCert => {
            match certificates::parse_certificate(&state.view_input) {
                Ok(info) => {
                    let mut lines = Vec::new();
                    lines.push(format!("Subject: {}", info.subject));
                    lines.push(format!("Issuer: {}", info.issuer));
                    lines.push(format!("Serial: {}", info.serial));
                    lines.push(format!("Not Before: {}", info.not_before));
                    lines.push(format!("Not After: {}", info.not_after));
                    lines.push(format!("Signature Algo: {}", info.signature_algorithm));
                    lines.push(format!("Public Key Algo: {}", info.public_key_algorithm));
                    lines.push(format!("Is CA: {}", info.is_ca));
                    if !info.san.is_empty() {
                        lines.push(format!("SANs: {}", info.san.join(", ")));
                    }
                    state.cert_info = lines.join("\n");
                    Some("Certificate parsed".into())
                }
                Err(e) => { state.error = e.to_string(); None }
            }
        }
        Msg::CopyResult => Some("copy".into()),
    }
}

pub fn view(state: &State) -> Element<'_, Msg> {
    let title = text("X.509 Certificates").size(24).color(theme::TEXT_DARK);

    let tabs = row![
        tab_btn("Generate", SubTab::Generate, state.sub_tab),
        tab_btn("View Certificate", SubTab::ViewCert, state.sub_tab),
        tab_btn("Generate CSR", SubTab::Csr, state.sub_tab),
    ]
    .spacing(5);

    let body: Element<'_, Msg> = match state.sub_tab {
        SubTab::Generate => view_generate(state),
        SubTab::ViewCert => view_cert(state),
        SubTab::Csr => view_csr(state),
    };

    let error_el: Element<'_, Msg> = if !state.error.is_empty() {
        text(&state.error).size(13).color(theme::ERROR).into()
    } else {
        column![].into()
    };

    let content = Column::new()
        .push(title)
        .push(iced::widget::vertical_space().height(10))
        .push(tabs)
        .push(iced::widget::vertical_space().height(15))
        .push(body)
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

fn view_generate(state: &State) -> Element<'_, Msg> {
    let form = column![
        text("Common Name (CN)").size(13),
        text_input("example.com", &state.cn).on_input(Msg::CnChanged).padding(8).size(13),
        text("Organization").size(13),
        text_input("My Organization", &state.org).on_input(Msg::OrgChanged).padding(8).size(13),
        text("Validity (days)").size(13),
        text_input("365", &state.days).on_input(Msg::DaysChanged).padding(8).size(13),
        text("Subject Alternative Names (comma-separated)").size(13),
        text_input("example.com, www.example.com", &state.san).on_input(Msg::SanChanged).padding(8).size(13),
        checkbox("Certificate Authority (CA)", state.is_ca).on_toggle(Msg::CaToggled),
        styled_btn("Generate Self-Signed Certificate", Msg::GenerateCert),
    ]
    .spacing(6);

    let result = if !state.cert_pem.is_empty() {
        column![
            text("Certificate PEM").size(13).color(theme::TEXT_DARK),
            pem_box(&state.cert_pem),
            text("Private Key PEM").size(13).color(theme::TEXT_DARK),
            pem_box(&state.key_pem),
        ].spacing(8)
    } else {
        column![]
    };

    column![form, result].spacing(15).into()
}

fn view_cert(state: &State) -> Element<'_, Msg> {
    let form = column![
        text("Paste Certificate PEM").size(13),
        text_input("-----BEGIN CERTIFICATE-----...", &state.view_input)
            .on_input(Msg::CertPemChanged).padding(8).size(13),
        styled_btn("Parse Certificate", Msg::ParseCert),
    ].spacing(6);

    let info: Element<'_, Msg> = if !state.cert_info.is_empty() {
        let info_text: iced::widget::Text<'_, Theme> = text(state.cert_info.clone())
            .size(12)
            .font(iced::Font::MONOSPACE)
            .color(theme::TEXT_DARK);
        container(info_text)
            .padding(12)
            .width(Length::Fill)
            .style(|_: &Theme| container::Style {
                background: Some(Background::Color(iced::Color::from_rgb(0.93, 0.95, 0.98))),
                border: Border { radius: 4.0.into(), width: 1.0, color: theme::BORDER },
                ..container::Style::default()
            })
            .into()
    } else {
        column![].into()
    };

    column![form, info].spacing(15).into()
}

fn view_csr(state: &State) -> Element<'_, Msg> {
    let form = column![
        text("Common Name (CN)").size(13),
        text_input("example.com", &state.cn).on_input(Msg::CnChanged).padding(8).size(13),
        text("Organization").size(13),
        text_input("My Organization", &state.org).on_input(Msg::OrgChanged).padding(8).size(13),
        text("SANs (comma-separated)").size(13),
        text_input("example.com", &state.san).on_input(Msg::SanChanged).padding(8).size(13),
        styled_btn("Generate CSR", Msg::GenerateCsr),
    ].spacing(6);

    let result = if !state.csr_pem.is_empty() {
        column![
            text("CSR PEM").size(13).color(theme::TEXT_DARK),
            pem_box(&state.csr_pem),
        ].spacing(8)
    } else {
        column![]
    };

    column![form, result].spacing(15).into()
}

fn pem_box(pem: &str) -> Element<'static, Msg> {
    container(text(pem.to_string()).size(11).font(iced::Font::MONOSPACE).color(theme::TEXT_DARK))
        .padding(10)
        .width(Length::Fill)
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(iced::Color::from_rgb(0.93, 0.95, 0.98))),
            border: Border { radius: 4.0.into(), width: 1.0, color: theme::BORDER },
            ..container::Style::default()
        })
        .into()
}

fn tab_btn(label: &str, tab: SubTab, active: SubTab) -> Element<'static, Msg> {
    let is_active = tab == active;
    button(text(label.to_string()).size(13).color(if is_active { iced::Color::WHITE } else { theme::TEXT_DARK }).center())
        .on_press(Msg::SubTabSelected(tab))
        .padding([8, 16])
        .style(move |_: &Theme, _| button::Style {
            background: Some(Background::Color(if is_active { theme::ACCENT } else { iced::Color::from_rgb(0.9, 0.9, 0.92) })),
            text_color: if is_active { iced::Color::WHITE } else { theme::TEXT_DARK },
            border: Border { radius: 4.0.into(), ..Border::default() },
            ..button::Style::default()
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
