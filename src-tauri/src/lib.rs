use tauri::{command, Emitter, Manager, Window, WindowEvent, Listener};
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{Write, Read};
use std::fs::OpenOptions;

const VOW_CMDS: &[&str] = &[
    "cycle_next",
    "cycle_prev",
    "live",
    "sacrifice",
    "fight",
    "return",
    "locate",
    "leech",
    "run",
    "summon",
    "explode",
    "sleep"
];


#[command]
async fn start_vow_macro(app: tauri::AppHandle, window: Window) {
    let vow_window = tauri::WebviewWindowBuilder::new(
        &app,
        "vow-main",
        tauri::WebviewUrl::App("vow-main.html".into()),
    )
        .inner_size(200.0, 80.0)
        .title("Vow Macro")
        .resizable(true)
        .minimizable(false)
        .maximizable(false)
        .always_on_top(true)
        .devtools(true)
        .build()
        .unwrap();

    vow_window.on_window_event(move |event| {
        if let WindowEvent::CloseRequested { .. } = event {
            // Emit a custom event to notify the main window
            let result = window.emit("vow-window-closed", ());

            match result {
                Ok(_) => println!("Emitted successfully"),
                Err(e) => println!("Error emitting: {e}"),
            }
        }
    })
}

//i needed a way to close the window from js and this is the only way i could figure out
#[command]
async fn stop_vow_macro(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("vow-main") {
        let _ = window.close();
    }
}

fn append_to_file(file_path: &Path, content: &str) -> Result<(), String> {
    // Open the file in append mode
    let mut file = OpenOptions::new()
        .append(true) // Open the file for appending
        .create(true) // Create the file if it doesn't exist
        .open(file_path)
        .map_err(|e| format!("Failed to open file: {}", e))?;

    // Write the content to the file
    file.write_all(content.as_bytes())
        .map_err(|e| format!("Failed to write to file: {}", e))?;

    Ok(())
}

fn update_key(file_path: &PathBuf, name: &str, new_key: &str) -> std::io::Result<()> {
    // Read the file content into a string
    let content = fs::read_to_string(file_path)?;

    println!("Action: {name}");
    println!("New key: {new_key}");
    // Process the content line by line
    let lines = content.lines();
    println!("Lines: {lines:?}");
    let updated_content: String = content
        .lines() // Iterate over lines
        .map(|line| {
            if let Some((line_name, _)) = line.split_once(": ") {
                println!("LINE NAME: {line_name}");
                if line_name.trim() == name {
                    // If the name matches, update the key
                    println!("match: {line_name}");
                    return format!("{}: {}", line_name, new_key);
                }
            }
            line.to_string() // Otherwise, keep the line as is
        })
        .collect::<Vec<String>>() // Collect lines into a vector
        .join("\n"); // Join the lines back into a single string

    // Write the updated content back to the file
    fs::write(file_path, updated_content)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_vow_macro, stop_vow_macro])
        .setup(|app| {
            #[cfg(desktop)]
            {
                //check for keybinds file
                let cur_exe = std::env::current_exe().unwrap();
                let parent_dir = cur_exe.parent().unwrap();
                
                let mut save_path = PathBuf::from(parent_dir);
                save_path.push("keybinds.txt");
                
                let exists = save_path.exists();
                println!("Exists: {exists}");    

                if !save_path.exists() {
                    println!("Keybind save file does not exist, creating new...");
                    let _ = fs::write(&save_path, "");
                    for cmd in VOW_CMDS {
                        let mut content = String::from(*cmd);
                        content.push_str("_key: None\n");
                        let _ = append_to_file(&save_path, &content);
                    }
                }

                //listener for keybind changes
                let main_window = app.get_webview_window("main").unwrap();

                let _ = main_window.listen("keybind_changed", move |event| {
                    let payload = event.payload()
                        .replace("{", "")
                        .replace("}", "")
                        .replace("\"", "");
                    println!("{payload}");
                    let chars: Vec<char> = payload.chars().collect();
                    
                    //use this to get the words and shit
                    let mut start_index = 0;

                    let mut collected_words: Vec<String> = vec!();

                    for (i, char) in chars.clone().into_iter().enumerate() {
                        if char == ':' { //start of word is after this
                            start_index = i + 1;
                        }
                        if char == ',' { //end of word
                            let mut word = String::new();
                            let end_index = i;
                            for character in chars.iter().take(end_index).skip(start_index) {
                                word.push(*character);
                            }

                            collected_words.push(word);
                        }
                        if i == chars.len() - 1 {
                            let mut word = String::new();
                            for character in chars.iter().take(chars.len() - 1).skip(start_index) {
                                word.push(*character);
                            }
                            word.push(*chars.last().unwrap());
                            collected_words.push(word);
                        }
                    }
                    
                    let key = &collected_words[0];
                    let action = &collected_words[1].replace("-", "_");

                    //update the old line
                    let _ = update_key(&save_path, action, key);
                });
            }
            Ok(())
        })
    .run(tauri::generate_context!())
        .expect("error while running tauri application");
    }
