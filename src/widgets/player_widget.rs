use std::collections::HashMap;
use std::thread::current;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::rect::Rect;
use crate::entity::Entity;
use crate::game::{Game, Turn};
use crate::render;
use crate::render::AssetData;
use crate::resource_location::ResourceLocation;
use crate::widget::{Alignment, Widget};

pub struct PlayerWidget {
    active : bool,
    selected : bool,
    asset_data: AssetData,
    asset_data_score : AssetData,
    alignment: Alignment,
    coords : (i32, i32),
    game : *mut Game,
    player : Turn,
    score : u32
}

impl Widget for PlayerWidget {
    fn on_click(&mut self) {}

    fn get_selected(&mut self) -> bool {
        self.selected
    }

    fn set_selected(&mut self, tf: bool) {
        self.selected = tf
    }

    fn get_screen_coordinates(&mut self) -> (i32, i32) {
        self.coords.clone()
    }

    fn set_screen_coordinates(&mut self, x: i32, y: i32) {
        self.coords = (x, y)
    }

    fn get_asset_data(&mut self) -> AssetData {
        let game = unsafe { &mut *self.game};
        if game.prev_success {
            self.asset_data.clone()
        }
        else {
            match self.player {
                Turn::Player1 => {
                    if (self.active && game.wait_timer < 0.0) || (game.wait_timer > 0.0) && game.current_turn == Turn::Player2 {
                        self.asset_data = AssetData {
                            uv: Some(Rect::new(0, 0, 68, 21)),
                            origin: (0, 0),
                            resource_location: ResourceLocation::new("memory_game", "gui/player_1.png"),
                        };
                        self.asset_data.clone()
                    } else {
                        self.asset_data = AssetData {
                            uv: Some(Rect::new(0, 21, 68, 21)),
                            origin: (0, 0),
                            resource_location: ResourceLocation::new("memory_game", "gui/player_1.png"),
                        };
                        self.asset_data.clone()
                    }
                }
                Turn::Player2 => {
                    if (self.active && game.wait_timer < 0.0) || (game.wait_timer > 0.0) && game.current_turn == Turn::Player1 {
                        self.asset_data = AssetData {
                            uv: Some(Rect::new(0, 0, 68, 21)),
                            origin: (0, 0),
                            resource_location: ResourceLocation::new("memory_game", "gui/player_2.png"),
                        };
                        self.asset_data.clone()
                    } else {
                        self.asset_data = AssetData {
                            uv: Some(Rect::new(0, 21, 68, 21)),
                            origin: (0, 0),
                            resource_location: ResourceLocation::new("memory_game", "gui/player_2.png"),
                        };
                        self.asset_data.clone()
                    }
                }
            }
        }
    }

    fn set_asset_data(&mut self, ass: AssetData) {
        todo!()
    }

    fn get_resource_location(&mut self) -> ResourceLocation {
        todo!()
    }

    fn get_allignment(&mut self) -> Alignment {
        self.alignment.clone()
    }

    fn set_allignment(&mut self, alignment: Alignment) {
        self.alignment = alignment
    }

    fn get_game(&mut self) {
        todo!()
    }

    fn render(&mut self, textures: &HashMap<String, Texture>, sf: i32, canvas: &mut WindowCanvas, dims: (u32, u32), debug: bool) {
        let game = unsafe { &mut *self.game};

        if self.selected {
            game.use_finger=false;
        }
        match self.player {
            Turn::Player1 => {self.score = game.get_player_1_score(); self.active= game.current_turn==Turn::Player1;}
            Turn::Player2 => {self.score = game.get_player_2_score(); self.active= game.current_turn==Turn::Player2;}
        }
        let x_y = self.correct_coords(dims);
        if debug {
            render::draw_pp_texture(x_y.0, x_y.1, &self.get_debug_asset_data(), canvas, sf, &textures);
        }
        render::draw_pp_texture(x_y.0, x_y.1, &self.get_asset_data(), canvas, sf, &textures);
        for x in 0..self.score {
            if debug {
                render::draw_pp_texture(x_y.0+26, 16 * x as i32 + (x_y.1 + 30), {
                    let mut debug_thing = self.get_debug_asset_data();
                    debug_thing.uv = self.asset_data_score.uv;
                    &debug_thing.clone()
                }, canvas, sf, textures);
            }
            render::draw_pp_texture(x_y.0+26, 16 * x as i32 + (x_y.1 + 30), &self.asset_data_score, canvas, sf, textures);
        }

    }
}

impl PlayerWidget {
    pub fn create(alignment: Alignment, x : i32, y : i32, player : Turn ,game : *mut Game) -> Box<Self>
    where
        Self: Sized
    {

        let ret = Self {
            active: false,
            selected: false,
            asset_data : AssetData::empty(),
            asset_data_score: AssetData {
                uv: Some(Rect::new(0, 0, 16, 16)),
                origin: (0, 0),
                resource_location: ResourceLocation::new("memory_game", "gui/score_indicator.png"),
            },
            alignment,
            coords: (x, y),
            game,
            player,
            score: 0
        };
        Box::new(ret)
    }
}