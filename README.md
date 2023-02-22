#### I was working on this instead of my finals, hope you appreciate it.

I'll add all relevant executables when I can

# Infinite-Storage-Glitch

![ezgif com-gif-maker](https://user-images.githubusercontent.com/96934612/219563410-7728447d-5482-41ae-a3ff-cf8446e16ab7.gif)

AKA ISG (written entirely in Rust my beloved) lets you embed files into video and upload them to youtube as storage.

YouTube has no limit on amount of video that you can upload. This means that it is effectively infinite cloud storage if you were able to embed files into video with some kind of tool. ISG is the tool.

This has been quite heavily inspired by suckerpinch's [Harder Drive](https://www.youtube.com/watch?v=JcJSW7Rprio) video and [discord as a filesystem](https://github.com/pixelomer/discord-fs). Unfortunately no filesystem functionality as of right now.

If you are having trouble with the program please dm me.

# Now, you might be asking yourself:

<details>
<summary><b>But is this against TOS ?</b></summary>
<b>Answer is:</b> maybe?

I doubt there is any part of the TOS saying that you can't upload videos containing files, but I also did not want to shovel through all the legalese. I still don't condone using this tool for anything serious/large. YouTube might understandably get mad.
</details>

# Installation
**Recommended way (building from source):**\
=== *Please note: building from source takes **a lot of CPU and RAM** usage.* ===\
You need to have installed:
- [Rust](https://www.rust-lang.org/tools/install) 
- [opencv](https://github.com/twistedfall/opencv-rust)

If having any issues also try installing [ffmpeg](https://ffmpeg.org/).

If you want to or already have went through the hassle of installing Rust, you can ```git clone``` this repository, then ```cargo build --release```.
Cd to `/target/release` directory and run the program `./isg_4real`.

<b>The easier way:</b>
1. Download the executable from the releases 
2. Place the executable inside a folder
3. Open terminal inside the directory
4. `chmod +x isg_4real`
5. Run the executable: `./isg_4real`
6. Enjoy!

Note that the binary **is not an exe file**, it will not work on windows, try following the first method instead.\
I might add an .exe if this somehow gets popular

How to use
-------------
1. Archive to zip all the files you will be uploading
2. Run the executable
3. Use the embed option on the archive (**THE VIDEO WILL BE SEVERAL TIMES LARGER THAN THE FILE**, 4x in case of optimal compression resistance preset)
4. Upload the video to your YouTube channel. You probably want to keep it up as unlisted
5. Use the download option to get the video back
6. Use the dislodge option to get your files back from the downloaded video
7. PROFIT

![2023-02-16_22-12](https://user-images.githubusercontent.com/96934612/219563769-c05370e9-3f40-406a-85b8-eca14a118be8.png)


# Demo
**Flashing lights warning !!!1!1** - [YouTube Link](https://www.youtube.com/watch?v=8I4fd_Sap-g)

Try to use the program on this video and find the files hidden inside.

***No it's not just a rick roll.***

Explanation 4 nerds
-------------
The principle behind this is pretty simple. All files are made of bytes and bytes can be interpreted as numbers ranging from 0-255. This number can be represented with pixels using one of two modes: RGB or binary.

**RGB**:
The cooler mode. Every byte perfectly fits inside one of the colors of an rgb pixel. One rgb pixel can contain 3 bytes at a time. You just keep adding pixels like this until you run out of data. It is leagues more efficient and quick than binary.

**Binary**:
Born from YouTube compression being absolutely brutal. RGB mode is very sensitive to compression as a change in even one point of one of the colors of one of the pixels dooms the file to corruption. Black and white pixels are a lot harder to mess up. Every pixel is either bright representing a 1 or dark representing a 0. We string these bits together to get bytes and continue until we run out of data. 

Both of these modes can be corrupted by compression, so we need to increase the size of the pixels to make it less compressable. 2x2 blocks of pixels seem to be good enough in binary mode.

To make it easier on the user, we also include all the relevant settings used to create the video on the first frame of the video. This allows the program to know what mode the video is in and what size to use in order to avoid making the user remember.

# Final comments
I will come back to this project a bit later after I work on something else

I appreciate any and all roasting of the code so I can improve.

Do what you want with the code, but credit would be much appreciated and if you have any trouble with ISG, please contact me over discord.
