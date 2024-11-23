use std::{env, fs};
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: [unorganised folder] [organised folder]");
        return Ok(());
    }
    let paths = fs::read_dir(&args[1])?;
    let mut images = vec![];
    let mut text = vec![];
    let mut videos = vec![];
    let mut misls = vec![];
    let mut compressed = vec![];
    let mut docs = vec![];
    for path in paths {
        let path = path?;
        let entry = path
            .file_name()
            .into_string()
            .expect("failed to convert to string");
        if entry.ends_with("png") || entry.ends_with("jpg") || entry.ends_with("jpeg")|| entry.ends_with("webp") {
            images.push(path.path());
            continue;
        }
        if entry.ends_with("txt") {
            text.push(path.path());
            continue;
        }
        if entry.ends_with("mp4") || entry.ends_with("mkv") || entry.ends_with("mov") {
            videos.push(path.path());
            continue;
        }
        if entry.ends_with("zip") || entry.ends_with("rar") || entry.ends_with("7z") {
            compressed.push(path.path());
            continue;
        }
        if entry.ends_with("pdf") || entry.ends_with("html") || entry.ends_with("docx") {
            docs.push(path.path());
            continue;
        }
        misls.push(path.path());
    }
    match fs::remove_dir_all(&args[2]) {
        Ok(_) => {}
        Err(_) => {}
    }
    fs::create_dir(&args[2])?;
    fs::create_dir(format!("{}/img", &args[2]))?;
    fs::create_dir(format!("{}/video", &args[2]))?;
    fs::create_dir(format!("{}/txt", &args[2]))?;
    fs::create_dir(format!("{}/compressed", &args[2]))?;
    fs::create_dir(format!("{}/miscellaneous", &args[2]))?;
    for img in images {
        fs::copy(
            img.to_string_lossy().to_string(),
            format!("{}/img/{}", &args[2], img.file_name().unwrap().to_string_lossy()),
        )?;
    }
    for video in videos {
        fs::copy(
            video.to_string_lossy().to_string(),
            format!(
                "{}/video/{}", &args[2],
                video.file_name().unwrap().to_string_lossy()
            ),
        )?;
    }
    for txt in text {
        fs::copy(
            txt.to_string_lossy().to_string(),
            format!("{}/txt/{}", &args[2], txt.file_name().unwrap().to_string_lossy()),
        )?;
    }
    for misl in misls {
        fs::copy(
            misl.to_string_lossy().to_string(),
            format!(
                "{}/miscellaneous/{}", &args[2],
                misl.file_name().unwrap().to_string_lossy()
            ),
        )?;
    }
    for comp in compressed {
        fs::copy(
            comp.to_string_lossy().to_string(),
            format!(
                "{}/compressed/{}", &args[2],
                comp.file_name().unwrap().to_string_lossy()
            ),
        )?;
    }
    Ok(())
}
