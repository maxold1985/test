
use crate::scripting::util::glmconversion::{Vec3, Vec4};
use crate::scripting::util::horizonentity::HorizonEntity;

use serde::*;
use crate::components::transform::Transform;
use crate::renderer::primitives::lights::pointlight::PointLight;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformComponent
{
    pub position: Vec3,
    pub rotation:Vec3,
    pub scale: Vec3,
    pub model: Option<HorizonEntity>,
}
impl TransformComponent{
    pub fn new(position: Vec3, rotation: Vec3, scale: Vec3, model: Option<HorizonEntity>) -> Self {
        TransformComponent { position, rotation, scale, model }
    }
}

impl From<Transform> for TransformComponent{
    fn from(val:Transform ) -> Self {
            let model_ent = val.model.map(|model| HorizonEntity::from_entity_id(model.id()  ));
        TransformComponent::new(val.position.into(),val.rotation.into(),val.scale.into(),model_ent)
    }
}
#[derive(Serialize,Deserialize)]
pub struct PointLightComponent {
    position: Vec3,
    color: Vec4<f64>,
    constant: f32,
    linear: f32,
    quadratic: f32,
    attached_to: Option<HorizonEntity>,
}

impl PointLightComponent {
    pub fn new(position: Vec3, color: Vec4<f64>, constant: f32, linear: f32, quadratic: f32, attached_to: Option<HorizonEntity>) -> Self {
        PointLightComponent { position, color, constant, linear, quadratic, attached_to }
    }
}

impl From<PointLight> for PointLightComponent {
    fn from(val: PointLight) -> Self {
        let attached_ent = val.attached_to.map(|model| HorizonEntity::from_entity_id(model.id()  ));
        PointLightComponent::new(val.position.into(),val.color.into(),val.constant,val.linear,val.quadratic,attached_ent)
    }
}