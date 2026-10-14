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
