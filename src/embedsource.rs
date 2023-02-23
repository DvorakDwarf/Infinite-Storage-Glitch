use opencv::core::prelude::*;
use opencv::core::{Mat, CV_8UC3, Size};

pub struct EmbedSource {
    pub image: Mat,
    pub size: i32,
    pub frame_size: Size,
    pub actual_size: Size,
}

impl EmbedSource {
    pub fn new(size: i32, width: i32, height: i32) -> EmbedSource {
        let frame_size = Size::new(width, height);
        let actual_width = width - (width % size);
        let actual_height = height - (height % size);
        let actual_size = Size::new(actual_width, actual_height);

        let image = Mat::new_rows_cols(frame_size.height, frame_size.width, CV_8UC3)
            .expect("Failed to create new Mat");

        EmbedSource {
            image,
            size,
            frame_size,
            actual_size,
        }
    }

    pub fn from(image: Mat, size: i32) -> Result<EmbedSource, String> {
        let width = image.cols();
        let height = image.rows();
        let frame_size = Size::new(width, height);

        if width % size != 0 || height % size != 0 {
            return Err("Image size is not a multiple of the embedding size".to_string());
        }

        let actual_size = Size::new(width - (width % size), height - (height % size));

        Ok(EmbedSource {
            image,
            size,
            frame_size,
            actual_size,
        })
    }
}
