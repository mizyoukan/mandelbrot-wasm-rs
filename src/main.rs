extern crate num_complex;
extern crate stdweb;

use num_complex::Complex;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, CanvasRenderingContext2d, HtmlElement};
use stdweb::web::html_element::CanvasElement;

fn create_canvas(width: u32, height: u32) -> CanvasRenderingContext2d {
    let app: HtmlElement = document()
        .get_element_by_id("app")
        .unwrap()
        .try_into()
        .unwrap();

    let canvas: CanvasElement = document()
        .create_element("canvas")
        .unwrap()
        .try_into()
        .unwrap();
    canvas.set_width(width);
    canvas.set_height(height);

    app.append_child(&canvas);

    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
    context
}

fn mandelbrot(c: Complex<f64>, iter_max: usize) -> Option<usize> {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..iter_max {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

fn main() {
    stdweb::initialize();

    let context = create_canvas(400, 400);

    for y in 0..400 {
        for x in 0..400 {
            let c = Complex::new((x - 200) as f64 * 0.009, (y - 200) as f64 * 0.009);
            let color = if let Some(i) = mandelbrot(c, 100) {
                format!("rgba(0,0,0,{})", i as f64 * 0.01)
            } else {
                "black".into()
            };
            context.set_fill_style_color(&color);
            context.fill_rect(x as f64, y as f64, 1.0, 1.0);
        }
    }

    stdweb::event_loop();
}
