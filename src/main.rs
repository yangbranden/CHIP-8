extern crate minifb;
extern crate rand;
mod interface;
use interface::Interface;
mod chip8;
use chip8::Chip8;
use minifb::{Key, Scale};
use std::path::Path;

fn main() {
    // Define scaling size for screen (original is 64x32, so scale_factor = 10 will make it 640x320)
    let scale_factor = Scale::X16;

    // Create an Interface instance with specified scaling
    let interface = Interface::new(scale_factor);

    // Create a Chip8 instance with our Graphics instance
    let mut chip8 = Chip8::new(interface);

    // Define the path to the ROM file and load it into memory
    let rom_path = Path::new("pong.ch8");
    chip8.load_program(rom_path);

    // Main loop; exit if window is closed or Escape is pressed
    while chip8.interface.window.is_open() && !chip8.interface.window.is_key_down(Key::Escape) {
        let keys = chip8.interface.window.get_keys();

        // Process keys
        chip8.interface.process_keys(keys);

        // Emulate one cycle
        chip8.emulate_cycle();
    }
}
