use eframe::egui;

use super::main_window::MainWindow;

pub fn view_phonebook(window: &mut MainWindow, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("button_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Connect").clicked() {
                window.call_bbs(0);
            }
            ui.text_edit_singleline(&mut window.addresses[0].address);
        });
    });

    
    egui::CentralPanel::default()
    .show(ctx, |ui| {
        for i in 1..window.addresses.len() {
            let addr = &window.addresses[i];
            if ui.button(addr.system_name.clone()).clicked() {
                window.call_bbs(i);
            }
        }
    });
    /* 
    let list_header = Column::new()
        .push(
            Row::new()
                .push(horizontal_space(Length::Units(20)))
                .push(
                    create_icon_button("\u{F54D}")
                        .on_press(Message::ListAction(HoverListMessage::CallBBS(0))),
                )
                .push(horizontal_space(Length::Units(10)))
                .push(
                    text_input(
                        "Quick connect to…",
                        &main_window.addresses[0].address,
                        Message::QuickConnectChanged,
                    )
                    .id(INPUT_ID.clone())
                    .size(18),
                )
                .push(horizontal_space(Length::Units(10)))
                .align_items(Alignment::Center),
        );

    let h = main_window.address_list.get_height();
    let canvas: Element<HoverListMessage> = Canvas::new(&main_window.address_list)
        .width(Length::Units(250))
        .height(Length::Units(h))
        .into();

    let canvas = canvas.map(Message::ListAction);

    let scrollable_content = iced::widget::scrollable(canvas).height(Length::Fill);

    let button_row = Row::new()
        .push(horizontal_space(Length::Fill))
        .push(
            Button::new(
                Text::new("\u{F56B}")
                    .width(Length::Shrink)
                    .height(Length::Shrink)
                    .font(iced_aw::ICON_FONT)
                    .size(24),
            )
            .on_press(Message::AskDeleteEntry)
            .padding(5)
            .style(theme::Button::Custom(Box::new(CircleButtonStyle::new(
                theme::Button::Primary,
            )))),
        )
        .push(
            Button::new(
                Text::new(Icon::Plus.to_string())
                    .width(Length::Shrink)
                    .height(Length::Shrink)
                    .font(iced_aw::ICON_FONT)
                    .size(24),
            )
            .on_press(Message::CreateNewBBS)
            .padding(5)
            .style(theme::Button::Custom(Box::new(CircleButtonStyle::new(
                theme::Button::Primary,
            )))),
        )
        .padding(10)
        .spacing(10);

    let content = Column::new()
        .push(scrollable_content)
        .push(button_row)
        .width(Length::Units(250))
        .height(Length::Fill)
        .max_width(250);

    Column::new()
        .push(list_header)
        .push(
            Row::new()
                .push(content)
                .push(vertical_rule(5))
                .push(view_edit_bbs(main_window))
        )
        .padding(8)
        .spacing(8)
        .into()*/
}
