use std::{fs, vec};

use anyhow;
use anyhow::Error; //anyhow::Error::msg("My err");
use opencv::prelude::*;
use opencv::highgui::{self, WINDOW_FULLSCREEN};
use opencv::core::{Mat, Vector, VecN, Size, CV_8UC3,};
use opencv::imgcodecs::{imread, imwrite, IMREAD_COLOR};
use opencv::videoio::{VideoWriter, VideoCapture, CAP_ANY};

use crate::settings::{Settings, OutputMode, Data};
use crate::embedsource::EmbedSource;

//Get and write bytes from and to files. Start and end of app
//sounds cooler than og name (encode)
pub fn rip_bytes(path: &str) -> anyhow::Result<Vec<u8>> {
    let byte_data = fs::read(path)?;

    println!("Bytes ripped succesfully");
    println!("Byte length: {}", byte_data.len());
    return Ok(byte_data);
}

pub fn rip_binary(byte_data: Vec<u8>) -> anyhow::Result<Vec<bool>> {
    let mut binary_data: Vec<bool> = Vec::new();

    for byte in byte_data {
        //Returns binary but doesn't include all 8 bits if a number fits into less than 8.
        let mut bits = format!("{:b}", byte);
        let missing_0 = 8 - bits.len();

        //Adding the missing 0's, could be faster
        for _ in 0..missing_0 {
            bits.insert(0, '0');
        }

        for bit in bits.chars() {
            if bit == '1' {
                binary_data.push(true);
            } else {
                binary_data.push(false);
            }
        }
    }
    println!("Binary ripped succesfully");
    println!("Binary length: {}", binary_data.len());
    return Ok(binary_data);
}

//Bit of a waste
pub fn rip_binary_u64(byte: u64) -> anyhow::Result<Vec<bool>> {
    let mut binary_data: Vec<bool> = Vec::new();

    let mut bits = format!("{:b}", byte);
    let missing_0 = 64 - bits.len();

    //Adding the missing 0's, could be faster
    for _ in 0..missing_0 {
        bits.insert(0, '0');
    }

    for bit in bits.chars() {
        if bit == '1' {
            binary_data.push(true);
        } else {
            binary_data.push(false);
        }
    }
    return Ok(binary_data);
}

fn write_bytes(path: &str, data: Vec<u8>) -> anyhow::Result<()> {
    fs::write(path, data)?;
    println!("File written succesfully");
    return Ok(());
}

//Returns average value of the pixel given size and location
fn get_pixel(frame: &EmbedSource, x: i32, y: i32) -> Option<Vec<u8>> {
    if frame.size % 2 != 1 {
        panic!("Used even size for pixels, please choose something odd");
    }

    let half_size = frame.size/2;
    
    let mut r_list: Vec<u8> = Vec::new();
    let mut g_list: Vec<u8> = Vec::new();
    let mut b_list: Vec<u8> = Vec::new();

    for i in -half_size..half_size+1 {
        for j in -half_size..half_size+1 {
            let bgr = frame.image.at_2d::<opencv::core::Vec3b>(y+i, x+j).unwrap();
            //could reduce size of integers ?
            r_list.push(bgr[2]);
            g_list.push(bgr[1]);
            b_list.push(bgr[0]);
        }
    }

    //A hacked on solution, do better
    let r_sum: usize = r_list.iter().map(|&x| x as usize).sum();
    let r_average = r_sum / r_list.len(); 
    let g_sum: usize = g_list.iter().map(|&x| x as usize).sum();
    let g_average = g_sum / g_list.len(); 
    let b_sum: usize = b_list.iter().map(|&x| x as usize).sum();
    let b_average = b_sum / b_list.len();
    
    //Potentially unnecessary conversion
    let rgb_average = vec![
        r_average as u8,
        g_average as u8,
        b_average as u8
    ];
    dbg!(&rgb_average);
    
    return Some(rgb_average);
}

//Draws the pixels, exists so you can draw bigger blocks
fn etch_pixel(frame: &mut EmbedSource, rgb: Vec<u8>, x: i32, y: i32) -> anyhow::Result<()> {
    let half_size = frame.size/2;

    for i in -half_size..half_size+1 {
        for j in -half_size..half_size+1 {
            // dbg!(x, y);
            let bgr = frame.image.at_2d_mut::<opencv::core::Vec3b>(y+i, x+j)?;
            //Opencv devs are reptilians who believe in bgr
            bgr[2] = rgb[0];
            bgr[1] = rgb[1];
            bgr[0] = rgb[2];
        }
    }

    return Ok(());
}

