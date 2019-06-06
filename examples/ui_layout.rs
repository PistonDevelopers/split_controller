extern crate sdl2_window;
extern crate opengl_graphics;
extern crate piston;
extern crate graphics;
extern crate split_controller;

use split_controller::{SplitLayoutController, SplitLayoutSettings, SplitState};
use sdl2_window::Sdl2Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{Events, EventSettings};
use piston::window::{Window, WindowSettings};
use piston::input::{RenderEvent};
use graphics::*;

fn main() {
    let opengl = OpenGL::V3_2;

    let settings = WindowSettings::new("UI Layout", [1024; 2])
        .exit_on_esc(true)
        .graphics_api(opengl);
    let mut window: Sdl2Window = settings.build().unwrap();

    let ref mut split_layout_settings = SplitLayoutSettings::new(2.0, 100.0);
    split_layout_settings.center_min_size = [10.0; 2];
    let mut split_layout = SplitLayoutController::new(split_layout_settings);
    let show_min_size = false;
    let margin = 10.0;

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        let window_size = window.size();
        let split_layout_bounds = [
            margin, margin,
            window_size.width as f64 - margin * 2.0, window_size.height as f64 - margin * 2.0
        ];

        split_layout.event(split_layout_bounds, math::identity(), &e);

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                clear([1.0; 4], g);

                let rectangles = split_layout.rectangles(split_layout_bounds);
                let states = split_layout.states();
                for i in 0..4 {
                    let color = match states[i] {
                        SplitState::Inactive => [0.5, 0.5, 0.5, 1.0],
                        SplitState::Hover => [0.8, 0.8, 0.8, 1.0],
                        SplitState::Drag => [0.6, 0.6, 0.6, 1.0],
                        SplitState::DragNotFollowing => [1.0, 0.8, 0.8, 1.0],
                    };
                    rectangle(color, rectangles[i], c.transform, g);
                }

                let panels = split_layout.panel_rectangles(split_layout_bounds);
                for (i, &panel) in panels.iter().enumerate() {
                    let color = match i {
                        0 | 1 => [0.9, 0.9, 0.9, 1.0],
                        2 | 3 => [0.7, 0.7, 0.7, 1.0],
                        _ => [0.2, 0.2, 0.2, 1.0],
                    };
                    rectangle(color, panel, c.transform, g);
                }

                if show_min_size {
                    let min_size = split_layout.min_size();
                    rectangle([0.0, 0.0, 0.0, 0.1], [0.0, 0.0, min_size[0], min_size[1]],
                              c.transform, g);
                }
            });
        }
    }
}
