use image::ImageError;

use std::path::Path;
use crate::pixel::Pixel;

/// given the path to the file, returns the css to replicate it. standlone is true if the program
/// will generate a standalone html file, false if not
pub fn css_from_filepath(filepath: &String, standalone: bool) -> Result<String, ImageError>
{
    // open image file
    let filepath: &Path = &Path::new(filepath.trim());
    let image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::open(filepath)?.to_rgba8();

    // setup html tag and css
    let mut css: String = if standalone
    {
        "<div style=\"width: 1px; height: 1px; box-shadow: "
    }
    else
    {
        ".img2css {\n\twidth: 1px;\n\theight: 1px;\n\tbox-shadow: "
    }.to_string();

    // add each border-shadow pixel
    for (x, y, pixel) in image.enumerate_pixels()
    {
        css.push_str( &format!("{}px {}px {}, ", x, y, Pixel::from(pixel).to_css_rgba()).to_string() );
    }

    // removes last comma and space
    css.pop();
    css.pop();

    if standalone
    {
        // close style and curly braces
        css.push_str(";\"></div>");
    }
    else
    {
        // close style and div tag
        css.push_str(";\"\n}");
    }

    return Ok(css);
}
