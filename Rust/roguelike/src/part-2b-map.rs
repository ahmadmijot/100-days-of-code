//This is the part three: off-screen consoles

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
            self.x;
            self.y;
        }
    }

    //set the color and then draw the character that represents
    // this object at its position
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

//-----------------Mapping
fn make_map() -> Map {
    // fill map with unblocked tiles
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    
    // place two pillars to test the map
    map[30][22] = Tile::wall();
    map[50][22] = Tile::wall(); // can also access tile prop as: map[30][22].blocked = true

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
    // Player
    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', WHITE);

    //NPC
    let npc = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '8', YELLOW);

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