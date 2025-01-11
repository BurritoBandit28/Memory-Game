use uuid::Uuid;
use crate::entity::Entity;
use crate::game::Game;
use crate::render::AssetData;
use crate::resource_location::ResourceLocation;
use crate::utils::create_uuid;

pub struct DummyEntity {
    coords: (f32, f32),
    pub asset_data: AssetData,
    // hitbox : matrix,
    velocity : (f32, f32),
    uuid : Uuid,
    game : *mut Game,
    health : f32,
    resource_location: ResourceLocation,
    index : usize,
}

impl Entity for DummyEntity {
    fn get_coords(&mut self) -> (f32, f32) {
        self.coords
    }

    fn set_coords(&mut self, coords: (f32, f32)) {
        self.coords = coords
    }

    fn get_health(&mut self) -> f32 {
        self.health
    }

    fn change_health(&mut self, amount: f32) {
        todo!()
    }

    fn set_resource_location(&mut self, rl: ResourceLocation) {
        self.resource_location = rl
    }

    fn get_resource_location(&self) -> &ResourceLocation {
        &self.resource_location
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_velocity(&mut self) -> (f32, f32) {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: (f32, f32)) {
        self.velocity = velocity
    }

    fn get_asset_data(&mut self) -> AssetData {
        self.asset_data.clone()
    }

    fn set_asset_data(&mut self, ass: AssetData) {
        self.asset_data = ass;
    }
}

impl DummyEntity {
    pub fn create(game : &mut Game, ass : AssetData) -> DummyEntity {

        let uuid = create_uuid();

        let rl = ResourceLocation::empty();

        Self {
            coords: (0.0, 0.0),
            asset_data: ass,
            velocity: (0.0, 0.0),
            uuid,
            game,
            health: 0.0,
            resource_location: rl,
            index : game.entities.len()
        }
    }
}