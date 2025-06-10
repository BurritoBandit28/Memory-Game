mod entities;
mod entity;
mod game;
mod render;
mod utils;
mod level;
mod tile;
mod resource_location;
mod screen;
mod widgets;
mod screens;
mod widget;
mod sound;

use crate::sound::Sound;
use std::collections::HashMap;
use std::{env, fs};
use std::fs::File;
use std::hash::Hash;
use std::io::BufReader;
use std::ops::DerefMut;
use std::path::Path;
use crate::screens::main_menu_screen;
use crate::entity::Entity;
use crate::game::Game;
use num::clamp;
use render::TARGET_DIMENSIONS;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Instant;
use gjson::Value;
use log::info;
use sdl2::event::Event::KeyDown;
use sdl2::render::Texture;
use walkdir::WalkDir;
use resource_location::ResourceLocation;
use widget::Widget;
use crate::entities::card_entity::{Card, CardEntity};
use crate::level::Level;
use crate::render::AssetData;
use crate::screen::Screen;
use crate::tile::{Tile, TileSize, TileType};
use crate::widget::Alignment;
use crate::screens::main_menu_screen::MainMenuScreen;


#[cfg(target_os = "emscripten")]
pub mod emscripten;

#[cfg(target_os = "emscripten")]
macro_rules! info {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);

        std::println!("{}", message);

    }};
}

