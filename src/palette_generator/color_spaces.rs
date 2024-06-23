use std::{cmp::max, cmp::min, ops::Mul};

use cgmath::{Matrix3, Vector3};


fn linearalize(a: u8, b: u8, c: u8) -> (u8, u8, u8) {
    return (a / 0xFF, b / 0xFF, c / 0xFF);
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union ThreeColor {
    rgb: RGBColor,
    hsv: HSVColor,
    yuv: YUVColor,
    xyz: XYZColor,
    lab: LABColor,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RGBColor {
    r: u8, g: u8, b: u8
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HSVColor {
    h: u8, s: u8, v: u8
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct XYZColor {
    x: u8, y: u8, z: u8
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct YUVColor {
    y: f32, u: f32, v: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LABColor {
    l: f32, a: f32, b: f32
}

impl ThreeColor {
    fn from_rgb(&mut self, red: u8, green: u8, blue: u8) {
        self.rgb.r = red;
        self.rgb.g = green;
        self.rgb.b = blue;
    }
}

impl RGBColor {
    // Returns a normalized copy where each component is in the range of 0-1.
    fn normalize(&self) -> Self {
        let r = self.r / 0xFF;
        let g = self.g / 0xFF;
        let b = self.b / 0xFF;
        return Self {r, g, b};
    }

    // Normalizes each component into the range 0-1
    fn normalized(&mut self) {
        self.r /= 0xFF;
        self.g /= 0xFF;
        self.b /= 0xFF;
    }
}

impl From<RGBColor> for HSVColor {
    fn from(col: RGBColor) -> Self {
        let norm = col.normalize();

        let M = max(max(norm.r, norm.g), norm.b) as f32;
        let m = min(min(norm.r, norm.g), norm.b) as f32;
        let c = M - m;

        let s = (c/M) * 100.0;

        let r = (M - norm.r as f32) / c;
        let g = (M - norm.g as f32) / c;
        let b = (M - norm.b as f32) / c;

        let hn = match M {
            value if value == m => 0.0,
            value if value == r => 0.0 + b - g,
            value if value == g => 2.0 + r - b,
            value if value == b => 4.0 + g - r,
            _ => unreachable!()
        };

        let h = ((hn/6.0) % 1.0) * 360.0;
        let v = M * 100.0;

        return Self { h: h as u8, s: s as u8, v: v as u8 };
    }
}


// from -> RGBColor conversions

impl From<YUVColor> for RGBColor {
    fn from(value: YUVColor) -> Self {
        let r = value.y + 1.403 * (value.v - 128.0);
        let g = value.y - (0.344 * value.u - 128.0) - (0.714 * (value.v - 128.0));
        let b = value.y + 1.770 * (value.u - 128.0);

        return Self { r: r as u8, g: g as u8, b: b as u8 };
    }
}

// RGB <-> XYZ requires linearization and gamma correction

/// !Note! Normalize the RGB values into 0-1 range first!
// V' = V/12.92 if V <= 0.04045 else ((V + 0.055) / 1.055)^2.4
/// Transformation Matrix for sRGB to XYZ
// |X| |0.4124564 0.3575761 0.1794375| |sR|
// |Y| |0.2126729 0.7151522 0.0721750| |sG|
// |Z| |0.0193339 0.1191920 0.9503041| |sB|
//
//
/// Apply Gamma
// V = V*12.92 if V' <= 0.0031308 else 1.055 * v^(1/2.4) - 0.055
/// Transformation Matrix for XYZ to sRGB
// |sR| | 3.2404542 -1.5371385 -0.4985314| |X|
// |sG| |-0.9692660  1.8760108  0.0415560| |Y|
// |sB| | 0.0556434 -0.2040259  1.0572252| |Z|

fn apply_gamma(value: f32) -> f32 {
    return if value <= 0.0031308 { value * 12.92 } else { 1.055 * value.powf(1.0/2.4) - 0.055 }
}

impl From<XYZColor> for RGBColor {
    fn from(value: XYZColor) -> Self {
        let mat = Matrix3::new(
            0.4124564, 0.3575761, 0.1794375,
            0.2126729, 0.7151522, 0.0721750,
            0.0193339, 0.1191920, 0.9503041
        );

        let xyz = Vector3::new(value.x as f32, value.y as f32, value.z as f32);

        let a = mat.mul(xyz);

        let r = apply_gamma(a.x) as u8;
        let g = apply_gamma(a.y) as u8;
        let b = apply_gamma(a.z) as u8;

        return Self { r, g, b };
    }
}

impl From<RGBColor> for YUVColor {
    fn from(rgb: RGBColor) -> Self {
     // |  Y' |     |  0.299     0.587    0.114   | | R |
     // |  U  |  =  | -0.14713  -0.28886  0.436   | | G |
     // |  V  |     |  0.615    -0.51499 -0.10001 | | B |
        let y = (0.299 * rgb.r as f32) + (0.587 * rgb.g as f32) + (0.114 * rgb.b as f32);
        let u = 0.492 * (rgb.b as f32 - y) + 128.0;
        let v = 0.877 * (rgb.r as f32 - y) + 128.0;

        return Self { y, u, v };
    }
}

impl From<RGBColor> for XYZColor {
    fn from(value: RGBColor) -> Self {
        let x = (0.4124564 * value.r as f32) + (0.3575761 * value.g as f32) + (0.1804375 * value.b as f32);
        let y = (0.2126729 * value.r as f32) + (0.7151522 * value.g as f32) + (0.0721750 * value.b as f32);
        let z = (0.0193339 * value.r as f32) + (0.1191920 * value.g as f32) + (0.9503041 * value.b as f32);

        return Self { x: x as u8, y: y as u8, z: z as u8 };
    }
}

impl XYZColor {
    fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let x = (0.4124564 * r as f32) + (0.3575761 * g as f32) + (0.1804375 * b as f32);
        let y = (0.2126729 * r as f32) + (0.7151522 * g as f32) + (0.0721750 * b as f32);
        let z = (0.0193339 * r as f32) + (0.1191920 * g as f32) + (0.9503041 * b as f32);

        return Self { x: x as u8, y: y as u8, z: z as u8 };
    }
}

impl From<RGBColor> for LABColor {
    fn from(rgb: RGBColor) -> Self {
        let xyz = XYZColor::from_rgb(rgb.r, rgb.g, rgb.b);

        fn f(value: f32) -> f32 {
            // (6/29)^3 = 0.0088564516790356
            // 1/3 = 0.33333333333333
            return if value > 0.0088564516790356 {
                value.powf(0.33333333333333)
            }
            else {
                7.87 * value + (0.13793103448276)
            }
        }

        let x = xyz.x as f32;
        let y = xyz.y as f32;
        let z = xyz.z as f32;

        // White achromatic reference values
        const XN: f32 = 95.0489;
        const YN: f32 = 100.0;
        const ZN: f32 = 108.8840;

        let l = 116.0 * f(y/YN) - 16.0;
        let a = 500.0 * (f(x/XN) - f(y/YN));
        let b = 200.0 * (f(y/YN) - f(z/ZN));

        return Self { l, a, b };
    }
}

