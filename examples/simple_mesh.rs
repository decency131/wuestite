use wuestite::prelude::*;
use wuestite_renderer::{Mesh, Vertex};

fn main() {
    let mesh = Mesh::new(
        vec![
            Vertex {
                position: [0.0, 0.5, 0.0],
                color: [1.0, 0.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [0.0, 1.0, 0.0, 1.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [0.0, 0.0, 1.0, 1.0],
            },
        ],
        vec![0, 1, 2],
    );

    App::new()
        .with_window("Wuestite Meshes", 1280, 720)
        .add_mesh(mesh)
        .run();
}
