use minifb::{Key, MouseButton, Window, WindowOptions};
use rayon::prelude::*;
use rand::Rng; 

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Fractal: Space=Random, RightClick=Reset, Arrows=Tweak",
        WIDTH, HEIGHT, WindowOptions::default()
    ).unwrap();

    // Initial Seed
    let mut seed_x = -0.7;
    let mut seed_y = 0.27;

    // View boundaries
    let mut x_min = -1.5; let mut x_max = 1.5;
    let mut y_min = -1.0; let mut y_max = 1.0;
    let max_iter = 150; 

    let mut rng = rand::thread_rng();
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        // 1. Randomize Feature (Spacebar)
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            seed_x = rng.gen_range(-1.0..1.0);
            seed_y = rng.gen_range(-1.0..1.0);
            println!("New Seed: {}, {}", seed_x, seed_y);
        }

        // 2. Reset Feature (Right Click)
        if window.get_mouse_down(MouseButton::Right) {
            x_min = -1.5; x_max = 1.5;
            y_min = -1.0; y_max = 1.0;
        }

        // 3. Arrow Key Tweaks
        if window.is_key_down(Key::Up)    { seed_y += 0.002; }
        if window.is_key_down(Key::Down)  { seed_y -= 0.002; }
        if window.is_key_down(Key::Left)  { seed_x -= 0.002; }
        if window.is_key_down(Key::Right) { seed_x += 0.002; }
        
        // 4. Zoom Logic (Left Click)
        if let Some((m_x, m_y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
            if window.get_mouse_down(MouseButton::Left) {
                let zoom = 0.97; // Slightly slower zoom for better control
                let w = x_max - x_min; let h = y_max - y_min;
                let cx = x_min + (m_x as f64 / WIDTH as f64) * w;
                let cy = y_min + (m_y as f64 / HEIGHT as f64) * h;
                x_min = cx - (m_x as f64 / WIDTH as f64) * w * zoom;
                x_max = x_min + w * zoom;
                y_min = cy - (m_y as f64 / HEIGHT as f64) * h * zoom;
                y_max = y_min + h * zoom;
            }
        }

        // 5. Render Engine
        buffer.par_chunks_mut(WIDTH).enumerate().for_each(|(y, row)| {
            for x in 0..WIDTH {
                let mut zx = x_min + (x as f64 / WIDTH as f64) * (x_max - x_min);
                let mut zy = y_min + (y as f64 / HEIGHT as f64) * (y_max - y_min);
                let mut i = 0;

                while zx * zx + zy * zy <= 4.0 && i < max_iter {
                    let tmp = zx * zx - zy * zy + seed_x;
                    zy = 2.0 * zx * zy + seed_y;
                    zx = tmp;
                    i += 1;
                }

                if i == max_iter {
                    row[x] = 0; 
                } else {
                    // Smooth-ish color mapping
                    let r = (i * 4) % 255;
                    let g = (i * 7) % 255;
                    let b = (i * 13) % 255;
                    row[x] = (r << 16) | (g << 8) | b;
                }
            }
        });

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}