pub type ConstVecString = &'static [&'static str];

pub const ACCEPTABLE_AUDIO_FORMATS: ConstVecString = &[
    "wav", "mp3", "ogg", "aac", "m4a", "flac", "opus"
];

pub const ACCEPTABLE_VIDEO_FORMATS: ConstVecString = &[
    "mkv", "avi", "mov", "mp4", "m4v", "flv", "wmv", "ts", "webm", "mpeg",
];


pub fn get_all_acceptable_file_formats() -> Vec<&'static str> {
    let mut combined = Vec::with_capacity(ACCEPTABLE_AUDIO_FORMATS.len() + ACCEPTABLE_VIDEO_FORMATS.len());
    combined.extend_from_slice(ACCEPTABLE_AUDIO_FORMATS);
    combined.extend_from_slice(ACCEPTABLE_VIDEO_FORMATS);
    combined
}