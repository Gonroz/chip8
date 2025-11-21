# CHIP-8 Interpreter
## This is a relatively simple CHIP-8 interpreter written entirely in Rust using Bevy.
## Screenshot here
## About
This project is one that has been a labor of love and attention (or lack-there-of) for the past couple years. I set out to teach myself Rust and gain more knowledge on lower level programming by making a CHIP-8 interpreter. At one point I got stuck and decided to set it aside to focus on my college studies. I always wanted to return but life always finds a way to prevent that. In the summer of 2025 I came back and finally fixed the bugs that prevented proper function. Then in the following November I came back to update the code to modern practices and had to refactor a few things since Bevy had changed quite a bit. It is now an ongoing project while I take the chance to gain more experience developing a program.
## What is a CHIP-8 interpreter?
CHIP-8 is essentially a programming language that allowed programmers to write code for this format that could then be interpreted and ran on other machines. This allowed for programs to be made that could be ran on a variety of computers before things were standardized.
In order to learn more I would highly recommend checking out the [Wikipedia page](https://en.wikipedia.org/wiki/CHIP-8). As well as there is an excellent [article](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/) written by Tobias Langhoff that breaks down how to write an interpreter and explains a lot of the reasons why certain things are done and other specifications.
## How to run
### Prerequisite
Before you can do anything you need to make sure that you have Rust installed. You can find out how [here](https://rust-lang.org/tools/install/). After that it is important that you also follow the instructions on Bevy's website ([here](https://bevy.org/learn/quick-start/getting-started/setup/)) in order to make sure that you can get whatever dependencies you need in order to compile their code.
### Compiling and running
In *rom_to_load.txt* you can type in the name of the ROM that you would like to run. Make sure it is the only line of the file and that the *.ch8* file exists inside of the **roms** folder. From there you can perform *cargo run* and the program will compile and boot up. At any point, either before running or during, you can change the name of the file in the txt and then press Ctrl+R in order to reload the ROM so you don't need to perform cargo run again.
## Controls
The keyboard was originally on a 4x4 grid and obviously our modern keyboards are much larger. The keyboard is mapped as follows.

| CHIP-8: |
| :---: | :---: | :---: | :---: |
| 1   | 2   | 3   | C   |
| 4   | 5   | 6   | D   |
| 7   | 8   | 9   | E   |
| A   | 0   | B   | F   |


| PC Keyboard |
| :---: | :---: | :---: | :---: |
| 1   | 2   | 3   | 4   |
| Q   | W   | E   | R   |
| A   | S   | D   | F   |
| Z   | X   | C   | V   |

Also, as stated prior at any point you can press *Ctrl+R* to reload/restart the CHIP-8 interpreter.
## Next Goal(s)
The next major goal is to remove the roms_to_load.txt and move it to a config file I'm planning to add. It would allow you to enter the name of the ROM and the name of a theme to change the colors of the display. There will most likely be a coinciding themes folder that will allow you to create your own themes and paste them into the folder easily so that you can share and change them.
