use once_cell::sync::OnceCell;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::iter::Copied;
use std::sync::Mutex;
use log::warn;
use sdl2::keyboard::Scancode::I;
use sdl2::libc::stat;
use sdl2::pixels::Color;
use crate::render;
use crate::resource_location::ResourceLocation;

pub const TARGET_DIMENSIONS: (u32, u32) = (448, 252);

/// A struct which holds all relevant information for displaying an item's texture.
#[derive(Debug)]
pub struct AssetData {
    pub(crate) uv: Option<Rect>,
    pub(crate) origin: (i32, i32),
    pub (crate) resource_location: ResourceLocation,
}

impl AssetData {
    /// Creates a resource location with no information
    pub fn empty() -> Self {
        Self {
            uv: Some(Rect::new(0,0,32,32)),
            origin: (0, 0),
            resource_location: ResourceLocation::empty()
        }
    }
}

impl Clone for AssetData {
    fn clone(&self) -> Self {
        Self {
            uv: self.uv.clone(),
            origin: self.origin.clone(),
            resource_location: self.resource_location.clone(),
        }
    }
}

pub struct Dimensions {
    x : u32,
    y : u32
}

impl Dimensions {
    pub fn get_x(&self) -> u32 {
        self.x
    }
    pub fn get_y(&self) -> u32 {
        self.y
    }
    pub fn set_x(&mut self, x : u32) {
        self.x = x
    }
    pub fn set_y(&mut self, y : u32) {
        self.y = y
    }
    pub fn set(&mut self, dims : (u32, u32)) {
        self.x = dims.0;
        self.y = dims.1
    }
    pub fn get(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}


/// Access a static list of ResourceLocations with missing textures.
pub fn get_missing_list() -> &'static Mutex<Vec<String>>{
    static INSTANCE : OnceCell<Mutex<Vec<String>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let vec : Vec<String> = vec![];
        Mutex::new(vec)
    })
}

pub fn get_actual_dimensions() -> &'static Mutex<Dimensions> {
    static INSTANCE : OnceCell<Mutex<Dimensions>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        Mutex::new(Dimensions{
            x: TARGET_DIMENSIONS.0,
            y: TARGET_DIMENSIONS.1,
        })
    })
}

/// Access a static list of mouse icons
pub fn get_icons() -> &'static Mutex<HashMap<&'static str, AssetData>> {
    static INSTANCE: OnceCell<Mutex<HashMap<&'static str, AssetData>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(
            "cursor",
            AssetData {
                uv: Option::from(Rect::new(0, 0, 16, 16)),
                origin: (0, 0),
                resource_location: ResourceLocation::new("game", "gui/icons.png"),
            },
        );
        m.insert(
            "finger",
            AssetData {
                uv: Option::from(Rect::new(32, 0, 16, 16)),
                origin: (3, 1),
                resource_location: ResourceLocation::new("game", "gui/icons.png"),
            },
        );
        m.insert(
            "cursor_old",
            AssetData {
                uv: Option::from(Rect::new(16, 0, 16, 16)),
                origin: (0, 0),
                resource_location: ResourceLocation::new("game", "gui/icons.png"),
            }
        );
        Mutex::new(m)
    })
}



/// Draws textures to the screen pixel-perfectly
pub fn draw_pp_texture(x: i32, y: i32, ass: &AssetData, mut canvas: &mut WindowCanvas, sf: i32, textures : &HashMap<String, Texture>) {
    let uv = ass.uv.unwrap();
    let tex_rect = Rect::new(x - ass.origin.0, y - ass.origin.1, uv.w as u32, uv.h as u32);
    let mut id = ass.resource_location.clone();

    canvas
        .set_scale(sf as f32, sf as f32)
        .expect("TODO: panic message");

    // get texture from the map
    let mut texture = textures.get(&id.to_string());

    // if the texture is missing, show missing texture
    if texture.is_none(){
        if !get_missing_list().lock().unwrap().contains(&&id.to_string()) {
            warn!("Texture at {} could not be found!", id.to_string())
        }
        get_missing_list().lock().unwrap().push(id.clone().to_string());
        texture = textures.get(&ResourceLocation::new("game", "missing.png").to_string());
    }


    canvas
        .copy_ex(&texture.unwrap(),
                 uv,
                 tex_rect,
                 0.0,
                 None,
                 false,
                 false,
        )
        .expect("TODO: panic message");
}






/*

this is the way I wanted to do textures; statically saving them as not to constantly recall the assets. I might use Bevy as a way to save the data better

pub fn get_missing_texture(texture_creator: TextureCreator<WindowContext>) -> &'static Result<Texture<'static>, String> {
    static INSTANCE: OnceCell<Result<Texture, String>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        texture_creator.load_texture("assets/missing.png")
    })
}
 */
/*
pub fn get_texture<'a>(tt : TextureType, canvas: WindowCanvas) -> Texture<'a> {
    let texture_creator = canvas.texture_creator();
    match tt {
        TextureType::icon => { let text = texture_creator.load_texture("assets/gui/icons.png").unwrap(); text}
        _ => { let text = texture_creator.load_texture("assets/missing.png").unwrap(); text }
    }
}

 */
