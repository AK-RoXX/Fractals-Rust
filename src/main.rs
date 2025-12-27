use pixels::{Pixels, SurfaceTexture};
use rayon::prelude::*;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent, MouseButton, VirtualKeyCode, ElementState};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use rand::Rng;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct FractalState {
    seed_x: f64,
    seed_y: f64,
    x_min: f64, x_max: f64,
    y_min: f64, y_max: f64,
    max_iter: u32,
    color_speed: f32,
}

struct InputState {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Fractal Studio: Smooth Glide Mode")
        .with_inner_size(LogicalSize::new(WIDTH as f64, HEIGHT as f64))
        .build(&event_loop)
        .unwrap();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };

    let mut state = FractalState {
        seed_x: -0.7, seed_y: 0.27,
        x_min: -1.5, x_max: 1.5,
        y_min: -1.0, y_max: 1.0,
        max_iter: 150,
        color_speed: 0.1,
    };

    let mut input = InputState { left: false, right: false, up: false, down: false };
    let mut mouse_pos = (0.0, 0.0);
    let mut rng = rand::thread_rng();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::CursorMoved { position, .. } => mouse_pos = (position.x, position.y),
                
                // Track key states for smooth gliding
                WindowEvent::KeyboardInput { input: k_input, .. } => {
                    let is_pressed = k_input.state == ElementState::Pressed;
                    if let Some(key) = k_input.virtual_keycode {
                        match key {
                            VirtualKeyCode::Left  => input.left = is_pressed,
                            VirtualKeyCode::Right => input.right = is_pressed,
                            VirtualKeyCode::Up    => input.up = is_pressed,
                            VirtualKeyCode::Down  => input.down = is_pressed,
                            VirtualKeyCode::Space => if is_pressed {
                                state.seed_x = rng.gen_range(-1.0..1.0);
                                state.seed_y = rng.gen_range(-1.0..1.0);
                            },
                            VirtualKeyCode::W => if is_pressed { state.max_iter += 20 },
                            VirtualKeyCode::S => if is_pressed && state.max_iter > 20 { state.max_iter -= 20 },
                            _ => (),
                        }
                    }
                }

                WindowEvent::MouseInput { state: m_state, button, .. } => {
                    if m_state == ElementState::Pressed {
                        match button {
                            MouseButton::Left => {
                                let w = state.x_max - state.x_min;
                                let h = state.y_max - state.y_min;
                                let cx = state.x_min + (mouse_pos.0 / WIDTH as f64) * w;
                                let cy = state.y_min + (mouse_pos.1 / HEIGHT as f64) * h;
                                let zoom = 0.5;
                                state.x_min = cx - (mouse_pos.0 / WIDTH as f64) * w * zoom;
                                state.x_max = state.x_min + w * zoom;
                                state.y_min = cy - (mouse_pos.1 / HEIGHT as f64) * h * zoom;
                                state.y_max = state.y_min + h * zoom;
                            }
                            MouseButton::Right => {
                                state.x_min = -1.5; state.x_max = 1.5;
                                state.y_min = -1.0; state.y_max = 1.0;
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            },

            Event::MainEventsCleared => {
                // Apply Velocity
                let speed = 0.003;
                if input.left  { state.seed_x -= speed; }
                if input.right { state.seed_x += speed; }
                if input.up    { state.seed_y += speed; }
                if input.down  { state.seed_y -= speed; }

                render_fractal(pixels.frame_mut(), &state);
                window.request_redraw();
            }

            Event::RedrawRequested(_) => {
                pixels.render().unwrap();
            }
            _ => (),
        }
    });
}

fn render_fractal(frame: &mut [u8], state: &FractalState) {
    frame.par_chunks_exact_mut(WIDTH as usize * 4).enumerate().for_each(|(y, row)| {
        for x in 0..WIDTH as usize {
            let mut zx = state.x_min + (x as f64 / WIDTH as f64) * (state.x_max - state.x_min);
            let mut zy = state.y_min + (y as f64 / HEIGHT as f64) * (state.y_max - state.y_min);
            let mut i = 0;

            while zx * zx + zy * zy <= 100.0 && i < state.max_iter {
                let tmp = zx * zx - zy * zy + state.seed_x;
                zy = 2.0 * zx * zy + state.seed_y;
                zx = tmp;
                i += 1;
            }

            let (r, g, b) = if i == state.max_iter {
                (0, 0, 0)
            } else {
                let log_zn = (zx * zx + zy * zy).ln() / 2.0;
                let nu = (log_zn / 2.0f64.ln()).ln() / 2.0f64.ln();
                let smooth_i = i as f64 + 1.0 - nu;

                let r = ((smooth_i * state.color_speed as f64 + 0.0).sin() * 127.0 + 128.0) as u8;
                let g = ((smooth_i * state.color_speed as f64 + 2.0).sin() * 127.0 + 128.0) as u8;
                let b = ((smooth_i * state.color_speed as f64 + 4.0).sin() * 127.0 + 128.0) as u8;
                (r, g, b)
            };

            let pixel_index = x * 4;
            row[pixel_index] = r;
            row[pixel_index + 1] = g;
            row[pixel_index + 2] = b;
            row[pixel_index + 3] = 255;
        }
    });
}