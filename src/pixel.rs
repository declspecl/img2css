use image;

pub struct Pixel
{
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Pixel
{
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self
    {
        return Pixel{ r, g, b, a };
    }

    pub fn to_css_rgba(&self) -> String
    {
        return format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a as f32 / 255f32);
    }
}

impl From< &image::Rgba<u8> > for Pixel
{
    fn from(rgb_pixel: &image::Rgba<u8>) -> Self
    {
        return Pixel{ r: rgb_pixel.0[0], g: rgb_pixel.0[1], b: rgb_pixel.0[2], a: rgb_pixel.0[3] };
    }
}

impl ToString for Pixel
{
    fn to_string(&self) -> String
    {
        return format!("({}, {}, {}, {})", self.r, self.g, self.b, self.a);
    }
}
