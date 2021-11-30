use foo as _;

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}

extern "C" {
    fn CGDisplayCreateUUIDFromDisplayID(display: u32) -> *const std::os::raw::c_void;
}

fn main() {
    unsafe { CGDisplayCreateUUIDFromDisplayID(0) };
}
