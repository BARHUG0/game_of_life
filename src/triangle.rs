use crate::framebuffer::{self, Framebuffer};
use crate::line::line;
use raylib::math::{Vector2, Vector3};

pub fn triangle(framebuffer: &mut Framebuffer, v1: Vector3, v2: Vector3, v3: Vector3) {
    let a = Vector2::new(v1.x, v1.y);
    let b = Vector2::new(v2.x, v2.y);
    let c = Vector2::new(v3.x, v3.y);

    line(framebuffer, a, b);
    line(framebuffer, b, c);
    line(framebuffer, c, a);
}

pub fn barycentric_triangle(v1: &Vector3, v2: &Vector3, v3: Vector3) {
    //    let mut fragment = Vec::new();

    let v1_x = v1.x;
    let v1_y = v1.y;
    let v2_x = v2.x;
    let v2_y = v2.y;
    let v3_x = v3.x;
    let v3_y = v3.y;

    /*
        let min_x = v1_x.min(b_x).min(c_x).floor() as i32;
        let min_y = v1_y.min(v2_y).min(v3_y).floor() as i32;

        let max_x = = v1_x.max(v2_x).max(v3_x).ceil() as i32;
        let max_y = = v1_y.max(v2_y).max(v3_y).ceil() as i32;

        for y in min_y..=max_y{
            for x in min_x..=max_x {
                let (w, v, u) = barycentric(x as f32, y as f32, v1, v2, v3);



                if w >=0.0 && v >= 0.0 && u >= 0.0 {

                }
            }
        }
    */
}

pub fn barycentric(p_x: f32, p_y: f32, a: Vector3, b: Vector3, c: Vector3) -> (f32, f32, f32) {
    let a_x = a.x;
    let a_y = a.y;
    let b_x = b.x;
    let b_y = b.y;
    let c_x = c.x;
    let c_y = c.y;

    let area = (b_y - c_y) * (a_x - c_x) + (c_x - b_x) * (a_y - c_y);

    if area.abs() < 1e-10 {
        return (-1.0, -1.0, -1.0);
    }

    let w = ((b_y - c_y) * (p_x - c_x) + (c_x - b_x) * (p_y - c_y)) / area;
    let v = ((c_y - a_y) * (p_x - c_x) + (a_x - c_x) * (p_y - c_y)) / area;
    let u = 1.0 - w - v;

    (w, v, u)
}
