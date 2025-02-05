mod api;
use std::{cell::RefCell, f32::consts::FRAC_PI_2};

use api::*;

mod camera;
use camera::Camera;

mod mesh;
mod texture;

use glam::{Mat4, Vec3};
use texture::{generate_matcap_bytes, generate_texture, TEXTURE_HEIGHT, TEXTURE_WIDTH};

// X Y Z, R G B
// const DATA: &[f32] = &[
//     0.0, 1.0, 0.0, 1.0, 0.0, 0.0, // Top, Red
//     -1.0, -1.0, 0.0, 0.0, 1.0, 0.0, // Bottom Left, Green
//     1.0, -1.0, 0.0, 0.0, 0.0, 1.0, // Bottom Right, Blue
// ];
// const PIPELINE: i32 = 0;

// X Y Z, U, V
const DATA: &[f32] = &[
    // Positions (X, Y, Z)   // UVs (U, V)
    // Triangle 1 (Top-Left Half)
    -2.0, 2.0, 0.0, 0.0, 0.0, // Top-left
    -2.0, -2.0, 0.0, 0.0, 1.0, // Bottom-left
    2.0, -2.0, 0.0, 1.0, 1.0, // Bottom-right
    // Triangle 2 (Top-Right Half)
    -2.0, 2.0, 0.0, 0.0, 0.0, // Top-left
    2.0, -2.0, 0.0, 1.0, 1.0, // Bottom-right
    2.0, 2.0, 0.0, 1.0, 0.0, // Top-right
];
const PIPELINE: i32 = 1;

//X Y Z, NX, NY, NZ
// const DATA: &[f32] = &[
//     // Positions (X, Y, Z)   // UVs (U, V)
//     // Triangle 1 (Top-Left Half)
//     -2.0, 2.0, 0.0, 0.0, 0.0, 1.0,// Top-left
//     -2.0, -2.0, 0.0, 0.0, 0.0, 1.0,// Bottom-left
//     2.0, -2.0, 0.0, 0.0, 0.0, 1.0,// Bottom-right
//     // Triangle 2 (Top-Right Half)
//     -2.0, 2.0, 0.0, 0.0, 0.0, 1.0,// Top-left
//     2.0, -2.0, 0.0, 0.0, 0.0, 1.0,// Bottom-right
//     2.0, 2.0, 0.0, 0.0, 0.0, 1.0, // Top-right
// ];
// const PIPELINE: i32 = 4;

struct State {
    camera: Camera,
    proj: Mat4,
    mesh_id: i32,
    t: f32,
    texture_id: i32,
    matcap_id: i32,
    torus_mesh: Vec<f32>,
    torus_id: i32,

    audio_data: Vec<f32>,
    audio_t: f32,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        camera: Camera::new(Vec3::new(0.0, 1.0, 5.0), 0.0, 0.0),
        proj: Mat4::perspective_infinite_reverse_rh(71.0_f32.to_radians(), 16.0 / 9.0, 0.1),
        mesh_id: 0,
        t: 0.0,
        texture_id: 0,
        matcap_id: 0,
        torus_mesh: mesh::generate_torus(1.5, 0.5, 32, 16),
        torus_id: 0,
        audio_data: Vec::new(),
        audio_t: 0.0,
    });
}

const CAM_SPEED: f32 = 0.05;
const ROT_SPEED: f32 = 0.01;

static IDENT: Mat4 = Mat4::IDENTITY;

#[no_mangle]
pub unsafe extern "C" fn init() {
    let texture = generate_texture();
    STATE.with_borrow_mut(|state| {
        state.mesh_id = load_static_mesh(DATA.as_ptr() as *const u8, DATA.len() as i32, PIPELINE);
        state.texture_id = load_texture(
            texture.as_ptr(),
            TEXTURE_WIDTH as i32,
            TEXTURE_HEIGHT as i32,
            1,
        );

        let matcap = generate_matcap_bytes(256);

        state.torus_id = load_static_mesh(
            state.torus_mesh.as_ptr() as *const u8,
            state.torus_mesh.len() as i32,
            4,
        );

        state.matcap_id = load_texture(matcap.as_ptr(), 256, 256, 1);
    })
}

#[no_mangle]
pub unsafe extern "C" fn update() {
    STATE.with_borrow_mut(|state| {
        state.t += 0.01;

        state.camera.pitch += analog_right_y(0) * ROT_SPEED;
        state.camera.yaw += analog_right_x(0) * ROT_SPEED;
        state.camera.pitch = state.camera.pitch.clamp(-FRAC_PI_2, FRAC_PI_2);

        let forward = state.camera.get_forward();
        let right = Vec3::new(-forward.z, 0.0, forward.x).normalize(); // Correctly perpendicular to forward in the XZ plane

        state.camera.position += forward * analog_left_y(0) * CAM_SPEED;
        state.camera.position += right * analog_left_x(0) * CAM_SPEED;

        let frequency = if button_a_held(0) == 1 {
            440.0
        } else if button_b_held(0) == 1 {
            493.88
        } else if button_c_held(0) == 1 {
            261.63
        } else if button_d_held(0) == 1 {
            293.66
        } else {
            return;
        };

        state.audio_data.clear();

        const SAMPLE_RATE: usize = 24_000;

        for _ in 0..(SAMPLE_RATE / 60) {
            // Sine Wave Generation Code
            state.audio_t += frequency / SAMPLE_RATE as f32;

            // Wrap
            state.audio_t = state.audio_t.fract();

            // Generate the FM-modulated sine wave sample
            let v = (state.audio_t * std::f32::consts::TAU).sin();
            state.audio_data.push(v);
        }

        push_audio(
            state.audio_data.as_ptr() as *const u8,
            state.audio_data.len() as i32,
            1,
            SAMPLE_RATE as i32,
        );
    })
}

#[no_mangle]
pub unsafe extern "C" fn render() {
    STATE.with_borrow(|state| {
        let model =
            Mat4::from_translation(Vec3::new(0.0, 0.0, -2.0)) * Mat4::from_rotation_z(state.t);
        let view = state.camera.get_view();

        set_texture(state.matcap_id, 0, 0);

        push_view_matrix_pos(
            &raw const view as *const u8,
            &raw const state.camera.position as *const u8,
        );
        push_proj_matrix(&raw const state.proj as *const u8);

        push_model_matrix(&raw const IDENT as *const u8);

        draw_static_mesh(state.torus_id);

        push_model_matrix(&raw const model as *const u8);
        draw_static_mesh(state.mesh_id);
    });
}
