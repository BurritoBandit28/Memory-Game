use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::ops::DerefMut;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;
use chrono::Month;
use rodio::{Decoder, OutputStream, source::Source, OutputStreamHandle};
use log::{info, warn};
use num::bigint::U32Digits;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::keyboard::Keycode::C;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use crate::entity::{Entity};
use crate::level::{Level, TileGraph};
use crate::{entities, render, sound};
use crate::entities::card_entity::{Card, CardEntity};
use crate::entities::dummy::DummyEntity;
use crate::game::Turn::Player2;
use crate::render::{draw_pp_texture, AssetData};
use crate::resource_location::ResourceLocation;
use crate::screen::Screen;
use crate::sound::{AudioManager, Sound};
use crate::tile::{Tile, TileSize};
use crate::utils::order_sort;
use crate::widget::{Alignment, Widget};
use crate::widgets::play_widget::PlayWidget;

/// An object that manages a game instance. It holds all game data and manages the render, physics and screen loops
pub struct Game {
    pub entities: Vec<Box<Mutex<dyn Entity>>>, // new (uses traits) (better)
    pub player : Option<usize>,
    pub events: Vec<Event>,
    pub held_keys : Vec<Scancode>,
    pub running : bool,
    pub current_level : Option<Level>,
    pub current_screen : Option<Box<dyn Screen>>,
    pub tiles :  HashMap<String, Tile>,
    pub sounds : HashMap<String, Sound>,
    pub draw_mouse : bool,
    pub sf : i32,
    pub use_finger : bool,
    pub dims : (u32,u32),
    pub score : f32,
    debug : bool,
    pub mouse : (u32, u32),
    pub cards : HashMap<String, Card>,
    pub selected_count : u8,
    pub selected_cards : (Card, Card),
    pub current_turn : Turn,
    pub wait_timer : f32,
    pub prev_success : bool,
    player_1_score : u32,
    player_2_score : u32,
    audio_manager: AudioManager
}

#[derive(Debug)]
pub enum Turn {
    Player1,
    Player2
}

impl Turn {
    pub(crate) fn clone(&self) -> Turn {
        match self {
            Turn::Player1 => {Self::Player1}
            Turn::Player2 => {Self::Player2}
        }
    }
}

impl Turn {
    pub fn swith(&self) -> Self{
        match self {
            Turn::Player1 => {Turn::Player2}
            Turn::Player2 => {Turn::Player1}
        }
    }
}

impl PartialEq for Turn {
    fn eq(&self, other: &Self) -> bool {
        let mut bl1 = false;
        match other {
            Turn::Player1 => {bl1 = true;}
            Turn::Player2 => {}
        }
        let mut bl2 = false;
        match self {
            Turn::Player1 => {bl2 = true;}
            Turn::Player2 => {}
        }
        (!bl1 && !bl2) || (bl1 && bl2)
    }
}

impl Game {

