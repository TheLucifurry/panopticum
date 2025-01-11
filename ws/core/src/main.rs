// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use panopticum_lib::utils::cmd::is_command_available;

fn main() {
    colog::init();
    log::info!("Logger initialized!");

    log::info!("Is available yt-dlp: {}", is_command_available("yt-dlp", "--version"));
    log::info!("Is available ffmpeg: {}", is_command_available("ffmpeg", "-version"));

    panopticum_lib::run()
}
