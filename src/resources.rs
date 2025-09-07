use bevy::ecs::{entity::Entity, system::Resource};

#[allow(dead_code)]
#[derive(Resource)] 
pub struct Board {
    pub grid: Vec<Vec<Option<String>>>,
    pub rows: usize,
    pub cols: usize,
}

#[derive(Resource)] 
pub struct Selection {
    pub first: Option<Entity>,
    pub second: Option<Entity>,
}

impl Default for Selection {
    fn default() -> Self {
        Selection {
            first: None,
            second: None,
        }
    }
}