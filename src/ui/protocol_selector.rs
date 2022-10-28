use iced::widget::{ column, row, button, text, horizontal_space};
use iced::{
    Element, Length
};
use crate::protocol::ProtocolType;
use super::main_window::{Message};


pub fn view_protocol_selector<'a>(download: bool) -> Element<'a, Message> {
    let button_width = 90;
    let space = 8;
    let left = 20;

    column![
        row![
            button("Cancel")
                .on_press(Message::Back),
        ].padding(4)
        .spacing(8),
        text(format!("Select {} protocol", if download { "download" } else { "upload" })).size(40),
        row![ horizontal_space(Length::Units(left)), button("Zmodem").on_press(Message::SelectProtocol(ProtocolType::ZModem, download)).width(Length::Units(button_width)), horizontal_space(Length::Units(space)), text("The standard protocol")],
        row![ horizontal_space(Length::Units(left)), button("ZedZap").on_press(Message::SelectProtocol(ProtocolType::ZedZap, download)).width(Length::Units(button_width)), horizontal_space(Length::Units(space)), text("8k Zmodem")],
        row![ horizontal_space(Length::Units(left)), button("Xmodem").on_press(Message::SelectProtocol(ProtocolType::XModem, download)).width(Length::Units(button_width)), horizontal_space(Length::Units(space)), text("Outdated protocol")],
        row![ horizontal_space(Length::Units(left)), button("Ymodem").on_press(Message::SelectProtocol(ProtocolType::YModem, download)).width(Length::Units(button_width)), horizontal_space(Length::Units(space)), text("Ok but Zmodem is better")],
        row![ horizontal_space(Length::Units(left)), button("YmodemG").on_press(Message::SelectProtocol(ProtocolType::YModemG, download)).width(Length::Units(button_width)), horizontal_space(Length::Units(space)), text("A fast Ymodem variant")]
    ].spacing(8).into()
}