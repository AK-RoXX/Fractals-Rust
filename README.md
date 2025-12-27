# 🌀 Rust Fractal Generator

A high-performance, real-time fractal generator built with Rust. This project uses the **Julia Set** formula and utilizes multi-threading to render smooth, infinite patterns.

## 🚀 Getting Started

### Prerequisites

- **Rust & Cargo:** [Install Rust](https://rustup.rs/)
- **C++ Build Tools:** (Required for Windows users to link the graphics library)

### Installation

1. Clone your project or navigate to the folder.
2. Ensure your `Cargo.toml` includes the following dependencies:
   ```toml
   [dependencies]
   pixels = "0.13"
   winit = "0.28"
   rayon = "1.8"
   rand = "0.8"
   image = "0.24"
   ```

## 📦 ### Running the App

For the best experience, always run in **release mode** to enable compiler optimizations:

To run the fractal generator, execute the following command:

```bash
cargo run --release
```

## 🎮 Controls

| Key/Mouse         | Action                                                                          |
| ----------------- | ------------------------------------------------------------------------------- |
| Arrow Keys        | Smooth Glide: Hold to fluidly morph the fractal shape in real-time.             |
| Spacebar          | Chaos: Jump to a completely new random Julia Seed.                              |
| Mouse Left-Click  | Dive: Zoom in exactly where your cursor is pointing.                            |
| Mouse Right-Click | Return: Instantly reset zoom and view to the default scale.                     |
| W / S             | Detail: Increase (W) or decrease (S) iterations to sharpen or soften the image. |
| Esc               | Exit the application.                                                           |

## 🎨 Features

- Real-time rendering of fractals
- Multi-threaded performance
- Customizable parameters for fractal generation

## 🛠️ Tech Stack

- Rust: The core engine, providing memory safety and C++ level performance.

- Pixels: A hardware-accelerated 2D frame buffer using the WGPU graphics API.

- Winit: A professional-grade windowing and event handling library for cross-platform support.

- Rayon: A data-parallelism library that automatically distributes pixel calculations across all available CPU cores.

- Rand: Used for generating unique procedural seeds for the Julia Set constants.

## 🔬 Mathematical Features

- Smooth Coloring: Implements Renormalization (logarithmic mapping) to eliminate harsh "color banding" and create silky gradients.

- Liquid Morphing: The "Smooth Glide" system bypasses standard keyboard repeat delays, allowing the Julia constant $c$ to update frame-by-frame.

- Escape Time Algorithm: Optimized calculation of $z_{n+1} = z_n^2 + c$ with an expanded escape radius for better color accuracy.
