#![windows_subsystem = "windows"]


use std::fmt::Debug;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use crate::game::GameProcess;
use crate::game::jvm::JVM_Control;
use crate::module::{ModuleCollection, Tick};
use crate::ui::UI;

mod memory;
mod game;
mod module;
mod ui;
pub static mut collection:Option<Mutex<ModuleCollection>> = None;
fn main() -> eframe::Result {

    unsafe { collection = Some(Mutex::from(ModuleCollection::new())); }
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_inner_size([400.0, 400.0])
            .with_min_inner_size([400.0, 400.0])
            .with_resizable(false)
            .with_transparent(true),
        ..Default::default()
    };
    thread::spawn(|| unsafe {
        let mut game = GameProcess::craftrise().unwrap();
        loop {
            let collection_wrapper = collection.as_ref().unwrap();
            let mut lock = collection_wrapper.lock().unwrap();
            lock.tick(&mut game);
            drop(lock);
            thread::sleep(Duration::from_millis(1));
        }
    });
    eframe::run_native(
        "Hestia",
        options,
        Box::new(|_cc|
            Ok(Box::new(UI {
            visible: true,
        }))),
    )
}
