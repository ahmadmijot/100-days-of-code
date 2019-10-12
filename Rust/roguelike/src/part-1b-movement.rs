/*
Learning Rust by building a Roguelike game
Taken from https://tomassedovic.github.io/roguelike-tutorial/index.html

This is the second step: Movement
*/

// import use statements
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


// Keyboard handling fn
// fn fn_name(param:type,..) -> return_type
fn handle_keys(tcod: &mut Tcod, player_x: &mut i32, player_y: &mut i32) -> bool {
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

        Key { code: Up, ..} => *player_y -= 1, // .. = Don't care abt other fields
        Key { code: Down, ..} => *player_y +=1,
        Key { code: Left, ..} => *player_x -=1,
        Key { code: Right, ..} => *player_x +=1,
 
        _ => {} // value that matches everything else
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
    
    let mut tcod = Tcod { root };

    //movement
    let mut player_x = SCREEN_WIDTH / 2; // /2 to place @ in the centre of screen
    let mut player_y = SCREEN_HEIGHT /2;


    // render white @ character on the screen until libtcod windows closed
    while !tcod.root.window_closed() { //loop will ecxecuted x (max FPS) times
        tcod.root.set_default_foreground(WHITE);
        tcod.root.clear();
        tcod.root
            .put_char(player_x, player_y, '@', BackgroundFlag::None);
        tcod.root.flush(); //flush = draw everything at once
        tcod.root.wait_for_keypress(true); // not in later tut

        // handle keys and exit game if needed
        let exit = handle_keys(&mut tcod, &mut player_x, &mut player_y);
        if exit {
            break;
        }
    }

}