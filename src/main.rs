use colored::*;
use dialoguer::{
    theme::ColorfulTheme,
    Confirm,
    Input,
    Select,
};
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
struct Config {
    scale: [f32; 3],
    position: [f32; 3],
    rotation: [f32; 3],
    body_part: String,
    backslot_model: String,
    backslot_model_armored: String,
    relocate_with_chestplate: bool,
}

fn parse_vector(input: &str) -> Result<[f32; 3], String> {
    let parts: Vec<&str> = input.split(',').collect();

    if parts.len() != 3 {
        return Err("Expected exactly 3 values".into());
    }

    let mut values = [0.0; 3];

    for (i, part) in parts.iter().enumerate() {
        values[i] = part
            .trim()
            .parse::<f32>()
            .map_err(|_| format!("Invalid number: {}", part))?;
    }

    Ok(values)
}

fn main() {
    let theme = ColorfulTheme::default();

    println!();
    println!(
        "{}",
        "Backslot Configuration Generator"
            .bright_cyan()
            .bold()
    );

    println!(
        "{}",
        "Generate attachment configuration JSON\n"
            .dimmed()
    );

    let scale_input: String = Input::with_theme(&theme)
        .with_prompt("Scale")
        .default("1.0,1.0,1.0".into())
        .interact_text()
        .unwrap();

    let position_input: String = Input::with_theme(&theme)
        .with_prompt("Position")
        .default("0.0,0.0,0.0".into())
        .interact_text()
        .unwrap();

    let rotation_input: String = Input::with_theme(&theme)
        .with_prompt("Rotation")
        .default("0.0,0.0,0.0".into())
        .interact_text()
        .unwrap();

    let body_parts = vec![
        "body",
        "head",
        "left_arm",
        "right_arm",
        "left_leg",
        "right_leg",
    ];

    let body_part_selection = Select::with_theme(&theme)
        .with_prompt("Body Part")
        .items(&body_parts)
        .default(0)
        .interact()
        .unwrap();

    let body_part = body_parts[body_part_selection].to_string();

    let backslot_model: String = Input::with_theme(&theme)
        .with_prompt("Backslot Model")
        .default("".into())
        .interact_text()
        .unwrap();

    let backslot_model_armored: String = Input::with_theme(&theme)
        .with_prompt("Armored Backslot Model")
        .default("".into())
        .interact_text()
        .unwrap();

    let relocate_with_chestplate = Confirm::with_theme(&theme)
        .with_prompt("Relocate With Chestplate")
        .default(true)
        .interact()
        .unwrap();

    let config = Config {
        scale: parse_vector(&scale_input).unwrap(),
        position: parse_vector(&position_input).unwrap(),
        rotation: parse_vector(&rotation_input).unwrap(),
        body_part,
        backslot_model,
        backslot_model_armored,
        relocate_with_chestplate,
    };

    let json = serde_json::to_string_pretty(&config).unwrap();

    fs::write("output.json", &json).unwrap();

    println!();
    println!(
        "{} {}",
        "Status".bright_green().bold(),
        "Configuration written to output.json"
    );

    println!();
    println!("{}", "Preview".bright_magenta().bold());
    println!("{}", "─".repeat(48).dimmed());
    println!("{}", json.white());
    println!("{}", "─".repeat(48).dimmed());
}
