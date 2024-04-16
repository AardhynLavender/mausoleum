use crate::engine::rendering::renderer::layer;

pub mod ripper;
pub mod spiky;
pub mod zoomer;
pub mod buzz;

pub type CreatureLayer = layer::Layer4;

#[derive(Default)]
/// A marker component for entities that are "creatures"
pub struct Creature;