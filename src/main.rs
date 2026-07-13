#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use image::{DynamicImage, imageops::FilterType};
use wxdragon::{prelude::*, widgets::GenericStaticBitmap};

fn main() {
    init_platform();

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

    scroll.set_sizer(sizer.clone(), true);

    let mut pic_ctr = 0;
    button1.on_click({
        let scroll_clone = scroll.clone();
        let sizer_clone = sizer.clone();
        let frame_clone = frame.clone();
        move |_| {
            pic_ctr += 1;
            let row_sizer = BoxSizer::builder(Orientation::Horizontal).build();
            let row = Panel::builder(&scroll_clone).build();

            let row_picture =
                GenericStaticBitmap::new_with_bitmap(&row, pic_ctr + 50, &Bitmap::null_bitmap());
            row_sizer.add(&row_picture, 0, SizerFlag::AlignCenterVertical, 0);

            let picture_pick_button = Button::builder(&row).with_label("Выбрать картинку").build();
            let section_delete_button = Button::builder(&row).with_label("Удалить пункт").build();

            picture_pick_button.on_click({
                let scroll_clone2 = scroll_clone.clone();
                let frame_clone2 = frame_clone.clone();
                let row_picture_clone = row_picture.clone();
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

                        row_picture_clone.set_bitmap(&bitmap);
                        row.layout();
                        scroll_clone2.layout();
                        frame_clone2.layout();
                    }
                }
            });

            section_delete_button.on_click({
                let row_clone = row.clone();
                let scroll_clone3 = scroll_clone.clone();
                let frame_clone3 = frame_clone.clone();
                move |_| {
                    call_after(Box::new(move || {
                        row_clone.destroy();
                        scroll_clone3.layout();
                    }));
                    frame_clone3.layout();
                }
            });

            row_sizer.add_stretch_spacer(1);
            row_sizer.add(&picture_pick_button, 0, SizerFlag::AlignCenterVertical, 0);
            row_sizer.add(&section_delete_button, 0, SizerFlag::AlignCenterVertical, 0);

            row.set_sizer(row_sizer, true);
            sizer_clone.add(&row, 0, SizerFlag::Expand, 10);
            scroll_clone.layout();
            frame_clone.layout();
        }
    });

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

fn init_platform() {
    #[cfg(target_os = "windows")]
    unsafe {
        std::env::set_var("wx_msw_dark_mode", "1");
    }
}
