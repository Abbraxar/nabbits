use sdl2::{rect::Point, render::WindowCanvas, pixels::Color as SdlColor};

use crate::components::{Color, Position, Size};

pub fn render_entities(canvas: &mut WindowCanvas, position: &Position, color: &Color, size: &Size) {
  canvas.set_draw_color(SdlColor::BLACK);
  canvas.clear();
  canvas.set_draw_color(color.0);
  draw_circle(canvas, position.0, size.0).unwrap();
  canvas.present();
}

fn draw_circle(canvas: &mut WindowCanvas, center: Point, radius: i32) -> Result<(), String>
{
    let mut x = radius;
    let mut y = 0;

    let mut re = x * x + y * y - radius * radius;
    while x >= y
    {
        canvas.draw_point(Point::new(center.x() + x, center.y() + y))?;
        canvas.draw_point(Point::new(center.x() + y, center.y() + x))?;

        canvas.draw_point(Point::new(center.x() - x, center.y() + y))?;
        canvas.draw_point(Point::new(center.x() - y, center.y() + x))?;

        canvas.draw_point(Point::new(center.x() - x, center.y() - y))?;
        canvas.draw_point(Point::new(center.x() - y, center.y() - x))?;

        canvas.draw_point(Point::new(center.x() + x, center.y() - y))?;
        canvas.draw_point(Point::new(center.x() + y, center.y() - x))?;

        if 2 * (re + 2 * y + 1) + 1 - 2 * x > 0
        {
            re += 1 - 2 * x;
            x -= 1;
        }
        re += 2 * y + 1;
        y += 1;
    }

    Ok(())
}
