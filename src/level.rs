use std::cmp::PartialEq;
use std::collections::HashMap;
use image::imageops::tile;
use log::warn;
use sdl2::render::{Canvas, Texture, WindowCanvas};
use crate::game::Game;
use crate::resource_location::ResourceLocation;
use crate::tile::{Tile, TileSize, TileType};
use crate::utils::get_dist;

/// Holds all the tile data using multiple [`TileGraphs`].
///
/// [`TileGraphs`]: TileGraph
pub struct Level {
    pub(crate) tile_big : TileGraph,
    pub(crate) tile_medium : TileGraph,
    pub(crate) tile_small : TileGraph,
    pub(crate) tile_nav : TileGraph
}

impl Level {

    /// Calls the [`render`] function on all the tile graphs
    ///
    /// [`render`]: TileGraph::render
    pub fn render(&mut self, player_coords :  (f32, f32), texture : &HashMap<String, Texture>, canvas: &mut WindowCanvas, sf : i32, debug : bool) {
        self.tile_big.render(player_coords, texture, canvas, sf);
        self.tile_medium.render(player_coords, texture, canvas, sf);
        self.tile_small.render(player_coords, texture, canvas, sf);
        if debug {
            self.tile_nav.render(player_coords, texture, canvas, sf)
        }
    }

    /// Returns the tile at a given world space coordinate and tile size, see more @ [`TileGraph::get_tile`]
    pub fn get_tile(&mut self, size : TileSize, coordinates: (f32, f32)) -> Tile {



        if coordinates.0 <= 0.0 || coordinates.1 <= 0.0 {
            //return Tile::create_none(size);
        }

        match size {
            TileSize::BIG => {self.tile_big.get_tile(coordinates.0 as i32, coordinates.1 as i32)}
            TileSize::MEDIUM => {self.tile_medium.get_tile(coordinates.0 as i32, coordinates.1 as i32)}
            TileSize::SMALL => {self.tile_small.get_tile(coordinates.0 as i32, coordinates.1 as i32)}
        }


    }

    /// Creates the level for the demo, entities are loaded @ [`Game::load_demo_level`]
    pub fn create_scene_background(tiles : &HashMap<String, Tile>) -> Self {

        // get tiles from tile map

        // create level
        let mut level = Self{
            tile_big: TileGraph::create(TileSize::BIG),
            tile_medium: TileGraph::create(TileSize::MEDIUM),
            tile_small: TileGraph::create(TileSize::SMALL),
            tile_nav: TileGraph::create(TileSize::SMALL),
        };



        // return the new level
        level
    }



}

/// A graph data structure for holding tiles and their connections
pub struct TileGraph {
    nodes : HashMap<(i32, i32), Tile>,
    connections : HashMap<(i32,i32), Vec<((i32,i32), u32)>>,
    tile_size: TileSize
}

impl TileGraph {

    /// Add a new tile to the graph given a tile and a tile space coordinate
    pub fn append(&mut self, tile : Tile,value : (i32, i32), connections : Vec<((i32, i32), u32)>) {
        // make sure the tile being appended is the right size
        if tile.clone().get_size() != self.tile_size {
            warn!("Tried appending tile \"{}\" of size \"{}\" to the wrong tile graph!", tile.clone().get_resource_location().to_string(), tile.clone().get_size().get().0)
        }
        else {
            // using the same key:
            // add tile to node hashmap
            self.nodes.insert(value, tile);
            // add tile to connections hashmap
            self.connections.insert(value, connections);
        }
    }

    /// Add a new tile to the graph given a tile and a world space coordinate
    pub fn append_from_wolrd_space(&mut self, tile : Tile,value : (i32, i32), connections : Vec<((i32, i32), u32)>) {
        self.append(tile, (value.0 / self.tile_size.get().0 as i32, value.1 / self.tile_size.get().0 as i32), connections)
    }

    /// Get a tile from the graph given a world space coordinate
    pub fn get_tile(&mut self, x : i32, y : i32) -> Tile {
        // get the tile size as a scale factor
        let sf = self.tile_size.get().0 as i32;

        // divide the x and y by the scale factor to get the tile x and y
        let tx = (if x < 0 {x - sf} else { x }) / sf;
        let ty = (if y < 0 {y - sf} else { y }) / sf;

        // use the coordinates as the hashmap key
        let result = self.nodes.get(&(tx, ty));

        // make sure the tile exists before returning...
        if result.is_some() {
            result.unwrap().clone()
        }
        // ...else return a `none` tile
        else {
            Tile::create_none(self.tile_size)
        }

    }

    /// Renders the tile to screen
    pub fn render(&mut self, player_coords :  (f32, f32), texture : &HashMap<String, Texture>, canvas: &mut WindowCanvas, sf : i32) {
        // get scale
        let tile_scale = self.tile_size.get().0 as i32;

        // iterate through the tiles
        for tile in self.nodes.clone() {
            // multiply the tile space coordinate by the tile size, to convert to world space
            tile.1.render(texture, (tile.0.0 * tile_scale, tile.0.1 * tile_scale), canvas, sf, player_coords);
        }
    }