fn main() {

    // initial set up

    // start logger
    utils::init_logger();

    info!("Initialising SDL2");

    // start SDL2
    let sdl_ctx = sdl2::init().unwrap();
    let scale_factor =
        sdl_ctx.video().unwrap().current_display_mode(0).unwrap().w / TARGET_DIMENSIONS.0 as i32;
    let video_subsys = sdl_ctx.video().unwrap();

    // the actual dimensions of the game screen, as they won't always be 320/180 due to differing aspect ratios
    let dims = ((sdl_ctx.video().unwrap().current_display_mode(0).unwrap().w / scale_factor) as u32, (sdl_ctx.video().unwrap().current_display_mode(0).unwrap().h / scale_factor) as u32);

    println!("{:?}", dims);
    render::get_actual_dimensions().lock().unwrap().set(dims);

    let scale_offset = (sdl_ctx.video().unwrap().current_display_mode(0).unwrap().h / scale_factor as i32) - 180;
    //todo : get the difference in height and display 2 black bars to give 16:9 ratio screen

    //hide mouse
    sdl_ctx.mouse().show_cursor(false);

    // create window
    #[cfg(not(target_os = "emscripten"))]
    let mut window = video_subsys
        .window("Memory Game", TARGET_DIMENSIONS.0, TARGET_DIMENSIONS.1)
        .vulkan()
        .fullscreen_desktop()
        .build()
        .unwrap();

    // create window
    #[cfg(target_os = "emscripten")]
    let mut window = video_subsys
        .window("Memory Game", TARGET_DIMENSIONS.0, TARGET_DIMENSIONS.1)
        .opengl() // emscripten doesnt support vulkan
        .fullscreen_desktop()
        .build()
        .unwrap();

    info!("complete");

    // lock mouse to window
    &window.set_mouse_grab(true);

    //create canvas
    let mut canvas = &mut window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    // counter to count how many objects are loaded for the debug logs
    let mut counter = 0;

    // initialise textures - new
    info!("Loading textures...");
    // create hashmap
    let mut textures : HashMap<String, Texture> = HashMap::new();
    // iterate through the assets directory
    for dir in WalkDir::new("./assets/") {
        let mut path = String::from(dir.unwrap().path().to_str().unwrap()).replace("\\", "/");
        // if the file is an image, save it - in future there will likely be a hashmap for other files, like animation data or other bits idk yet
        if path.clone().to_lowercase().ends_with(".png") {
            // create the resource location
            let mut rl = ResourceLocation::empty();

            // split the path by \s
            let split : Vec<_> = path.split("/").collect();

            // name space is in ./assets/>>namespace<<, so it is the third element in the list
            let namespace = &split[2];
            rl.set_namespace(namespace.to_string());

            // the path is just everything after the namespace
            let path = path.split(format!("/{}/", namespace).as_str()).collect::<Vec<_>>()[1];
            rl.set_path(path.to_string());

            //load the texture
            let texture = texture_creator.load_texture(format!("./assets/{}/{}", namespace, path).as_str());

            // insert the hashmap
            textures.insert(rl.clone().to_string(), texture.unwrap());

            info!("Loaded texture : {}", rl.to_string());
            counter+=1;
        }
    }
    info!("{} textures loaded!", counter);

    counter = 0;

    info!("Loading sounds...");
    // create hashmap
    let mut sounds : HashMap<String, Sound> = HashMap::new();

    // iterate through the assets directory
    for dir in WalkDir::new("./assets/") {
        let mut path = String::from(dir.unwrap().path().to_str().unwrap()).replace("\\", "/");
        // if the file is a sound, save it
        if path.clone().to_lowercase().ends_with(".ogg") {
            // create the resource location
            let mut rl = ResourceLocation::empty();

            // split the path by \s
            let split : Vec<_> = path.split("/").collect();

            // name space is in ./assets/>>namespace<<, so it is the third element in the list
            let namespace = &split[2];
            rl.set_namespace(namespace.to_string());

            // the path is just everything after the namespace
            let path = path.split(format!("/{}/", namespace).as_str()).collect::<Vec<_>>()[1];
            rl.set_path(path.to_string());

            //load the sound
            let sound = Sound {
                path : format!("assets/{}/{}", namespace, path),
                resource_location: rl.clone(),
            };

            // insert the hashmap
            sounds.insert(rl.clone().to_string(), sound);


            info!("Loaded sound : {}", rl.to_string());
            counter+=1;
        }
    }
    info!("{} sounds loaded!", counter);

    counter = 0;
    // entirely data driven tile system

    // initialise tiles
    info!("Loading tiles...");

    //create hashmap
    let mut tiles: HashMap<String, Tile> = HashMap::new();
    // get the immediate subdirectories for the name spaces
    let namespaces = fs::read_dir("./data").unwrap();

    // iterate through the namespaces
    for namepath in namespaces {

        // get the actual namespace
        let mut namespace = String::from(namepath.unwrap().path().to_str().unwrap()).replace("\\", "/");
        //                                            .\data\>>namespace<<
        namespace = namespace.split("/").collect::<Vec<_>>()[2].to_string();

        if !namespace.clone().contains(".") {

            for dir in WalkDir::new(format!("./data/{}/tiles/", namespace.clone())) {
                let path = String::from(dir.unwrap().path().to_str().unwrap()).replace("\\", "/");
                // if the file is tile data, continue
                if path.clone().to_lowercase().ends_with(".json") {
                    // get the json file as a string
                    let json = fs::read_to_string(path.clone()).unwrap();

                    // read the values from the json file

                    if gjson::get(json.as_str(), "type").to_string() != "tile" {
                        continue;
                    };

                    // "name" : string
                    let name = gjson::get(json.as_str(), "name");

                    // the ResourceLocation of this JSON file
                    let resource_location = ResourceLocation::new(
                        &*namespace.clone(),
                        path.split(format!("/{}/", namespace).as_str()).collect::<Vec<_>>()[1]);

                    // "resource_location" : string
                    let texture = ResourceLocation::parse(
                        gjson::get(json.as_str(), "resource_location")
                        .to_string());

                    // "uv" {"x" : int,  "y" : int}
                    let uv : (u32, u32) = (
                        gjson::get(json.as_str(), "uv.x").to_string().parse::<u32>().unwrap(),
                        gjson::get(json.as_str(), "uv.y").to_string().parse::<u32>().unwrap()
                    );

                    // "type" : string    - might remove this bit as it may not be needed
                    let ttype = TileType::parse(gjson::get(json.as_str(), "type").to_string());

                    // "size" : string
                    let size = TileSize::parse(gjson::get(json.as_str(), "size").to_string().as_str());

                    // "origin" {"x" : int ,  "y" :  int }  - at the moment it refers to where the centre point of the sprite is,
                    //                                  but will be changed to be the centre point of the hitbox, as the sprites
                    //                                  should all have the origin of (0,0) to render correctly
                    let origin : (i32, i32) = (
                        gjson::get(json.as_str(), "origin.x").to_string().parse::<i32>().unwrap(),
                        gjson::get(json.as_str(), "origin.y").to_string().parse::<i32>().unwrap()
                    );

                    // "collision" : bool
                    let collision : bool = gjson::get(json.as_str(), "collision").to_string().parse::<bool>().unwrap();
                    let mut collison_box : Option<(u32,u32)>;

                    // "collision_box" {"x" : int ,  "y" :  int }
                    if collision {
                        collison_box = Some((
                            gjson::get(json.as_str(), "collision_box.x").to_string().parse::<u32>().unwrap(),
                            gjson::get(json.as_str(), "collision_box.y").to_string().parse::<u32>().unwrap()
                        ));
                    }
                    else {
                        collison_box = None
                    }

                    // create the tile and append it to the hashmap
                    let tile = Tile::create(name.to_string(), resource_location.clone(), texture, uv, ttype, size, origin, collision, collison_box);
                    tiles.insert(resource_location.to_string(), tile);

                    info!("Loaded tile : {}", resource_location.to_string());

                    counter += 1;
                }

            }
        }
    }

    info!("{} tiles loaded!", counter);

    counter = 0;
    // entirely data driven card system

    // initialise cards
    info!("Loading cards...");

    //create hashmap
    let mut cards: HashMap<String, Card> = HashMap::new();
    // get the immediate subdirectories for the name spaces
    let namespaces = fs::read_dir("./data").unwrap();

    // iterate through the namespaces
    for namepath in namespaces {

        // get the actual namespace
        let mut namespace = String::from(namepath.unwrap().path().to_str().unwrap()).replace("\\", "/");
        //                                            .\data\>>namespace<<
        namespace = namespace.split("/").collect::<Vec<_>>()[2].to_string();

        if !namespace.clone().contains(".") {

            for dir in WalkDir::new(format!("./data/{}/cards/", namespace.clone())) {
                let path = String::from(dir.unwrap().path().to_str().unwrap()).replace("\\", "/");
                // if the file is tile data, continue
                if path.clone().to_lowercase().ends_with(".json") {
                    // get the json file as a string
                    let json = fs::read_to_string(path.clone()).unwrap();

                    // read the values from the json file

                    if gjson::get(json.as_str(), "type").to_string() != "card" {
                        continue;
                    };

                    // "name" : string
                    let name = gjson::get(json.as_str(), "name");

                    // the ResourceLocation of this JSON file
                    let resource_location = ResourceLocation::new(
                        &*namespace.clone(),
                        path.split(format!("/{}/", namespace).as_str()).collect::<Vec<_>>()[1]);

                    // "resource_location" : string
                    let texture = ResourceLocation::parse(
                        gjson::get(json.as_str(), "resource_location")
                            .to_string());

                    // create the card and append it to the hashmap
                    let card = Card::create(name.to_string(), resource_location.clone(), texture.clone());
                    cards.insert(resource_location.to_string(), card);

                    info!("Loaded card : {}", resource_location.to_string());

                    counter += 1;
                }

            }
        }
    }

    info!("{} cards loaded!", counter);


    // register event pump to handle inputs
    let mut event_pump = sdl_ctx.event_pump().unwrap();

    canvas.present();

    info!("Initiating game controller");

    let mut game = Game::initiate();

    // register extra values to the game
    game.sf = scale_factor;
    game.current_screen = Some(MainMenuScreen::create(&mut game));

    // append hashmaps to game instance
    game.tiles = tiles;
    game.sounds = sounds;
    game.dims = dims;
    game.cards = cards;

    /// Delta refers to the time taken between showing two frames. This value is often used for physics related operations, as this allows the simulation to not be affected by the frame rate of the computer.
    /// You can see that happening in [`Entity::physics`]
    // initiate delta
    let mut delta: f32 = 0.0;

    // load the test level
    //game.current_level = Some(Level::create_test_level(&tiles));

    info!("Game instance initiated!");


    let mut main_loop = || {

        if game.running {
            //canvas.clear();

            // begin timer for delta
            let start = Instant::now();

            game.use_finger = false;

            // draw background texture
            canvas
                .copy_ex(
                    &textures.get("memory_game:background.png").unwrap(),
                    None,
                    Rect::new(0, 0, dims.0, dims.1),
                    0.0,
                    None,
                    false,
                    false,
                )
                .expect("TODO: panic message");

            // get keys that are held down
            game.held_keys = vec![];
            for key in event_pump.keyboard_state().pressed_scancodes() {
                game.held_keys.push(key);
            }

            // get keys that are pressed
            game.events = vec![];
            for event in event_pump.poll_iter() {
                game.events.push(event.clone());
            }

            game.mouse = ((event_pump.mouse_state().x() / scale_factor) as u32, (event_pump.mouse_state().y() / scale_factor) as u32);

            // run a game cycle
            game.cycle(delta, (event_pump.mouse_state().x() / scale_factor) as u32, (event_pump.mouse_state().y() / scale_factor) as u32, dims);

            // run game render
            game.render(canvas, scale_factor, &textures, dims, (event_pump.mouse_state().x() / scale_factor) as u32, (event_pump.mouse_state().y() / scale_factor) as u32);

            // present screen buffer to user
            canvas.present();
            delta = start.elapsed().as_secs_f32();
        }
        else {
            std::process::exit(1);
        }

    };

    #[cfg(target_os = "emscripten")]
    use crate::{emscripten};

    #[cfg(target_os = "emscripten")]
    emscripten::emscripten::set_main_loop_callback(main_loop);



    #[cfg(not(target_os = "emscripten"))]
    loop { main_loop(); }

}
