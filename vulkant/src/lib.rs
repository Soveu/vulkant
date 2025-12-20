use core::{fmt, marker::PhantomData, ptr::NonNull};

mod version;
pub use version::Version;

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

#[repr(transparent)]
pub struct PhysicalDevice<'instance> {
    handle: NonNull<vulkant_sys::VkPhysicalDevice_T>,
    phantom: PhantomData<&'instance Instance>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CombinedProperties(
    pub vulkant_sys::VkPhysicalDeviceProperties2,
    pub vulkant_sys::VkPhysicalDeviceVulkan11Properties,
    pub vulkant_sys::VkPhysicalDeviceVulkan12Properties,
    pub vulkant_sys::VkPhysicalDeviceVulkan13Properties,
    pub vulkant_sys::VkPhysicalDeviceVulkan14Properties,
);

impl PhysicalDevice<'_> {
    pub fn id(&self) -> usize {
        self.handle.addr().get()
    }

    pub fn get_properties(&self) -> CombinedProperties {
        let mut properties = CombinedProperties::default();
        properties.0.sType = vulkant_sys::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2;
        properties.1.sType = vulkant_sys::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES;
        properties.2.sType = vulkant_sys::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES;
        properties.3.sType = vulkant_sys::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES;
        properties.4.sType = vulkant_sys::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_4_PROPERTIES;
        properties.0.pNext = core::ptr::from_mut(&mut properties.1).cast();
        properties.1.pNext = core::ptr::from_mut(&mut properties.2).cast();
        properties.2.pNext = core::ptr::from_mut(&mut properties.3).cast();
        properties.3.pNext = core::ptr::from_mut(&mut properties.4).cast();

        unsafe { vulkant_sys::vkGetPhysicalDeviceProperties2(
            self.handle.as_ptr(),
            &mut properties.0,
        ) };

        return properties;
    }
}

impl fmt::Debug for PhysicalDevice<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PhysicalDevice({:X})", self.id())
    }
}
