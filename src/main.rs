#![allow(non_snake_case)]
#![windows_subsystem = "windows"]

use tempfile::tempdir;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::process::Command;
use std::{thread, time};


fn main() {
    let bundled_soundvolumeview = include_bytes!("../data/soundvolumeview-x64/SoundVolumeView.exe");

    // Create a directory inside of `std::env::temp_dir()`
    let dir = tempdir().unwrap();
    let soundvolumeview_path = dir.path().join("SoundVolumeView.exe");
    let audioout_path = dir.path().join("audioout.txt");
    let mut file = File::create(&soundvolumeview_path).unwrap();
    file.write_all(bundled_soundvolumeview).unwrap();
    drop(file);

    change_audio_device(soundvolumeview_path.to_str().unwrap(), audioout_path.to_str().unwrap());

    drop(dir);
}

fn change_audio_device(soundvolumeview_path: &str, audioout_path: &str) {
    Command::new(soundvolumeview_path)
        .arg("/RunAsAdmin")
        .arg("/scomma")
        .arg(audioout_path)
        .status().unwrap();

    thread::sleep(time::Duration::from_millis(500));

    let mut audioout_file = File::open(audioout_path).unwrap();
    let mut audioout_contents = String::new();
    audioout_file.read_to_string(&mut audioout_contents).unwrap();

    let mut default_audio_device = "";
    for line in audioout_contents.trim().split('\n') {
        if line.contains("Render,Render") {
            default_audio_device = line.split(',').collect::<Vec<&str>>()[0];
        }
    }

    match default_audio_device {
        "Headset" => {
            Command::new(soundvolumeview_path)
                .arg("/RunAsAdmin")
                .arg("/SetDefault")
                .arg("Speakers")
                .arg("all")
                .status().unwrap();
        },
        "Speakers" => {
            Command::new(soundvolumeview_path)
                .arg("/RunAsAdmin")
                .arg("/SetDefault")
                .arg("Headset")
                .arg("all")
                .status().unwrap();
        },
        _ => ()
    }

    thread::sleep(time::Duration::from_millis(500));
}
