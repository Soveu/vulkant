use core::{fmt, marker::PhantomData, ptr::NonNull};
use crate::PhysicalDevice;

#[repr(transparent)]
pub struct Instance(NonNull<vulkant_sys::VkInstance_T>);

impl Instance {
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

    pub fn enumerate_physical_devices(&self) -> Vec<PhysicalDevice<'_>> {
        let mut buf = Vec::new();
        buf.resize(1024, Default::default());
        let mut count = buf.len() as u32;

        let result = unsafe { vulkant_sys::vkEnumeratePhysicalDevices(
            self.0.as_ptr(),
            &mut count,
            buf.as_mut_ptr(),
        ) };

        assert_eq!(result, 0);

        buf.resize(count as usize, Default::default());
        return buf
            .into_iter()
            .map(|handle| PhysicalDevice {
                handle: NonNull::new(handle).unwrap(),
                phantom: PhantomData
            })
            .collect();
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

impl fmt::Debug for Instance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Instance").field(&self.0.addr()).finish()
    }
}
