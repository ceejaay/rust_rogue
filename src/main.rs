use tcod::colors::*;
use tcod::console::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

const MAP_WIDTH: i32  = 80;
const MAP_HEIGHT: i32  = 45;

const COLOR_DARK_WALL: Color = Color {r: 0, g: 0, b: 100};
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};


// size of map:
struct Tcod {
    root: Root,
    con: Offscreen,
}

type Map = Vec<Vec<Tile>>;

struct Game {
    map: Map,
}

#[derive(Clone, Copy, Debug)]

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
            blocked_sight: true,
        }
    }
}
#[derive(Debug)]
struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object {x, y, char, color}
    }

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

    fn handle_keys(tcod: &mut Tcod, player: &mut Object) -> bool {
        // you can import just into a function, cool
        use tcod::input::Key;
        use tcod::input::KeyCode::*;

        let key = tcod.root.wait_for_keypress(true);
        match key {
            Key {
                code: Enter,
                alt: true,
                ..
            } => {
                let fullscreen = tcod.root.is_fullscreen();
                tcod.root.set_fullscreen(!fullscreen);
            }
            Key {code: Escape, ..} => return true,
            //
            // movement keys
            Key { code: Down, .. } => player.move_by(0, -1),
            Key { code: Up, .. } => player.move_by(0, 1),
            Key { code: Left, .. } => player.move_by(-1, 0),
            Key { code: Right, .. } => player.move_by(1, 0),

            _ => {}
        }
        false
    }



fn make_map() -> Map {
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize] MAP_WIDTH as usize];
    map
}


fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    let mut player_x = SCREEN_WIDTH / 2;
    let mut player_y = SCREEN_HEIGHT / 2;


    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();

    let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut tcod = Tcod { root, con };

    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', WHITE);

    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', YELLOW);

    // here's where all the objects will go into the array
    let mut objects = [player, npc];

    // Main game loop.
    while !tcod.root.window_closed() {
        tcod.con.clear();

        // loop through the objects array
        for object in &objects {
            // draw all the objects
            object.draw(&mut tcod.con)
        }
        // not sure what blit is?
        blit(
            &tcod.con,
            (0, 0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut tcod.root,
            (0, 0),
            1.0,
            1.0,
            );
        //
        // tcod.con.put_char(player_x, player_y, '@', BackgroundFlag::None);
        tcod.root.flush();
        // tcod.root.wait_for_keypress(true);
        let player = &mut objects[0];
        let exit = handle_keys(&mut tcod, player);
        if exit {
            break;
        }
    }
}
