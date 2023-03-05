use std::{fs, thread, vec};

use anyhow::{anyhow, Error}; //anyhow::Error::msg("My err");

use opencv::core::Mat;
use opencv::prelude::*;
use opencv::videoio::{VideoCapture, VideoWriter, CAP_ANY};

use crate::embedsource::EmbedSource;
use crate::settings::{Data, OutputMode, Settings};
use crate::timer::Timer;

//Get and write bytes from and to files. Start and end of app
//sounds cooler than og name (encode)
pub fn rip_bytes(path: &str) -> anyhow::Result<Vec<u8>> {
    let byte_data = fs::read(path)?;

    if byte_data.is_empty() {
        return Err(anyhow!(
            "Empty files cannot be embedded! File names are not retained, so it's pointless anyway"
        ));
    }
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
    println!("Binary ripped successfully");
    // println!("Binary length: {}", binary_data.len());
    return Ok(binary_data);
}

pub fn rip_binary_u32(bytes: Vec<u32>) -> anyhow::Result<Vec<bool>> {
    let mut binary_data: Vec<bool> = Vec::new();

    for byte in bytes {
        let mut bits = format!("{:b}", byte);
        let missing_0 = 32 - bits.len();

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

    return Ok(binary_data);
}

fn translate_u8(binary_data: Vec<bool>) -> anyhow::Result<Vec<u8>> {
    let mut buffer: Vec<bool> = Vec::new();
    let mut byte_data: Vec<u8> = Vec::new();

    for bit in binary_data {
        buffer.push(bit);

        if buffer.len() == 8 {
            //idk how this works but it does
            let byte = buffer.iter().fold(0u8, |v, b| (v << 1) + (*b as u8));

            byte_data.push(byte);
            buffer.clear();
        }
    }

    return Ok(byte_data);
}

fn translate_u32(binary_data: Vec<bool>) -> anyhow::Result<Vec<u32>> {
    let mut buffer: Vec<bool> = Vec::new();
    let mut byte_data: Vec<u32> = Vec::new();

    for bit in binary_data {
        buffer.push(bit);

        if buffer.len() == 32 {
            //idk how this works but it does
            let u32_byte = buffer.iter().fold(0u32, |v, b| (v << 1) + (*b as u32));
            byte_data.push(u32_byte);
            buffer.clear();
        }
    }

    return Ok(byte_data);
}

pub fn write_bytes(path: &str, data: Vec<u8>) -> anyhow::Result<()> {
    fs::write(path, data)?;
    println!("File written successfully");
    return Ok(());
}

//Returns average value of the pixel given size and location
fn get_pixel(frame: &EmbedSource, x: i32, y: i32) -> Option<Vec<u8>> {
    let mut r_list: Vec<u8> = Vec::new();
    let mut g_list: Vec<u8> = Vec::new();
    let mut b_list: Vec<u8> = Vec::new();

    for i in 0..frame.size {
        for j in 0..frame.size {
            let bgr = frame
                .image
                .at_2d::<opencv::core::Vec3b>(y + i, x + j)
                .unwrap();
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
    let rgb_average = vec![r_average as u8, g_average as u8, b_average as u8];
    // dbg!(&rgb_average);

    return Some(rgb_average);
}

//Draws the pixels, exists so you can draw bigger blocks
fn etch_pixel(frame: &mut EmbedSource, rgb: Vec<u8>, x: i32, y: i32) -> anyhow::Result<()> {
    for i in 0..frame.size {
        for j in 0..frame.size {
            // dbg!(x, y);
            let bgr = frame.image.at_2d_mut::<opencv::core::Vec3b>(y + i, x + j)?;
            //Opencv devs are reptilians who believe in bgr
            bgr[2] = rgb[0];
            bgr[1] = rgb[1];
            bgr[0] = rgb[2];
        }
    }

    return Ok(());
}

fn etch_bw(
    source: &mut EmbedSource,
    data: &Vec<bool>,
    global_index: &mut usize,
) -> anyhow::Result<()> {
    let _timer = Timer::new("Etching frame");

    let width = source.actual_size.width;
    let height = source.actual_size.height;
    let size = source.size as usize;

    for y in (0..height).step_by(size) {
        for x in (0..width).step_by(size) {
            let local_index = global_index.clone();

            let brightness = if data[local_index] == true {
                255 // 1
            } else {
                0 // 0
            };
            let rgb = vec![brightness, brightness, brightness];

            //Actually embeds the data
            etch_pixel(source, rgb, x, y).unwrap();

            //Increment index so we move along the data
            *global_index += 1;
            if *global_index >= data.len() {
                return Err(Error::msg("Index beyond data"));
            }
        }
    }

    return Ok(());
}

fn etch_color(
    source: &mut EmbedSource,
    data: &Vec<u8>,
    global_index: &mut usize,
) -> anyhow::Result<()> {
    let _timer = Timer::new("Etching frame");

    let width = source.actual_size.width;
    let height = source.actual_size.height;
    let size = source.size as usize;

    for y in (0..height).step_by(size) {
        for x in (0..width).step_by(size) {
            let local_index = global_index.clone();

            let rgb = vec![
                data[local_index],     //Red
                data[local_index + 1], //Green
                data[local_index + 2], //Blue
            ];

            etch_pixel(source, rgb, x, y).unwrap();

            //Increment index so we move along the data
            *global_index += 3;
            if *global_index + 2 >= data.len() {
                return Err(Error::msg("Index beyond data"));
            }
        }
    }

    return Ok(());
}

fn read_bw(
    source: &EmbedSource,
    current_frame: i32,
    final_frame: i32,
    final_bit: i32,
) -> anyhow::Result<Vec<bool>> {
    // let _timer = Timer::new("Dislodging frame");

    let width = source.actual_size.width;
    let height = source.actual_size.height;
    let size = source.size as usize;

    let mut binary_data: Vec<bool> = Vec::new();
    for y in (0..height).step_by(size) {
        for x in (0..width).step_by(size) {
            let rgb = get_pixel(&source, x, y);
            if rgb == None {
                continue;
            } else {
                let rgb = rgb.unwrap();
                if rgb[0] >= 127 {
                    binary_data.push(true);
                } else {
                    binary_data.push(false);
                }
            }
        }
    }

    //Cut off nasty bits at the end
    if current_frame == final_frame {
        let slice = binary_data[0..final_bit as usize].to_vec();
        return Ok(slice);
    }

    // dbg!(binary_data.len());
    return Ok(binary_data);
}

fn read_color(
    source: &EmbedSource,
    current_frame: i32,
    final_frame: i32,
    final_byte: i32,
) -> anyhow::Result<Vec<u8>> {
    // let _timer = Timer::new("Dislodging frame");

    let width = source.actual_size.width;
    let height = source.actual_size.height;
    let size = source.size as usize;

    let mut byte_data: Vec<u8> = Vec::new();
    for y in (0..height).step_by(size) {
        for x in (0..width).step_by(size) {
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

    //Cut off nasty bits at the end
    if current_frame == final_frame {
        let slice = byte_data[0..final_byte as usize].to_vec();
        return Ok(slice);
    }

    return Ok(byte_data);
}

/*
Instructions:
Etched on first frame, always be wrtten in binary despite output mode
Output mode is the first byte
Size is constant 5
11111111 = Color (255), 00000000 = Binary(0),
Second byte will be the size of the pixels
FPS doesn't matter, but can add it anyways
Potentially add ending pointer so it doesn't make useless bytes
^^Currently implemented(?), unused
*/

fn etch_instructions(settings: &Settings, data: &Data) -> anyhow::Result<EmbedSource> {
    let instruction_size = 5;

    let mut u32_instructions: Vec<u32> = Vec::new();

    //calculating at what frame and pixel the file ends
    let frame_size = (settings.height * settings.width) as usize;

    //Adds the output mode to instructions
    //Instead of putting entire size of file, add at which frame and pixel file ends
    //Saves space on instruction frame
    match data.out_mode {
        OutputMode::Color => {
            u32_instructions.push(u32::MAX);

            let frame_data_size = frame_size / settings.size.pow(2) as usize;
            let final_byte = data.bytes.len() % frame_data_size;
            let mut final_frame = data.bytes.len() / frame_data_size;

            //In case of edge case where frame is right on the money
            if data.bytes.len() % frame_size != 0 {
                final_frame += 1;
            }

            dbg!(final_frame);
            u32_instructions.push(final_frame as u32);
            u32_instructions.push(final_byte as u32);
        }
        OutputMode::Binary => {
            u32_instructions.push(u32::MIN);

            let frame_data_size = frame_size / settings.size.pow(2) as usize;
            let final_byte = data.binary.len() % frame_data_size;
            let mut final_frame = data.binary.len() / frame_data_size;

            //In case of edge case where frame is right on the money
            if data.binary.len() % frame_size != 0 {
                final_frame += 1;
            }

            dbg!(final_frame);
            u32_instructions.push(final_frame as u32);
            u32_instructions.push(final_byte as u32);
        }
    };

    u32_instructions.push(settings.size as u32);
    u32_instructions.push(u32::MAX); //For some reason size not readable without this

    let instruction_data = rip_binary_u32(u32_instructions)?;

    let mut source = EmbedSource::new(instruction_size, settings.width, settings.height);
    let mut index = 0;
    match etch_bw(&mut source, &instruction_data, &mut index) {
        Ok(_) => {}
        Err(_) => {
            println!("Instructions written")
        }
    }

    // highgui::named_window("window", WINDOW_FULLSCREEN)?;
    // highgui::imshow("window", &source.image)?;
    // highgui::wait_key(10000000)?;

    // imwrite("src/out/test1.png", &source.image, &Vector::new())?;

    return Ok(source);
}

fn read_instructions(
    source: &EmbedSource,
    threads: usize,
) -> anyhow::Result<(OutputMode, i32, i32, Settings)> {
    //UGLY
    let binary_data = read_bw(source, 0, 1, 0)?;
    let u32_data = translate_u32(binary_data)?;
    // dbg!(&u32_data);

    let out_mode = u32_data[0];

    let out_mode = match out_mode {
        u32::MAX => OutputMode::Color,
        _ => OutputMode::Binary,
    };

    let final_frame = u32_data[1] as i32;
    let final_byte = u32_data[2] as i32;
    let size = u32_data[3] as i32;

    let height = source.frame_size.height;
    let width = source.frame_size.width;

    let settings = Settings::new(size, threads, 1337, width, height);

    return Ok((out_mode, final_frame, final_byte, settings));
}

pub fn etch(path: &str, data: Data, settings: Settings) -> anyhow::Result<()> {
    let _timer = Timer::new("Etching video");

    let mut spool = Vec::new();
    match data.out_mode {
        OutputMode::Color => {
            let length = data.bytes.len();

            //UGLY
            //Required so that data is continuous between each thread
            let frame_size = (settings.width * settings.height) as usize;
            let frame_data_size = frame_size / settings.size.pow(2) as usize * 3;
            let frame_length = length / frame_data_size;
            let chunk_frame_size = (frame_length / settings.threads) + 1;
            let chunk_data_size = chunk_frame_size * frame_data_size;

            //UGLY DUPING
            let chunks = data.bytes.chunks(chunk_data_size);
            for chunk in chunks {
                //source of perf loss ?
                let chunk_copy = chunk.to_vec();

                let thread = thread::spawn(move || {
                    let mut frames = Vec::new();
                    let mut index: usize = 0;

                    loop {
                        let mut source =
                            EmbedSource::new(settings.size, settings.width, settings.height);
                        match etch_color(&mut source, &chunk_copy, &mut index) {
                            Ok(_) => frames.push(source),
                            Err(_v) => {
                                frames.push(source);
                                println!("Embedding thread complete!");
                                break;
                            }
                        }
                    }

                    return frames;
                });

                spool.push(thread);
            }
        }
        OutputMode::Binary => {
            let length = data.binary.len();
            //UGLY
            //Required so that data is continuous between each thread
            let frame_size = (settings.width * settings.height) as usize;
            let frame_data_size = frame_size / settings.size.pow(2) as usize;
            let frame_length = length / frame_data_size;
            let chunk_frame_size = (frame_length / settings.threads) + 1;
            let chunk_data_size = chunk_frame_size * frame_data_size;

            //UGLY DUPING
            let chunks = data.binary.chunks(chunk_data_size);
            for chunk in chunks {
                //source of perf loss ?
                let chunk_copy = chunk.to_vec();

                let thread = thread::spawn(move || {
                    let mut frames = Vec::new();
                    let mut index: usize = 0;

                    loop {
                        let mut source =
                            EmbedSource::new(settings.size, settings.width, settings.height);
                        match etch_bw(&mut source, &chunk_copy, &mut index) {
                            Ok(_) => frames.push(source),
                            Err(_v) => {
                                frames.push(source);
                                println!("Embedding thread complete!");
                                break;
                            }
                        }
                    }

                    return frames;
                });

                spool.push(thread);
            }
        }
    }

    let mut complete_frames = Vec::new();

    let instructional_frame = etch_instructions(&settings, &data)?;
    complete_frames.push(instructional_frame);

    for thread in spool {
        let frame_chunk = thread.join().unwrap();
        complete_frames.extend(frame_chunk);
    }

    //Mess around with lossless codecs, png seems fine
    //Fourcc is a code for video codecs, trying to use a lossless one
    let fourcc = VideoWriter::fourcc('p', 'n', 'g', ' ')?;
    // let fourcc = VideoWriter::fourcc('j', 'p', 'e', 'g')?;

    //Check if frame_size is flipped
    let frame_size = complete_frames[1].frame_size;
    let video = VideoWriter::new(path, fourcc, settings.fps, frame_size, true);

    //Use different codec if png failed
    let mut video = match video {
        Ok(v) => v,
        Err(_) => {
            let fourcc = VideoWriter::fourcc('a', 'v', 'c', '1')?;
            VideoWriter::new(path, fourcc, settings.fps, frame_size, true)
                .expect("Both png and avc1 codecs failed, please raise an issue on github")
        }
    };

    //Putting them in vector might be slower
    for frame in complete_frames {
        let image = frame.image;
        video.write(&image)?;
    }
    video.release()?;

    println!("Video embedded successfully at {}", path);

    return Ok(());
}

pub fn read(path: &str, threads: usize) -> anyhow::Result<Vec<u8>> {
    let _timer = Timer::new("Dislodging frame");
    let instruction_size = 5;

    let mut video = VideoCapture::from_file(&path, CAP_ANY).expect("Could not open video path");
    let mut frame = Mat::default();

    //Could probably avoid cloning
    video.read(&mut frame)?;
    let instruction_source =
        EmbedSource::from(frame.clone(), instruction_size, true).expect("Couldn't create instructions");
    let (out_mode, final_frame, final_byte, settings) =
        read_instructions(&instruction_source, threads)?;

    let mut byte_data = Vec::new();
    let mut current_frame = 1;
    loop {
        // let _timer = Timer::new("Reading frame  (clone included)");
        video.read(&mut frame)?;

        //If it reads an empty image, the video stopped
        if frame.cols() == 0 {
            break;
        }

        if current_frame % 20 == 0 {
            println!("On frame: {}", current_frame);
        }

        let source = EmbedSource::from(frame.clone(), settings.size, false).expect("Reading frame failed");

        let frame_data = match out_mode {
            OutputMode::Color => read_color(&source, current_frame, 99999999, final_byte).unwrap(),
            OutputMode::Binary => {
                let binary_data = read_bw(&source, current_frame, final_frame, final_byte).unwrap();
                translate_u8(binary_data).unwrap()
            }
        };

        current_frame += 1;

        byte_data.extend(frame_data);
    }

    println!("Video read successfully");
    return Ok(byte_data);
}

//Uses literally all the RAM
// pub fn read(path: &str, threads: usize) -> anyhow::Result<Vec<u8>> {
//     let _timer = Timer::new("Dislodging frame");
//     let instruction_size = 5;

//     let mut video = VideoCapture::from_file(&path, CAP_ANY)
//             .expect("Could not open video path");
//     let mut frame = Mat::default();

//     //Could probably avoid cloning
//     video.read(&mut frame)?;
//     let instruction_source = EmbedSource::from(frame.clone(), instruction_size);
//     let (out_mode, final_frame, final_byte, settings) = read_instructions(&instruction_source, threads)?;

//     let mut frames: Vec<Mat> = Vec::new();
//     loop {
//         // let _timer = Timer::new("Reading frame  (clone included)");
//         video.read(&mut frame)?;

//         //If it reads an empty image, the video stopped
//         if frame.cols() == 0 {
//             break;
//         }

//         frames.push(frame.clone());
//     }

//     //Required so that data is continuous between each thread
//     let chunk_size = (frames.len() / settings.threads) + 1;

//     let mut spool = Vec::new();
//     let chunks = frames.chunks(chunk_size);
//     //Can get rid of final_frame because of this
//     for chunk in chunks {
//         let chunk_copy = chunk.to_vec();
//         //Checks if this is final thread
//         let final_frame = if spool.len() == settings.threads - 1 {
//             chunk_copy.len() as i32
//         } else {
//             -1
//         };

//         let thread = thread::spawn(move || {
//             let mut byte_data = Vec::new();
//             let mut current_frame = 1;

//             for frame in chunk_copy {
//                 let source = EmbedSource::from(frame, settings.size);

//                 let frame_data = match out_mode {
//                     OutputMode::Color => {
//                         read_color(&source, current_frame, final_frame, final_byte).unwrap()
//                     },
//                     OutputMode::Binary => {
//                         let binary_data = read_bw(&source, current_frame, final_frame, final_byte).unwrap();
//                         translate_u8(binary_data).unwrap()
//                     }
//                 };
//                 current_frame += 1;

//                 byte_data.extend(frame_data);
//             }

//             println!("Dislodging thread complete!");
//             return byte_data;
//         });

//         spool.push(thread);
//     }

//     let mut complete_data = Vec::new();
//     for thread in spool {
//         let byte_chunk = thread.join().unwrap();
//         complete_data.extend(byte_chunk);
//     }

//     println!("Video read succesfully");
//     return Ok(complete_data);
// }
