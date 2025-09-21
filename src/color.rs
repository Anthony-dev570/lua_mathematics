use crate::angle::Angle;
use crate::interpolation::Interpolation;
use crate::percentage::Percentage;
use crate::scalar::Scalar;

pub type ColorF = Color<f32>;
pub type ColorD = Color<f64>;

#[derive(Debug, Clone, Copy)]
pub enum ColorComponent<S: Scalar> {
    Byte(u8),
    Percentage(Percentage<S>),
}

impl<S: Scalar> ColorComponent<S> {
    pub fn to_percentage(self) -> Percentage<S> {
        match self {
            ColorComponent::Byte(b) => Percentage::new(S::from_u8(b) / S::from_f64(255.0)),
            ColorComponent::Percentage(p) => p,
        }
    }
    pub fn to_byte(self) -> u8 {
        match self {
            ColorComponent::Byte(b) => b,
            ColorComponent::Percentage(p) => (p.take() * S::from_f64(255.0)).to_u8(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Color<S: Scalar> {
    RGB {
        r: ColorComponent<S>,
        g: ColorComponent<S>,
        b: ColorComponent<S>,
    },
    RGBA {
        r: ColorComponent<S>,
        g: ColorComponent<S>,
        b: ColorComponent<S>,
        a: ColorComponent<S>,
    },

    HSV {
        h: Angle<S>,
        s: ColorComponent<S>,
        v: ColorComponent<S>,
    },

    HSVA {
        h: Angle<S>,
        s: ColorComponent<S>,
        v: ColorComponent<S>,
        a: ColorComponent<S>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RGBColorComponent {
    R,
    G,
    B,
}

impl<S: Scalar> Color<S> {
    pub fn to_hsv(self) -> Self {
        match self {
            Color::HSV { .. } => self,
            Color::HSVA { h, s, v, .. } => Self::HSV { h, s, v },
            Color::RGB { r, g, b } => {
                let r = r.to_percentage().take();
                let g = g.to_percentage().take();
                let b = b.to_percentage().take();

                let (c_max, max_c) = {
                    let (mut a, mut _b) = (r, RGBColorComponent::R);

                    if g >= a {
                        a = g;
                        _b = RGBColorComponent::G;
                    }
                    if b >= a {
                        a = b;
                        _b = RGBColorComponent::B;
                    }
                    (a, _b)
                };

                let c_min = {
                    let mut a = r;

                    if g <= a {
                        a = g;
                    }
                    if b <= a {
                        a = b;
                    }

                    a
                };

                let delta = c_max - c_min;

                let hue = match delta == S::ZERO {
                    false => {
                        S::from_f64(60.0)
                            * match max_c {
                                RGBColorComponent::R => ((g - b) / delta) % S::from_f64(6.0),
                                RGBColorComponent::G => ((b - r) / delta) + S::TWO,
                                RGBColorComponent::B => ((r - g) / delta) + S::FOUR,
                            }
                    }
                    true => S::ZERO,
                };

                let saturation = match c_max == S::ZERO {
                    true => S::ZERO,
                    false => delta / c_max,
                };

                let value = c_max;

                Color::HSV {
                    h: Angle::Degrees(hue),
                    s: Percentage::new(saturation).to_color_component(),
                    v: Percentage::new(value).to_color_component(),
                }
            }
            Color::RGBA { .. } => self.to_rgb().to_hsv(),
        }
    }

    pub fn to_hsva(self) -> Self {
        match self {
            Color::RGB { .. } => self.to_rgb().to_hsva(),
            Color::RGBA { r, g, b, a } => {
                let r = Self::RGB { r, g, b }.to_hsv();
                match r {
                    Color::HSV { h, s, v } => Color::HSVA { h, s, v, a },
                    _ => panic!("Not possible"),
                }
            }
            Color::HSV { .. } => self.to_rgb().to_hsva(),
            Color::HSVA { .. } => self,
        }
    }

    pub fn to_rgb(self) -> Self {
        match self {
            Color::RGB { .. } => self,
            Color::RGBA { r, g, b, .. } => Self::RGB { r, g, b },
            Color::HSV { h, s, v } => {
                let h = h.take_degrees() / S::from_f64(360.0);
                let s = s.to_percentage();
                let v = v.to_percentage();

                let i = (h * S::from_f64(6.0)).s_floor();
                let s = s.take();
                let v = v.take();

                let f = h * S::from_f64(6.0) - i;
                let p = v * (S::ONE - s);
                let q = v * (S::ONE - f * s);
                let t = v * (S::ONE - (S::ONE - f) * s);

                let _i = (i % S::from_f64(6.0)).to_u8();

                let (r, g, b) = match _i {
                    0 => (v, t, p),
                    1 => (q, v, p),
                    2 => (p, v, t),
                    3 => (p, q, v),
                    4 => (t, p, v),
                    5 => (v, p, q),
                    _ => (S::ZERO, S::ZERO, S::ZERO),
                };

                Color::RGB {
                    r: ColorComponent::Percentage(r.to_percentage()),
                    g: ColorComponent::Percentage(g.to_percentage()),
                    b: ColorComponent::Percentage(b.to_percentage()),
                }
            }
            Color::HSVA { h, s, v, .. } => Self::HSV { h, s, v }.to_rgb(),
        }
    }

    pub fn to_rgba(self) -> Self {
        match self {
            Color::RGB { r, g, b } => Self::RGBA {
                r,
                g,
                b,
                a: ColorComponent::Percentage(S::ONE.to_percentage()),
            },
            Color::RGBA { .. } => self,
            Color::HSV { .. } => {
                let rgb = self.rgb_take_bytes();
                Self::RGBA {
                    r: ColorComponent::Byte(rgb[0]),
                    g: ColorComponent::Byte(rgb[1]),
                    b: ColorComponent::Byte(rgb[2]),
                    a: ColorComponent::Byte(255),
                }
            }
            Color::HSVA { a, .. } => {
                let rgb = self.rgb_take_bytes();
                Self::RGBA {
                    r: ColorComponent::Byte(rgb[0]),
                    g: ColorComponent::Byte(rgb[1]),
                    b: ColorComponent::Byte(rgb[2]),
                    a,
                }
            }
        }
    }

    pub fn rgb_take_bytes(&self) -> [u8; 3] {
        match self.to_rgb() {
            Color::RGB { r, g, b } => [r.to_byte(), g.to_byte(), b.to_byte()],
            _ => panic!("Not possible."),
        }
    }

    pub fn rgba_take_bytes(self) -> [u8; 4] {
        match self.to_rgba() {
            Color::RGBA { r, g, b, a } => [r.to_byte(), g.to_byte(), b.to_byte(), a.to_byte()],
            _ => panic!("Not possible."),
        }
    }

    pub fn is_rgb(&self) -> bool {
        match self {
            Color::RGB { .. } => true,
            _ => false,
        }
    }

    pub fn is_rgba(&self) -> bool {
        match self {
            Color::RGBA { .. } => true,
            _ => false,
        }
    }

    pub fn is_hsv(&self) -> bool {
        match self {
            Color::HSV { .. } => true,
            _ => false,
        }
    }

    pub fn is_hsva(&self) -> bool {
        match self {
            Color::HSVA { .. } => true,
            _ => false,
        }
    }

    pub fn is_rgb_colorspace(&self) -> bool {
        match self {
            Self::RGB { .. } | Self::RGBA { .. } => true,
            _ => false,
        }
    }

    pub fn is_hsv_colorspace(&self) -> bool {
        match self {
            Self::HSV { .. } | Self::HSVA { .. } => true,
            _ => false,
        }
    }

    pub fn has_alpha(&self) -> bool {
        match self {
            Self::HSVA { .. } | Self::RGBA { .. } => true,
            _ => false,
        }
    }

    pub fn alpha(&self) -> Option<ColorComponent<S>> {
        match self {
            Color::RGBA { a, .. } => Some(*a),
            Color::HSVA { a, .. } => Some(*a),
            _ => None,
        }
    }

    pub fn red(&self) -> Option<ColorComponent<S>> {
        match self {
            Color::RGB { r, .. } => Some(*r),
            Color::RGBA { r, .. } => Some(*r),
            _ => None,
        }
    }

    pub fn green(&self) -> Option<ColorComponent<S>> {
        match self {
            Color::RGB { g, .. } => Some(*g),
            Color::RGBA { g, .. } => Some(*g),
            _ => None,
        }
    }

    pub fn blue(&self) -> Option<ColorComponent<S>> {
        match self {
            Color::RGB { b, .. } => Some(*b),
            Color::RGBA { b, .. } => Some(*b),
            _ => None,
        }
    }
}

impl<S: Scalar> Interpolation<S> for Color<S> {
    fn lerp(a: Self, _b: Self, t: S) -> Self {
        match a {
            Color::RGB { r, g, b } => {
                let a_r = r;
                let a_g = g;
                let a_b = b;

                match _b.to_rgb() {
                    Color::RGB { r, g, b } => Color::RGB {
                        r: ColorComponent::Percentage(Percentage::new(S::lerp(
                            a_r.to_percentage().take(),
                            r.to_percentage().take(),
                            t,
                        ))),
                        g: ColorComponent::Percentage(Percentage::new(S::lerp(
                            a_g.to_percentage().take(),
                            g.to_percentage().take(),
                            t,
                        ))),
                        b: ColorComponent::Percentage(Percentage::new(S::lerp(
                            a_b.to_percentage().take(),
                            b.to_percentage().take(),
                            t,
                        ))),
                    },
                    _ => panic!("Not possible."),
                }
            }
            Color::RGBA { r, g, b, a } => {
                let a_r = r;
                let a_g = g;
                let a_b = b;
                let a_a = a;

                let (r, g, b, a) = match _b.has_alpha() {
                    true => match _b.to_rgba() {
                        Color::RGBA { r, g, b, a } => (r, g, b, a),
                        _ => panic!("Not possible."),
                    },
                    false => match _b.to_rgb() {
                        Color::RGB { r, g, b } => (r, g, b, a_a),
                        _ => panic!("Not possible."),
                    },
                };

                Color::RGBA {
                    r: ColorComponent::Percentage(Percentage::new(S::lerp(
                        a_r.to_percentage().take(),
                        r.to_percentage().take(),
                        t,
                    ))),
                    g: ColorComponent::Percentage(Percentage::new(S::lerp(
                        a_g.to_percentage().take(),
                        g.to_percentage().take(),
                        t,
                    ))),
                    b: ColorComponent::Percentage(Percentage::new(S::lerp(
                        a_b.to_percentage().take(),
                        b.to_percentage().take(),
                        t,
                    ))),
                    a: ColorComponent::Percentage(Percentage::new(S::lerp(
                        a_a.to_percentage().take(),
                        a.to_percentage().take(),
                        t,
                    ))),
                }
            }
            Color::HSV { h, s, v } => {
                let a_h = h;
                let a_s = s;
                let a_v = v;

                match _b.to_hsv() {
                    Color::HSV { h, s, v } => Color::HSV {
                        h: Angle::lerp(a_h, h, t),
                        s: ColorComponent::Percentage(Percentage::new(S::lerp(
                            a_s.to_percentage().take(),
                            s.to_percentage().take(),
                            t,
                        ))),
                        v: ColorComponent::Percentage(Percentage::new(S::lerp(
                            a_v.to_percentage().take(),
                            v.to_percentage().take(),
                            t,
                        ))),
                    },
                    _ => panic!("Not possible."),
                }
            }
            Color::HSVA { h, s, v, a } => {
                let a_h = h;
                let a_s = s;
                let a_v = v;
                let a_a = a;

                match _b.has_alpha() {
                    true => match _b.to_hsva() {
                        Self::HSVA { h, s, v, a } => Self::HSVA {
                            h: Angle::lerp(a_h, h, t),
                            s: S::lerp(a_s.to_percentage().take(), s.to_percentage().take(), t)
                                .to_percentage()
                                .to_color_component(),
                            v: S::lerp(a_v.to_percentage().take(), v.to_percentage().take(), t)
                                .to_percentage()
                                .to_color_component(),
                            a: S::lerp(a_a.to_percentage().take(), a.to_percentage().take(), t)
                                .to_percentage()
                                .to_color_component(),
                        },
                        _ => panic!("Not possible"),
                    },
                    false => match _b.to_hsv() {
                        Color::HSV { h, s, v } => Self::HSVA {
                            h: Angle::lerp(a_h, h, t),
                            s: S::lerp(a_s.to_percentage().take(), s.to_percentage().take(), t)
                                .to_percentage()
                                .to_color_component(),
                            v: S::lerp(a_v.to_percentage().take(), v.to_percentage().take(), t)
                                .to_percentage()
                                .to_color_component(),
                            a: a_a,
                        },
                        _ => panic!("Not possible."),
                    },
                }
            }
        }
    }

    fn inverse_lerp<F: Fn(&Self) -> S>(_a: Self, _b: Self, _v: Self, _f: F) -> S {
        todo!()
    }
}
