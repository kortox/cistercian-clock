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

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        // const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        // const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let bg_color: [f32; 4] = BLACK;
        let fg_color: [f32; 4] = WHITE;

        let rotation = self.rotation;
        // center of window
        let (center_x, center_y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(bg_color, gl);

            let transform = c
                .transform
                .trans(center_x, center_y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            let center_trans = c.transform.trans(center_x, center_y);

            // Draw a box rotating around the middle of the screen.
            line(fg_color, 1.0, [0.0, 0.0, 15.0, 15.0],center_trans , gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
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
        rotation: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
