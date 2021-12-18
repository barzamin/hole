use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};
use std::time::Instant;
use color_eyre::eyre::{Result, eyre};

pub struct GfxState {
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface,
    pub surf_cfg: wgpu::SurfaceConfiguration,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: PhysicalSize<u32>,

    pub t0: Instant,
}

impl GfxState {
    // fn construct_pipeline(layout: &wgpu::PipelineLayout, device: &wgpu::Device, shader: &wgpu::ShaderModule, fragment_format: wgpu::TextureFormat) -> wgpu::RenderPipeline {
    //     device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    //         label: Some("render pipeline"),
    //         layout: Some(&layout),
    //         vertex: wgpu::VertexState {
    //             module: &shader,
    //             entry_point: "main_vs",
    //             buffers: &[],
    //         },
    //         fragment: Some(wgpu::FragmentState {
    //             module: &shader,
    //             entry_point: "main_fs",
    //             targets: &[wgpu::ColorTargetState {
    //                 format: fragment_format,
    //                 blend: Some(wgpu::BlendState::REPLACE),
    //                 write_mask: wgpu::ColorWrite::ALL,
    //             }],
    //         }),
    //         primitive: wgpu::PrimitiveState {
    //             topology: wgpu::PrimitiveTopology::TriangleList,
    //             strip_index_format: None,
    //             front_face: wgpu::FrontFace::Ccw,
    //             cull_mode: Some(wgpu::Face::Back),
    //             polygon_mode: wgpu::PolygonMode::Fill,
    //             clamp_depth: false,
    //             conservative: false,
    //         },
    //         depth_stencil: None,
    //         multisample: wgpu::MultisampleState {
    //             count: 1,
    //             mask: !0, // all samples
    //             alpha_to_coverage_enabled: false,
    //         },
    //     })
    // }

    // fn build_shader_module(device: &wgpu::Device, spirv: &[u8]) -> wgpu::ShaderModule {
    //     device.create_shader_module(&wgpu::ShaderModuleDescriptor {
    //         label: Some("main pipeline ShaderModule"),
    //         source: wgpu::util::make_spirv(spirv),
    //         flags: wgpu::ShaderFlags::empty(), // don't validate; LLVM (probably) knows better than wgpu :3
    //     })
    // }

    pub async fn new(window: &Window) -> Result<Self> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        // shld scan all adaptors
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(eyre!("couldn't get an adapter"))?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),

                    // features: wgpu::Features::PUSH_CONSTANTS,
                    // limits: wgpu::Limits {
                    //     max_push_constant_size: 256,
                    //     ..Default::default()
                    // },
                },
                None,
            )
            .await?;

        device.on_uncaptured_error(|error| {
            panic!("uncaptured wgpu error: {:#?}", error);
        });

        let surf_cfg = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surf_cfg);

        let t0 = Instant::now();

        Ok(Self {
            size,
            instance,
            surf_cfg,
            surface,
            adapter,
            device,
            queue,
            t0,
        })
    }

    // pub fn load_shader_code(&mut self, new_spirv: &[u8]) {
    //     let shader = Self::build_shader_module(&self.device, new_spirv);
    //     self.render_pipeline = Self::construct_pipeline(&self.render_pipeline_layout, &self.device, &shader, self.sc_desc.format);
    // }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.surf_cfg.width  = new_size.width;
            self.surf_cfg.height = new_size.height;
            self.surf_cfg.width = new_size.width;
            self.surface.configure(&self.device, &self.surf_cfg);
        }
    }

    pub fn input(&mut self, evt: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
        // todo!();
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("render encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("clear pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())

        // {
        //     let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        //         label: Some("clear pass"),
        //         color_attachments: &[wgpu::RenderPassColorAttachment {
        //             view: render_to,
        //             resolve_target: None,
        //             ops: wgpu::Operations {
        //                 load: wgpu::LoadOp::Clear(wgpu::Color {
        //                     r: 0.1,
        //                     g: 0.2,
        //                     b: 0.3,
        //                     a: 1.0,
        //                 }),
        //                 store: true,
        //             },
        //         }],
        //         depth_stencil_attachment: None,
        //     });

        //     let push_constants = ShaderConstants {
        //         width_px: self.sc_desc.width,
        //         height_px: self.sc_desc.height,
        //         time: self.t0.elapsed().as_secs_f32(),
        //     };

        //     render_pass.set_pipeline(&self.render_pipeline);
        //     render_pass.set_push_constants(
        //         wgpu::ShaderStage::all(),
        //         0,
        //         bytemuck::bytes_of(&push_constants),
        //     );
        //     render_pass.draw(0..3, 0..1);
        // }

        // self.queue.submit(std::iter::once(encoder.finish()));
    }
}