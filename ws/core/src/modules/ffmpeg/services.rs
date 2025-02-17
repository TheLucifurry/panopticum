use std::{
    env,
    path::{Path, PathBuf},
};

use ffmpeg_sidecar::{command::FfmpegCommand, event::FfmpegEvent};

use crate::{consts::FFMPEG_PATH, utils::cmd::is_command_available};

pub struct FfmpegService {
    ffmpeg_path: PathBuf,
    ffmpeg: FfmpegCommand,
}

pub fn resolve_ffmpeg_path() -> PathBuf {
    let current_path = env::current_exe().unwrap();
    let parent_dir = current_path.parent().unwrap();
    let ffmpeg_path = parent_dir.join(FFMPEG_PATH);
    ffmpeg_path
}

// fn create_json_message(msg_type: &str, data: &str, level: Option<LogLevel>) -> Value {
//     let log = match level {
//         Some(LogLevel::Info) => "info",
//         Some(LogLevel::Warning) => "warning",
//         Some(LogLevel::Error) => "error",
//         Some(LogLevel::Fatal) => "fatal",
//         Some(LogLevel::Unknown) => "unknown",
//         None => "",
//     };
//     let mut json = serde_json::json!({
//         "type": msg_type,
//         "data": data
//     });
//     if !log.is_empty() {
//         json["level"] = serde_json::json!(log);
//     }
//     json
// }

impl FfmpegService {
    pub fn new() -> Self {
        log::info!(
            "Is available ffmpeg: {}",
            is_command_available("ffmpeg", "-version")
        );
        log::info!(
            "Is available ffprobe: {}",
            is_command_available("ffprobe", "-version")
        );
        log::info!("ffmpeg path: {}", resolve_ffmpeg_path().display());

        let ffmpeg_path = resolve_ffmpeg_path();
        let mut ffmpeg = FfmpegCommand::new_with_path(&ffmpeg_path);

        #[cfg(target_os = "windows")]
        ffmpeg.create_no_window();

        Self {
            ffmpeg_path,
            ffmpeg,
        }
    }

    pub fn generate_thumbnail(
        &self,
        video_file_path: &Path,
        output_path: &Path,
    ) -> Result<(), tauri::Error> {
        let ffmpeg_path = self.ffmpeg_path.clone();

        let in_path = video_file_path
            .to_str()
            .expect("Invalid video file path")
            .to_owned();
        let out_path = output_path
            .to_str()
            .expect("Invalid output file path")
            .to_owned();

        tauri::async_runtime::spawn(async move {
            log::info!(
                "Starting ffmpeg process for thumbnail generation of {:?}",
                &in_path
            );
            log::info!("to {:?}", &out_path);

            let mut ffmpeg = FfmpegCommand::new_with_path(ffmpeg_path);

            #[cfg(target_os = "windows")]
            ffmpeg.create_no_window();

            let mut ffmpeg_runner = ffmpeg
                .input(&in_path)
                .args(&[
                    "-vframes",
                    "1", // Capture a single frame
                    "-vf",
                    "scale=356:200", // Scale the output to 356x200
                    "-q:v",
                    "2", // Set quality level
                ])
                .output(&out_path)
                .seek("00:00:05") // Extract at the 5-second mark
                .spawn()
                .map_err(|e| {
                    tauri::Error::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        e.to_string(),
                    ))
                })
                .unwrap();

            // ffmpeg_runner
            //     .iter()
            //     .unwrap()
            //     .for_each(|e| {
            //         match e {
            //         FfmpegEvent::Progress(FfmpegProgress { frame, .. }) =>
            //             log::info!("Current frame: {frame}"),
            //         FfmpegEvent::Log(_level, msg) =>
            //             log::info!("[ffmpeg] {msg}"),
            //         _ => {}
            //         }
            //     });

            ffmpeg_runner
                .iter()
                .map_err(|e| {
                    tauri::Error::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        e.to_string(),
                    ))
                })
                .unwrap()
                .for_each(|event| {
                    match event {
                        // FfmpegEvent::Progress(progress) => {
                        //     log::info!("Current frame: {:?}", progress.);
                        // }
                        FfmpegEvent::Log(_level, msg) => {
                            log::info!("Current frame: {msg}");
                        }
                        FfmpegEvent::Error(err) => {
                            log::error!("Current frame: {err}");
                        }
                        FfmpegEvent::LogEOF | FfmpegEvent::Done => {
                            log::info!("Thumbnail generated successfully");
                        }
                        _ => {}
                    }
                });

            ffmpeg_runner.wait().unwrap(); // <- Wait for the child process to finish
            log::info!("Thumbnail generation process finished");

            // match result {
            //     Ok(output) => {
            //         log::info!("Thumbnail generated");
            //         // output.take_stdout().iter().for_each(|eee| {

            //         //     log::info!(
            //         //         "ffmpeg process failed. stderr: {}",
            //         //         eee.to_owned().read_to_string().unwrap().
            //         //     );
            //         // })
            //         // if output.success() {
            //         //     log::info!("Thumbnail generated successfully: {:?}", output_path);
            //         // } else {
            //         //     log::error!(
            //         //         "ffmpeg process failed. stderr: {}",
            //         //         String::from_utf8_lossy(&output.take_stdout())
            //         //     );
            //         // }
            //     }
            //     Err(e) => {
            //         log::error!("Failed to execute ffmpeg: {:?}", e);
            //     }
            // }

            // return Ok(true);
        });

        Ok(())
    }
}
