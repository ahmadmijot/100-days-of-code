/*
Learning Rust by building a Roguelike game
Taken from https://tomassedovic.github.io/roguelike-tutorial/index.html

This is the first step: Graphic
*/

// imposrt use statements
use tcod::colors::*;
use tcod::console::*;

// add constant so dont end up littering src with bunch of opaque numbers
// actual size of windows
const SCREEN_WIDTH: i32 = 80; //const written using uppercase
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20;//max 20 fps

//encapsulate all libcotd-related values into single Struct
struct Tcod {
    root: Root,
}

fn main () {
    //create window
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcos tutorial")
        .init();
    
    let mut tcod = Tcod { root };

    // render white @ character on the screen until libtcod windows closed
    while !tcod.root.window_closed() { //loop will ecxecuted x (max FPS) times
        tcod.root.set_default_foreground(WHITE);
        tcod.root.clear();
        tcod.root.put_char(1, 1, '@', BackgroundFlag::None);
        tcod.root.flush(); //flush = draw everything at once
        tcod.root.wait_for_keypress(true);
    }

}