    /// Physics and inputs
    pub fn cycle(&mut self, delta : f32, mousex : u32, mousey : u32, dims : (u32, u32)) {

        if self.selected_count ==2 {
            info!("{:?} picked {} and {}", self.current_turn, self.selected_cards.0.get_name(), self.selected_cards.1.get_name());
            self.current_turn = self.current_turn.swith();
            self.selected_count = 0;
            self.prev_success = self.selected_cards.0.get_name() == self.selected_cards.1.get_name();
            self.selected_cards = (Card::empty(), Card::empty());
            self.wait_timer = 2.0;
            if self.prev_success {
                self.current_turn = self.current_turn.swith();
            }
            info!("{:?}'s turn", self.current_turn)
        }

        if !self.entities.is_empty() {
            self.score += delta;
        }

        if self.wait_timer < 0.0 {
            // Run physics for every entity
            for entity in self.entities.iter() {
                entity.lock().unwrap().physics(delta)
            }
            if self.prev_success {
                match self.current_turn {
                    Turn::Player1 => {self.player_1_score+=1;}
                    Turn::Player2 => {self.player_2_score+=1;}
                }
                if self.player_1_score + self.player_2_score == 9 {
                    if self.player_1_score > self.player_2_score {
                        info!("Player 1 wins!")
                    }
                    else {
                        info!("Player 2 wins!")
                    }

                }
            }
            self.prev_success = false;
        }
        else {
            self.wait_timer -= delta;
        }

        // if there is a current screen, run its cycle function
        let _ = if self.current_screen.is_some() {
            self.current_screen.as_mut().unwrap().cycle(mousex, mousey, dims, self.events.clone())
        };

        // handle user inputs
        for event in self.events.clone() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    info!("Quitting game!");
                    // close game on Escape, or app closure
                    self.running=false
                },
                Event::KeyDown {
                    keycode: Some(Keycode::F3),
                    ..
                } => {
                    self.debug= !self.debug
                },
                Event::MouseButtonDown {
                    mouse_btn : MouseButton::Left,
                    ..
                } => {
                    // on left click, check if the mouse is over a widget, if so, execute its on_click function
                    if self.current_screen.is_some() {
                        for wl in self.current_screen.as_mut().unwrap().get_widgets() {
                            for w in wl {
                                if w.get_selected() {
                                    w.on_click()
                                }
                            }
                        }
                    }
                }

