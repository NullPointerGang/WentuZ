use dioxus::prelude::*;

use WentuZ_Backend::{Player, Track};
use std::sync::{Arc, Mutex};
use std::fs;
use std::time::Duration;

#[component]
pub fn Footer() -> Element {
    static DEFAULT_COVER: Asset = asset!("/assets/img/default-album-artwork.png", ImageAssetOptions::new().with_avif());
    let mut current_track_artist = use_signal(|| "Uknown Artist");
    let mut current_track_title = use_signal(|| "Uknown Title");
    let player = Arc::new(Mutex::new(Player::new()));

    rsx! {
        footer {
            div { id: "current_track_info",
                img { src: DEFAULT_COVER, id: "current_track_cover" }
                div { id: "current_track_credits",
                    p { id:"current_track_title", "{current_track_title}" }
                    p { id:"current_track_artist", "{current_track_artist}" }
                }
            }
            div {  }
            div { 
                button {
                    class: "playback-buttons",
                    onclick: {
                        let player = player.clone();
                        move |_| {
                            let player_lock = player.lock().unwrap();
                            player_lock.play_previous();
                        }
                    },
                    "back"
                }
                button { class: "playback-buttons", 
                    onclick: {
                        let player = player.clone();
                        move |evt| {
                            let player_lock = player.lock().unwrap();
                            player_lock.pause();
                        }
                    },
                    "pause"
                }
                button { class: "playback-buttons", 
                    onclick: {
                        let player = player.clone();
                        move |evt| {
                            let player_lock = player.lock().unwrap();
                            player_lock.resume();
                        }
                    },
                    "resume"
                }
                button { class: "playback-buttons", 
                    onclick: {
                        let player = player.clone();
                        move |evt| {
                            let player_lock = player.lock().unwrap();
                            player_lock.play_next();
                        }
                    },
                    "skip"
                }
                input {  
                    r#type: "file",
                    accept: ".flac,audio/*",
                    onchange: move |evt| {
                        if let Some(file_engine) = evt.files() {
                            let files = file_engine.files();
                            for file_name in files {
                                println!("{}", file_name);
                                let file_path = file_name.to_string();
                                let player_button = player.clone();
                                add_button(&player_button, &file_path);
                                play_lol(&player);
                            }
                        }
                    }
                }
            }
            div {  }
        }
    }
}


fn add_button(player: &Arc<Mutex<Player>>, track_path: &str) {
    if let Ok(track) = load_track(track_path) {
        let player = player.lock().unwrap();
        player.add_to_queue(track);
        println!("Added track to queue: {}", track_path);
    } else {
        println!("Failed to load track: {}", track_path);
    }
}

fn play_lol(player: &Arc<Mutex<Player>>){
    let mut player = player.lock().unwrap();
    player.auto_play();
}

fn load_track(file_path: &str) -> Result<Track, String> {
    match fs::read(file_path) {
        Ok(track_data) => Ok(Track::new(
            file_path.to_string(),
            track_data,
            None,
            None,
            Some(Duration::new(240, 0)),
            Some(file_path.to_string()),
        )),
        Err(_) => Err("Failed to read the file.".to_string()),
    }
}