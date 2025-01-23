use std::sync::OnceLock;

pub const TEXTURE_WIDTH: usize = 64;
pub const TEXTURE_HEIGHT: usize = 64;

const R_SCALE: f32 = 20.0;
const G_SCALE: f32 = 24.0;
const B_SCALE: f32 = 16.0;

pub fn generate_texture() -> &'static [u8] {
    const BYTES_PER_PIXEL: usize = 4; // RGBA
    const TEXTURE_SIZE: usize = TEXTURE_WIDTH * TEXTURE_HEIGHT * BYTES_PER_PIXEL;

    static TEXTURE: OnceLock<[u8; TEXTURE_SIZE]> = OnceLock::new();

    TEXTURE
        .get_or_init(|| {
            let mut data = [0u8; TEXTURE_SIZE];

            for y in 0..TEXTURE_HEIGHT {
                for x in 0..TEXTURE_WIDTH {
                    // Normalized coordinates
                    let nx = x as f32 / (TEXTURE_WIDTH - 1) as f32;
                    let ny = y as f32 / (TEXTURE_HEIGHT - 1) as f32;

                    // Create color patterns using trigonometric functions
                    let r = ((nx * R_SCALE).sin() * 127.0 + 128.0) as u8;
                    let g = ((ny * G_SCALE).cos() * 127.0 + 128.0) as u8;
                    let b = (((nx + ny) * B_SCALE).sin() * 127.0 + 128.0) as u8;

                    // Calculate the index in the byte array
                    let index = (y * TEXTURE_WIDTH + x) * BYTES_PER_PIXEL;
                    data[index] = r;
                    data[index + 1] = g;
                    data[index + 2] = b;
                    data[index + 3] = 255;
                }
            }
            data
        })
        .as_slice()
}
