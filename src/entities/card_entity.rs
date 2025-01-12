use std::cmp::PartialEq;
use log::info;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use uuid::Uuid;
use crate::entity::Entity;
use crate::game::{Game, Turn};
use crate::render::AssetData;
use crate::resource_location::ResourceLocation;
use crate::utils::create_uuid;

/// The entity type for cards
pub struct CardEntity {
    coords: (f32, f32),
    pub asset_data: AssetData,
    back_texture : AssetData,
    // hitbox : matrix,
    velocity : (f32, f32),
    uuid : Uuid,
    game : Option<*mut Game>,
    health : f32,
    resource_location: ResourceLocation,
    index : usize,
    hover : bool,
    prev_hover : bool,
    selected : bool,
    x: f32,
    y : f32,
    card: Card,
    turn: Turn,
    success : bool,
    prev_selected : u8
}



impl Entity for CardEntity {
    fn get_coords(&mut self) -> (f32, f32) {
        self.coords
    }

    fn set_coords(&mut self, coords: (f32, f32)) {
        self.coords = coords
    }

    fn get_health(&mut self) -> f32 {
        self.health
    }

    fn change_health(&mut self, amount: f32) {
        todo!()
    }

    fn set_resource_location(&mut self, rl: ResourceLocation) {
        self.resource_location = rl
    }

    fn tick(&mut self, delta: f32) {

        // get the game instance
        let game = unsafe { &mut *self.game.unwrap() };
        // get the player, which in this game is a dummy entity representing the camera
        let mut player : &mut dyn Entity;
        if game.get_player().is_some() {
            player = game.get_player().unwrap().get_mut().unwrap()
        }
        else {
            return;
        }

        // get the screen coordinates of the entity
        let screen = self.screen(player.get_coords());

        // if previous selected count wasn't 0 and game selected count is 0 now,
        if self.prev_selected!=0  && game.selected_count ==0 {
            // if this card was selected last frame, and the previous turn was a success
            if game.prev_success && self.selected {
                self.success = true; // set success to true
            }
            self.hover = false; // reset hover
            self.selected = false; // reset selected
            //self.turn = self.turn.swith();
        }
        self.prev_selected = game.selected_count; // previous selected is set for next frame
        if self.success {
            self.coords = (self.x, self.y); // if successfully picked, be locked into the default card position
            return;
        }

        // check if mouse is hovering over
        if (screen.0-23..screen.0+22).contains(&(game.mouse.0 as i32)) && (screen.1-34..screen.1+(if self.hover {42} else {34})).contains(&(game.mouse.1 as i32)) {
            self.hover = true; // if it is, set hover state to true
        }
        else {
            self.hover = false; // if not set it to false, in case it was last frame.
        }
        // if hovering and not selected
        if self.hover && !self.selected {
            for events in game.events.clone() {
                match events {
                    Event::MouseButtonDown { // check for a left click
                        mouse_btn: MouseButton::Left,
                        ..
                    } => {
                        self.selected = true; // set selected to true
                        game.play_sound(ResourceLocation::new("memory_game", "sounds/flip.ogg"));
                        if game.selected_count==0 { // if none selected
                            game.selected_count+=1; // increment selected counter
                            game.selected_cards.0 = self.card.clone() // set the first value to this card
                        }
                        else if game.selected_count==1 { // if one already selected
                            game.selected_count+=1; // increment selected counter
                            game.selected_cards.1 = self.card.clone() // set second value to this card
                        }
                        else {
                            // nothing
                        }
                    },
                    _ => {}
                }
            }
        }
        if self.selected { // if selected, hover
            self.hover = true;
        }
        if self.hover { // if hovering, offset the card upwards and set the mouse to the finger
            self.coords = (self.x, self.y - 8.0);
            if !self.selected {
                game.use_finger = true;
            }
        }
        else { // otherwise set to default coordinates, incase it was hovered last frame.
            self.coords = (self.x, self.y)
        }
        if self.hover && !self.prev_hover {
            game.play_sound(ResourceLocation::new("memory_game", "sounds/pop.ogg"))
        }
        self.prev_hover = self.hover
    }

    fn get_resource_location(&self) -> &ResourceLocation {
        &self.resource_location
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_velocity(&mut self) -> (f32, f32) {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: (f32, f32)) {
        self.velocity = velocity
    }

    fn get_asset_data(&mut self) -> AssetData {

        // if the card is selected or successfully picked, display front texture
        if self.selected || self.success {
            self.asset_data.clone()
        }
        // otherwise use the default back texture
        else {
            self.back_texture.clone()
        }
    }

    fn set_asset_data(&mut self, ass: AssetData) {
        self.asset_data = ass;
    }
}

impl CardEntity {

    /// Create a card entity instance
    pub fn create(game : Option<*mut Game>, ass : AssetData, x : f32, y : f32, card: Card) -> CardEntity {

        let uuid = create_uuid();

        Self {
            coords: (x, y),
            asset_data: ass,
            back_texture: AssetData {
                uv: Some(Rect::new(0,0,45,68)),
                origin: (23, 34),
                resource_location: ResourceLocation {
                    namespace: "memory_game".to_string(),
                    path: "cards/card_reverse.png".to_string(),
                },
            },
            velocity: (0.0, 0.0),
            uuid,
            game,
            health: 0.0,
            resource_location: ResourceLocation::empty(),//card.resource_location,
            index : 0,
            hover : false,
            prev_hover: false,
            selected: false,
            x,
            y,
            card,
            turn: Turn::Player1,
            success: false,
            prev_selected : 0
        }
    }
    pub fn set_game(&mut self, game : *mut Game) {
        self.game = Some(game)
    }
}

/// The card data type
pub struct Card {
    name : String, // The name of the card
    resource_location: ResourceLocation, // The resource location of the card data file
    texture: ResourceLocation // The resource location of the card texture
}
impl Card {

    ///Create a new card instance
    pub fn create(name: String, resource_location: ResourceLocation, texture : ResourceLocation) -> Self {
        Self{
            name,
            resource_location,
            texture
        }
    }

    /// Get the card data resource location
    pub fn get_resource_location(&self) -> ResourceLocation {
        self.resource_location.clone()
    }
    /// Get the card name
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Get the card texture
    pub fn get_texture_location(&self) -> ResourceLocation {
        self.texture.clone()
    }

    /// Create an empty card instance
    pub fn empty() -> Self{
        Self {
            name: "Blank Card".to_string(),
            resource_location: ResourceLocation::empty(),
            texture: ResourceLocation::new("memory_game", "memory_game:cards/card_base.png"),
        }
    }
 }

// Implement clone for card
impl Clone for Card {
    fn clone(&self) -> Self {
        Self{
            name : self.name.clone(),
            resource_location : self.resource_location.clone(),
            texture : self.texture.clone()
        }
    }
}