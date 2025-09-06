use std::{path::PathBuf, process};

use colored::Colorize;
use log::{error, info};
use normpath::PathExt;
use regex::Regex;
// use predicates::path;

use crate::gopro::GoProChapteredVideoFile;

// Create "concat demux" input files
pub fn combine_multichapter_videos(
    multichapter_videos_sorted: std::collections::HashMap<u16, Vec<GoProChapteredVideoFile>>,
    output_dir: PathBuf,
) {
    if multichapter_videos_sorted.len() == 0 {
        info!("{}", "No multichapter videos to combine".blue().bold());
        return;
    }
    // Iterate through multichapter video map, and mp4-merge it.
    for video in multichapter_videos_sorted {
        let number = video.0;
        let mut paths_to_chapters = Vec::<PathBuf>::new();
        if video.1.len() == 0 {
            info!("{}", "No chapters to combine".blue().bold());
            return;
        }
        let first_chapter_filename = video.1[0].abs_path.file_name().unwrap().to_str().unwrap();
        info!(
            "First chapter filename: {}",
            first_chapter_filename.blue().bold()
        );
        // TODO: Accumulate the chapters into a vec, then pass to mp4-merge
        for chapter in video.1.clone() {
            paths_to_chapters.push(chapter.abs_path.clone());
            info!(
                "Concatenating chapter {:?} of video {}...",
                chapter.abs_path.to_str(),
                number
            );
        }
        let output_filename =
            generate_merged_chaptered_video_output_file_name(&output_dir, first_chapter_filename);
        info!("Writing to {}", output_filename.display());
        mp4_merge::join_files(&paths_to_chapters, &output_filename, |progress| {
            info!("Merging... {:.2}%", progress * 100.0);
        })
        .unwrap();
    }
}

fn generate_merged_chaptered_video_output_file_name(
    output_dir: &PathBuf,
    first_chapter_filename: &str,
) -> PathBuf {
    let mut output_file_name = match PathBuf::from(output_dir.clone()).normalize() {
        Ok(path) => path,
        Err(e) => {
            error!("Could not normalize output directory path: {}", e);
            process::exit(1);
        }
    };

    output_file_name.push(format!(
        "{}",
        add_m_to_gopro_video_prefix(first_chapter_filename)
    ));
    let mut output_file_name = output_file_name.as_path().to_path_buf();
    output_file_name.set_extension("MP4");
    if output_file_name.exists() {
        error!("Output file already exists: {}", output_file_name.display());
        process::exit(1);
    }
    output_file_name
}

pub fn add_m_to_gopro_video_prefix(first_chapter_filename: &str) -> String {
    let re = Regex::new(r"^G(.)").unwrap();
    let replacement_string = r"G${1}M";
    let result = re.replace(first_chapter_filename, replacement_string);
    result.to_string()
}
