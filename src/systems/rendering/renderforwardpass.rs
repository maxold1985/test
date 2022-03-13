use crate::{
    renderer::{
        bindgroupcontainer::BindGroupContainer,
        bindgroups::{
            deferred::DeferredBindGroup, lighting::LightBindGroup, uniforms::UniformBindGroup,
        },
        pipelines::{forwardpipeline::ForwardPipeline},
        state::State,
    },
    resources::{
        bindingresourcecontainer::BindingResourceContainer,
        commandencoder::HorizonCommandEncoder, renderresult::RenderResult,
    },
};
use crate::{resources::deltatime::DeltaTime};
use chrono::Duration;


use specs::prelude::*;
use crate::resources::bindingresourcecontainer::BufferTypes::DeferredVao;
use crate::resources::surfacetexture::SurfaceTexture;

pub struct RenderForwardPass;

impl<'a> System<'a> for RenderForwardPass {
    type SystemData = (
        WriteExpect<'a, HorizonCommandEncoder>,
        ReadExpect<'a, State>,
        ReadExpect<'a, BindingResourceContainer>,
        ReadStorage<'a, LightBindGroup>,
        ReadStorage<'a, UniformBindGroup>,
        ReadStorage<'a, BindGroupContainer>,
        ReadExpect<'a, ForwardPipeline>,
        Write<'a, RenderResult>,
        ReadStorage<'a, DeferredBindGroup>,
        ReadExpect<'a,SurfaceTexture>
    );

    fn run(
        &mut self,
        (
            mut encoder,
            state,
            binding_resource_container,
            light_bind_group,
            uniform_bind_group,
            bind_group_containers,
            forward_pipeline,
            render_result,
            deferred_bind_group,
            surface_texture,
        ): Self::SystemData,
    ) {
        if render_result.result.is_some()
        {
            return;
        }

        let cmd_encoder = encoder.get_encoder();

        let view = surface_texture.texture.as_ref().unwrap().texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut render_pass = cmd_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("forward pass"),
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



        render_pass.set_pipeline(&forward_pipeline.0);

        // //TODO: Convert to resource
        let (_, deffered_bind_group_container) = (&deferred_bind_group, &bind_group_containers)
            .join()
            .next()
            .unwrap();
        let (_, uniform_bind_group_container) = (&uniform_bind_group, &bind_group_containers)
            .join()
            .next()
            .unwrap();
        let (_, light_bind_group_container) = (&light_bind_group, &bind_group_containers)
            .join()
            .next()
            .unwrap();
        render_pass.set_bind_group(0, &deffered_bind_group_container.bind_group, &[]);
        render_pass.set_bind_group(1, &uniform_bind_group_container.bind_group, &[]);
        render_pass.set_bind_group(2, &light_bind_group_container.bind_group, &[]);

        render_pass.set_vertex_buffer(
            0,
            binding_resource_container.buffers[DeferredVao].as_ref().unwrap().slice(..),
        );
        render_pass.draw(0..6, 0..1);
        drop(render_pass);
    }
}
