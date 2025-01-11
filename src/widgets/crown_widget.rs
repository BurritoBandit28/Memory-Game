use std::collections::HashMap;
use sdl2::keyboard::Keycode::N;
use sdl2::rect::Rect;
use crate::game::{Game, Turn};
use crate::render::AssetData;
use crate::resource_location::ResourceLocation;
use crate::screen::Screen;
use crate::screens::hud_screen::HudScreen;
use crate::widget::Alignment;
use crate::widget::Widget;


pub struct CrownWidget {
    asset_data: AssetData,
    alignment: Alignment,
    coords : (i32, i32),
    game : *mut Game,
    player : Turn,
    selected : bool
}

impl CrownWidget {

    /*
    pub fn create(asset_data: AssetData, alignment: Alignment, x : i32, y : i32) -> Self {
        Self {
            selected: false,
            asset_data,
            alignment,
            coords: (x, y),
        }
    }
     */

    pub fn create(alignment: Alignment, x : i32, y : i32, game : *mut Game, player : Turn) -> Box<Self>
    where
        Self: Sized
    {

        let ret = Self {
            asset_data: AssetData {
                uv: Some(Rect::new(0, 0, 22, 20)),
                origin: (11, 10),
                resource_location: ResourceLocation::new("memory_game", "gui/crown.png"),
            },
            alignment,
            coords: (x, y),
            game,
            player,
            selected: false,
        };
        Box::new(ret)
    }
}

impl Widget for CrownWidget {
    fn on_click(&mut self) {

    }

    fn get_selected(&mut self) -> bool {
        self.selected
    }

    fn set_selected(&mut self, tf : bool) {
        self.selected = tf
    }

    fn get_screen_coordinates(&mut self) -> (i32, i32) {
        self.coords.clone()
    }

    fn set_screen_coordinates(&mut self, x: i32, y: i32) {
        self.coords = (x, y)
    }

    // Dude I have so much repeating code, but I don't really care atp
    fn get_asset_data(&mut self) -> AssetData {

        // get game instant
        let game = unsafe { &mut *self.game };

        // don't use the finger icon when hovered over
        if self.selected {
            game.use_finger=false;
        }

        if game.get_player_1_score() + game.get_player_2_score() == 9 {
            match self.player {
                Turn::Player1 => {
                    if game.get_player_1_score() > game.get_player_2_score() {
                        self.asset_data.clone()
                    }
                    else {
                        let mut ass = AssetData::empty();
                        ass.resource_location = ResourceLocation::new("memory_game", "empty.png");
                        ass
                    }
                }
                Turn::Player2 => {
                    if game.get_player_2_score() > game.get_player_1_score() {
                        self.asset_data.clone()
                    }
                    else {
                        let mut ass = AssetData::empty();
                        ass.resource_location = ResourceLocation::new("memory_game", "empty.png");
                        ass
                    }
                }
            }

        }
        else {
            let mut ass = AssetData::empty();
            ass.resource_location = ResourceLocation::new("memory_game", "empty.png");
            ass
        }
    }

    fn set_asset_data(&mut self, ass: AssetData) {
        self.asset_data = ass
    }

    fn get_resource_location(&mut self) -> ResourceLocation {
        ResourceLocation::new("game", "widgets/crown")
    }


    fn get_allignment(&mut self) -> Alignment {
        self.alignment.clone()
    }

    fn set_allignment(&mut self, alignment: Alignment) {
        self.alignment = alignment;
    }

    fn get_game(&mut self) {
        self.game;
    }


}