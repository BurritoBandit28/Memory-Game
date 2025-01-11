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


// When designing a widget, it is a good idea to keep in mind things like colour and contrast.
// Some users may have issues looking at certain colours, so keeping a high contrast is important.
// If you do not want to have high contrast by default, you can add it as a config option
pub struct PlayAgainWidget {
    selected : bool,
    asset_data: AssetData,
    asset_data_selected : AssetData,
    alignment: Alignment,
    coords : (i32, i32),
    game : *mut Game
}

impl PlayAgainWidget {

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
                uv: Some(Rect::new(0, 0, 93, 23)),
                origin: (0, 0),
                resource_location: ResourceLocation::new("memory_game", "gui/play_again.png"),
            },
            asset_data_selected: AssetData {
                uv: Some(Rect::new(0, 23, 93, 23)),
                origin: (0, 0),
                resource_location: ResourceLocation::new("memory_game", "gui/play_again.png"),
            },
            alignment,
            coords: (x, y),
            game
        };
        Box::new(ret)
    }
}

impl Widget for PlayAgainWidget {
    fn on_click(&mut self) {
        // only run function if the game is over
        let game = unsafe { &mut *self.game };
        if game.get_player_1_score() + game.get_player_2_score() == 9 {
            game.create_memory_game_scene();
            game.current_screen = Some(HudScreen::create(game));
            info!("Resetting and playing again")
            //(*self.game).unwrap().current_screen = None;
        }
    }

    fn get_selected(&mut self) -> bool {
        self.selected
    }

    fn set_selected(&mut self, tf : bool) {
        self.selected=tf
    }

    fn get_screen_coordinates(&mut self) -> (i32, i32) {
        self.coords.clone()
    }

    fn set_screen_coordinates(&mut self, x: i32, y: i32) {
        self.coords = (x, y)
    }

    fn get_asset_data(&mut self) -> AssetData {
        let game = unsafe { &mut *self.game };

        // only show finger pointer if the game is over
        if (game.get_player_1_score() + game.get_player_2_score() != 9) && self.selected{
            game.use_finger=false;
        }

        // get asset data
        if game.get_player_1_score() + game.get_player_2_score() == 9 {
            if self.selected {
                self.asset_data_selected.clone()
            } else {
                self.asset_data.clone()
            }
        }
        else {
            // return invisible if game not over
            let mut ass = AssetData::empty();
            ass.resource_location = ResourceLocation::new("memory_game", "empty.png");
            ass
        }
    }

    fn set_asset_data(&mut self, ass: AssetData) {
        self.asset_data = ass
    }

    fn get_resource_location(&mut self) -> ResourceLocation {
        ResourceLocation::new("game", "widgets/play")
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