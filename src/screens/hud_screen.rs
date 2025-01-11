use std::collections::HashMap;
use sdl2::event::Event;
use sdl2::render::{Texture, WindowCanvas};
use crate::game::{Game, Turn};
use crate::screen::Screen;
use crate::widget::{Alignment, Widget};
use crate::widgets::crown_widget::CrownWidget;
use crate::widgets::end_screen_quit_widget::EndQuitWidget;
use crate::widgets::play_again_widget::PlayAgainWidget;
use crate::widgets::play_widget::PlayWidget;
use crate::widgets::player_widget::PlayerWidget;
use crate::widgets::score_widget::ScoreWidget;

pub struct HudScreen {
    game : *mut Game,
    widgets : Vec<Vec<Box<dyn Widget>>>,
}

impl Screen for HudScreen {
    fn get_widgets(&mut self) -> &mut Vec<Vec<Box<dyn Widget>>> {
        &mut self.widgets
    }

    fn get_game(&mut self) -> *mut Game {
        self.game
    }

    fn set_game(&mut self, game: *mut Game) {
        self.game = game;
    }

    fn create(game: &mut Game) -> Box<Self>
    where
        Self: Sized
    {
        let mut ret = Self{
            widgets: vec![],
            game,
        };
        //ret.add_widget(ScoreWidget::create(Alignment::TOP, 0, 0, game), 0, 0);

        // Add the widgets to show the players current turn
        ret.add_widget(PlayerWidget::create(Alignment::LEFT, 20, 80, Turn::Player1, game),0,0);
        ret.add_widget(PlayerWidget::create(Alignment::RIGHT, -88, 80, Turn::Player2, game),0,0);

        // Add the widgets that only show once the game is over
        ret.add_widget(PlayAgainWidget::create(Alignment::LEFT, 20, -110, game),0,0);
        ret.add_widget(EndQuitWidget::create(Alignment::RIGHT, -75, -110, game),0,0);

        // Add two crowns that will display when the respective player wins
        ret.add_widget(CrownWidget::create(Alignment::LEFT, 54, 90, game, Turn::Player1), 0,0);
        ret.add_widget(CrownWidget::create(Alignment::RIGHT, -54, 90, game, Turn::Player2), 0,0);
        Box::new(ret)
    }


}
