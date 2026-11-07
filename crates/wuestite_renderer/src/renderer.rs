use std::sync::Arc;
use winit::window::Window;

use crate::{
    context::GraphicsContext,
    mesh::{GpuMesh, Mesh},
    pipeline::create_mesh_pipeline,
};

pub struct Renderer {
    context: GraphicsContext,
    pipeline: wgpu::RenderPipeline,
    meshes: Vec<GpuMesh>,
    window: Arc<Window>,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        let context = GraphicsContext::new(window.clone()).await;
        let pipeline = create_mesh_pipeline(&context);

        Self {
            context,
            pipeline,
            meshes: Vec::new(),
            window,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.context.resize(width, height);
    }

    pub fn render(&mut self) {
        let ctx = &self.context;

        let frame = match ctx.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame)
            | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,

            wgpu::CurrentSurfaceTexture::Outdated => {
                return;
            }

            wgpu::CurrentSurfaceTexture::Lost => {
                return;
            }

            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
                return;
            }

            wgpu::CurrentSurfaceTexture::Validation => {
                panic!("Surface validation error");
            }
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Mesh Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            pass.set_pipeline(&self.pipeline);

            for mesh in &self.meshes {
                pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));

                pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

                pass.draw_indexed(0..mesh.index_count, 0, 0..1);
            }
        }

        ctx.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        ctx.queue.present(frame);
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> usize {
        let gpu_mesh = GpuMesh::new(&self.context.device, &mesh);

        let handle = self.meshes.len();
        self.meshes.push(gpu_mesh);

        handle
    }
}
