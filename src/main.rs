extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use chrono::prelude::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use std::{thread, time};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    // rotation: f64,  // Rotation for the square.
    which_glyph: i32,
}

type GlyphComponent = [f64; 4];

type Glyph = Vec<GlyphComponent>;


impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        // const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        // const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        // vertical line
        const ZERO_VERT:  GlyphComponent = [0.0, 0.0, 0.0, 10.0];
        // top horizontal line
        const ONE_HORIZ:  GlyphComponent = [0.0, 0.0, 10.0, 0.0];
        // lower horizontal line
        const TWO_HORIZ:  GlyphComponent = [0.0, 10.0, 10.0, 10.0];
        // backslash line from top
        const THREE_SLASH:  GlyphComponent = [0.0, 0.0, 10.0, 10.0];
        // forward slash line from top
        const FOUR_SLASH:  GlyphComponent = [0.0, 10.0, 10.0, 0.0];
        // vert line out
        const SIX_VERT:  GlyphComponent = [10.0, 0.0, 10.0, 10.0];
        let ALL_GLYPH_COMPS: [GlyphComponent; 6] = [ZERO_VERT, ONE_HORIZ, TWO_HORIZ, THREE_SLASH, FOUR_SLASH, SIX_VERT];

        let ZERO: Glyph = vec![ZERO_VERT];
        let ONE: Glyph = vec![ZERO_VERT, ONE_HORIZ];
        let TWO: Glyph = vec![ZERO_VERT, TWO_HORIZ];
        let THREE: Glyph = vec![ZERO_VERT, THREE_SLASH];
        let FOUR: Glyph = vec![ZERO_VERT, FOUR_SLASH];
        let FIVE: Glyph = vec![ZERO_VERT, FOUR_SLASH, ONE_HORIZ];
        let SIX: Glyph = vec![ZERO_VERT, SIX_VERT];
        let SEVEN: Glyph = vec![ZERO_VERT, SIX_VERT, ONE_HORIZ];
        let EIGHT: Glyph = vec![ZERO_VERT, SIX_VERT, TWO_HORIZ];
        let NINE: Glyph = vec![ZERO_VERT, SIX_VERT, ONE_HORIZ, TWO_HORIZ];
        let ALL_GLYPHS = vec![ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];


        let bg_color: [f32; 4] = BLACK;
        let fg_color: [f32; 4] = WHITE;

        //let rotation = self.rotation;
        let which_glyph = self.which_glyph;
        // center of window
        let (center_x, center_y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(bg_color, gl);

            // let transform = c
            //     .transform
            //     .trans(center_x, center_y)
            //     .rot_rad(rotation)
            //     .trans(-25.0, -25.0);

            let center_trans = c.transform.trans(center_x, center_y);

            // Draw a box rotating around the middle of the screen.
            for glyph_component in ALL_GLYPHS[which_glyph as usize].to_owned() {
                line(fg_color, 1.0, glyph_component, center_trans, gl);
            }

        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        // self.rotation += 2.0 * args.dt;
        self.which_glyph += 1;
        self.which_glyph %= 10;
    }
}

fn main() {
    let local: DateTime<Local> = Local::now();

    println!("hour: {}", local.time().hour());
    println!("minute: {}", local.time().minute());
    println!("day: {}", local.date().day());
    println!("month: {}", local.date().month());
    println!("year: {}", local.date().year());

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("cistercian clock", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        // rotation: 0.0,
        which_glyph: 0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        let ten_millis = time::Duration::from_millis(250);
        thread::sleep(ten_millis);

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
