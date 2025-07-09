// std
pub use std::{
    collections::HashSet,
    num::NonZeroUsize,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

// external crates
pub use eframe::{
    CreationContext,
    egui::{
        self, CentralPanel, Color32, Context, FontData, FontDefinitions, FontFamily, FontId, Image,
        Layout, Rect, RichText, Spinner, TextEdit, TextStyle, TextureHandle, Ui, Vec2, Window,
    },
};
pub use image::{AnimationDecoder, DynamicImage, GenericImageView, codecs::gif::GifDecoder};
pub use log::{debug, warn};
pub use lru::LruCache;

// crate modules
pub use crate::{
    app::CBZViewerApp,
    cache::{
        SharedImageCache,
        image_cache::{LoadedPage, PageImage},
        load_image_async, new_image_cache, composite_dual_page,
        texture_cache::TextureCache,
    },
    config::*,
    error::AppError,
    ui::{
        clamp_pan, handle_pan, handle_zoom,
        image::{draw_dual_page, draw_single_page, draw_spinner, get_or_generate_dual_texture},
        log::UiLogger,
        manifest_editor::ManifestEditor,
    },
};
pub use comic_archive::prelude::*;
