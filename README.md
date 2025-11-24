![Rust Graphics Header](header.png)

# Rust Graphics Engine

![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

A 3D software rendering engine built from scratch in Rust. Implements a custom graphics pipeline, linear algebra library, and scene graph without hardware acceleration APIs.

Based on concepts from *Computer Graphics from Scratch* by Gabriel Gambetta and university course materials.

## ✨ Features

* **Software Rasterization**: Custom implementation of the graphics pipeline (`src/renderer/rasterizer.rs`) handling triangle projection and pixel drawing.
* **Interactive Scene Graph**: Runtime manipulation of objects (Translate, Rotate, Scale) with support for hierarchical selection.
* **Dynamic Lighting**: Move and rotate light sources in real-time to test shading.
* **Custom Math Library**: Hand-rolled Vector, Matrix, and Geometry implementations (`src/types/math`) to handle 3D transformations.
* **Robust Input System**: Custom state-tracking wrapper around `minifb`, enabling distinct checks for Pressed, Held, and Released states.

## 📂 Project Structure

* **`src/engine.rs`**: The main game loop and engine lifecycle management.
* **`src/renderer/`**: The core rendering pipeline.
    * `rasterizer.rs`: Converts projected triangles into pixels (Barycentric/Edge functions).
    * `draw_command.rs`: Handles the queue of drawing operations.
    * `frustum.rs`: Handles view culling.
* **`src/types/math/`**: Linear algebra library (Matrices, Vectors, Points).
* **`src/scene/`**: Manages objects in the world, bounding boxes, and scene nodes.
* **`src/models/`**: Procedural geometry for testing (Cube, Ball, Plane).

## 🚀 Getting Started

### Prerequisites
* **Rust**: Ensure you have the latest stable version of Rust and Cargo installed.

### Installation

1.  Clone the repository:
    ```bash
    git clone https://github.com/JoeKL/rust_graphics.git
    cd rust_graphics
    ```

2.  Run the engine:
    ```bash
    cargo run --release
    ```
    > **Note:** The `--release` flag is highly recommended. Since this is a software renderer doing heavy math on the CPU, debug builds may be significantly slower.

## 🎮 Controls

The engine operates as a model viewer. Instead of moving the camera, you manipulate the scene and lighting.

| Input / Key | Category | Action | Logic / Details |
| :--- | :--- | :--- | :--- |
| **Space** | **Selection** | **Cycle Object Focus** | Cycles focus between Root, Child, or Grandchild node. |
| **Arrow Up / Down** | **Transform** | **Move Object (Z-Axis)** | Translates the currently focused object along the Z-axis. |
| **Arrow Left / Right** | **Transform** | **Move Object (X-Axis)** | Translates the currently focused object along the X-axis. |
| **Mouse Drag (Left)** | **Transform** | **Rotate Object** | Rotates the focused object based on mouse distance from screen center (Virtual Trackball). |
| **N** | **Transform** | **Scale Down** | Decreases the isotropic scale of the focused object. |
| **M** | **Transform** | **Scale Up** | Increases the isotropic scale of the focused object. |
| **W / S** | **Lighting** | **Rotate Lights (X)** | Rotates all light sources around the X-axis. |
| **A / D** | **Lighting** | **Rotate Lights (Y)** | Rotates all light sources around the Y-axis. |
| **O** | **Camera** | **Increase FOV** | Widens the camera Field of View (+0.5° per frame). |
| **P** | **Camera** | **Decrease FOV** | Narrows the camera Field of View (-0.5° per frame). |
| **K** | **Debug** | **Toggle Axis** | Toggles drawing of the coordinate system axis. |
| **L** | **Debug** | **Toggle Light Vecs** | Toggles visualization of light direction vectors. |

## 🤝 Contributing

This is an educational project, but contributions are welcome!

1.  Fork the Project
2.  Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3.  Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4.  Push to the Branch (`git push origin feature/AmazingFeature`)
5.  Open a Pull Request

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.

## Model Source

https://github.com/alecjacobson/common-3d-test-models
