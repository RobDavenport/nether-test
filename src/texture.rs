use std::sync::OnceLock;

use glam::Vec3A;

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

pub fn generate_matcap_bytes(size: u32) -> Vec<u8> {
    let mut pixels = Vec::with_capacity((size * size * 4) as usize);
    let center = (size as f32 / 2.0, size as f32 / 2.0);
    let radius = size as f32 / 2.0;

    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center.0;
            let dy = y as f32 - center.1;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance > radius {
                pixels.extend(&[0, 0, 0, 0]);
                continue;
            }

            // Normalized coordinates (-1 to 1)
            let nx = dx / radius;
            let ny = -dy / radius;
            let nz = (1.0 - nx * nx - ny * ny).sqrt().max(0.0);
            let normal = Vec3A::new(nx, ny, nz);

            // Base material properties
            let base_color = Vec3A::new(0.2, 0.3, 0.4);
            let light_color = Vec3A::new(1.0, 1.0, 1.0);
            let light_pos = Vec3A::new(0.5, -0.3, 0.8);

            // Lighting calculations
            let light_dir = (light_pos - normal).normalize();
            let view_dir = Vec3A::new(0.0, 0.0, 1.0);
            
            // Diffuse component with light color
            let diffuse = normal.dot(light_dir)
                .max(0.0)
                .powf(0.5);
            let diffuse_contrib = light_color * diffuse * 0.4;

            // Specular component with light color
            let reflect_dir = light_dir.reflect(normal);
            let specular = reflect_dir.dot(view_dir)
                .max(0.0)
                .powf(32.0)
                * 2.0;
            let specular_contrib = light_color * specular;

            // Combine all components
            let final_color = (base_color * 0.8 + diffuse_contrib + specular_contrib)
                .clamp(Vec3A::ZERO, Vec3A::ONE);

            pixels.push((final_color.x * 255.0) as u8);
            pixels.push((final_color.y * 255.0) as u8);
            pixels.push((final_color.z * 255.0) as u8);
            pixels.push(255);
        }
    }

    pixels
}