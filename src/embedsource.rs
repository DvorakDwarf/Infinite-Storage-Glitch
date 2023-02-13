use opencv::core::prelude::*;
use opencv::core::{Mat, CV_8UC3, Size};

//Putting width and height here potentially overkill, mostly here for convenience
//I WANT THAT MAX PERFORMANCE
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

        // dbg!(actual_size);
        // dbg!(frame_size);

        // if actual_height != height {
        //     panic!("Height bug");
        // }

        //WHy does this have to be unsafe smh
        unsafe {
            let image = Mat::new_rows_cols(frame_size.height, frame_size.width, CV_8UC3).unwrap();

            EmbedSource {
                image,
                size, 
                frame_size,
                actual_size
            }
        }
    }

    pub fn from(image: Mat, size: i32) -> EmbedSource {
        let width = image.cols();
        let height = image.rows();
        let frame_size = Size::new(width, height);

        //Cuts off borders if size doesn't perfectly fit
        let width = width - (width % size);
        let height = height - (height % size);
        let actual_size = Size::new(width, height);

        EmbedSource {
            image,
            size, 
            frame_size,
            actual_size
        }
    }
}