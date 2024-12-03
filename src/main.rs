use std::{collections::HashMap, time::Duration};

use color_eyre::Result;
use image::{
    imageops::{resize, FilterType},
    ImageBuffer, Pixel, RgbImage, Rgba,
};
use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;
use serde::Deserialize;
use serde_json::Value;
// use rten::Model;
// #[allow(unused)]
// use rten_tensor::prelude::*;

use xcap::Window;

fn take_temtem_screenshot() -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let windows = Window::all().unwrap();

    let Some(w) = windows.iter().find(|w| w.title() == "Temtem") else {
        panic!("Temtem ist nicht offen")
    };

    // println!(
    //     "Window: {:?} {:?} {:?}",
    //     w.title(),
    //     (w.x(), w.y(), w.width(), w.height()),
    //     (w.is_minimized(), w.is_maximized())
    // );

    let image = w.capture_image().unwrap();
    image.save("autoscreen.png").unwrap();
    return image;
}
type WeakMap = HashMap<Types, HashMap<Types, f32>>;
// type WeakMap = HashMap<String, HashMap<String, f32>>;

fn main() -> Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");
    color_eyre::install()?;
    let tems: Value =
        serde_json::from_str(include_str!("../data/knownTemtemSpecies.json")).unwrap();
    let attack_type_modifier: WeakMap =
        serde_json::from_str(include_str!("../data/weaknesses.json")).unwrap();

    let defense_type_modifier = get_defense_modifier(&attack_type_modifier);

    // let args = parse_args()?;

    // Use the `download-models.sh` script to download the models.
    // let detection_model_path = file_path("data/text-detection.rten");
    // let rec_model_path = file_path("text-recognition.rten");

    // let detection_model = Model::load_file(detection_model_path)?;
    // let recognition_model = Model::load_file(rec_model_path)?;

    let detection_model = Model::load_static_slice(include_bytes!("../data/text-detection.rten"))?;
    let recognition_model =
        Model::load_static_slice(include_bytes!("../data/text-recognition.rten"))?;

    let engine = OcrEngine::new(OcrEngineParams {
        detection_model: Some(detection_model),
        recognition_model: Some(recognition_model),
        ..Default::default()
    })
    .unwrap();

    loop {
        // Read image using image-rs library, and convert to RGB if not already
        // in that format.
        // let img = image::open("image.png").unwrap();
        let img = take_temtem_screenshot();

        // Load the image from a file
        // let img = image::open("path/to/your/image.png").unwrap();

        // Get the dimensions of the image
        let (width, height) = img.dimensions();

        // Calculate the coordinates and dimensions of the desired region
        let upper_third_height = height / 3;
        let right_half_width = width / 2;
        let x_offset = right_half_width;
        let y_offset = 0;

        // Create a new image buffer for the cut-out region
        let mut cut_out: RgbImage = ImageBuffer::new(right_half_width, upper_third_height);

        // Copy the desired region to the new image buffer
        for y in 0..upper_third_height {
            for x in 0..right_half_width {
                let pixel = img.get_pixel(x + x_offset, y + y_offset);
                cut_out.put_pixel(x, y, pixel.to_rgb());
            }
        }

        // Iterate over the pixels
        for pixel in cut_out.pixels_mut() {
            let channels = pixel.channels_mut();

            let check_grey = channels[0] == channels[1] && channels[0] == channels[2];
            let check_bright = channels[0] > 200 && channels[1] > 200 && channels[2] > 200;

            // Check if the pixel is not white
            if !(check_bright && check_grey) {
                // Make the pixel black
                channels[0] = 0;
                channels[1] = 0;
                channels[2] = 0;
            }
        }

        cut_out.save("cut_out.png").unwrap();
        // Resize the cut-out region to half its original size
        let resized_cut_out = resize(
            &cut_out,
            cut_out.dimensions().0 / 3,
            cut_out.dimensions().1 / 3,
            FilterType::Lanczos3,
        );
        resized_cut_out.save("resized_cut_out.png").unwrap();

        let img = cut_out;

        // Apply standard image pre-processing expected by this library (convert
        // to greyscale, map range to [-0.5, 0.5]).
        let img_source = ImageSource::from_bytes(img.as_raw(), img.dimensions())?;
        let ocr_input = engine.prepare_input(img_source).unwrap();

        // Detect and recognize text. If you only need the text and don't need any
        // layout information, you can also use `engine.get_text(&ocr_input)`,
        // which returns all the text in an image as a single string.

        // Get oriented bounding boxes of text words in input image.
        let word_rects = engine.detect_words(&ocr_input).unwrap();

        // Group words into lines. Each line is represented by a list of word
        // bounding boxes.
        // let line_rects = engine.find_text_lines(&ocr_input, &word_rects);

        let line_rects = word_rects
            .iter()
            .map(|wr| vec![wr.to_owned()])
            .collect::<Vec<_>>();

        // Recognize the characters in each line.
        let line_texts = engine.recognize_text(&ocr_input, &line_rects).unwrap();

        for line in line_texts
            .iter()
            .flatten()
            // Filter likely spurious detections. With future model improvements
            // this should become unnecessary.
            .filter(|l| l.to_string().len() > 1)
        {
            let tem = tems.as_array().unwrap().iter().find(|t| {
                t["name"].as_str().unwrap()
                    == line
                        .to_string()
                        .split_whitespace()
                        .next()
                        .unwrap_or_default()
            });
            if let Some(t) = tem {
                let comtype = t["types"][0].as_str().unwrap();
                println!(
                    "{line} ({comtype}), {}",
                    find_weakness(&[comtype], &defense_type_modifier)?
                );
            } else {
                println!("{line}");
            }
        }
        std::thread::sleep(Duration::from_secs(5));
    }
}

#[derive(Debug, Deserialize, Hash, PartialEq, Eq, Clone, Copy)]
enum Types {
    Neutral,
    Wind,
    Earth,
    Water,
    Fire,
    Nature,
    Electric,
    Mental,
    Digital,
    Melee,
    Crystal,
    Toxic,
}

fn find_weakness(t: &[&str], weaknesses: &WeakMap) -> Result<String> {
    let t = t.first().unwrap();
    let t = format!(r#""{t}""#);
    let t: Types = serde_json::from_str(&t)?;
    let modifier = weaknesses.get(&t).unwrap();

    let out: Vec<String> = modifier
        .iter()
        .filter(|m| *m.1 != 1.0)
        .map(|m| format!("{:?}: {}", m.0, m.1))
        .collect();

    Ok(out.join(", "))
}

fn get_defense_modifier(attack_modifier: &WeakMap) -> WeakMap {
    let mut defense: HashMap<Types, HashMap<Types, f32>> = HashMap::new();

    for attacker in attack_modifier {
        for defender in attacker.1 {
            let defend_entry = defense.entry(*defender.0).or_default();
            defend_entry.insert(*attacker.0, *defender.1);
        }
    }

    defense
}
