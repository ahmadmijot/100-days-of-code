//This is the part 4a: field of views
use std::cmp;

use rand::Rng;
use tcod::colors::*;
use tcod::console::*;
use tcod::map::{FovAlgorithm, Map as FovMap}; //bring tcod::map::Map type in and alias it to FovMap

// actual size of windows
const SCREEN_WIDTH: i32 = 80; 
const SCREEN_HEIGHT: i32 = 50;

//map size
const MAP_WIDTH:i32 = 80;
const MAP_HEIGHT:i32 = 45;

// params for dungeon generator
const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;

const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic; //default fov algo
const FOV_LIGHT_WALLS: bool = true; //light walls or not
const TORCH_RADIUS: i32 = 10;

const LIMIT_FPS: i32 = 20;//max 20 fps

// tiles colors
const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100};
const COLOR_LIGHT_WALL: Color = Color {
    r: 130,
    g: 110,
    b: 50,
};
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150
};
const COLOR_LIGHT_GROUND: Color = Color {
    r: 200,
    g: 180,
    b: 50,
};


struct Tcod {              //encapsulate all libcotd-related values into single Struct
    root: Root,
    con: Offscreen,
    fov: FovMap,
}

type Map = Vec<Vec<Tile>>; //a vec composed of the vecs of tiles

struct Game {
    map: Map,
}

// Tile map and properties
#[derive(Clone, Copy, Debug)] //implement certain behaviors/Rust = traits
struct Tile {
    blocked: bool,
    block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
        }
    }
}

/// A rectangle on the map, used to characterise a room.
#[derive(Clone, Copy, Debug)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    pub fn intersects_with(&self, other: &Rect) -> bool {
        // return true if this rect intersect with another one
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }

}

//---------- Generic object: player, monster, item, stairs, etc
// It's always representated by character on the screen. 
#[derive(Debug)]
struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object { x, y, char, color }
    }

    // move by given amount
    pub fn move_by(&mut self, dx: i32, dy: i32, game: &Game) {
        if !game.map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            self.x += dx;
            self.y += dy;
        }
    }

    //set the color and then draw the character that represents
    // this object at its position
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

/// take rect a place it in map, making sure all tiles are empty
fn create_room(room: Rect, map: &mut Map) {
    // go tru the tiles in the rectangle and make them passable
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    //horizontal tunnel. 'min()' and 'max()' are used in case 'x1 > x2'
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) +1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x:i32, map: &mut Map) {
    //vertical tunnel
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) +1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

//-----------------Mapping
fn make_map(player: &mut Object) -> Map {
    // fill map with blocked tiles
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    
    let mut rooms = vec![];

    for _ in 0..MAX_ROOMS {
        //rand width and height
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        //rand position w/o goung out of the boundary of the map
        let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
        let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);

        let new_room = Rect::new(x, y, w, h);

        //run thru the other rooms and see if they intersect with this one
        let failed = rooms
            .iter()
            .any( |other_room| new_room.intersects_with(other_room));

        if !failed {
            // this means there are no intersections, so this room is valid

            //'paint' it to the map's tiles
            create_room(new_room, &mut map);

            // center coordinates of the new room, will be useful later
            let (new_x, new_y) = new_room.center();

            if rooms.is_empty() {
                //this is the first room, where player starts at
                player.x = new_x;
                player.y = new_y;
            } else {
                //all rooms after the first:
                // connect it to the preivious room with a tunnel

                //center coords of previous room
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                //toss a coin 
                if rand::random() {
                    //first move horizontally, then vert
                    create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                    create_v_tunnel(prev_y, new_y, new_x, &mut map);
                } else {
                    // first mv vert, then horz
                    create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                    create_h_tunnel(prev_x, new_x, new_y, &mut map);
                }
            }
            
            //finally append the new room to the list
            rooms.push(new_room);
        }
    }

    map 
}

fn render_all(tcod: &mut Tcod, game: &Game, objects: &[Object], fov_recompute: bool) {
    if fov_recompute {
        //recompute FOV if needed (the player moved or something)
        let player = &objects[0];
        tcod.fov
            .compute_fov(player.x, player.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);
    }

    // go tru all tiles, and set their background color
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let visible = tcod.fov.is_in_fov(x, y);
            let wall = game.map[x as usize][y as usize].block_sight;
            let color = match (visible, wall) {
                //outside fov: 
                (false, true) => COLOR_DARK_WALL,
                (false, false) => COLOR_DARK_GROUND,
                //inside fov: 
                (true, true) => COLOR_LIGHT_WALL,
                (true, false) => COLOR_LIGHT_GROUND,
            };
            tcod.con
                .set_char_background(x, y, color, BackgroundFlag::Set);
        }
    }

    // draw all obj in the list
    for object in objects {
        if tcod.fov.is_in_fov(object.x, object.y) {
            object.draw(&mut tcod.con);
        }
    }
    

    //blit the content of "con" to the root console and present it
    blit(
        &tcod.con, //take the console we want to blit from
        (0,0),     //the coord where to start
        (MAP_WIDTH, MAP_HEIGHT), //width n height of area to blit
        &mut tcod.root, //destination
        (0,0), //where to start blit
        1.0, // foreground transparency 0.0 fully transparent, 1.0 opaque
        1.0, // background transparency
    );
}


//-------------Keyboard handling fn
fn handle_keys(tcod: &mut Tcod, game: &Game, player: &mut Object) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;    
    
    let key = tcod.root.wait_for_keypress(true);
    match key {
        //key movement
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            // Alt+enter: toggle fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, ..} => return true, //exit game

        Key { code: Up, ..} => player.move_by(0, -1, game), 
        Key { code: Down, ..} => player.move_by(0, 1, game),
        Key { code: Left, ..} => player.move_by(-1, 0, game),
        Key { code: Right, ..} => player.move_by(1, 0, game),
 
        _ => {} 
    }

    false
}

fn main () {
    tcod::system::set_fps(LIMIT_FPS);

    //create window
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcos tutorial")
        .init();

    let con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);
        
    let mut tcod = Tcod {
        root,
        con: Offscreen::new(MAP_WIDTH, MAP_HEIGHT), //initialise con inline
        fov: FovMap::new(MAP_WIDTH, MAP_HEIGHT), //initiaise (itl) fov
         };
    
    //----- Object
    // Player, place inside first room
    let player = Object::new(0, 0, '@', WHITE);

    //NPC
    let npc = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', YELLOW);

    // list of objects with those two
    let mut objects = [player, npc];

    let game = Game {
        //generate map
        map: make_map(&mut objects[0]),
    };

    // populate the FOV map, according to the generated map
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            tcod.fov.set(
                x,
                y,
                !game.map[x as usize][y as usize].block_sight,
                !game.map[x as usize][y as usize].blocked,
            );
        }
    }

    //force FOV "recompute" first time through the game loop
    let mut previous_player_position = (-1, -1);

    while !tcod.root.window_closed() { //loop will ecxecuted x (max FPS) times
        tcod.con.clear(); //clear screen from previous frame


        //render the screen
        let fov_recompute = previous_player_position != (objects[0].x, objects[0].y);
        render_all(&mut tcod, &game, &objects, fov_recompute);

        tcod.root.flush();
        // tcod.root.wait_for_keypress(true); 

        // handle keys and exit game if needed
        let player = &mut objects[0];
        previous_player_position = (player.x, player.y);
        let exit = handle_keys(&mut tcod, &game, player);
        if exit {
            break;
        }
    }

}