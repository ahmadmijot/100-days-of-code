/*
Learning Rust by building a Roguelike game
Taken from https://tomassedovic.github.io/roguelike-tutorial/index.html

This is the part three: off-screen consoles
*/

use tcod::colors::*;
use tcod::console::*;

// actual size of windows
const SCREEN_WIDTH: i32 = 80; 
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20;//max 20 fps

//encapsulate all libcotd-related values into single Struct
struct Tcod {
    root: Root,
    con: Offscreen,
}

//---------- Generic object: player, monster, item, stairs, etc
// It's always representated by character on the screen. 
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
    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    //set the color and then draw the character that represents
    // this object at its position
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}


//-------------Keyboard handling fn
fn handle_keys(tcod: &mut Tcod, player: &mut Object) -> bool {
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

        Key { code: Up, ..} => player.move_by(0, -1), 
        Key { code: Down, ..} => player.move_by(0, 1),
        Key { code: Left, ..} => player.move_by(-1, 0),
        Key { code: Right, ..} => player.move_by(1, 0),
 
        _ => {} 
    }

    false
}

fn main () {
    //create window
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcos tutorial")
        .init();

    let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);
        
    let mut tcod = Tcod { root, con };
    
    //----- Object
    // Player
    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', WHITE);

    //NPC
    let npc = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '8', YELLOW);

    // list of objects with those two
    let mut objects = [player, npc];

    
    while !tcod.root.window_closed() { //loop will ecxecuted x (max FPS) times
        tcod.con.clear(); //clear screen from previous frame

        for object in &objects {
            object.draw(&mut tcod.con);
        }

        //blit the content of "con" to the root console and present it
        blit(
            &tcod.con, //take the console we want to blit from
            (0,0),     //the coord where to start
            (SCREEN_WIDTH, SCREEN_HEIGHT), //width n height of area to blit
            &mut tcod.root, //destination
            (0,0), //where to start blit
            1.0, // foreground transparency 0.0 fully transparent, 1.0 opaque
            1.0, // background transparency
        );

        tcod.root.flush();
        // tcod.root.wait_for_keypress(true); 

        // handle keys and exit game if needed
        let player = &mut objects[0];
        let exit = handle_keys(&mut tcod, player);
        if exit {
            break;
        }
    }

}