// std
pub use std::collections::HashSet;
pub use std::fs::{self, File};
pub use std::io::{Cursor, Read};
pub use std::num::NonZeroUsize;
pub use std::path::{Path, PathBuf};
pub use std::sync::{Arc, Mutex};
pub use std::time::{Duration, Instant};

// external crates
pub use eframe::{
    egui::{
        self, CentralPanel, Color32, Context, FontData, FontDefinitions, FontFamily, FontId, Image,
        Layout, /*Pos2,*/ Rect, RichText, Spinner, TextEdit, TextStyle, TextureHandle, Ui,
        Vec2, Window,
        /*ViewportCommand,*/
    },
    CreationContext,
};
pub use image::{codecs::gif::GifDecoder, AnimationDecoder, DynamicImage, GenericImageView};
pub use log::{debug, warn};
pub use lru::LruCache;
pub use zip::read::ZipArchive;

// crate modules
pub use crate::{
    app::CBZViewerApp,
    archive::{
        image_archive::manifest::{Manifest, ManifestEditor},
        ImageArchive,
    },
    cache::{
        image_cache::{LoadedPage, PageImage},
        load_image_async, new_image_cache,
        texture_cache::TextureCache,
        SharedImageCache,
    },
    config::*,
    error::AppError,
    ui::{
        clamp_pan, draw_bottom_bar, draw_central_image_area, draw_dual_page, draw_single_page,
        draw_spinner, draw_top_bar, handle_pan, handle_zoom, log::UiLogger,
    },
};
