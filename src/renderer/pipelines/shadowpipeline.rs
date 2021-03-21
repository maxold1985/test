use crate::renderer::bindgroupcontainer::BindGroupContainer;
use crate::renderer::pipelines::RenderPipelineContainer;
use crate::renderer::primitives::vertex::{ModelVertex, Vertex};

use super::HorizonPipeline;

pub struct ShadowPipeline;

impl<'a> HorizonPipeline<'a> for ShadowPipeline {
    type RequiredLayouts = &'a BindGroupContainer;

    fn create_pipeline(
        device: &wgpu::Device,
        swap_chain_desc: &wgpu::SwapChainDescriptor,
        bind_group_layouts: Self::RequiredLayouts,
    ) -> super::RenderPipelineContainer {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("shadow"),
            bind_group_layouts: &[&bind_group_layouts.layout],
            push_constant_ranges: &[],
        });

        let vs_module =
            device.create_shader_module(&wgpu::include_spirv!("../../shaders/shadow.vert.spv"));
        let depth_stencil_state = wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            bias: wgpu::DepthBiasState {
                clamp: 0.0,
                constant: 2, // bilinear filtering
                slope_scale: 2.0,
            },
            depth_compare: wgpu::CompareFunction::LessEqual,
            depth_write_enabled: true,
            stencil: wgpu::StencilState::default(),
            clamp_depth: device.features().contains(wgpu::Features::DEPTH_CLAMPING),
        };
        let vertex_state = wgpu::VertexState {
            buffers: &[ModelVertex::desc()],
            entry_point: "main",
            module: &vs_module,
        };
        let primitve_state = wgpu::PrimitiveState {
            front_face: wgpu::FrontFace::Ccw,
            topology: wgpu::PrimitiveTopology::TriangleList,
            cull_mode: Some(wgpu::Face::Front),
            strip_index_format: if cfg!(target_arch = "wasm32") {
                Some(wgpu::IndexFormat::Uint32)
            } else {
                None
            },
            polygon_mode: wgpu::PolygonMode::Fill,
            ..Default::default()
        };
        RenderPipelineContainer::create_pipeline(
            None,
            primitve_state,
            vertex_state,
            &device,
            &pipeline_layout,
            Some("Shadow pipeline"),
            Some(depth_stencil_state),
        )
    }
}
