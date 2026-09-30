use std::{
    string::{String, ToString},
    sync::OnceLock,
};

static ANDROID_MAJOR_VERSION: OnceLock<Option<i32>> = OnceLock::new();

pub fn android_major_version() -> Option<i32> {
    *ANDROID_MAJOR_VERSION.get_or_init(|| android_major_version_with(read_string_property))
}

pub fn android_major_version_with(read_property: impl Fn(&str) -> Option<String>) -> Option<i32> {
    // 先读系统真实的底层 SDK 版本（你的手机是 33，精确对应 Android 13）
    read_property("ro.build.version.sdk")
        .and_then(|sdk| sdk.parse::<i32>().ok())
        .map(|sdk| match sdk {
            31 | 32 => 12,
            33 => 13,
            34 => 14,
            35 => 15,
            36 => 16,
            value if value >= 37 => 17,
            _ => panic!("unrecognized or unsupported SDK version: {}", sdk),
        })
        .or_else(|| {
            // 如果读取 SDK 失败，再去读 release 作为备选
            read_property("ro.build.version.release_or_codename")
                .or_else(|| read_property("ro.build.version.release"))
                .and_then(|value| value.parse::<i32>().ok())
        })
}

fn read_string_property(name: &str) -> Option<String> {
    rsproperties::get::<String>(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}
