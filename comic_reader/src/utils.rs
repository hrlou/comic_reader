use crate::prelude::*;

pub fn save_image_to_file(image: &DynamicImage, path: &Path) {
    use std::fs::File;
    use std::io::BufWriter;

    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
        let file = match File::create(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to create file: {}", e);
                return;
            }
        };
        let mut writer = BufWriter::new(file);
        let result = match ext.to_lowercase().as_str() {
            "png" => image.write_to(&mut writer, image::ImageOutputFormat::Png),
            "jpg" | "jpeg" => image.write_to(&mut writer, image::ImageOutputFormat::Jpeg(90)),
            _ => {
                eprintln!("Unsupported format: {}", ext);
                return;
            }
        };
        if let Err(e) = result {
            eprintln!("Failed to write image: {}", e);
        }
    }
}

// pub fn save_page_image(app: &CBZViewerApp, page_number: usize) {
//     if let Ok(mut image_lru) = app.image_lru.lock() {
//         if let Some(page) = image_lru.get(&page_number) {
//             if let Some(image) = &page.image {
//                 if let Some(path) = rfd::FileDialog::new()
//                     .add_filter("PNG image", &["png"])
//                     .add_filter("JPEG image", &["jpg", "jpeg"])
//                     .set_file_name(&format!("page_{}.png", page_number))
//                     .save_file()
//                 {
//                     save_image_to_file(image, &path);
//                 }
//             } else {
//                 eprintln!("Page {} has no image data loaded.", page_number);
//             }
//         } else {
//             eprintln!("Page {} not found in cache.", page_number);
//         }
//     }
// }

pub fn save_page_image(app: &CBZViewerApp, page_number: usize) {
    let Ok(mut image_lru) = app.image_lru.lock() else {
        eprintln!("Failed to lock image LRU cache.");
        return;
    };

    let Some(page) = image_lru.get(&page_number) else {
        eprintln!("Page {} not found in cache.", page_number);
        return;
    };

    match &page.image {
        PageImage::Static(image) => {
            let Some(path) = rfd::FileDialog::new()
                .add_filter("PNG image", &["png"])
                .add_filter("JPEG image", &["jpg", "jpeg"])
                .set_file_name(&format!("page_{}.png", page_number))
                .save_file()
            else {
                eprintln!("No file selected.");
                return;
            };

            save_image_to_file(image, &path);
        }
        PageImage::AnimatedGif { .. } => {
            eprintln!("Saving animated GIF frames not implemented.");
            // You can optionally save the first frame or warn user.
        }
    }
}