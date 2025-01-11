use std::collections::HashMap;
use log::info;
use sdl2::keyboard::Keycode::N;
use sdl2::rect::Rect;
use crate::game::Game;
use crate::render::AssetData;
use crate::resource_location::ResourceLocation;
use crate::screen::Screen;
use crate::screens::hud_screen::HudScreen;
use crate::widget::Alignment;
use crate::widget::Widget;

pub struct EndQuitWidget {
    selected : bool,
    asset_data: AssetData,
    asset_data_selected : AssetData,
    alignment: Alignment,
    coords : (i32, i32),
    game : *mut Game
}

impl EndQuitWidget {

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

    pub fn create(alignment: Alignment, x : i32, y : i32, game : *mut Game) -> Box<Self>
    where
        Self: Sized
    {

        let ret = Self {
            selected: false,
            asset_data: AssetData {
                uv: Some(Rect::new(0, 0, 38, 24)),
                origin: (0, 0),
                resource_location: ResourceLocation::new("memory_game", "gui/quit_endscreen.png"),
            },
            asset_data_selected: AssetData {
                uv: Some(Rect::new(0, 24, 38, 24)),
                origin: (0, 0),
                resource_location: ResourceLocation::new("memory_game", "gui/quit_endscreen.png"),
            },
            alignment,
            coords: (x, y),
            game
        };
        Box::new(ret)
    }
}

impl Widget for EndQuitWidget {
    fn on_click(&mut self) {
        // only run if game over
        let game = unsafe { &mut *self.game };
        if game.get_player_1_score() + game.get_player_2_score() == 9 {
            info!("Quitting");
            game.running = false
            //(*self.game).unwrap().current_screen = None;
        }
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

    fn get_asset_data(&mut self) -> AssetData {

        // dont show finger if game is not over
        let game = unsafe { &mut *self.game };
        if (game.get_player_1_score() + game.get_player_2_score() != 9) && self.selected{
            game.use_finger=false;
        }

        // ger asset data
        if game.get_player_1_score() + game.get_player_2_score() == 9 {
            if self.selected {
                self.asset_data_selected.clone()
            } else {
                self.asset_data.clone()
            }
        }
        else {
            // return invisible texture if game not over
            let mut ass = AssetData::empty();
            ass.resource_location = ResourceLocation::new("memory_game", "empty.png");
            ass
        }
    }

    fn set_asset_data(&mut self, ass: AssetData) {
        self.asset_data = ass
    }

    fn get_resource_location(&mut self) -> ResourceLocation {
        ResourceLocation::new("game", "widgets/quit")
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