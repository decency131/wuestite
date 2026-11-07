use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

use wuestite_renderer::{Mesh, Renderer};

pub use crate::{Event, EventHandler, System, SystemRegistry, World};

pub struct App {
    running: bool,
    started: bool,
    world: World,
    system: SystemRegistry,

    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    meshes: Vec<Mesh>,

    title: String,
    width: u32,
    height: u32,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            running: false,
            started: false,
            world: World::new(),
            system: SystemRegistry::new(),

            window: None,
            renderer: None,
            meshes: Vec::new(),

            title: "Wuestite".into(),
            width: 1280,
            height: 720,
        }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(self).unwrap();
    }

    pub fn quit(&mut self) {
        if self.started && self.running {
            self.system.shutdown(&mut self.world);
        }

        self.running = false;
    }

    pub fn add_system<S: System + 'static>(&mut self, system: S) -> &mut App {
        self.system.add_system(system);
        self
    }

    pub fn with_window(&mut self, title: impl Into<String>, width: u32, height: u32) -> &mut Self {
        self.title = title.into();
        self.width = width;
        self.height = height;
        self
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> &mut Self {
        self.meshes.push(mesh);
        self
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let attributes = Window::default_attributes()
            .with_title(&self.title)
            .with_inner_size(LogicalSize::new(self.width, self.height));

        let window = Arc::new(event_loop.create_window(attributes).unwrap());

        let mut renderer = pollster::block_on(Renderer::new(window.clone()));

        for mesh in self.meshes.drain(..) {
            renderer.add_mesh(mesh);
        }

        self.renderer = Some(renderer);

        self.window = Some(window.clone());

        if !self.started {
            self.system.run(&mut self.world);
            self.started = true;
        }

        self.running = true;
        window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                self.quit();
                event_loop.exit();
            }

            WindowEvent::Resized(size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(size.width, size.height);
                }
            }

            WindowEvent::RedrawRequested => {
                if !self.running {
                    event_loop.exit();
                    return;
                }

                self.system.update(&mut self.world);

                if self.world.get_events::<AppExit>().is_some() {
                    self.quit();
                    event_loop.exit();
                    return;
                }

                if let Some(renderer) = &mut self.renderer {
                    renderer.render();
                }

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }

            _ => {}
        }
    }
}

#[derive(Event)]
pub struct AppExit {
    pub error: Option<String>,
}
