use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::line::line;

pub fn draw_poligon(fb: &mut Framebuffer, arr: &[Vector2]) {
    if arr.len() < 4 {
        return;
    }
    for i in 0..arr.len() {
        let start = arr[i];
        let end = arr[(i + 1) % arr.len()];
        line(fb, start, end);
    }
}

fn point_in_polygon(p: Vector2, vertices: &[Vector2]) -> bool {
    let mut inside = false;
    let n = vertices.len();
    let mut j = n - 1;
    for i in 0..n {
        let xi = vertices[i].x;
        let yi = vertices[i].y;
        let xj = vertices[j].x;
        let yj = vertices[j].y;
        let intersect = ((yi > p.y) != (yj > p.y)) && (p.x < (xj - xi) * (p.y - yi) / (yj - yi + 1e-10) + xi);
        if intersect {
            inside = !inside;
        }
        j = i;
    }
    inside
}

pub fn fill_polygon(fb: &mut Framebuffer, vertices: &[Vector2]) {
    if vertices.len() < 3 {
        return;
    }

    let min_x = vertices.iter().map(|v| v.x).fold(f32::INFINITY, f32::min).floor() as i32;
    let max_x = vertices.iter().map(|v| v.x).fold(f32::NEG_INFINITY, f32::max).ceil() as i32;
    let min_y = vertices.iter().map(|v| v.y).fold(f32::INFINITY, f32::min).floor() as i32;
    let max_y = vertices.iter().map(|v| v.y).fold(f32::NEG_INFINITY, f32::max).ceil() as i32;

    for y in min_y.max(0)..max_y.min(fb.height as i32) {
        for x in min_x.max(0)..max_x.min(fb.width as i32) {
            let p = Vector2::new(x as f32 + 0.5, y as f32 + 0.5);
            if point_in_polygon(p, vertices) {
                fb.set_pixel(x as u32, y as u32);
            }
        }
    }
}