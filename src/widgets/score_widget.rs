use std::collections::HashMap;
use num::clamp;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use crate::entity::Entity;
use crate::game::Game;
use crate::render;
use crate::render::AssetData;
use crate::resource_location::ResourceLocation;
use crate::widget::{Alignment, Widget};
use crate::widgets::play_widget::PlayWidget;

pub struct ScoreWidget {
    selected : bool,
    base_asset_data: AssetData,
    alignment: Alignment,
    coords : (i32, i32),
    game : *mut Game,
    score : u32
}

impl ScoreWidget {
    pub fn create(alignment: Alignment, x : i32, y : i32, game : *mut Game) -> Box<Self>
    where
        Self: Sized
    {

        let ret = Self {
            selected: false,
            base_asset_data: AssetData {
                uv: Some(Rect::new(0, 0, 8, 10)),
                origin: (0, 0),
                resource_location: ResourceLocation::new("game", "gui/numbers.png"),
            },
            alignment,
            coords: (x, y),
            game,
            score: 0
        };
        Box::new(ret)
    }
}

impl Widget for ScoreWidget {
    fn on_click(&mut self) {}

    fn get_selected(&mut self) -> bool {
        false
    }

    fn set_selected(&mut self, tf: bool) {}

    fn get_screen_coordinates(&mut self) -> (i32, i32) {
        self.coords
    }


    fn set_screen_coordinates(&mut self, x: i32, y: i32) {
        self.coords = (x, y)
    }

    fn get_asset_data(&mut self) -> AssetData {
        self.base_asset_data.clone()
    }

    fn set_asset_data(&mut self, ass: AssetData) {
        self.base_asset_data = ass
    }

    fn get_resource_location(&mut self) -> ResourceLocation {
        ResourceLocation::new("game", "widgets/score")
    }

    fn get_allignment(&mut self) -> Alignment {
        self.alignment.clone()
    }

    fn set_allignment(&mut self, alignment: Alignment) {
        self.alignment = alignment
    }

    // again why is this here what was I on??
    fn get_game(&mut self) {
    }

    fn render(&mut self, textures: &HashMap<String, Texture>, sf: i32, canvas: &mut WindowCanvas, debug : bool) {
        let game = unsafe { &mut *self.game };
        let score_as_string = format!("{}",game.score.clone() as u32);
        let mut counter = 0;
        for character in score_as_string.chars() {
            let mut asset_data = self.base_asset_data.clone();
            asset_data.uv = Some(Rect::new(character.to_string().parse::<i32>().unwrap() * 8, 0, 8, 10));
            if debug {
                let mut d_ass = asset_data.clone();
                d_ass.uv = Some(Rect::new(0,0,d_ass.uv.unwrap().width(),d_ass.uv.unwrap().height()));
                d_ass.resource_location = ResourceLocation::new("game", "gui/widgets/debug_background.png");;
                render::draw_pp_texture(self.coords.0 + (8 * counter), self.coords.1, &d_ass, canvas, sf, textures);
            }
            render::draw_pp_texture(self.coords.0 + (8 * counter), self.coords.1, &asset_data, canvas, sf, textures);
            counter+=1;
        }

    }

}