//! The keyboard, for search: Java's invisible text box hands us the whole
//! text after every edit; we ask Java to show or hide the keyboard.

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

use android_activity::AndroidApp;
use holo_platform::jni_util::with_activity;
use jni::objects::{JClass, JString, JValue};
use jni::sys::jint;
use jni::{EnvUnowned, jni_sig, jni_str};
use launcher_core::ime::ImeState;

static LATEST: Mutex<Option<ImeState>> = Mutex::new(None);
static ACTION: AtomicBool = AtomicBool::new(false);
/// How far the keyboard reaches up from the bottom, in pixels (0: closed).
static HEIGHT: AtomicI32 = AtomicI32::new(0);

pub fn show(app: &AndroidApp, show: bool) {
    if let Err(e) = with_activity(app, |env, a| env.call_method(a, jni_str!("showKeyboard"), jni_sig!("(Z)V"), &[JValue::Bool(show)]).map(|_| ())) {
        log::warn!("showKeyboard({show}) failed: {e}");
    }
}

/// Empty the text, the keyboard's copy too.
pub fn clear(app: &AndroidApp) {
    if let Err(e) = with_activity(app, |env, a| env.call_method(a, jni_str!("clearText"), jni_sig!("()V"), &[]).map(|_| ())) {
        log::warn!("clearText failed: {e}");
    }
}

/// The newest text since the last call.
pub fn take() -> Option<ImeState> {
    LATEST.lock().unwrap().take()
}

/// Whether the keyboard's Search / Enter key was pressed since the last call.
pub fn take_action() -> bool {
    ACTION.swap(false, Ordering::AcqRel)
}

pub fn height() -> f32 {
    HEIGHT.load(Ordering::Acquire).max(0) as f32
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_launcherrs_gallery_MainActivity_onText<'l>(
    mut unowned_env: EnvUnowned<'l>,
    _class: JClass<'l>,
    text: JString<'l>,
    sel_start: jint,
    sel_end: jint,
    comp_start: jint,
    comp_end: jint,
) {
    let outcome = unowned_env.with_env(|env| -> jni::errors::Result<()> {
        let text: String = text.mutf8_chars(env)?.into();
        *LATEST.lock().unwrap() = Some(ImeState::from_java(&text, sel_start, sel_end, comp_start, comp_end));
        Ok(())
    });
    outcome.resolve::<jni::errors::LogErrorAndDefault>();
    holo_platform::waker::wake();
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_launcherrs_gallery_MainActivity_onSearchAction<'l>(_env: EnvUnowned<'l>, _class: JClass<'l>) {
    ACTION.store(true, Ordering::Release);
    holo_platform::waker::wake();
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_launcherrs_gallery_MainActivity_onKeyboard<'l>(_env: EnvUnowned<'l>, _class: JClass<'l>, bottom: jint) {
    HEIGHT.store(bottom, Ordering::Release);
    holo_platform::waker::wake();
}
