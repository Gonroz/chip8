# CHIP-8 Interpreter
## This is a relatively simple CHIP-8 interpreter written entirely in Rust using Bevy.
screenshot here

## About
This project is one that has been a labor of love and attention (or lack-there-of) for the past couple years. I set out to teach myself Rust and gain more knowledge on lower level programming by making a CHIP-8 interpreter. At one point I got stuck and decided to set it aside to focus on my college studies. I always wanted to return but life always finds a way to prevent that. In the summer of 2025 I came back and finally fixed the bugs that prevented proper function. Then in the following November I came back to update the code to modern practices and had to refactor a few things since Bevy had changed quite a bit. It is now an ongoing project while I take the chance to gain more experience developing a program.
## What is a CHIP-8 interpreter?
CHIP-8 is an interpreted language originally created to make development for computers easier (such as the COSMAC VIP). Every computer could have their own interpreter that would allow code compiled for CHIP-8 to be run on different systems. 
In order to learn more I would highly recommend checking out the [Wikipedia page](https://en.wikipedia.org/wiki/CHIP-8). As well as there is an excellent [article](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/) written by Tobias Langhoff that breaks down how to write an interpreter and explains a lot of the reasons why certain things are done and other specifications. [This](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM) technical reference by Cowgod was also incredibly important to me during development.
## How to run
### Prerequisite
Before you can do anything you need to make sure that you have Rust installed. You can find out how [here](https://rust-lang.org/tools/install/). After that it is important that you also follow the instructions on Bevy's website ([here](https://bevy.org/learn/quick-start/getting-started/setup/)) in order to make sure that you can get whatever dependencies you need in order to compile their code. You may require different dependencies on your system in order to build. Unfortunately, I don't know what you may or may not have, so you may need to download them as errors arise in *cargo build*.
### Compiling and running
First, it is **very important** that you perform *cargo run* in the main root of the folder. The program currently looks for all files through that main section using cargo to find the necessary roms and themes.
There is a *config.toml* file that has two sections: rom and theme. You can change either of those to load a different rom or theme, and when you hot reload it will update those accordingly. Obviously, it is important that the rom or theme actually exists because the interpreter will crash if not appropriate. Do **NOT** include the file endings because those are done automatically.
## Controls
The keyboard was originally on a 4x4 grid and obviously our modern keyboards are much larger. The keyboard is mapped as follows.

CHIP-8:

| C8: | C8: | C8: | C8: |
| :-: | :-: | :-: | :-: |
|  1  |  2  |  3  |  C  |
|  4  |  5  |  6  |  D  |
|  7  |  8  |  9  |  E  |
|  A  |  0  |  B  |  F  |

PC Keyboard:

| PC: | PC: | PC: | PC: |
| :-: | :-: | :-: | :-: |
|  1  |  2  |  3  |  4  |
|  Q  |  W  |  E  |  R  |
|  A  |  S  |  D  |  F  |
|  Z  |  X  |  C  |  V  |

Also, as stated prior at any point you can press *Ctrl+R* to reload/restart the CHIP-8 interpreter.
## Themes
Five themes are included currently and as stated earlier you can change the theme by editing the *config.toml* file with the chosen name.
### Adding your own theme.
Adding your own theme is incredibly easy. If you are creating one I would recommend copying one of the existing themes and change the name to something that you want. The theme name is whatever the name of the file is, and it is case specific. There are two fields in the toml file: foreground and background. Foreground is the color of when a pixel is at a value of 1 or on. Background is when a pixel is at 0 or off. Both are an array of 3 numbers and each number corresponds to RGB, in that order. Simply change each portion to get whatever color you want.
If you are adding a preexisting theme simply drop it in the themes folder and then change the *config.toml* file to match the theme name (the file name minus the .toml ending).
