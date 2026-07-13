#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use image::{DynamicImage, imageops::FilterType};
use wxdragon::appearance::*;
use wxdragon::{prelude::*, widgets::GenericStaticBitmap};

fn main() {
    wxdragon::main(|_| run()).unwrap();
}

fn run() {
    if let Some(app) = wxdragon::app::get_app() {
        app.set_appearance(Appearance::System);
    }

    let frame = Frame::builder()
        .with_title("Catpack Maker")
        .with_size(Size {
            width: 600,
            height: 400,
        })
        .build();

    let panel = Panel::builder(&frame).build();
    let scroll = ScrolledWindow::builder(&frame).build();
    scroll.set_scroll_rate(0, 10);

    let sizer = BoxSizer::builder(Orientation::Vertical).build();

    let panel_sizer = BoxSizer::builder(Orientation::Vertical).build();
    let button1 = Button::builder(&panel)
        .with_label("Выбрать папку набора")
        .build();
    panel_sizer.add(&button1, 1, SizerFlag::AlignCenterHorizontal, 0);
    panel.set_sizer(panel_sizer, true);

    let frame_sizer = BoxSizer::builder(Orientation::Vertical).build();
    frame_sizer.add(&panel, 0, SizerFlag::Expand, 0);
    frame_sizer.add(&scroll, 1, SizerFlag::Expand, 0);
    frame.set_sizer(frame_sizer, true);

    let mut pic_ctr = 0;
    button1.on_click({
        move |_| {
            pic_ctr += 1;
            let row_sizer = BoxSizer::builder(Orientation::Horizontal).build();
            let row = Panel::builder(&scroll).build();

            let row_picture =
                GenericStaticBitmap::new_with_bitmap(&row, pic_ctr + 50, &Bitmap::null_bitmap());
            row_sizer.add(&row_picture, 0, SizerFlag::AlignCenterVertical, 0);

            let picture_pick_button = Button::builder(&row).with_label("Выбрать картинку").build();
            let section_delete_button = Button::builder(&row).with_label("Удалить пункт").build();

            picture_pick_button.on_click({
                move |_| {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Images", &["png", "jpg", "jpeg", "webp"])
                        .pick_file()
                    {
                        let img = image::ImageReader::open(path).unwrap().decode().unwrap();
                        let img_resized = fit_image(img, 300, 150);
                        let img_rgba8 = img_resized.to_rgba8();
                        let bitmap = Bitmap::from_rgba(
                            &img_rgba8.as_raw(),
                            img_rgba8.width(),
                            img_rgba8.height(),
                        )
                        .unwrap();

                        row_picture.set_bitmap(&bitmap);
                        row.layout();
                        scroll.layout();
                        frame.layout();
                    }
                }
            });

            section_delete_button.on_click({
                move |_| {
                    call_after(Box::new(move || {
                        row.destroy();
                        scroll.layout();
                    }));
                    frame.layout();
                }
            });

            row_sizer.add_stretch_spacer(1);
            row_sizer.add(&picture_pick_button, 0, SizerFlag::AlignCenterVertical, 0);
            row_sizer.add(&section_delete_button, 0, SizerFlag::AlignCenterVertical, 0);

            row.set_sizer(row_sizer, true);
            sizer.add(&row, 0, SizerFlag::Expand, 10);
            scroll.layout();
            frame.layout();
        }
    });

    scroll.set_sizer(sizer, true);

    frame.set_min_size(Size {
        width: 300,
        height: 200,
    });
    frame.show(true);
    frame.centre();
}

fn fit_image(img: DynamicImage, max_width: u32, max_height: u32) -> DynamicImage {
    let w = img.width() as f32;
    let h = img.height() as f32;

    let scale = (max_width as f32 / w).min(max_height as f32 / h);

    let new_w = (w * scale).round() as u32;
    let new_h = (h * scale).round() as u32;

    img.resize_exact(new_w, new_h, FilterType::Triangle)
}
