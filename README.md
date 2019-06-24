# Chip 8 Rust

This project is an implementation of the CHIP 8 interpreted language.

I followed [Cowgod's Chip 8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM) for a description of the language instructions and virtual machine.

This project is intended to be run stand alone and also to be imported into a JavaScript project using Web Assembly (Not yet implemented)

# Task List

| Status   | Task                                        | Notes                                                   |
|----------|---------------------------------------------|---------------------------------------------------------|
| Complete | Write Instruction Data Model                |                                                         |
| TODO     | Create VM/Interpreter                       |                                                         |
| TODO     | Hook up to Rust Graphics                    | Need to investigate Rust crates for Display/Input/Audio |
| TODO     | Create WASM Package                         |                                                         |
| TODO     | Hook up to React graphics with WASM package |                                                         |
