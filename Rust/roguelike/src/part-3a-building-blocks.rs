//This is the part 3a: building blocks
use std::cmp;

use tcod::colors::*;
use tcod::console::*;

// actual size of windows
const SCREEN_WIDTH: i32 = 80; 
const SCREEN_HEIGHT: i32 = 50;

//map size
const MAP_WIDTH:i32 = 80;
const MAP_HEIGHT:i32 = 45;

const LIMIT_FPS: i32 = 20;//max 20 fps

// tiles colors
const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100};
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150
};

struct Tcod {              //encapsulate all libcotd-related values into single Struct
    root: Root,
    con: Offscreen,
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
fn make_map() -> Map {
    // fill map with blocked tiles
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    
    // create two rooms
   let room1 = Rect::new(20, 15, 10, 15);
   let room2 = Rect::new(50, 15, 10, 15);
   create_room(room1, &mut map);
   create_room(room2, &mut map);
   create_h_tunnel(25, 55, 23, &mut map);

    map
}

fn render_all(tcod: &mut Tcod, game: &Game, objects: &[Object]) {
    // go tru all tiles, and set their background color
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = game.map[x as usize][y as usize].block_sight;
            if wall {
                tcod.con
                    .set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                tcod.con
                    .set_char_background(x,y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }
    // draw all obj in the list
    for object in objects {
        object.draw(&mut tcod.con);
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
        
    let mut tcod = Tcod { root, con };
    
    //----- Object
    // Player, place inside first room
    let player = Object::new(25, 23, '@', WHITE);

    //NPC
    let npc = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', YELLOW);

    // list of objects with those two
    let mut objects = [player, npc];

    let game = Game {
        //generate map
        map: make_map(),
    };

    while !tcod.root.window_closed() { //loop will ecxecuted x (max FPS) times
        tcod.con.clear(); //clear screen from previous frame


        //render the screen
        render_all(&mut tcod, &game, &objects);

        tcod.root.flush();
        // tcod.root.wait_for_keypress(true); 

        // handle keys and exit game if needed
        let player = &mut objects[0];
        let exit = handle_keys(&mut tcod, &game, player);
        if exit {
            break;
        }
    }

}