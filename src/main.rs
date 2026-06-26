use wxdragon::appearance::*;
use wxdragon::prelude::*;

fn main() {
    let _ = wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("Hello, World!")
            .with_size(Size::new(300, 200))
            .build();

        if let Some(app) = wxdragon::app::get_app() {
            app.set_appearance(Appearance::Light);
        }

        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        let button = Button::builder(&frame).with_label("Click me").build();

        button.on_click(|_| {
            println!("Button clicked");
        });

        sizer.add(
            &button,
            0,
            SizerFlag::AlignCenterHorizontal | SizerFlag::AlignCenterVertical,
            0,
        );

        frame.set_sizer(sizer, true);
        frame.show(true);
        frame.centre();
    });
}
