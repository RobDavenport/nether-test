use glam::Vec3A;

pub fn generate_torus(
    major_radius: f32,
    minor_radius: f32,
    major_segments: usize,
    minor_segments: usize,
) -> Vec<f32> {
    let mut vertices = Vec::new();
    
    // Generate vertex grid
    for i in 0..=major_segments {
        let phi = 2.0 * std::f32::consts::PI * (i as f32) / (major_segments as f32);
        let major_pos = Vec3A::new(phi.cos(), phi.sin(), 0.0);
        
        for j in 0..=minor_segments {
            let theta = 2.0 * std::f32::consts::PI * (j as f32) / (minor_segments as f32);
            
            // Position calculation
            let minor_pos = Vec3A::new(
                theta.cos(),
                theta.sin(),
                0.0,
            );
            let position = major_pos * (major_radius + minor_radius * minor_pos.x)
                + Vec3A::Z * minor_radius * minor_pos.y;

            // Normal calculation (points outward from torus surface)
            let normal = major_pos * minor_pos.x
                + Vec3A::Z * minor_pos.y;

            vertices.push((position, normal.normalize()));
        }
    }

    // Generate triangles and build output buffer
    let mut output = Vec::with_capacity(major_segments * minor_segments * 6 * 6);
    
    for i in 0..major_segments {
        for j in 0..minor_segments {
            // Get indices for quad vertices
            let i_next = (i + 1) % (major_segments + 1);
            let j_next = (j + 1) % (minor_segments + 1);

            // Get all four vertices of the quad
            let v0 = vertices[i * (minor_segments + 1) + j];
            let v1 = vertices[i_next * (minor_segments + 1) + j];
            let v2 = vertices[i * (minor_segments + 1) + j_next];
            let v3 = vertices[i_next * (minor_segments + 1) + j_next];

            // Add two triangles per quad
            for &(pos, normal) in &[
                // First triangle
                v0, v1, v2,
                // Second triangle
                v1, v3, v2,
            ] {
                output.push(pos.x);
                output.push(pos.y);
                output.push(pos.z);
                output.push(normal.x);
                output.push(normal.y);
                output.push(normal.z);
            }
        }
    }

    output
}