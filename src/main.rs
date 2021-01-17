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
    // OpenGL drawing backend.
    gl: GlGraphics,
    right_now: DateTime<Local>,
}

type GlyphComponent = [f64; 4];
type Glyph = Vec<GlyphComponent>;

struct Flippy {
    x_scalar: f64,
    y_scalar: f64,
    x_shift: f64,
    y_shift: f64,
}

impl Default for Flippy {
    fn default() -> Flippy {
        Flippy {
            x_scalar: 1.0,
            y_scalar: 1.0,
            x_shift: 0.0,
            y_shift: 0.0,
        }
    }
}

// Expects numbers used in dates & times i.e. in [0, 59]
fn split_date_comp_to_digits(n: u32) -> [u32; 2] {
    assert!(n < 60);
    let ones = n % 10;
    let tens = n / 10;
    return [tens, ones];
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        // vertical line going down
        const ZERO_VERT: GlyphComponent = [0.0, 0.0, 0.0, 10.0];
        // top horizontal line going to the right
        const ONE_HORIZ: GlyphComponent = [0.0, 0.0, 10.0, 0.0];
        // lower horizontal line going to the right
        const TWO_HORIZ: GlyphComponent = [0.0, 10.0, 10.0, 10.0];
        // backslash line from top
        const THREE_SLASH: GlyphComponent = [0.0, 0.0, 10.0, 10.0];
        // forward slash line from top
        const FOUR_SLASH: GlyphComponent = [0.0, 10.0, 10.0, 0.0];
        // vert line out over to the right
        const SIX_VERT: GlyphComponent = [10.0, 0.0, 10.0, 10.0];

        let zero: Glyph = vec![ZERO_VERT];
        let one: Glyph = vec![ZERO_VERT, ONE_HORIZ];
        let two: Glyph = vec![ZERO_VERT, TWO_HORIZ];
        let three: Glyph = vec![ZERO_VERT, THREE_SLASH];
        let four: Glyph = vec![ZERO_VERT, FOUR_SLASH];
        let five: Glyph = vec![ZERO_VERT, FOUR_SLASH, ONE_HORIZ];
        let six: Glyph = vec![ZERO_VERT, SIX_VERT];
        let seven: Glyph = vec![ZERO_VERT, SIX_VERT, ONE_HORIZ];
        let eight: Glyph = vec![ZERO_VERT, SIX_VERT, TWO_HORIZ];
        let nine: Glyph = vec![ZERO_VERT, SIX_VERT, ONE_HORIZ, TWO_HORIZ];
        let all_glyphs: Vec<Glyph> =
            vec![zero, one, two, three, four, five, six, seven, eight, nine];

        let bg_color: [f32; 4] = BLACK;
        let fg_color: [f32; 4] = WHITE;

        let (init_x, init_y) = (args.window_size[0] / 4.0, args.window_size[1] / 4.0);
        let right_now = self.right_now;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(bg_color, gl);

            let date_comp_groups = vec![
                vec![
                    split_date_comp_to_digits(
                        (((right_now.time().nanosecond() % 1_000_000_000) as f64) / 1_000_000_000.0
                            * 60.0) as u32,
                    ),
                    split_date_comp_to_digits(right_now.time().second() as u32),
                ],
                vec![
                    split_date_comp_to_digits(right_now.time().minute() as u32),
                    split_date_comp_to_digits(right_now.time().hour() as u32),
                ],
                vec![
                    split_date_comp_to_digits(right_now.date().day() as u32),
                    split_date_comp_to_digits(right_now.date().month() as u32),
                ],
                vec![
                    split_date_comp_to_digits((right_now.date().year() % 100) as u32),
                    split_date_comp_to_digits((right_now.date().year() / 100) as u32),
                ],
            ];

            let flippies = [
                [
                    Flippy {
                        ..Default::default()
                    },
                    Flippy {
                        x_scalar: -1.0,
                        ..Default::default()
                    },
                ],
                [
                    Flippy {
                        y_scalar: -1.0,
                        y_shift: 10.0,
                        ..Default::default()
                    },
                    Flippy {
                        x_scalar: -1.0,
                        y_scalar: -1.0,
                        y_shift: 10.0,
                        ..Default::default()
                    },
                ],
            ];

            let mut curr_x = init_x;
            for date_comp_group in date_comp_groups {
                let mut curr_y = init_y;
                for (comp_pos, date_comps_as_digits) in date_comp_group.iter().enumerate() {
                    // Draw top two entries
                    for (dig_pos, digit) in date_comps_as_digits.iter().rev().enumerate() {
                        let curr_trans = c.transform.trans(curr_x, curr_y);
                        let flippy = &flippies[comp_pos][dig_pos];
                        for glyph_component in all_glyphs[*digit as usize].to_owned() {
                            let maybe_flipped = [
                                glyph_component[0] * flippy.x_scalar + flippy.x_shift,
                                glyph_component[1] * flippy.y_scalar + flippy.y_shift,
                                glyph_component[2] * flippy.x_scalar + flippy.x_shift,
                                glyph_component[3] * flippy.y_scalar + flippy.y_shift,
                            ];
                            line(fg_color, 1.0, maybe_flipped, curr_trans, gl);
                        }
                    }
                    if comp_pos + 1 < date_comp_group.len() {
                        curr_y += 10.0;
                        let curr_trans = c.transform.trans(curr_x, curr_y);
                        line(fg_color, 1.0, ZERO_VERT, curr_trans, gl);
                        curr_y += 10.0;
                    }
                }
                curr_x += 25.0;
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        self.right_now = Local::now();
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("cistercian clock", [200, 100])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create app to run.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        right_now: Local::now(),
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
