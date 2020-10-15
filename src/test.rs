use parking_lot::{ReentrantMutex, ReentrantMutexGuard};
use std::ptr;

use imgui::Context;

lazy_static! {
    pub static ref TEST_MUTEX: ReentrantMutex<()> = ReentrantMutex::new(());
}

pub fn test_ctx() -> (ReentrantMutexGuard<'static, ()>, Context) {
    let guard = TEST_MUTEX.lock();
    let mut ctx = Context::create();
    ctx.set_ini_filename(None);
    (guard, ctx)
}

pub fn test_ctx_initialized() -> (ReentrantMutexGuard<'static, ()>, Context) {
    let (guard, mut ctx) = test_ctx();
    let io = ctx.io_mut();
    io.display_size = [1024.0, 768.0];
    io.delta_time = 1.0 / 60.0;
    io.mouse_pos = [0.0, 0.0];
    ctx.fonts().build_rgba32_texture();
    (guard, ctx)
}
