use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::process;
use std::{env, fs};

use downloader::Downloader;
use serde::{
    de::{self, Unexpected},
    Deserialize, Deserializer,
};

#[derive(Debug, Deserialize)]
struct Record {
    id: usize,
    uploader_id: usize,
    created_at: String,
    md5: String,
    source: Option<String>,
    rating: char,
    image_width: isize,
    image_height: isize,
    tag_string: String,
    locked_tags: String,
    fav_count: usize,
    file_ext: String,
    parent_id: Option<usize>,
    change_seq: usize,
    approver_id: Option<usize>,
    file_size: usize,
    comment_count: isize,
    description: Option<String>,
    duration: Option<f32>,
    updated_at: String,
    #[serde(deserialize_with = "bool_from_string")]
    is_deleted: bool,
    #[serde(deserialize_with = "bool_from_string")]
    is_pending: bool,
    #[serde(deserialize_with = "bool_from_string")]
    is_flagged: bool,
    score: isize,
    up_score: usize,
    down_score: isize,
    #[serde(deserialize_with = "bool_from_string")]
    is_rating_locked: bool,
    #[serde(deserialize_with = "bool_from_string")]
    is_status_locked: bool,
    #[serde(deserialize_with = "bool_from_string")]
    is_note_locked: bool,
}

fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_ref() {
        "t" => Ok(true),
        "f" => Ok(false),
        _ => Ok(true),
    }
}

fn main() {
    let file_path = env::args().nth(1).expect("No file path inputted.");
    let file = File::open(file_path).expect("Could not open file.");
    let reader = BufReader::new(file);

    let mut rdr = csv::Reader::from_reader(reader);
    let posts = rdr
        .deserialize()
        .map(|x| x.expect("Could not deserialize"))
        // filter it and do whatever you want i guess
        .filter(|x: &Record| !x.is_deleted && x.tag_string.contains("bondage"))
        .take(100)
        // Convert to url
        .map(|x| {
            format!(
                "https://static1.e621.net/data/{}/{}/{}.{}",
                x.md5[0..2].to_owned(),
                x.md5[2..4].to_owned(),
                x.md5,
                x.file_ext
            )
        });

    fs::create_dir("output").unwrap();

    let mut downloader = Downloader::builder()
        .download_folder(std::path::Path::new("output/"))
        .parallel_requests(16)
        .build()
        .unwrap();

    let result = downloader
        .download(
            &posts
                .map(|x| downloader::Download::new(&x))
                .collect::<Vec<_>>(),
        )
        .unwrap();

    for r in result {
        match r {
            Err(e) => println!("Error: {}", e),
            Ok(s) => println!("Success: {}", &s),
        };
    }
}
