use core::ptr::NonNull;

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
pub struct UtfCStr(core::ffi::CStr);

#[repr(transparent)]
pub struct Instance(NonNull<vulkant_sys::VkInstance_T>);

impl Instance {
    /// SAFETY: Must come from VkCreateInstace with allocator=null
    pub unsafe fn from_raw(raw: vulkant_sys::VkInstance) -> Option<Self> {
        Some(Self(NonNull::new(raw)?))
    }

    /// SAFETY: info structure must be properly filled
    pub unsafe fn create(info: &vulkant_sys::VkInstanceCreateInfo) -> Self {
        let mut instance = std::ptr::null_mut();
        let result = unsafe { vulkant_sys::vkCreateInstance(
            info,
            std::ptr::null(),
            &mut instance,
        ) };

        assert_eq!(result, 0);
        return Self(NonNull::new(instance).unwrap());
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        let vk_allocator = core::ptr::null();
        unsafe { vulkant_sys::vkDestroyInstance(
            self.0.as_ptr(),
            vk_allocator,
        ) };
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(u32);
pub const fn make_version(variant: u8, major: u8, minor: u8, patch: u8) -> u32 {
    return 0
        | ((variant as u32) << 29)
        | ((major as u32) << 22)
        | ((minor as u32) << 12)
        | ((patch as u32) << 0);
}
