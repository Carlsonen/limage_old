pub mod core;
pub mod prelude;
pub mod shiterators;

// straight up wrong but can be fixed another time
pub fn blend_color(old: [u8; 4], new: [u8; 4]) -> [u8; 4] {
    let a = new[3] as f32 / 255.;
    let b = 1. - a;
    let red = b * old[0] as f32 + a * new[0] as f32;
    let green = b * old[1] as f32 + a * new[1] as f32;
    let blue = b * old[2] as f32 + a * new[2] as f32;
    let a = a.max(old[3] as f32 / 255.) * 255.999;
    [red as u8, green as u8, blue as u8, a as u8]
}

// range (360 1 1)
pub fn hsl_to_rgb(hsl: [f32; 3]) -> [u8; 3] {
    let h = hsl[0] % 360.0;
    let s = hsl[1];
    let l = hsl[2];

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let rgb_tmp = match h {
        _ if h < 60.0 => (c, x, 0.0),
        _ if h < 120.0 => (x, c, 0.0),
        _ if h < 180.0 => (0.0, c, x),
        _ if h < 240.0 => (0.0, x, c),
        _ if h < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    [
        ((rgb_tmp.0 + m) * 256.) as u8,
        ((rgb_tmp.1 + m) * 256.) as u8,
        ((rgb_tmp.2 + m) * 256.) as u8,
    ]
}

pub const RED: [u8; 3] = [255, 0, 0];
pub const GREEN: [u8; 3] = [0, 255, 0];
pub const BLUE: [u8; 3] = [0, 0, 255];
pub const YELLOW: [u8; 3] = [255, 255, 0];
pub const MAGENTA: [u8; 3] = [255, 0, 255];
pub const CYAN: [u8; 3] = [0, 255, 255];

pub const BEIGE: [u8; 3] = [222, 184, 135];
pub const FOREST_GREEN: [u8; 3] = [34, 139, 34];

#[cfg(test)]
mod test {
    #[test]
    fn hsl_is_correct() {
        use super::hsl_to_rgb;

        assert_eq!(hsl_to_rgb([0., 0., 0.]), [0, 0, 0]);
        assert_eq!(hsl_to_rgb([360., 0.5, 1.]), [255, 255, 255]);
        assert_eq!(hsl_to_rgb([120., 0.5, 0.25]), [32, 96, 32]);
    }
}
