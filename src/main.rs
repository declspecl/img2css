pub mod pixel;

pub mod img;
use img::css_from_filepath;

use std::fs;
use std::io::{self, Write};
use std::env::{self, Args};

fn main()
{
    let args: Args = env::args();
    let args: Vec<String> = args.collect();

    // no cli arguments provided
    if args.len() <= 1
    {
        print!("Input the path of the image file: ");
        match io::stdout().flush()
        {
            Ok(..) => {},
            Err(err) => println!("An error occured when trying to flush the output buffer: {}", err),
        }

        let mut filepath: String = String::new();
        match io::stdin().read_line(&mut filepath)
        {
            Ok(..) => {},
            Err(err) =>
            {
                println!("An error occured when reading the image path: {}", err);
                return;
            }
        }

        print!("Do you want to create a standalone html file(y)? Or a css file(n)? ");
        match io::stdout().flush()
        {
            Ok(..) => {},
            Err(err) => println!("An error occured when trying to flush the output buffer: {}", err),
        }

        let mut standalone_option: String = String::new();
        match io::stdin().read_line(&mut standalone_option)
        {
            Ok(..) => {},
            Err(err) =>
            {
                println!("An error occured when receiving input for the standalone option: {}", err);
                return;
            }
        }

        let standalone_option: String = standalone_option.trim().to_lowercase();

        if standalone_option == "y"
        {
            let mut html: String = String::from("<!DOCTYPE html>\n<html>\n\t<head>\n\t\t<title>img2css result</title>\n\t</head>\n\t<body style=\"margin: 0 0 0 0;\">\n\t\t");
            html.push_str(match css_from_filepath(&filepath, true)
            {
                Ok(css) => css,
                Err(err) =>
                {
                    println!("An error occured when opening the image file: {}", err);
                    return;
                }
            }.as_str());

            html.push_str("\n\t</body>\n</html>");

            match fs::write("img2css.html", html)
            {
                Ok(..) => {},
                Err(err) =>
                {
                    println!("An error occured when writing css to img2css.html: {}", err);
                    return;
                }
            }

            println!("Successfully wrote code to img2css.html");
        }
        else if standalone_option == "n"
        {
            let css: String = match css_from_filepath(&filepath, false)
            {
                Ok(css) => css,
                Err(err) =>
                {
                    println!("An error occured when opening the image file: {}", err);
                    return;
                }
            };

            match fs::write("img2css.css", css)
            {
                Ok(..) => {},
                Err(err) =>
                {
                    println!("An error occured when writing css to img2css.css: {}", err);
                    return;
                }
            }

            println!("Successfully wrote css to img2css.css");
        }
        else
        {
            println!("Please input y or n. They are the only valid options");
            return;
        };
    }
    else // cli arguments provided
    {
        let filepath: &String = &args[1];

        if args.len() == 2
        {
            if args[1] == "-h" || args[1] == "--help"
            {
                println!("img2css help:");
                println!("command\t\t\taction");
                println!("-------\t\t\t------");
                println!("img2css.exe\t\texecutes the interactive img2css client");
                println!("img2css.exe [path]\texecutes img2css and writes the resulting css into img2css.css");
                println!("img2css.exe [path] -s\texecutes img2css and writes the resulting css into a standalone img2css.html");
            }
            else
            {
                let css: String = match css_from_filepath(&filepath, false)
                {
                    Ok(css) => css,
                    Err(err) =>
                    {
                        println!("An error occured when opening the image file: {}", err);
                        return;
                    }
                };

                match fs::write("img2css.css", css)
                {
                    Ok(..) => {},
                    Err(err) =>
                    {
                        println!("An error occured when writing css to img2css.css: {}", err);
                        return;
                    }
                }

                println!("Successfully wrote css to img2css.css");
            }
        }
        else if args.len() == 3 // image path and a cli argument
        {
            // standalone option
            if args[2] == "-s" || args[2] == "--standalone"
            {
                let mut html: String = String::from("<!DOCTYPE html>\n<html>\n\t<head>\n\t\t<title>img2css result</title>\n\t</head>\n\t<body style=\"margin: 0 0 0 0;\">\n\t\t");
                html.push_str(match css_from_filepath(&filepath, true)
                {
                    Ok(css) => css,
                    Err(err) =>
                    {
                        println!("An error occured when opening the image file: {}", err);
                        return;
                    }
                }.as_str());

                html.push_str("\n\t</body>\n</html>");

                match fs::write("img2css.html", html)
                {
                    Ok(..) => {},
                    Err(err) =>
                    {
                        println!("An error occured when writing css to img2css.html: {}", err);
                        return;
                    }
                }

                println!("Successfully wrote code to img2css.html");
            }
            else
            {
                println!("img2css only supports one commandline argument: -s");
            }
        }
        else
        {
            println!("img2css only supports one commandline argument: -s");
        }
    }
}