fn etch_frame(source: &mut EmbedSource, data: &Data, global_index: &mut usize) 
        -> anyhow::Result<()>{
    let half_size = source.size/2;
    let width = source.width;
    let height = source.height;
    let size = source.size as usize;

    for y in (half_size..height).step_by(size) {
        for x in (half_size..width).step_by(size) {
            // dbg!(&global_index);
            let local_index = global_index.clone();

            let rgb = match data.out_mode {
                OutputMode::Color => {
                    let colors = vec![
                        data.bytes[local_index],  //Red
                        data.bytes[local_index+1],//Green
                        data.bytes[local_index+2] //Blue
                    ];
                    //Increment index so we move along the data
                    *global_index += 3;

                    //Hopefully this doesn't affect og ?
                    if *global_index+2 >= data.bytes.len() - 1 {
                        return Err(Error::msg("Index beyond data"));
                    }

                    colors
                },
                OutputMode::Binary => {
                    let brightness = if data.binary[local_index] == true {
                        255 // 1
                    } else {
                        0   // 0
                    };
                    let colors = vec![
                        brightness,
                        brightness,
                        brightness,
                    ];

                    //Increment index so we move along the data
                    *global_index += 1;

                    //Hopefully this doesn't affect og ?
                    if *global_index >= data.binary.len() - 1 {
                        return Err(Error::msg("Index beyond data"));
                    }

                    colors
                }
            };
            etch_pixel(source, rgb, x, y).unwrap();
        }
    }
    return Ok(());
}

pub fn read_frame(source: &EmbedSource, out_mode: &OutputMode) {
    let size = source.size as usize;
    let half_size = (source.size/2) as i32;
    let width = source.width;
    let height = source.height;

    //Fix this nesting spiral
    match out_mode {
        OutputMode::Color => {
            let mut byte_data: Vec<u8> = Vec::new();
            for y in (half_size..height).step_by(size) {
                for x in (half_size..width).step_by(size) {
                    let rgb = get_pixel(&source, x, y);
                    if rgb == None {
                        continue;
                    } else {
                        let rgb = rgb.unwrap();
                        byte_data.push(rgb[0]);
                        byte_data.push(rgb[1]);
                        byte_data.push(rgb[2]);
                    }
                }
            }
        },
        OutputMode::Binary => {
            let mut binary_data: Vec<bool> = Vec::new();
            for y in (half_size..height).step_by(size) {
                for x in (half_size..width).step_by(size) {
                    let rgb = get_pixel(&source, x, y);
                    if rgb == None {
                        continue;
                    } else {
                        let rgb = rgb.unwrap();
                        if rgb[0] == 255 {
                            binary_data.push(true);
                        } else {
                            binary_data.push(false);
                        }
                    }
                }
            }
        }
    }
}
/*
Instructions:
Etched on first frame, always be wrtten in binary despite output mode
Output mode is the first byte
11111111 = Color (255), 00000000 = Binary(0),
Second byte will be the size of the pixels
FPS doesn't matter, but can add it anyways
Potentially add ending pointer so it doesn't make useless bytes
*/

fn etch_instructions(settings: &Settings, data: &Data) 
        -> anyhow::Result<EmbedSource> {
    let mut u8_instructions: Vec<u8> = Vec::new();
    
    //Both adds the output mode to instructions and finds last byte
    let last_byte_pointer = match data.out_mode {
        OutputMode::Color => {
            u8_instructions.push(255);
            rip_binary_u64(data.bytes.len() as u64)?
        },
        OutputMode::Binary => {
            u8_instructions.push(0);
            rip_binary_u64(data.binary.len() as u64)?
        },
    };

    //Could choke and die
    u8_instructions.push(settings.size as u8);
    u8_instructions.push(settings.fps as u8);
    let mut binary_instructions = rip_binary(u8_instructions)?;
    binary_instructions.extend(last_byte_pointer);
    let instruction_data = Data::from_binary(binary_instructions);

    let mut source = EmbedSource::new(5, settings.width, settings.height);
    let mut index = 0;
    match etch_frame(&mut source, &instruction_data, &mut index) {
        Ok(_) => {},
        Err(_) => {println!("End of data reached")}
    }

    highgui::named_window("window", WINDOW_FULLSCREEN)?;
    highgui::imshow("window", &source.image)?;
    highgui::wait_key(10000000)?;

    imwrite("src/out/test1.png", &source.image, &Vector::new())?;

    return Ok(source);
}

fn read_instructions() {

}

pub fn etch(path: &str, data: Data, settings: Settings) -> anyhow::Result<()> {
    let mut frames = Vec::new();
    let mut index: usize = 0;

    let instructional_frame = etch_instructions(&settings, &data)?;
    frames.push(instructional_frame);

    loop {
        // dbg!("Looped!");
        let mut source = EmbedSource::new(settings.size, settings.width, settings.height);
        match etch_frame(&mut source, &data, &mut index) {
            Ok(_) => frames.push(source),
            Err(v) => {
                dbg!(v);
                break;}, 
        }
    }

    //Mess around with lossless codecs, png seems fine
    //Fourcc is a code for video codecs, trying to use a lossless one
    let fourcc = VideoWriter::fourcc('p', 'n', 'g', ' ')?;
    let frame_size = Size::new(frames[0].width, frames[0].height);
    let mut video = VideoWriter::new(path, fourcc, settings.fps, frame_size, true)?;

    //Putting them in vector might be slower
    for frame in frames {
        let image = frame.image;
        video.write(&image)?;
    }
    video.release()?;

    println!("Video embedded succesfully at {}", path);

    return Ok(());
}