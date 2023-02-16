I was working on this instead of my finals, hope you appreciate it

# Infinite-Storage-Glitch
AKA ISG (written entirely in Rust my beloved) lets you embed files into video and upload them to youtube as storage.

YouTube has no limit on amount of video that you can upload. This means that it is effectively infinite cloud storage if you were able to embed files into video with some kind of tool. ISG is the tool.

# Now, you might be asking yourself:

<details>
<summary><b>But is this legal ?</b></summary>
<b>Maybe ?</b>

I doubt there is any part of the TOS saying that you can't upload videos containing files, but I also did not want to shovel through all the legalese. I still don't condone using this tool for anything serious/large. YouTube might understandably get mad.
</details>

Installation
-------------
<details>
<summary><b>Recommended way</b></summary>
If want to or already have went through the hassle of installing Rust, you can ```git clone``` this repository, then ```cargo build --release``` and run the program. Probably better in case I forget to update the executable.
</details>

<details>
<summary><b>The easier way</b></summary>
1. Download the executable from the releases 
2. Place the executable inside a folder
3. Enjoy
</details>

How to use
-------------
1. Zip all the files you will be uploading
2. Run the executable
3. Use the embed option on the archive (**THE VIDEO WILL BE SEVERAL TIMES LARGER THAN THE FILE**, 4x in case of optimal compression resistance preset)
4. Upload the video to your YouTube channel. You probably want to keep it up as unlisted
5. Use the download option to get the video back
6. Use the dislodge option to get your files back from the downloaded video
7. PROFIT

Demo
-------------
**Flashing lights warning !!!1!1**


Explanation 4 nerds
-------------

# Final comments
I appreciate any and all roasting of the code so I can improve. 

Do what you want with the code, but credit would be much appreciated and if you have any trouble with ISG, please contact me over discord.
