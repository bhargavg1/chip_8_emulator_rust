# CHIP-8 Emulator 
*Written in rust, with a few libc bindings*

This is a (yet another) simple chip 8 emulatior that I wrote in the Rust language, in order to get more familiar with the syntax of rust. It currently only works with pure chip 8 programs, and doesnt support the newer chip-48 standard. 
I did not want to use too many external dependancies in my program, so it uses the terminal to draw the display on a 128 x 32 character rectangle, and uses stdin for the keyboard and the terminal bell for sound effects.

## To run a .ch8 program:
after building the program with ```cargo build```, you can find the executable in ```target/debug```.
just run this executable and provide the .ch8 file location. Ex:
```
./target/debug/chip_8_emulator programs/IBM Logo.ch8
```
Or, you can just run it through cargo:
```
cargo run -- programs/IBM Logo.ch8
```
The 4 x 4 chip8 keypad maps to the left side of your keyboard as follows:
```
1 2 3 4 -> 1 2 3 C
q w e r -> 4 5 6 D
a s d f -> 7 8 9 E
z x c v -> A 0 B F
```
### Emulator screenshots
Here is the emulator running a simple program which just displays the ch8 logo
