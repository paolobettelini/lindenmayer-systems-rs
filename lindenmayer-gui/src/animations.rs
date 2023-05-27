use cairo::{Context, Format, ImageSurface};
use gtk::{glib::*, prelude::*, *};
use std::{
    cell::RefCell,
    rc::Rc,
    time::{Duration, Instant},
};

use lindenmayer_renderer::{
    canvas::{Canvas, ExprContext},
    LSystemRenderer,
};
use lindenmayer_renderer_cairo::CairoCanvas;

use crate::helpers::*;

pub struct LSystemAnimator {
    renderer: Rc<RefCell<LSystemRenderer>>, // Just RefCell<T> ?
    drawing_area: Rc<DrawingArea>,
    frame_label: Rc<Label>,
    length_label: Rc<Label>,
    // For every animation reset a new Rc<RefCell<bool>> is created
    // The outer RefCell<T> is for inner mutability on Rc<RefCell<bool>>
    // When the animation needs to be stopped the inner Rc<RefCell<bool>> is set to false,
    // so the loop notices and stop, and then it is replaced in the struct with a new Rc<RefCell<true>>
    is_playing: RefCell<Rc<RefCell<bool>>>,
    frame_count: Rc<RefCell<u32>>,
    // Keeps track of the animation time
    // Every time the animation is stopped, the elapsed
    // from the Instant time is added to the u128.
    // The Instant is None whether the time does not need to increment
    animation_time: Rc<RefCell<(u128, Option<Instant>)>>,
}

impl LSystemAnimator {
    pub fn new(
        renderer: Rc<RefCell<LSystemRenderer>>,
        drawing_area: Rc<DrawingArea>,
        status_label: &Label,
        frame_label: Rc<Label>,
        time_label: Rc<Label>,
        elapsed_label: Rc<Label>,
        length_label: Rc<Label>,
    ) -> Self {
        let frame_count = Rc::new(RefCell::new(0));
        let animation_time = Rc::new(RefCell::new((0, None::<Instant>)));

        (*drawing_area).set_draw_func(clone!(@weak renderer,
            @weak frame_count,
            @weak status_label,
            @weak frame_label,
            @strong time_label,
            @weak animation_time,
            @strong elapsed_label => move |widget, context, _i, _j| {
            let renderer = &renderer.borrow_mut();

            // Update size
            widget.set_content_width(renderer.canvas.0);
            widget.set_content_height(renderer.canvas.1);

            // Context to do UI manipulation
            let main_context = MainContext::default();

            let surface = {
                let surface_res = ImageSurface::create(
                    Format::ARgb32,
                    renderer.canvas.0, // width
                    renderer.canvas.1, // height
                );

                if let Ok(v) = surface_res {
                    v
                } else {
                    set_error_status(&main_context, &status_label, "Could not get surface");
                    return;
                }
            };

            let cr = {
                if let Ok(cr) = Context::new(&surface) {
                    cr
                } else {
                    set_error_status(&main_context, &status_label, "Could not get context");
                    return;
                }
            };

            let start = Instant::now();

            // Setup variables
            let frame = {*frame_count.borrow()};
            let time = {
                let time_ref = animation_time.borrow();
                // compute total animation time
                time_ref.0 + if let Some(instant) = time_ref.1 {
                    instant.elapsed().as_millis()
                } else {
                    0
                }
            };

            let mut variables = ExprContext::new();
            variables.var("FRAME", frame as f64);
            variables.var("TIME", time as f64);

            // Draw fractal
            let cairo_canvas: &mut dyn Canvas = &mut CairoCanvas(cr);
            let draw_res = cairo_canvas.draw_fractal(renderer, &mut variables);

            let elapsed = start.elapsed();

            // Update frame and elapsed label
            let frame_text = format!("Frame: {}", frame);
            let time_text = format!("Time: {}", time);
            let elapsed_text = format!("Elapsed: {:?}", elapsed);
            main_context.spawn_local(
                clone!(@weak frame_label, @weak time_label, @weak elapsed_label => async move {
                frame_label.set_text(&frame_text);
                time_label.set_text(&time_text);
                elapsed_label.set_text(&elapsed_text);
            }));

            if let Err(err) = draw_res {
                set_error_status(&main_context, &status_label, &meval_error_msg(&err));
                return;
            }

            // Draw the surface to the context of the widget
            if context.set_source_surface(&surface, 0.0, 0.0).is_err() {
                set_error_status(&main_context, &status_label, "Could not draw picture");
                return;
            }
            if context.paint().is_err() {
                set_error_status(&main_context, &status_label, "Could not draw picture");
                return;
            }

            hide_status(&main_context, &status_label);
        }));

        let is_playing = RefCell::new(Rc::new(RefCell::new(false)));

        LSystemAnimator {
            renderer,
            drawing_area,
            is_playing,
            frame_label,
            length_label,
            frame_count,
            animation_time,
        }
    }

