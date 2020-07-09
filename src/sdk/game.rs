use memlib::memory::Handle;

pub struct FortniteContext {
    pub handle: Handle,
}

impl FortniteContext {
    pub fn new(handle: Handle) -> Self {
        Self { handle }
    }
}
