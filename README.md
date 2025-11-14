# CHIP-8 Interpreter
## This is a relatively simple CHIP-8 interpreter written entirely in Rust using Bevy.
## Screenshot here
## About
This project is one that has been a labor of love and attention (or lack-there-of) for the past couple years. I set out to teach myself Rust and gain more knowledge on lower level programming by making a CHIP-8 interpreter. I got stuck at one point when I ran into an issue with the way the opcodes executed and set it aside for the time being. The project stuck in the back of my head and I didn't want to just let it lie there. I ended up picking it back up and fixed all the errors with the opcodes and now it all works. It still runs rather slow, but that is something that I plan on tackling in the near future.
## What is a CHIP-8 interpreter?
CHIP-8 is essentially a programming language that allowed programmers to write code for this format that could then be interpreted and ran on other machines. This allowed for programs to be made that could be ran on a variety of computers before things were standardized.
In order to learn more I would highly recommend checking out the [Wikipedia page](https://en.wikipedia.org/wiki/CHIP-8). As well as there is an excellent [article](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/) written by Tobias Langhoff that breaks down how to write an interpreter and explains a lot of the reasons why certain things are done and other specifications.
## Basic Function
blah blah blah
## Next Goal
My next major goal for this project is to overhaul the way that the interpreter screen is rendered. Each cell/pixel is created as its own sprite which is horribly inefficient. My goal originally was to just get something showing on screen and I wasn't too worried about the speed at which it did so. However, obviously now that everything is working it's about time to refactor the code in order to bring the rendering into our 21st century. Unfortunately since I waited so long to come back to this a lot has changed with Bevy and I need to update my code before I can even worry about optimizing.
