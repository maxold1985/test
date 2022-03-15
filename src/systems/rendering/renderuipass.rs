use egui_wgpu_backend::ScreenDescriptor;
use epi::{IntegrationInfo, WebInfo};
use specs::{Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};
use wgpu::{FilterMode, Texture, TextureView};
use wgpu::util::DeviceExt;

use crate::{BindGroupContainer, BindingResourceContainer, BufferTypes, DebugTextureBindGroup, DebugTexturePipeline, DeltaTime, HorizonBindGroup, RawModel, renderer::state::State, SamplerTypes, TextureViewTypes};
use crate::renderer::utils::texturerenderer::TextureRenderer;
use crate::resources::commandencoder::HorizonCommandEncoder;
use crate::resources::eguicontainer::EguiContainer;
use crate::resources::surfacetexture::SurfaceTexture;
use crate::ui::debugstats::DebugStats;
use crate::ui::UiComponent;

pub struct RenderUIPass;

impl<'a> System<'a> for RenderUIPass {
    type SystemData = (WriteExpect<'a,SurfaceTexture>,
                       WriteExpect<'a, EguiContainer>,
                       ReadExpect<'a,State>,
                       WriteExpect<'a,HorizonCommandEncoder>,
                       WriteExpect<'a,DebugStats>,
                       ReadExpect<'a,BindingResourceContainer>,
                       ReadStorage<'a,RawModel>,
                        ReadStorage<'a,DebugTextureBindGroup>,
                        WriteStorage<'a,BindGroupContainer>,
                        ReadExpect<'a, DebugTexturePipeline>
    );

    fn run(&mut self, (mut surface_texture,
        mut egui_container,
        state,mut command_encoder,
        mut debug_ui,binding_resource_container,
        models,debug_texture_bind_group,
        mut bind_group_container,
        debug_texture_pipeline): Self::SystemData) {

        egui_container.platform.begin_frame();
        
        if debug_ui.debug_texture.is_none()
        {
            let albedo_texture:Option<Texture> = Some(state.device.create_texture(&wgpu::TextureDescriptor {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Bgra8Unorm,
                mip_level_count: 1,
                label: Some("albedo_texture"),
                sample_count: 1,
                size: wgpu::Extent3d {
                    depth_or_array_layers: 1,
                    height: state.sc_descriptor.height,
                    width: state.sc_descriptor.width,
                },
            }));
            debug_ui.debug_texture =albedo_texture;
        }
       if debug_ui.debug_texture_view.is_none()
       {
           let albedo_view:Option<TextureView> = Some(debug_ui.debug_texture.as_ref().unwrap().create_view(&wgpu::TextureViewDescriptor{
               array_layer_count: std::num::NonZeroU32::new(1),
               base_array_layer: 0,
               ..Default::default()
           }));
           debug_ui.debug_texture_view = albedo_view;
       }
        let mut texture_view = None;
        let encoder = command_encoder.get_encoder();
        {
           let view = if let Some(entity) = debug_ui.selected_entity
            {

                  let model =   models.get(entity).unwrap();
                egui::Window::new("material visualizer").show(&egui_container.platform.context(),|ui|{
                    egui::ComboBox::from_label("material").selected_text(format!("material-{}",debug_ui.selected_material)).show_ui(ui,|ui|{
                        for index in  model.materials.keys() {
                            ui.selectable_value(&mut debug_ui.selected_material,*index, format!("material-{}",index));
                        }
                    });
                    egui::ComboBox::from_label("texture").selected_text(format!("texture-{}",debug_ui.selected_texture)).show_ui(ui,|ui|{
                        ui.selectable_value(&mut debug_ui.selected_texture,0,"base color texture");
                        ui.selectable_value(&mut debug_ui.selected_texture,1,"occlusion texture");
                        ui.selectable_value(&mut debug_ui.selected_texture,2,"normal map");
                        ui.selectable_value(&mut debug_ui.selected_texture,3,"emissive texture");
                        ui.selectable_value(&mut debug_ui.selected_texture,4,"roughness texture");

                    });
                });
                   match debug_ui.selected_texture {
                    0 => Some(&model.materials[&debug_ui.selected_material].base_color_texture.view),
                    1 => Some(&model.materials[&debug_ui.selected_material].occlusion_texture.view),
                    2 => Some(&model.materials[&debug_ui.selected_material].normal_map.view),
                    3 => Some(&model.materials[&debug_ui.selected_material].emissive_texture.view),
                    4 => Some(&model.materials[&debug_ui.selected_material].roughness_texture.view),
                    _ => None,
                }
            }
           else {
               None
           };
            //TODO: add debug renderer for depth aswell
            let texture = if let Some(tex_view) = view {
                tex_view
            }
            else {
                binding_resource_container.texture_views[debug_ui.selected_texture_name].as_ref().unwrap()
            };
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: debug_ui.debug_texture_view.as_ref().unwrap(),
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
                    label: Some("texture renderer"),
                });

                let (_,  debug_texture_bind_group_container) = (&debug_texture_bind_group, &mut bind_group_container).join().next().unwrap();
                *debug_texture_bind_group_container = DebugTextureBindGroup::create_container(&state.device, (texture, binding_resource_container.samplers[SamplerTypes::DebugTexture].as_ref().unwrap()));
                render_pass.set_pipeline(&debug_texture_pipeline.0);
                render_pass.set_vertex_buffer(0, binding_resource_container.buffers[BufferTypes::DeferredVao].as_ref().unwrap().slice(..));
                render_pass.set_bind_group(0, &debug_texture_bind_group_container.bind_group, &[]);
                render_pass.draw(0..6, 0..1);
                texture_view = Some(debug_ui.debug_texture_view.as_ref().unwrap());


        }
        //TODO: move egui begin to separate system... maybe.

        if debug_ui.texture_id.is_some() {
            let id = *debug_ui.texture_id.as_ref().unwrap();
            egui_container.render_pass.update_egui_texture_from_wgpu_texture(&state.device, texture_view.as_ref().unwrap(), FilterMode::Linear,id ).unwrap();
        }
        else {
            debug_ui.texture_id = Some(egui_container.render_pass.egui_texture_from_wgpu_texture(&state.device,texture_view.as_ref().unwrap(),FilterMode::Linear));
        }


        debug_ui.show(&egui_container.platform.context(),&mut true);
        let (output, paint_commands) = egui_container.platform.end_frame(None);
        let paint_jobs = egui_container.platform.context().tessellate(paint_commands);

        let screen_desc = ScreenDescriptor {
            scale_factor:state.scale_factor as f32,
            physical_height: state.sc_descriptor.height,
            physical_width: state.sc_descriptor.width,
        };
        let font_image = egui_container.platform.context().font_image();
        let  render_pass = &mut egui_container.render_pass;
        render_pass.update_texture(&state.device,&state.queue,&font_image);
        render_pass.update_user_textures(&state.device,&state.queue);
        render_pass.update_buffers(&state.device,&state.queue,&paint_jobs,&screen_desc);
        let output = surface_texture.texture.take().unwrap();
        let color_attachment = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        //TODO: add error resource and return the error if anything happens.
        render_pass.execute(encoder,&color_attachment,&paint_jobs,&screen_desc,None).unwrap();
        command_encoder.finish(&state.device,&state.queue);
        output.present();
    }
}
