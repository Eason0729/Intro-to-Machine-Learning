mod dqn;
mod ffi;
use std::ffi::OsString;

use burn::backend::{wgpu::AutoGraphicsApi, Wgpu};
use dqn::prelude::*;
use ffi::prelude::*;

static ARTIFACT_DIR: &str = "../output";

type Backend = Wgpu<AutoGraphicsApi, f32, i32>;

#[no_mangle]
pub extern "C" fn init(model_path: *const u8, len: i32) -> *mut DQNApp<'static> {
    let model_path = unsafe { std::slice::from_raw_parts(model_path, len as usize) };
    let model_path = unsafe { OsString::from_encoded_bytes_unchecked(model_path.to_vec()) };
    let app = DQNApp::new(model_path.to_str().unwrap());

    Box::into_raw(Box::new(app))
}

#[no_mangle]
pub extern "C" fn tick(app: *mut DQNApp, raw: *mut RawInfo) -> i32 {
    let app = unsafe { &mut *app };
    let info: Info<'static> = unsafe { Info::from_raw(raw) };
    cfg_if::cfg_if! {
        if #[cfg(feature = "train")]{
            app.collect_data(&info)as i32
        }else{
            app.predict_action(&info)as i32
        }
    }
}

#[no_mangle]
pub extern "C" fn flush(app: *mut DQNApp) {
    let app = unsafe { &mut *app };
    #[cfg(feature = "train")]
    {
        app.flush();
    }
}
