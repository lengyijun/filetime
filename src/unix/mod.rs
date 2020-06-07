use crate::FileTime;
use libc::{time_t, timespec};
use std::fs;
use std::os::unix::prelude::*;

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
    } else if #[cfg(target_os = "macos")] {
    } else if #[cfg(any(target_os = "android",
                        target_os = "solaris",
                        target_os = "illumos",
                        target_os = "emscripten",
                        target_os = "freebsd",
                        target_os = "netbsd",
                        target_os = "openbsd",
                        target_os = "haiku"))] {
    } else {
    }
}

#[allow(dead_code)]
fn to_timespec(ft: &Option<FileTime>) -> timespec {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "macos")] {
            // https://github.com/apple/darwin-xnu/blob/a449c6a3b8014d9406c2ddbdc81795da24aa7443/bsd/sys/stat.h#L541
            const UTIME_OMIT: i64 = -2;
        } else {
            const UTIME_OMIT: i64 = 1_073_741_822;
        }
    }

    if let &Some(ft) = ft {
        timespec {
            tv_sec: ft.seconds() as time_t,
            tv_nsec: ft.nanoseconds() as _,
        }
    } else {
        timespec {
            tv_sec: 0,
            tv_nsec: UTIME_OMIT as _,
        }
    }
}

pub fn from_last_modification_time(meta: &fs::Metadata) -> FileTime {
    FileTime {
        seconds: meta.mtime(),
        nanos: meta.mtime_nsec() as u32,
    }
}

pub fn from_last_access_time(meta: &fs::Metadata) -> FileTime {
    FileTime {
        seconds: meta.atime(),
        nanos: meta.atime_nsec() as u32,
    }
}

