use rayon::prelude::*;
use std::{env, fs, time::Instant};
use walkdir::WalkDir;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: [unorganised folder] [organised folder]");
        return Ok(());
    }
    let time = Instant::now();
    println!("Organizing Files...");

    let paths: Vec<_> = WalkDir::new(&args[1])
        .into_iter()
        .filter_map(Result::ok)
        .filter(|path| path.file_type().is_file())
        .collect();

    let output_dir = &args[2];
    fs::create_dir_all(format!("{}/img", output_dir))?;
    fs::create_dir_all(format!("{}/video", output_dir))?;
    fs::create_dir_all(format!("{}/txt", output_dir))?;
    fs::create_dir_all(format!("{}/compressed", output_dir))?;
    fs::create_dir_all(format!("{}/miscellaneous", output_dir))?;

    paths.into_par_iter().for_each(|path| {
        let entry = path.file_name().to_string_lossy();
        let dest_dir = if entry.ends_with("png")
            || entry.ends_with("jpg")
            || entry.ends_with("jpeg")
            || entry.ends_with("webp")
        {
            "img"
        } else if entry.ends_with("mp4") || entry.ends_with("mkv") || entry.ends_with("mov") {
            "video"
        } else if entry.ends_with("txt") {
            "txt"
        } else if entry.ends_with("zip") || entry.ends_with("rar") || entry.ends_with("7z") {
            "compressed"
        } else {
            "miscellaneous"
        };

        let dest_path = format!("{}/{}/{}", output_dir, dest_dir, entry);
        fs::copy(path.path(), dest_path).unwrap();
    });

    println!("Operation completed in {:.2?}", time.elapsed());
    Ok(())
}