                _ => {
                    // do nothing
                }
            }
        }
    }

    /// Returns the entity assigned as the "player", may not always be [`Player`]
    ///
    /// [`Player`]: player::Player
    pub fn get_player(&mut self) -> Option<&mut Box<Mutex<dyn Entity>>> {
        self.entities.get_mut(self.player.unwrap())
    }

    pub fn create_memory_game_scene(&mut self) {
        self.current_turn = Turn::Player1;
        self.selected_count = 0;
        self.selected_cards = (Card::empty(), Card::empty());
        self.entities = vec![];
        self.prev_success = false;
        self.wait_timer = -1.0;
        self.player_1_score = 0;
        self.player_2_score = 0;
        let mut positions: Vec<(f32, f32)> = vec![
            // Column 1 (Far Left)
            (-104.0, -70.0), (-104.0, 0.0), (-104.0, 70.0),
            // Column 2 (Left)
            (-56.0, -70.0), (-56.0, 0.0), (-56.0, 70.0),
            // Column 3 (Left Center)
            (-8.0, -70.0), (-8.0, 0.0), (-8.0, 70.0),
            // Column 4 (Right Center)
            (40.0, -70.0), (40.0, 0.0), (40.0, 70.0),
            // Column 5 (Right)
            (88.0, -70.0), (88.0, 0.0), (88.0, 70.0),
            // Column 6 (Far Right)
            (136.0, -70.0), (136.0, 0.0), (136.0, 70.0)
        ];
        let mut dummy_player = DummyEntity::create(self, {
            let mut ass = AssetData::empty();
            ass.resource_location = ResourceLocation::new("memory_game", "empty.png");
            ass
        });
        dummy_player.set_coords((0.0, 0.0));
        self.entities.push(Box::new(Mutex::new(dummy_player)));
        self.player = Some(self.entities.len() -1 );



        let mut card_asset_base = AssetData {
            uv: Some(Rect::new(0,0,45,68)),
            origin: (23, 34),
            resource_location: ResourceLocation {
                namespace: "memory_game".to_string(),
                path: "cards/card_reverse.png".to_string(),
            },
        };
        let mut cards = vec![];

        let mut counter = 0;
        while counter < self.cards.len()*2 {
            for (_,card) in &self.cards {

                let index = rand::thread_rng().gen_range(0..(self.cards.len())*2-counter);

                let pos = positions.get(index).unwrap().clone();
                positions.remove(index);
                let mut card = CardEntity::create(None, {
                    let mut ass = card_asset_base.clone();
                    ass.resource_location = card.get_texture_location();
                    ass
                }, pos.0, pos.1, card.clone());
                cards.push(card);
                counter +=1;
            }
        }
        for mut card in cards {
            card.set_game(self);
            self.entities.push(Box::new(Mutex::new(card)))
        }
        self.current_level = Some(Level::create_scene_background(&self.tiles));
    }

    pub fn get_player_1_score(&mut self) -> u32 {
        self.player_1_score
    }
    pub fn get_player_2_score(&mut self) -> u32 {
        self.player_2_score
    }

    /// The render loop for entities, screens and the mouse. The entity rendering is done here, for specifics on other elements see the render functions for [`Screens`]|[`Levels/Tiles`]|[`Widgets`]
    ///
    /// [`Screens`]: Screen::render
    /// [`Levels/Tiles`]: Level::render
    /// [`Widgets`]: Widget::render
    pub fn render(&mut self, canvas: &mut WindowCanvas, sf: i32, textures : &HashMap<String, Texture>, dims : (u32, u32), mousex : u32, mousey : u32) {

        // if there are entities, render them to screen
        if !self.entities.is_empty() {

            // calculate the order the entities are rendered in - it is essential that the order of entities in the list isn't changed
            let order = order_sort(&mut self.entities);

            // get the player and its coordinates
            let player = self.get_player().unwrap();
            let player_coords = player.lock().unwrap().get_coords();

            // get the level
            let mut level = &mut self.current_level;
            // make sure the level isn't None, and render it to screen
            if level.is_some() {
                level.as_mut().unwrap().render(player_coords, textures, canvas, sf, self.debug);
            }

            // iterate through the order
            for x in order {
                let mut list = &mut self.entities;
                let mut obj = list.get(x.1).unwrap().lock().unwrap();
                let screen_coords = &obj.screen(player_coords);
                let asset_data = &obj.get_asset_data();
                draw_pp_texture(screen_coords.0, screen_coords.1, &asset_data, canvas, sf, textures);
            }
        }

        // get the current screen
        let scrn = &mut self.current_screen;
        // make sure the screen isn't None, and render it to screen
        if scrn.is_some() {
            scrn.as_mut().unwrap().render(textures, sf, canvas, dims, self.debug);
        }

        // draw the mouse, unless instructed otherwise
        if self.draw_mouse {
            if self.use_finger {
                draw_pp_texture(
                    mousex as i32,
                    mousey as i32,
                    &render::get_icons().lock().unwrap().get("finger").unwrap(),
                    canvas,
                    sf,
                    &textures
                );
            }
            else {
                draw_pp_texture(
                    mousex as i32,
                    mousey as i32,
                    &render::get_icons().lock().unwrap().get("cursor").unwrap(),
                    canvas,
                    sf,
                    &textures
                );
            }
        }

    }

    /// Plays a sound file given a [`ResourceLocation`]
    pub fn play_sound(&self, resource_location : ResourceLocation) {
        // get sound from mao
        let sound  = self.sounds.get(&resource_location.to_string());
        // if the sound exists, play it
        if sound.is_some() {
            self.audio_manager.play_sound(sound.unwrap())
        }
        // else, warn in the logs.
        else {
            warn!("Sound {} not found!", resource_location.to_string())
        }

    }

    /// Create a [`Game`] instance
    pub fn initiate() -> Self {
        
        Self{
            entities: vec![],
            player : None,
            events: vec![],
            held_keys : vec![],
            running : true,
            current_level : None,
            current_screen : None,
            tiles: Default::default(),
            sounds : Default::default(),
            draw_mouse : true,
            sf : 6,
            use_finger : false,
            dims: (0,0),
            score: 0.0,
            debug : false,
            mouse: (0, 0),
            cards: Default::default(),
            selected_count: 0,
            selected_cards : (Card::empty(), Card::empty()),
            current_turn : Turn::Player1,
            wait_timer : -1.0,
            prev_success: false,
            player_1_score : 0,
            player_2_score : 0,
            audio_manager : AudioManager::create()
        }
        
    }

    pub fn get_turn(&mut self) -> Turn {
        self.current_turn.clone()
    }
    
}

