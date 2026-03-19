use crate::app::{Message, Tab};
use crate::theme;
use iced::widget::{button, column, container, text, Column};
use iced::{Background, Border, Element, Length, Theme};

struct SidebarButton {
    tab: Tab,
    icon: &'static str,
    label: &'static str,
}

const TABS: &[SidebarButton] = &[
    SidebarButton { tab: Tab::Hashing, icon: "#", label: "Hash" },
    SidebarButton { tab: Tab::SymmetricEncryption, icon: "E", label: "Symmetric" },
    SidebarButton { tab: Tab::AsymmetricKeys, icon: "K", label: "Asymmetric" },
    SidebarButton { tab: Tab::Certificates, icon: "C", label: "Certificates" },
    SidebarButton { tab: Tab::Signatures, icon: "S", label: "Signatures" },
    SidebarButton { tab: Tab::Encoding, icon: "B", label: "Base64" },
    SidebarButton { tab: Tab::RandomData, icon: "R", label: "Random" },
    SidebarButton { tab: Tab::FileEncryption, icon: "F", label: "File Encrypt" },
    SidebarButton { tab: Tab::TlsConnect, icon: "T", label: "TLS Connect" },
    SidebarButton { tab: Tab::KeyInspect, icon: "I", label: "Key Inspect" },
    SidebarButton { tab: Tab::Ciphers, icon: "?", label: "Algorithms" },
];

pub fn view(active: &Tab) -> Element<'static, Message> {
    let title = container(
        text("OpenSSL Tool")
            .size(20)
            .color(theme::TEXT_LIGHT)
            .width(Length::Fill)
            .center(),
    )
    .padding(20);

    let subtitle = container(
        text("Rust Edition")
            .size(12)
            .color(iced::Color::from_rgb(0.6, 0.6, 0.7))
            .width(Length::Fill)
            .center(),
    )
    .padding(iced::Padding { top: 0.0, right: 0.0, bottom: 15.0, left: 0.0 });

    let buttons: Vec<Element<Message>> = TABS
        .iter()
        .map(|sb| {
            let is_active = *active == sb.tab;
            let label = text(format!("  [{}]  {}", sb.icon, sb.label))
                .size(14)
                .color(if is_active {
                    iced::Color::WHITE
                } else {
                    theme::TEXT_LIGHT
                });

            let btn = button(label)
                .on_press(Message::TabSelected(sb.tab))
                .width(Length::Fill)
                .padding([10, 15])
                .style(move |_theme: &Theme, status| {
                    let bg = if is_active {
                        theme::SIDEBAR_ACTIVE
                    } else {
                        match status {
                            button::Status::Hovered => theme::SIDEBAR_HOVER,
                            _ => iced::Color::TRANSPARENT,
                        }
                    };
                    button::Style {
                        background: Some(Background::Color(bg)),
                        text_color: if is_active {
                            iced::Color::WHITE
                        } else {
                            theme::TEXT_LIGHT
                        },
                        border: Border::default(),
                        ..button::Style::default()
                    }
                });

            btn.into()
        })
        .collect();

    let sidebar_col = Column::with_children(
        std::iter::once(title.into())
            .chain(std::iter::once(subtitle.into()))
            .chain(buttons),
    )
    .spacing(2);

    container(sidebar_col)
        .width(220)
        .height(Length::Fill)
        .style(|_theme: &Theme| container::Style {
            background: Some(Background::Color(theme::SIDEBAR_BG)),
            ..container::Style::default()
        })
        .into()
}
