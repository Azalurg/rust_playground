// use walkdir::WalkDir;
// use id3::{Tag, TagLike};

// const PATH: &str = "...";

// fn is_hidden(entry: &walkdir::DirEntry) -> bool {
//     entry
//         .file_name()
//         .to_str()
//         .map(|s| s.starts_with('.'))
//         .unwrap_or(false)
// }

// fn main() {
//     let walker = WalkDir::new(PATH).into_iter();
//     for entery in walker.filter_entry(|e| !is_hidden(e)) {
//         // let path = entery.unwrap().path();
//         println!("File extension: {:?}", entery.unwrap().path().extension());
//         // let tag = Tag::read_from_path(entery.unwrap().path());
//         // match tag {
//         //     Ok(tag) => {
//         //         // Print metadata information
//         //         println!("Title: {}", tag.title().unwrap_or("Unknown"));
//         //         println!("Artist: {}", tag.artist().unwrap_or("Unknown"));
//         //         println!("Album Artist: {}", tag.album_artist().unwrap_or("Unknown"));
//         //         println!("Album: {}", tag.album().unwrap_or("Unknown"));
//         //         println!("Year: {:?}", tag.year());
//         //         println!("Genre: {:?}", tag.genre());
//         //     }
//         //     Err(e) => eprintln!("Error reading ID3 tag: {}", e),
//         // }
//     }
// }

// use std::fs::File;
// use std::io::Write;

// use rust_embed::RustEmbed;
// use tempfile::TempDir;
// use walkdir::WalkDir;

// #[derive(RustEmbed)]
// #[folder = "data/"]
// struct Asset;

// fn main(){

//     let tem_dir = TempDir::new().unwrap();
//     let templates_path = tem_dir.path();

//     for file in Asset::iter() {
//         let mut temp_file = File::create(templates_path.join(file.as_ref())).unwrap();

//         let file_str = std::str::from_utf8(Asset::get(file.as_ref()).unwrap().data.as_ref()).unwrap().to_string();
//         writeln!(temp_file, "{}", file_str);

//         print!("{file_str}")
//     }

//     let walker = WalkDir::new(templates_path).into_iter();

//     for entery in walker{
//                 println!("{}", entery.unwrap().path().display());
//             }

// }

use std::collections::HashMap;

use walkdir::WalkDir;
fn main() {
    let path = "...";
    let mut dict = HashMap::new();

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            match entry.path().extension() {
                Some(ext) => {
                    let count = dict.entry(ext.to_str().unwrap().to_string()).or_insert(0);
                    *count += 1;
                }
                None => (),
            }
        }
    }

    for (key, value) in dict.iter() {
        println!("{}: {}", key, value);
    }
}
