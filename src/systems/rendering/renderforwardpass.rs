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
            mut render_result,
            deferred_bind_group,
        ): Self::SystemData,
    ) {
        let frame_result = state.surface.get_current_texture();
        let frame;
        if let Ok(f) = frame_result {
            frame = f;
        } else {
            render_result.result = frame_result.err();
            return;
        }

        let frame_view = &frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let cmd_encoder = encoder.get_encoder();

        // {
        //     let renderer = TextureRenderer::new(
        //         &state.device,
        //         binding_resource_container
        //             .texture_views
        //             .get("albedo_view")
        //             .unwrap(),
        //         &state.sc_descriptor,
        //     );
        //     let mut render_pass = cmd_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        //         color_attachments: &[wgpu::RenderPassColorAttachment {
        //             view: frame_view,
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
        //         label: Some("texture renderer"),
        //     });

        //     render_pass.set_pipeline(&renderer.render_pipeline);
        //     render_pass.set_bind_group(0, &renderer.bind_group, &[]);
        //     render_pass.set_vertex_buffer(0, renderer.quad.slice(..));
        //     render_pass.draw(0..TextureRenderer::QUAD_VERTEX_ARRAY.len() as u32, 0..1);
        // }

        let mut render_pass = cmd_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("forward pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: frame_view,
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

        // Some(wgpu::RenderPassDepthStencilAttachment {
        //         view: &state.depth_texture.view,
        //         depth_ops: Some(wgpu::Operations {
        //             load: wgpu::LoadOp::Clear(0.0),
        //             store: false,
        //         }),
        //         stencil_ops: None,
        //     }),

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
        // TODO: move to it's own system
        render_pass.set_bind_group(0, &deffered_bind_group_container.bind_group, &[]);
        render_pass.set_bind_group(1, &uniform_bind_group_container.bind_group, &[]);
        render_pass.set_bind_group(2, &light_bind_group_container.bind_group, &[]);

        render_pass.set_vertex_buffer(
            0,
            binding_resource_container
                .buffers
                .get("deferred_vao")
                .unwrap()
                .slice(..),
        );
        // // // TODO: move this to it's own system
        render_pass.draw(0..6, 0..1);
        drop(render_pass);
    }
}