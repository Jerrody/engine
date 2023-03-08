use std::mem::ManuallyDrop;

mod device;
mod instance;

use instance::Instance;

pub struct Context {
    entry: ManuallyDrop<ash::Entry>,
    instance: Instance,
}

impl Context {
    pub fn new(window: &winit::window::Window) -> Self {
        let entry = unsafe { ash::Entry::load().unwrap() };
        let instance = Instance::new(&entry);

        Self {
            entry: ManuallyDrop::new(entry),
            instance,
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.entry);
        }
    }
}
