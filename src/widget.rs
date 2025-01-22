use std::collections::HashMap;
use log::warn;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use crate::game::Game;
use crate::render;
use crate::render::AssetData;
use crate::resource_location::ResourceLocation;

/// Used to create buttons and widget UI displays.
pub trait Widget {
    #[must_use]
    fn on_click(&mut self);

    #[must_use]
    fn get_selected(&mut self) -> bool;

    #[must_use]
    fn set_selected(&mut self, tf : bool);

    #[must_use]
    fn get_screen_coordinates(&mut self) -> (i32, i32);

    #[must_use]
    fn set_screen_coordinates(&mut self, x : i32, y : i32);

    fn correct_coords(&mut self) -> (i32, i32){
        let dims = render::get_actual_dimensions().lock().unwrap().get();
        let coords = self.get_screen_coordinates();
        match self.get_allignment() {
            Alignment::RIGHT => {
                (dims.0 as i32 + coords.0 , (dims.1 / 2) as i32 - coords.1)
            }
            Alignment::TOP => {
                ((dims.0/2) as i32 + coords.0 , coords.1)
            }
            Alignment::BOTTOM => {
                ((dims.0/2) as i32 + coords.0 , dims.1 as i32 - coords.1)
            }
            Alignment::CENTRE => {
                ((dims.0/2) as i32 + coords.0 , (dims.1 / 2) as i32 - coords.1)
            }
            Alignment::LEFT => {
                (coords.0 , (dims.1 / 2) as i32 - coords.1)
            }
            _ => {
                coords
            }
        }
    }

    #[must_use]
    fn get_asset_data(&mut self) -> AssetData;

    fn get_debug_asset_data(&mut self) -> AssetData {
        let mut ass = self.get_asset_data().clone();
        ass.uv = Some(Rect::new(0,0,ass.uv.unwrap().width(),ass.uv.unwrap().height()));
        ass.resource_location = ResourceLocation::new("game", "gui/widgets/debug_background.png");
        ass
    }

    #[must_use]
    fn set_asset_data(&mut self, ass : AssetData);

    #[must_use]
    fn get_resource_location(&mut self) -> ResourceLocation;

    #[must_use]
    fn get_allignment(&mut self) -> Alignment;

    #[must_use]
    fn set_allignment(&mut self, alignment: Alignment);

    #[must_use]
    fn get_game(&mut self);

    fn return_enum_int(&mut self) -> Option<usize> {
        None
    }

    fn render(&mut self, textures : &HashMap<String, Texture>, sf : i32, canvas : &mut WindowCanvas, debug : bool) {
        let coords = self.correct_coords();
        if debug {
            render::draw_pp_texture(coords.0, coords.1, &self.get_debug_asset_data(), canvas, sf, textures)
        }
        render::draw_pp_texture(coords.0, coords.1, &self.get_asset_data(), canvas, sf, textures)
    }
}



pub enum Alignment {
    LEFT,
    RIGHT,
    TOP,
    BOTTOM,
    CENTRE,
    NONE
}

impl Alignment {
    pub fn parse(val : String) -> Self {
        match val.to_lowercase().as_str() {
            "left" => {
                Self::LEFT
            }
            "right" => {
                Self::RIGHT
            }
            "top" => {
                Self::TOP
            }
            "bottom" => {
                Self::BOTTOM
            }
            "centre" => {
                Self::CENTRE
            }
            _ => {
                warn!("Alignment {} could not be found!", val);
                Self::NONE
            }
        }
    }
}

impl Clone for Alignment {
    fn clone(&self) -> Self {
        match self {
            Alignment::LEFT => {Self::LEFT}
            Alignment::RIGHT => {Self::RIGHT}
            Alignment::TOP => {Self::TOP}
            Alignment::BOTTOM => {Self::BOTTOM}
            Alignment::CENTRE => {Self::CENTRE}
            Alignment::NONE => {Self::NONE}
        }
    }
}