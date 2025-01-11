use std::fmt::Pointer;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use log::warn;
use sdl2::event::EventPollIterator;
use sdl2::rect::Rect;
use crate::game::Game;
use crate::render;
use crate::render::{AssetData};
use crate::resource_location::ResourceLocation;

/// a trait containing all basic functions an entity will need.
pub trait Entity {

    /// Returns whether an entity will be one that can move about or not
    fn is_static(&self) -> bool {
        false
    }

    /// returns the entity's coordinates
    #[must_use]
    fn get_coords(&mut self) -> (f32, f32);

    /// sets the entity's coordinates
    #[must_use]
    fn set_coords(&mut self, coords : (f32, f32));

    /// returns the entity's current health
    #[must_use]
    fn get_health(&mut self) -> f32;

    /// changes the entity's health by a given amount, positive or negative
    /// # Example
    /// ```
    /// let mut player = game.get_player()
    /// player.change_health(-15f32);
    /// ```
    #[must_use]
    fn change_health(&mut self, amount : f32);

    fn set_resource_location(&mut self, rl : ResourceLocation) {

    }

    /// Implement this function to add extra functionality to an entity, a good example in [`Turret`]
    fn tick(&mut self, delta : f32) {}

    #[must_use]
    fn get_resource_location(&self) -> &ResourceLocation;

    /// returns the entity's position in the entity list
    #[must_use]
    fn get_index(&self) -> usize;

    /// gets the entity's velocity
    #[must_use]
    fn get_velocity(&mut self) -> (f32,f32);

    /// Set the entity's velocity
    #[must_use]
    fn set_velocity(&mut self, velocity : (f32, f32));



    /// Applies the velocity to an entity every frame. Create an implementation for [`tick`] to add additional functionality.
    ///
    /// [`tick`]: Entity::tick
    fn physics(&mut self, delta : f32) {

        // if the current entity doesn't move, skip this
        if !self.is_static()
        {
            let x = self.get_coords().0 + self.get_velocity().0 * delta;
            let y = self.get_coords().1 + self.get_velocity().1 * delta;

            self.set_coords((x, y))
        }
        // run any additional code
        self.tick(delta)
    }

    /// converts world space coordinates to screen coordinates.
    fn screen(&mut self, player_coords :  (f32, f32)) -> (i32, i32) {
        let x = self.get_coords().0;
        let y = self.get_coords().1;
        let px = player_coords.0 as i32;
        let py = player_coords.1 as i32;
        let half_x = (render::DIMENSIONS.0 / 2) as i32;
        let half_y = (render::DIMENSIONS.1 / 2) as i32;

        // if the player coordinates are negative, then the rounding would be incorrect, as values such as -1.4 would round to -1, and not -2 as desired for the rendering to be correct.
        //let px = (if player_coords.0 < 0.0 {player_coords.0 - 1.0} else { player_coords.0 }) as i32;
        //let py = (if player_coords.1 < 0.0 {player_coords.1 - 1.0} else { player_coords.1 }) as i32;

        // the conversion for the different coordinate spaces
        (((half_x - px) as f32 + x) as i32, ((half_y - py ) as f32 + y) as i32)
    }

    /// Returns the asset data of the entity
    fn get_asset_data(&mut self) -> AssetData {
        warn!("No asset data provided for {}", self.get_resource_location().to_string());
        AssetData::empty()
    }

    /// Change the asset data of an entity
    fn set_asset_data(&mut self, ass : AssetData) {

    }


}

