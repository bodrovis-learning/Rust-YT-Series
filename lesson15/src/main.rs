#![warn(clippy::all, clippy::pedantic)]

use pulldown_cmark::{Options, Parser};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

enum ProcessorMessage {
    Success(String),
    Error(String),
}

fn all_md_files() -> std::io::Result<Vec<PathBuf>> {
    let mut md_files_path = env::current_dir()?;

    md_files_path.push("md");

    let entries = fs::read_dir(md_files_path)?;

    let mut paths = Vec::new();

    for entry in entries {
        paths.push(entry?.path());
    }

    Ok(paths)
}

fn convert_parallel(
    files: Vec<PathBuf>,
    tx: mpsc::Sender<ProcessorMessage>,
) -> Vec<thread::JoinHandle<()>> {
    files
        .into_iter()
        .enumerate()
        .map(|(index, file)| {
            let new_tx = tx.clone();

            thread::spawn(move || {
                // let contents = fs::read_to_string(&file).expect("Failed to read file");
                let contents = match fs::read_to_string(&file) {
                    Ok(contents) => contents,
                    Err(e) => {
                        new_tx
                            .send(ProcessorMessage::Error(format!(
                                "Failed to read file {file:?}: {e}"
                            )))
                            .unwrap();

                        return;
                    }
                };

                let parser = Parser::new_ext(&contents, Options::empty());
                let mut html_output = String::new();

                pulldown_cmark::html::push_html(&mut html_output, parser);

                let def = format!("file{index}");
                let current_file_name = file
                    .file_name()
                    .unwrap_or(OsStr::new(&def))
                    .to_str()
                    .unwrap();

                let mut html_file = env::current_dir().expect("Can't read current dir!");
                html_file.push("html");
                html_file.push(current_file_name);
                html_file.set_extension("html");

                let mut output = fs::File::create(html_file).expect("Can't create HTML file");

                output
                    .write_all(html_output.as_bytes())
                    .expect("Can't write to HTML file!");

                new_tx
                    .send(ProcessorMessage::Success(format!(
                        "File {file:?} is processed!"
                    )))
                    .expect("Failed to send message");
            })
        })
        .collect()
}

fn main() {
    match all_md_files() {
        Ok(files) => {
            let (tx, rx) = mpsc::channel();

            let processors = convert_parallel(files, tx);

            for (index, handle) in processors.into_iter().enumerate() {
                match handle.join() {
                    Ok(()) => println!("Processor {index} is finished!"),
                    Err(e) => {
                        if let Some(s) = e.downcast_ref::<String>() {
                            println!("Thread {index} panicked: {s:?}");
                        } else {
                            println!("Unknown error when processing a thread {index}");
                        }
                    }
                }
            }

            for received in rx {
                match received {
                    ProcessorMessage::Success(msg) => println!("Message incoming: {msg}"),
                    ProcessorMessage::Error(e) => println!("Error incoming: {e}"),
                }
            }
        }
        Err(error) => println!("There was an error when collecting MD files: {error:?}"),
    }
}
