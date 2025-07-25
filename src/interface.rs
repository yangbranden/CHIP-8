use minifb::{Key, Scale, Window, WindowOptions};
use rodio::{OutputStream, source::{SineWave, Source}};
use std::time::Duration;

pub struct Interface {
    pub window: Window,
    pub framebuffer: [u32; 64 * 32], // Chip-8 resolution is 64x32
    pub keypad: [bool; 16],
    pub muted: bool,
    pub debug_mode: bool, // Debug mode to print additional information
    sound_stream: Option<(OutputStream, rodio::OutputStreamHandle)>,
    is_beeping: bool,
}

impl Interface {
    pub fn new(scale: Scale) -> Self {
        // Create a new window with the specified options
        let window: Window = Window::new(
            "Chip-8 Emulator",
            64,
            32,
            WindowOptions {
                scale: scale,
                ..WindowOptions::default()
            },
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        // Initialize audio stream
        let sound_stream = OutputStream::try_default().ok();

        Interface {
            window,
            framebuffer: [0; 64 * 32], // Initialize screen with all pixels off (0 = black)
            keypad: [false; 16],   // Initialize keypad with all keys unpressed
            muted: false,
            debug_mode: false,
            sound_stream,
            is_beeping: false,
        }
    }

    // Render screen by updating the window with the current framebuffer
    pub fn render_screen(&mut self) {
        let width = 64; // Original screen width
        let height = 32; // Original screen height

        // Update the window with the screen buffer (no scaling)
        self.window
            .update_with_buffer(&self.framebuffer, width, height)
            .unwrap();
    }

    // Process key press events;
    // Mapping: https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
    // Keypad       Keyboard
    // +-+-+-+-+    +-+-+-+-+
    // |1|2|3|C|    |1|2|3|4|
    // +-+-+-+-+    +-+-+-+-+
    // |4|5|6|D|    |Q|W|E|R|
    // +-+-+-+-+ => +-+-+-+-+
    // |7|8|9|E|    |A|S|D|F|
    // +-+-+-+-+    +-+-+-+-+
    // |A|0|B|F|    |Z|X|C|V|
    // +-+-+-+-+    +-+-+-+-+
    pub fn process_keys(&mut self) {
        // Clear the current state of the keypad
        self.keypad = [false; 16];

        let keys = self.window.get_keys();

        // Update the keypad based on the pressed keys
        for key in keys {
            match key {
                Key::Key1 => self.keypad[0x1] = true,
                Key::Key2 => self.keypad[0x2] = true,
                Key::Key3 => self.keypad[0x3] = true,
                Key::Key4 => self.keypad[0xC] = true,
                Key::Q => self.keypad[0x4] = true,
                Key::W => self.keypad[0x5] = true,
                Key::E => self.keypad[0x6] = true,
                Key::R => self.keypad[0xD] = true,
                Key::A => self.keypad[0x7] = true,
                Key::S => self.keypad[0x8] = true,
                Key::D => self.keypad[0x9] = true,
                Key::F => self.keypad[0xE] = true,
                Key::Z => self.keypad[0xA] = true,
                Key::X => self.keypad[0x0] = true,
                Key::C => self.keypad[0xB] = true,
                Key::V => self.keypad[0xF] = true,
                _ => (),
            }
        }
    }


    pub fn set_beep(&mut self, should_beep: bool) {
        if self.muted {
            return; // If muted, do not play sound
        }

        if should_beep == self.is_beeping {
            return; // No change needed
        }

        if let Some((_, stream_handle)) = &self.sound_stream {
            if should_beep {
                // Create a sine wave at 440Hz (standard A note)
                let source = SineWave::new(440.0)
                    .take_duration(Duration::from_secs(1))
                    .amplify(0.10); // Reduce volume to 10%
                
                // Play the sound
                let _ = stream_handle.play_raw(source.convert_samples());
            }
        }

        self.is_beeping = should_beep;
    }
}
