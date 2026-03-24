pub(crate) mod traits;

#[cfg(target_os = "linux")]
pub(crate) mod v4l2;

#[cfg(target_os = "windows")]
pub(crate) mod windows_mf;
