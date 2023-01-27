use opencv::core::prelude::*;
use opencv::core::{Mat, CV_8UC3};

//Putting width and height here potentially overkill, mostly here for convenience
//I WANT THAT MAX PERFORMANCE
pub struct EmbedSource {
    pub image: Mat,
    pub size: i32,
    pub width: i32,
    pub height: i32,
}

impl EmbedSource {
    pub fn new(size: i32, width: i32, height: i32) -> EmbedSource {
        let width = width - (width % size);
        let height = height - (height % size);

        //WHy does this have to be unsafe smh
        unsafe {
            let image = Mat::new_rows_cols(height, width, CV_8UC3).unwrap();

            EmbedSource {
                image,
                size, 
                width,
                height
            }
        }
    }

    pub fn from(image: Mat, size: i32) -> EmbedSource {
        let width = image.cols();
        let height = image.rows();

        //Cuts off borders if size doesn't perfectly fit, also -1 cuz index
        let width = width - (width % size) - 1;
        let height = height - (height % size) - 1;

        EmbedSource {
            image,
            size, 
            width,
            height
        }
    }
}