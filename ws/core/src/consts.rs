pub type ConstVecString = &'static [&'static str];

pub const ACCEPTABLE_AUDIO_FORMATS: ConstVecString = &[
    "wav", "mp3", "ogg", "aac", "m4a", "flac", "opus"
];

pub const ACCEPTABLE_VIDEO_FORMATS: ConstVecString = &[
    "mkv", "avi", "mov", "mp4", "m4v", "flv", "wmv", "ts", "webm", "mpeg",
];
