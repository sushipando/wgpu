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
