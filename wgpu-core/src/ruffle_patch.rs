//! Ruffle patch: switches for Ruffle's changes to wgpu-core, so that their effect can be
//! measured at runtime.

use core::sync::atomic::{AtomicBool, Ordering};

static SHARE_COMMAND_BUFFERS: AtomicBool = AtomicBool::new(true);

/// Whether render passes may be recorded into the same raw command buffer as the commands
/// before them, when that's equivalent (see `command::render::can_share_command_buffer`).
pub fn set_share_command_buffers(enabled: bool) {
    SHARE_COMMAND_BUFFERS.store(enabled, Ordering::Relaxed);
}

pub(crate) fn share_command_buffers() -> bool {
    SHARE_COMMAND_BUFFERS.load(Ordering::Relaxed)
}

/// Presents frames on Metal so that each stays on screen for at least this long (see the
/// vendored wgpu-hal's `metal::set_present_min_duration`). No effect on other backends.
pub fn set_present_min_duration(seconds: Option<f64>) {
    #[cfg(metal)]
    hal::metal::set_present_min_duration(seconds);
    #[cfg(not(metal))]
    let _ = seconds;
}

/// Shows the next frame presented on this thread at the given time, on Metal (see the
/// vendored wgpu-hal's `metal::set_present_time`). No effect on other backends.
#[cfg(feature = "std")]
pub fn set_present_time(time: Option<std::time::Instant>) {
    #[cfg(metal)]
    hal::metal::set_present_time(time);
    #[cfg(not(metal))]
    let _ = time;
}

/// Measures how busy the GPU is (see `gpu_busy_fraction`). Only on Metal.
pub fn set_gpu_timing(enabled: bool) {
    #[cfg(metal)]
    hal::metal::set_gpu_timing(enabled);
    #[cfg(not(metal))]
    let _ = enabled;
}

/// The fraction of the last measured second the GPU spent on Ruffle's command buffers,
/// while measured (see `set_gpu_timing`). `None` on other backends.
pub fn gpu_busy_fraction() -> Option<f64> {
    #[cfg(metal)]
    return hal::metal::gpu_busy_fraction();
    #[cfg(not(metal))]
    None
}
