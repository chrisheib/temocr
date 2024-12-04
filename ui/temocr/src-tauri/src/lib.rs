// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_ocr_results() -> String {
    println!("rust: get_ocr_results called");
    let ad = get_global_var();
    serde_json::to_string(&ad).unwrap_or_default()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    std::thread::spawn(ocr_main_loop);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![get_ocr_results])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use std::sync::Mutex;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
struct Appdata {
    tem1: Option<Value>,
    tem2: Option<Value>,
}

fn init_global_var() {
    unsafe {
        GLOBAL_VAR = Some(Mutex::new(Appdata::default()));
    }
}

fn set_global_var(value: Appdata) {
    unsafe {
        if let Some(ref mut m) = GLOBAL_VAR {
            let mut data = m.lock().unwrap();
            *data = value;
        }
    }
}

fn get_global_var() -> Appdata {
    unsafe {
        if let Some(ref m) = GLOBAL_VAR {
            let data = m.lock().unwrap();
            data.clone()
        } else {
            Appdata::default()
        }
    }
}

static mut GLOBAL_VAR: Option<Mutex<Appdata>> = None;

use std::thread;
use std::{collections::HashMap, time::Duration};

use color_eyre::Result;
use image::{
    imageops::{resize, FilterType},
    ImageBuffer, Pixel, RgbImage, Rgba,
};
use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;
use serde::{Deserialize, Serialize};
use serde_json::Value;
// use rten::Model;
// #[allow(unused)]
// use rten_tensor::prelude::*;

use xcap::Window;

fn take_temtem_screenshot() -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let windows = Window::all().unwrap();

    let Some(w) = windows.iter().find(|w| w.title() == "Temtem") else {
        return None;
    };

    // println!(
    //     "Window: {:?} {:?} {:?}",
    //     w.title(),
    //     (w.x(), w.y(), w.width(), w.height()),
    //     (w.is_minimized(), w.is_maximized())
    // );

    let image = w.capture_image().unwrap();
    image.save("autoscreen.png").unwrap();
    return Some(image);
}
type WeakMap = HashMap<Types, HashMap<Types, f32>>;
// type WeakMap = HashMap<String, HashMap<String, f32>>;

fn ocr_main_loop() -> Result<()> {
    let wait_time = Duration::from_secs(5);
    std::env::set_var("RUST_BACKTRACE", "1");
    color_eyre::install()?;
    init_global_var();
    let mut all_tems: Value =
        serde_json::from_str(include_str!("../data/knownTemtemSpecies.json")).unwrap();
    let attack_type_modifier: WeakMap =
        serde_json::from_str(include_str!("../data/weaknesses.json")).unwrap();

    let defense_type_modifier = get_defense_modifier(&attack_type_modifier);
    for t in all_tems.as_array_mut().unwrap() {
        let t = t.as_object_mut().unwrap();
        let comtype1 = t["types"][0].as_str().unwrap();
        let comtype2 = t["types"][1].as_str();
        let types = if let Some(t2) = comtype2 {
            vec![comtype1, t2]
        } else {
            vec![comtype1]
        };
        let w = find_weakness(&types, &defense_type_modifier)?;

        t.extend([(
            "type_modifier".to_string(),
            serde_json::to_value(w).unwrap(),
        )]);
    }

    dbg!(find_weakness(&["Water", "Nature"], &defense_type_modifier)?);

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
        let Some(img) = take_temtem_screenshot() else {
            set_global_var(Appdata::default());
            thread::sleep(wait_time);
            continue;
        };

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

        let mut tems = vec![];

        for line in line_texts
            .iter()
            .flatten()
            // Filter likely spurious detections. With future model improvements
            // this should become unnecessary.
            .filter(|l| l.to_string().len() > 1)
        {
            let tem = all_tems.as_array().unwrap().iter().find(|t| {
                t["name"].as_str().unwrap()
                    == line
                        .to_string()
                        .split_whitespace()
                        .next()
                        .unwrap_or_default()
            });
            if let Some(t) = tem {
                let comtype1 = t["types"][0].as_str().unwrap();
                let comtype2 = t["types"][1].as_str();
                let types = if let Some(t2) = comtype2 {
                    vec![comtype1, t2]
                } else {
                    vec![comtype1]
                };
                println!(
                    "{line} ({types:?}), {:?}",
                    find_weakness(&types, &defense_type_modifier)?
                );
                tems.push(t);
            } else {
                println!("{line}");
            }
        }
        let mut ad = get_global_var();
        if tems.len() == 1 {
            ad.tem1 = Some(tems[0].to_owned());
            ad.tem2 = None;
        } else if tems.len() == 2 {
            ad.tem1 = Some(tems[0].to_owned());
            ad.tem2 = Some(tems[1].to_owned());
        }
        set_global_var(ad);
        std::thread::sleep(wait_time);
    }
}

#[derive(Debug, Deserialize, Hash, PartialEq, Eq, Clone, Copy, Serialize)]
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

fn find_weakness(t: &[&str], weaknesses: &WeakMap) -> Result<HashMap<Types, f32>> {
    let type1 = t[0];
    let type1 = format!(r#""{type1}""#);
    let type1: Types = serde_json::from_str(&type1)?;
    let mut modifier = weaknesses.get(&type1).unwrap().clone();

    if t.len() == 2 {
        let type2 = t[1];
        let type2 = format!(r#""{type2}""#);
        let type2: Types = serde_json::from_str(&type2)?;
        let modifier2 = weaknesses.get(&type2).unwrap();
        for m in modifier2 {
            *modifier.entry(*m.0).or_default() *= m.1;
        }
    }

    let mut values = modifier
        .into_iter()
        .filter(|m| m.1 != 1.0)
        .map(|v| (v.0, v.1))
        .collect::<Vec<(Types, f32)>>();
    values.sort_by(|a, b| b.1.total_cmp(&a.1));

    let out = values
        .into_iter()
        // .map(|m| format!("{:?}: {}", m.0, m.1))
        .collect();

    Ok(out)
    // Ok(out.join(", "))
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
