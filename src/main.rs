use genpdf::style::{self, Style};
use genpdf::{elements, Element as _};
use genpdf::{self, Scale};
use genpdf::elements::Image;
use std::path::Path;
use anyhow::Result;
use image::io::Reader;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn get_image_dimensions(file_path: &str) -> Result<(u32, u32)> {
    let path = Path::new(file_path);
    let reader = Reader::open(path)?;
    let dimensions = reader.into_dimensions()?;
    Ok(dimensions)
}

fn main() {

    // clean prev
    match fs::remove_file("output.pdf") {
        Ok(_) => {},
        Err(_) => {print!("File not found, creating a new one\n")},
    };
    let img_count: usize = 5;
    let mut img_paths:Vec<&str> = vec!["./images/test.jpg"];
    img_paths.push("./images/test2.png");
    img_paths.push("./images/test3.jpg");
    img_paths.push("./images/test4.jpg");
    img_paths.push("./images/test5.jpeg");

    // font
    let font_family = genpdf::fonts::from_files("./fonts", "LiberationSans", None)
        .expect("Failed to load font family");
    let mut doc = genpdf::Document::new(font_family);
    doc.set_title("Demo document");

    // title & subtitle stiles
    let title_style: Style = style::Style::new().bold().with_font_size(28).italic();
    let subtitle_style: Style = style::Style::new().bold().with_font_size(14);

    // decorator
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    // text
    doc.push(genpdf::elements::Paragraph::new("BetterLoop").aligned(
        genpdf::Alignment::Center).styled(title_style).padded(genpdf::Margins::trbl(0, 0, 5, 0)));
    let text_fields = 4;
    for i in 1..text_fields + 1 {

        //get text
        let text_path: String = format!("./texts/{}.txt", i);
        let input = File::open(text_path).unwrap();
        let mut buffer = BufReader::new(input);

        // subtitle text
        let mut subtitle_text: String = String::new();
        buffer.read_line(&mut subtitle_text).unwrap();
        subtitle_text.pop();
        let subtitle = elements::PaddedElement::new(elements::Paragraph::new(subtitle_text)
        .styled(subtitle_style), genpdf::Margins::trbl(5, 2, 5, 10));
        doc.push(subtitle);

        // content
        for line in buffer.lines() {
            let mut content: String = match line {
                Ok(String) => String,
                Err(error) => panic!("Problem opening the file: {:?}", error),
                
            };

            // sterg caracterele dubioase
            content.retain(|c| c.is_ascii_punctuation() || c.is_alphanumeric() || c == ' ');

            // verific daca este nume
            if content.ends_with('^') {
                content.pop();
                let section_content = elements::PaddedElement::new(
                    elements::Paragraph::new(content).styled(genpdf::style::Style::new().bold().italic()), 
                    genpdf::Margins::trbl(2, 50, 2, 10));
                    doc.push(section_content);
            } else {
            let section_content = elements::PaddedElement::new(
                elements::Paragraph::new(content), genpdf::Margins::trbl(2, 10, 2, 10));
                doc.push(section_content);
            }
        }
    }


    // images
    doc.push(genpdf::elements::PageBreak::new());
    doc.push(elements::PaddedElement::new(elements::Paragraph::new("Imagini de la oameni")
    .styled(subtitle_style), genpdf::Margins::trbl(5, 2, 5, 10)));
    let mut img_scale: Scale;
    let mut img: Image;
    let mut hor_images: Vec<usize> = vec![];
    let mut vert_images: Vec<usize> = vec![];
    for i in 0..img_count {
        let (x_img, y_img) = get_image_dimensions(img_paths[i]).unwrap();
        if (x_img as f64) / (y_img as f64) < 1.2 {
            vert_images.push(i);
        } else {
            hor_images.push(i);
        }
    }
    for i in vert_images {
        // scales
        let (x_img, _) = get_image_dimensions(img_paths[i]).unwrap();
        let scaling_factor:f64 = 1900.0 / (x_img as f64);
        img_scale = (scaling_factor, scaling_factor).into();
        img = elements::Image::from_path(img_paths[i])
            .expect("Failed to load test image")
            .with_alignment(genpdf::Alignment::Center);
        img.set_scale(img_scale);
        doc.push(elements::PaddedElement::new(img.clone(),
            genpdf::Margins::trbl(10, 2, 5, 5),
        ));
        doc.push(genpdf::elements::PageBreak::new());
    }
    let mut j = 0;
    let len = hor_images.len();
    for i in hor_images {
        // scales
        j += 1;
        let (x_img, _) = get_image_dimensions(img_paths[i]).unwrap();
        let scaling_factor:f64 = 1900.0 / (x_img as f64);
        img_scale = (scaling_factor, scaling_factor).into();
        img = elements::Image::from_path(img_paths[i])
            .expect("Failed to load test image")
            .with_alignment(genpdf::Alignment::Center);
        img.set_scale(img_scale);
        doc.push(elements::PaddedElement::new(img.clone(),
            genpdf::Margins::trbl(10, 2, 0, 5),
        ));
        if j % 2 == 0 && j < len {
            doc.push(genpdf::elements::PageBreak::new());
        }
    }
    //final
    doc.render_to_file("output.pdf")
        .expect("Failed to write PDF file");

}
