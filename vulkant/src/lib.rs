use core::ptr::NonNull;

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
pub struct UtfCStr(core::ffi::CStr);

#[repr(transparent)]
pub struct Instance(NonNull<vulkant_sys::VkInstance_T>);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(u32);
pub const fn make_version(variant: u8, major: u8, minor: u8, patch: u8) -> u32 {
    return 0
        | ((variant as u32) << 29)
        | ((major as u32) << 22)
        | ((minor as u32) << 12)
        | ((patch as u32) << 0);
}
