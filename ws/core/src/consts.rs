pub type ConstVecString = &'static [&'static str];

pub const ACCEPTABLE_AUDIO_FORMATS: ConstVecString = &[
    "wav", "mp3", "ogg", "aac", "m4a", "wma", "ape", "dsd", "dsf", "dff", "dts", "mpc", "msv",
    "wv", "wvc", "wvp", "flac", "aiff",
];

pub const ACCEPTABLE_VIDEO_FORMATS: ConstVecString = &["webm"];
