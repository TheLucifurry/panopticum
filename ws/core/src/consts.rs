pub type ConstVecString = &'static [&'static str];

pub const ACCEPTABLE_AUDIO_FORMATS: ConstVecString = &[
    "wav", "mp3", "ogg", "aac", "m4a", "wma", "ape", "dsd", "dsf", "dff", "dts", "mpc", "msv",
    "wv", "wvc", "wvp", "flac", "aiff",
];

pub const ACCEPTABLE_VIDEO_FORMATS: ConstVecString = &[
    "mkv", "avi", "mov", "mp4", "m4v", "flv", "wmv", "ogv", "3gp", "3g2", "mts",
    "m2t", "m2p", "m2a", "m2v", "m1v", "m1a",
    "ts", "webm", "m2ts", "mpeg",
];