    /// Create a new TileGraph given a [`TileSize`]
    pub fn create(tile_size: TileSize) -> Self{
        Self {
            nodes: HashMap::new(),
            connections: HashMap::new(),
            tile_size,
        }
    }

    /// Creates all the edges for the graph data structure. Must be run if a tile is added, removed or the graph's tiles are in any way changed. If not, path-finding may not function. Should only need to be run for `tile_nav`.
    pub fn build_connections(&mut self) {
        for t in self.nodes.clone() {
            // North tile
            let n = (t.0.0, t.0.1 + 1);
            // East tile
            let e = (t.0.0 + 1, t.0.1);
            // South tile
            let s = (t.0.0, t.0.1 - 1);
            // West tile
            let w = (t.0.0 - 1, t.0.1);

            // create connections list
            let mut connections = vec![];

            // check a tile exists in the given direction, if so and it is a floor, append it.
            if self.nodes.get(&n).is_some() {
                if self.nodes.clone().get_mut(&n).unwrap().get_type() == TileType::FLOOR {
                    connections.push((n, 1))
                }
            }
            if self.nodes.get(&e).is_some() {
                if self.nodes.clone().get_mut(&e).unwrap().get_type() == TileType::FLOOR {
                    connections.push((e, 1))
                }
            }
            if self.nodes.get(&s).is_some() {
                if self.nodes.clone().get_mut(&s).unwrap().get_type() == TileType::FLOOR {
                    connections.push((s, 1))
                }
            }
            if self.nodes.get(&w).is_some() {
                if self.nodes.clone().get_mut(&w).unwrap().get_type() == TileType::FLOOR {
                    connections.push((w, 1))
                }
            }

            // add connections to connection hashmap.
            self.connections.insert(t.0, connections);
        }
    }

    /// An implementation of the A* pathfinding algorithm. Will return a list of world space coordinates given starting coordinates and target coordinates.
    pub fn path_to(&mut self, x : i32, y : i32, tx : i32, ty : i32) -> Vec<(i32, i32)> {

        // get a value for infinty
        let inf = u32::MAX;

        // distance between current and start node
        let mut g : HashMap<(i32,i32), u32> = HashMap::new();
        // total cost
        let mut f : HashMap<(i32,i32), u32> = HashMap::new();

        // start coordinates in tile space
        let startx = x / self.tile_size.get().0 as i32;
        let starty = y / self.tile_size.get().0 as i32;

        // a clone of the graph - this is fine as the actual graph itself doesn't need to be changed by this algorithm
        let mut graph = self.connections.clone();

        // previous node/tile for a given node/tile
        let mut prev_vert : HashMap<(i32,i32), (i32, i32)> = HashMap::new();
        let mut optimal_path = vec![];

        // set every node to initially have a size of "infinity" which in reality is the largest value that u32 can represent.
        for vertex in graph.clone() {
            g.insert(vertex.0, inf);
            f.insert(vertex.0, inf);
        }
        // set cost and distance from start for the starting coordinates to be 0
        *g.get_mut(&(startx, starty)).unwrap() = 0;
        *f.get_mut(&(startx, starty)).unwrap() = 0;

        // Whilst the graph contains nodes
        while graph.len() > 0 {
            // initiate shortest distance node.
            let mut shortest : Option<(i32,i32)> = None;

            // iterate through to find actual shortest, or assign one if none has been assigned
            for vert in graph.clone() {
                if shortest.is_none() {
                    shortest = Some(vert.0)
                }
                else if f.get(&vert.0).unwrap() < f.get(&shortest.unwrap()).unwrap(){
                    shortest = Some(vert.0)
                }
            }

            // iterate through the connections of the shortest node
            for (neighbour, cost) in graph.get(&shortest.unwrap()).unwrap() {

                if graph.get(neighbour).is_some() && cost + g.get(&shortest.unwrap()).unwrap() < *f.get(&neighbour).unwrap() {
                    *g.get_mut(&neighbour).unwrap() = cost + *g.get_mut(&shortest.unwrap()).unwrap();
                    *f.get_mut(&neighbour).unwrap() = cost + g.get(&shortest.unwrap()).unwrap();
                    prev_vert.insert(*neighbour, shortest.unwrap());
                }
            }
            graph.remove(&shortest.unwrap());

        }

        let mut vertex = (tx / self.tile_size.get().0 as i32, ty / self.tile_size.get().0 as i32);
        while vertex != (startx, starty) && prev_vert.get(&vertex).is_some() {
            optimal_path.insert(0, (vertex.0 * self.tile_size.get().0 as i32, vertex.1 * self.tile_size.get().0 as i32));
            vertex = *prev_vert.get(&vertex).unwrap()

        }
        optimal_path.insert(0, (startx * self.tile_size.get().0 as i32, starty * self.tile_size.get().0 as i32));
        optimal_path

    }

}