    pub fn update_renderer(&mut self, renderer: LSystemRenderer) {
        log::debug!("Updating renderer. Generating fractal");

        let mut self_renderer = self.renderer.borrow_mut();
        *self_renderer = renderer;

        let start = Instant::now();

        self_renderer.update_expr();

        let elapsed = start.elapsed();
        log::debug!("String expansion took: {elapsed:?}");

        self_renderer.update_rng();

        let length_text = format!("Length: {}", self_renderer.expression.len());
        self.length_label.set_text(&length_text);

        let playing_value = {
            let playing = self.is_playing.borrow();
            let v = *playing.borrow();
            v
        };

        if !playing_value {
            // Simply refresh the drawing
            self.queue_drawing();
        }
    }

    pub fn toggle_playing(&self) {
        let playing_value = {
            let playing = self.is_playing.borrow();
            let v = *playing.borrow();
            v
        };

        if playing_value {
            // Set playing flag to true
            *self.is_playing.borrow().borrow_mut() = false;

            // Update passed time
            let elapsed = {
                if let Some(instant) = self.animation_time.borrow().1 {
                    instant.elapsed().as_millis()
                } else {
                    0
                }
            };

            let mut time_ref = self.animation_time.borrow_mut();
            (*time_ref).1 = None;
            (*time_ref).0 += elapsed;
        } else {
            *self.is_playing.borrow_mut() = Rc::new(RefCell::new(true));
            self.animation_time.borrow_mut().1 = Some(Instant::now());

            self.play();
        }
    }

    pub fn reset(&self) {
        *self.frame_count.borrow_mut() = 0;

        // If Instant was None leave it None, otherwise update with current time
        let is_instant_none = { self.animation_time.borrow().1.is_none() };
        let instant = if is_instant_none {
            None
        } else {
            Some(Instant::now())
        };
        *self.animation_time.borrow_mut() = (0, instant);

        let label = self.frame_label.clone();
        glib::idle_add_local(move || {
            label.set_text("Frame: 0");
            Continue(false)
        });

        let playing_value = {
            let playing = self.is_playing.borrow();
            let v = *playing.borrow();
            v
        };

        if !playing_value {
            // Simply refresh the drawing
            self.queue_drawing();
        }
    }

    fn play(&self) {
        let drawing_area = self.drawing_area.clone();

        let playing = self.is_playing.borrow().clone();
        let frame_count = self.frame_count.clone();
        glib::timeout_add_local(
            Duration::from_millis(30),
            clone!(@weak drawing_area,
                @strong playing,
                @strong frame_count => @default-return Continue(false), move || {

                if !*playing.borrow() {
                    return Continue(false);
                }

                // Draw loop
                // Heavy lifting
                (*drawing_area).queue_draw();

                // Increment frame
                let mut frame_count = frame_count.borrow_mut();
                *frame_count += 1;

                // Continue only if still playing and no error has occured
                Continue(*playing.borrow())
            }),
        );
    }

    pub fn queue_drawing(&self) {
        (*self.drawing_area).queue_draw();
    }
}
