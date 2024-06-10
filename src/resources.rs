use bevy::prelude::Handle;
use bevy::prelude::Image;
use bevy::ecs::system::Resource;

#[derive(Resource, Clone)]
pub struct GameTextures {
    pub head_up: Handle<Image>,
    pub head_down: Handle<Image>,
    pub head_left: Handle<Image>,
    pub head_right: Handle<Image>,
    pub body_horizontal: Handle<Image>,
    pub body_vertical: Handle<Image>,
    pub body_bottomleft: Handle<Image>,
    pub body_bottomright: Handle<Image>,
    pub body_topleft: Handle<Image>,
    pub body_topright: Handle<Image>,
    pub tail_up: Handle<Image>,
    pub tail_down: Handle<Image>,
    pub tail_left: Handle<Image>,
    pub tail_right: Handle<Image>,
    pub apple: Handle<Image>,
}
