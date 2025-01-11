use std::collections::HashMap;
use log::{error, warn};
use sdl2::rect::Rect;
use crate::game::Game;
use crate::render::AssetData;
use crate::resource_location::ResourceLocation;
use crate::widget::Alignment;
use crate::widget::Widget;

pub struct ErrWidget {
    selected : bool,
    asset_data: AssetData,
    asset_data_selected : AssetData,
    alignment: Alignment,
    coords : (i32, i32),
    game : Option<*mut Game>
}

impl ErrWidget {
    pub fn empty() -> Self {
        Self {
            selected : false,
            asset_data : AssetData::empty(),
            asset_data_selected : AssetData::empty(),
            alignment : Alignment::NONE,
            coords : (0,0),
            game: None,
        }
    }

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
}

impl ErrWidget {
    pub fn create(alignment: Alignment, x : i32, y : i32, game : *mut Game) -> Box<Self>
    where
        Self: Sized
    {

        let ret = Self {
            selected: false,
            asset_data: AssetData {
                uv: Some(Rect::new(0, 0, 20, 20)),
                origin: (0, 0),
                resource_location: ResourceLocation::new("game", "gui/widgets/err_widget.png"),
            },
            asset_data_selected: AssetData {
                uv: Some(Rect::new(0, 20, 20, 20)),
                origin: (0, 0),
                resource_location: ResourceLocation::new("game", "gui/widgets/err_widget.png"),
            },
            alignment,
            coords: (x, y),
            game : Some(game)
        };
        Box::new(ret)
    }
}

impl Widget for ErrWidget {
    fn on_click(&mut self) {
        error!("you flipped up buddy");
    }

    fn get_selected(&mut self) -> bool {
        self.selected
    }

    fn set_selected(&mut self, tf : bool) {
        self.selected = tf;
    }

    fn get_screen_coordinates(&mut self) -> (i32, i32) {
        self.coords.clone()
    }

    fn set_screen_coordinates(&mut self, x: i32, y: i32) {
        self.coords = (x, y)
    }

    fn get_asset_data(&mut self) -> AssetData {
        if self.selected {
            self.asset_data_selected.clone()
        }
        else {
            self.asset_data.clone()
        }
    }

    fn set_asset_data(&mut self, ass: AssetData) {
        self.asset_data = ass
    }

    fn get_resource_location(&mut self) -> ResourceLocation {
        ResourceLocation::new("game", "widgets/err_widget")
    }

    fn get_allignment(&mut self) -> Alignment {
        self.alignment.clone()
    }

    fn set_allignment(&mut self, alignment: Alignment) {
        self.alignment = alignment;
    }

    fn get_game(&mut self) {
        self.game.unwrap();
    }

}
