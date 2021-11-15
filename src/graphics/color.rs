#[derive(Debug)]
pub struct RGBA {
    r: f32,
    g: f32,
    b: f32,
    a: f32
}

impl RGBA {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: r,
            g: g,
            b: b,
            a: a
        } 
    }
}

pub fn black() -> RGBA {
    let black: RGBA = RGBA::new(0f32, 0f32, 0f32, 1f32);
    return black;
}

pub fn white() -> RGBA {
    let white: RGBA = RGBA::new(0f32, 0f32, 0f32, 0f32);
    return white;
}

pub fn red() -> RGBA {
    let red: RGBA = RGBA::new(255f32, 0f32, 0f32, 1f32);
    return red;
}

pub fn blue() -> RGBA {
    let blue: RGBA = RGBA::new(0f32, 0f32, 255f32, 1f32);
    return blue;
}

pub fn green() -> RGBA {
    let green: RGBA = RGBA::new(0f32, 255f32, 0f32, 1f32);
    return green;
}

