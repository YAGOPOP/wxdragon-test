use wxdragon::prelude::*;

fn main() {
    #[cfg(target_os = "windows")]
    unsafe {
        std::env::set_var("wx_msw_dark_mode", "2");
    }

    wxdragon::main(|_| run()).unwrap();
}

fn run() {
    let frame = Frame::builder()
        .with_title("Catpack Maker")
        .with_size(Size {
            width: 600,
            height: 400,
        })
        .build();

    let panel = Panel::builder(&frame).build();
    let sizer = BoxSizer::builder(Orientation::Vertical).build();

    let button1 = Button::builder(&panel)
        .with_label("Добавить кнопку")
        .build();

    sizer.add(&button1, 0, SizerFlag::All, 10);
    panel.set_sizer(sizer.clone(), true);

    let panel_clone = panel.clone();
    let sizer_clone = sizer.clone();

    let mut ctr = 0;
    button1.on_click(move |_| {
        ctr += 1;
        let row = Panel::builder(&panel_clone).build();

        let new_button = Button::builder(&row)
            .with_label(&format!("Новая кнопка {}", ctr))
            .build();
        let new_button_delete = Button::builder(&row)
            .with_label(&format!("Удалить новую кнопку {}", ctr))
            .build();

        let row_sizer = BoxSizer::builder(Orientation::Horizontal).build();
        let row_clone = row.clone();
        let panel_clone = panel_clone.clone();
        new_button_delete.on_click(move |_| {
            call_after(Box::new(move || {
                row_clone.destroy();
                panel_clone.layout();
            }))
        });

        row_sizer.add_stretch_spacer(1);
        row_sizer.add(&new_button, 0, SizerFlag::Shrink, 0);
        row_sizer.add(&new_button_delete, 0, SizerFlag::Shrink, 0);

        row.set_sizer(row_sizer, true);
        sizer_clone.add(&row, 0, SizerFlag::Expand, 10);
        panel_clone.layout();
    });

    frame.set_min_size(Size {
        width: 300,
        height: 200,
    });
    frame.show(true);
    frame.centre();
}
