use cairo::Context;
use lindenmayer_renderer::canvas::Canvas;

pub struct CairoCanvas(pub Context);

impl Canvas for CairoCanvas {
    fn move_to(&self, x: f64, y: f64) {
        Context::move_to(&self.0, x, y);
    }

    fn line_to(&self, x: f64, y: f64) {
        Context::line_to(&self.0, x, y);
    }

    fn stroke(&self) {
        let _ = Context::stroke(&self.0);
    }

    fn save(&self) {
        let _ = Context::save(&self.0);
    }

    fn restore(&self) {
        let _ = Context::restore(&self.0);
    }

    fn set_line_width(&self, thickness: f64) {
        Context::set_line_width(&self.0, thickness);
    }

    fn rectangle(&self, x: f64, y: f64, width: f64, height: f64) {
        Context::rectangle(&self.0, x, y, width, height);
    }

    fn set_color(&self, r: f64, g: f64, b: f64, a: f64) {
        Context::set_source_rgba(&self.0, r, g, b, a);
    }

    fn arc(&self, x: f64, y: f64, r: f64) {
        Context::arc(&self.0, x, y, r, 0.0, 2.0 * std::f64::consts::PI);
    }
    
    fn fill(&self) {
        let _ = Context::fill(&self.0);
    }
}
