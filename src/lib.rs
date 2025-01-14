mod api;
use std::f32::consts::{FRAC_PI_2, PI};

use api::*;

mod camera;
use camera::Camera;

use glam::{Mat4, Vec3};

// X Y Z, R G B
static DATA: &[f32] = &[
    0.0, 1.0, 0.0, 1.0, 0.0, 0.0, // Top, Red
    -1.0, -1.0, 0.0, 0.0, 1.0, 0.0, // Bottom Left, Green
    1.0, -1.0, 0.0, 0.0, 0.0, 1.0, // Bottom Right, Blue
];

struct State {
    camera: Camera,
    proj: Mat4,
    mesh_id: i32,
    t: f32,
}

const CAM_SPEED: f32 = 0.05;
const ROT_SPEED: f32 = 0.01;

static mut STATE: State = State {
    camera: Camera::new(Vec3::new(0.0, 1.0, 5.0), 0.0, 0.0),
    proj: Mat4::ZERO,
    mesh_id: 0,
    t: 0.0,
};

static IDENT: Mat4 = Mat4::IDENTITY;

#[no_mangle]
pub unsafe extern "C" fn init() {
    STATE.proj = Mat4::perspective_infinite_reverse_rh(45.0_f32.to_radians(), 16.0 / 9.0, 0.1);

    STATE.mesh_id = load_static_mesh(DATA.as_ptr() as *const u8, DATA.len() as i32, 0);
}

#[no_mangle]
pub unsafe extern "C" fn update() {
    STATE.t += 0.01;

    STATE.camera.pitch += analog_right_y(0) * ROT_SPEED;
    STATE.camera.yaw += analog_right_x(0) * ROT_SPEED;
    STATE.camera.pitch = STATE.camera.pitch.clamp(-FRAC_PI_2, FRAC_PI_2);

    let forward = STATE.camera.get_forward();
    let right = Vec3::new(-forward.z, 0.0, forward.x).normalize(); // Correctly perpendicular to forward in the XZ plane

    STATE.camera.position += forward * analog_left_y(0) * CAM_SPEED;
    STATE.camera.position += right * analog_left_x(0) * CAM_SPEED;
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
    let model = Mat4::from_rotation_z(STATE.t);
    let view = STATE.camera.get_view();

    push_view_matrix_pos(
        &raw const view as *const u8,
        &raw const STATE.camera.position as *const u8,
    );
    push_proj_matrix(&raw const STATE.proj as *const u8);

    push_model_matrix(&raw const IDENT as *const u8);

    draw_static_mesh(STATE.mesh_id);

    push_model_matrix(&raw const model as *const u8);
    draw_static_mesh(STATE.mesh_id);
}
