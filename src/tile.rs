use std::cmp::PartialEq;
use std::collections::HashMap;
use sdl2::keyboard::Scancode::S;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, WindowCanvas};
use crate::render;
use crate::render::AssetData;
use crate::resource_location::ResourceLocation;
use crate::widgets::enum_widget::WidgetEnum;

/// Determins whether a tile is a floor (no hitbox) or a wall (hitbox)
pub enum TileType {
    WALL,
    FLOOR
}
impl TileType {
    pub fn parse(ttype : String) -> Self {
        match ttype.as_str() {
            "wall" => {Self::WALL}
            _ => {Self::FLOOR}
        }
    }

    /// Returns an int version of each field
    pub fn as_int(self) -> u32 {
        match self {
            TileType::WALL => {0}
            TileType::FLOOR => {1}
        }
    }
}

impl Clone for TileType {
    fn clone(&self) -> Self {
        match self {
            TileType::WALL => {Self::WALL}
            TileType::FLOOR => {Self::FLOOR}
        }
    }
}

impl Copy for TileType {

}

impl PartialEq for TileType {
    fn eq(&self, other: &Self) -> bool {
        self.as_int() == other.as_int()
    }
}

/// The size of a tile, 64, 32 or 16
pub enum TileSize {
    BIG,
    MEDIUM,
    SMALL
}

impl TileSize {
    pub fn parse(ts : &str) -> Self {
        match ts.to_lowercase().as_str() {
            "small" => {
                Self::SMALL
            }
            "medium" => {
                Self::MEDIUM
            }
            _ => {
                Self::BIG
            }
        }
    }

    pub fn get(&self) -> (u32, u32) {
        match self {
            TileSize::BIG => {(64, 64)}
            TileSize::MEDIUM => {(32 ,32)}
            TileSize::SMALL => {(16, 16)}
        }
    }

}
// Implementation for the WidgetEnum trait
impl WidgetEnum for TileSize {
    fn get_as_string(&mut self) -> String {
        match self {
            TileSize::BIG => {String::from("big_tile")}
            TileSize::MEDIUM => {String::from("medium_tile")}
            TileSize::SMALL => {String::from("small_tile")}
        }
    }

    fn get_from_index(index: usize) -> Self {
        match index {
            1 => {TileSize::MEDIUM},
            2=> {TileSize::BIG}
            _ => {TileSize::SMALL}
        }
    }

    fn count(&mut self) -> usize {
        3usize
    }

    fn name(&mut self) -> String {
        String::from("tile_size")
    }
}

impl Clone for TileSize {
    fn clone(&self) -> Self {
        match self {
            TileSize::BIG => {Self::BIG}
            TileSize::MEDIUM => {Self::MEDIUM}
            TileSize::SMALL => {Self::SMALL}
        }
    }
}

impl Copy for TileSize {

}


/// Tiles loaded on runtime by a json file that defines its texture, material and if it's a wall
pub struct Tile {
    name : String,
    resource_location: ResourceLocation,
    tile_type: TileType,
    size : TileSize,
    origin : (i32, i32),
    collision : bool,
    collision_box : Option<(u32, u32)>,
    asset_data : AssetData
}

impl PartialEq for TileSize {
    fn eq(&self, other: &Self) -> bool {
        self.get().0 == other.get().0
    }
}

impl Tile {

    pub fn create(
        name : String,
        resource_location : ResourceLocation,
        texture : ResourceLocation,
        uv : (u32, u32),
        tile_type : TileType,
        size : TileSize,
        origin : (i32, i32),
        collision : bool,
        collision_box : Option<(u32, u32)>
    ) -> Self {

        let s = size.get();

        let ass = AssetData {
            uv : Some(Rect::new(uv.0 as i32, uv.1 as i32, s.0, s.1)),
            origin,
            resource_location : texture,
        };

        Self {
            name,
            resource_location,
            tile_type,
            size,
            origin,
            collision,
            collision_box,
            asset_data : ass
        }
    }

    pub fn create_none(size : TileSize) -> Self {

        let s = size.get();

        let ass = AssetData {
            uv : Some(Rect::new(0, 0, s.0, s.1)),
            origin : (0,0),
            resource_location : ResourceLocation::empty(),
        };

        Self {
            name: "None".to_string(),
            resource_location: ResourceLocation::new("game", "tiles/none"),
            tile_type: TileType::WALL,
            size,
            origin: (0, 0),
            collision: false,
            collision_box: None,
            asset_data: ass,
        }

    }

    pub fn create_nav() -> Self {


        let ass = AssetData {
            uv : Some(Rect::new(0, 0, 16, 16)),
            origin : (0,0),
            resource_location : ResourceLocation::new("game", "tiles/nav.png"),
        };

        Self {
            name: "nav".to_string(),
            resource_location: ResourceLocation::new("game", "tiles/nav"),
            tile_type: TileType::FLOOR,
            size : TileSize::SMALL,
            origin: (0, 0),
            collision: false,
            collision_box: None,
            asset_data: ass,
        }

    }

    fn screen(&self, coords : (i32, i32), player_coords :  (f32, f32)) -> (i32, i32) {
        let size = self.size.get();

        let x = coords.0;
        let y = coords.1;
        let px = (if player_coords.0 < 0.0 {player_coords.0 - 1.0} else { player_coords.0 }) as i32;
        let py = (if player_coords.1 < 0.0 {player_coords.1 - 1.0} else { player_coords.1 }) as i32;
        let half_x = (render::DIMENSIONS.0 / 2) as i32;
        let half_y = (render::DIMENSIONS.1 / 2) as i32;
        ((half_x - px) + x, (half_y - py ) + y)
    }

    pub fn render(&self, texture: &HashMap<String,Texture>, coords : (i32, i32), canvas: &mut WindowCanvas, sf : i32, player_coords :  (f32, f32)) {
        let screen = self.screen(coords, player_coords);
        render::draw_pp_texture(screen.0, screen.1, &self.asset_data, canvas, sf, texture)

    }

    pub fn get_name(&mut self) -> String {
        self.name.clone()
    }

    pub fn get_resource_location(&mut self) -> ResourceLocation {
        self.resource_location.clone()
    }

    pub fn get_size(self) -> TileSize {
        self.size
    }

    pub fn get_type(&mut self) -> TileType {
        self.tile_type.clone()
    }

}

impl Clone for Tile {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            resource_location: self.resource_location.clone(),
            tile_type: self.tile_type.clone(),
            size: self.size.clone(),
            origin: self.origin.clone(),
            collision: self.collision.clone(),
            collision_box: self.collision_box.clone(),
            asset_data: self.asset_data.clone(),
        }
    }
}
