use clap::{ValueHint, Parser};
use std::{arch::x86_64::_mm_lfence, fs, path::PathBuf};
use bytesize::ByteSize;
use walkdir::WalkDir;
use chrono::{Date, DateTime, Local};
use filetime::FileTime;

#[derive(Parser, Debug)]
#[command(name = "my_cli", version, about)]
struct Args {

    #[arg(value_hint = ValueHint::FilePath)]
    file: PathBuf,
}

struct DirStats {
    total_size: u64,
    file_count: u64,
    dir_count: u64,
}

fn analyze_dir(path: &PathBuf) -> DirStats {
    let mut stats = DirStats {
        total_size: 0,
        file_count: 0,
        dir_count: 0,
    };

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if let Ok(metadata) = entry.metadata() {
            stats.total_size += metadata.len();

            if entry.path() != path {
                if metadata.is_file() {
                    stats.file_count += 1;
                } else if metadata.is_dir() {
                    stats.dir_count += 1;
                }
            }
        }
    }

    stats

}

fn get_modified_time(path: &PathBuf) -> String {
    if let Ok(metadata) = fs::metadata(path) {
        let ft = FileTime::from_last_modification_time(&metadata);
        if let Some(dt) = DateTime::from_timestamp(ft.seconds(), 0) {
            let local_dt: DateTime<Local> = DateTime::from(dt);
            return local_dt.format("%Y-%m-%d %H:%M:%S").to_string();
        }
    }
    "Unknown".to_string()
}

fn main() {
    let args = Args::parse();

    if !args.file.exists() {
        eprintln!("No file of this name {:?} in directory", args.file);
        return;
    }

    let name = args.file.file_name().unwrap().to_string_lossy();
    let absolute_path = args.file.canonicalize().unwrap();
    let modified_time = get_modified_time(&args.file);

    let (raw_size, dir_info) = if args.file.is_dir() {
        let stats = analyze_dir(&args.file);
        (
            stats.total_size,
            Some(format!("{} files, {} folders", stats.file_count, stats.dir_count)),
        )
    } else {
        (fs::metadata(&args.file).unwrap().len(), None)
    };



    let file_type = if args.file.is_dir() {
        "directory".to_string()
    } else {
        match infer::get_from_path(&args.file) {
            Ok(Some(info)) => info.mime_type().to_string(),
            Ok(None) => "plain text / unkown binary".to_string(),
            Err(_) => "error reading file".to_string()
        }
    };

    let formatted_size = ByteSize(raw_size);

    println!("Name: {}",name);
    println!("Type: {}", file_type);
    println!("Path: {}", absolute_path.display());
    println!("Size: {}", formatted_size);
    println!("Last Modified: {}", modified_time);

    if let Some(info) = dir_info {
        println!("Contains: {}", info)
    }

}
