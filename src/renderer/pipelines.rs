pub mod forwardpipeline;
pub mod lightpipeline;
pub mod shadowpipeline;
pub mod texturepipeline;
use specs::*;
pub trait HorizonPipeline<'a> {
    type RequiredLayouts;
    fn create_pipeline(
        device: &wgpu::Device,
        swap_chain_desc: &wgpu::SwapChainDescriptor,
        bind_group_layouts: Self::RequiredLayouts,
    ) -> wgpu::RenderPipeline;
}
pub struct RenderPipelineBuilder;

impl RenderPipelineBuilder {
    pub fn create_pipeline(
        fragment_state: Option<wgpu::FragmentState>,
        primitve_state: wgpu::PrimitiveState,
        vertex: wgpu::VertexState,
        device: &wgpu::Device,
        pipeline_layout: &wgpu::PipelineLayout,
        label: Option<&str>,
        depth_stencil_state: Option<wgpu::DepthStencilState>,
    ) -> wgpu::RenderPipeline {
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label,
            layout: Some(&pipeline_layout),
            vertex,
            fragment: fragment_state,
            primitive: primitve_state,
            multisample: wgpu::MultisampleState {
                ..Default::default()
            },
            depth_stencil: depth_stencil_state,
        })
    }
}