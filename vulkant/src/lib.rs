use core::{fmt, marker::PhantomData, ptr::NonNull};

mod version;
mod instance;

pub use version::Version;
pub use instance::Instance;

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
pub struct UtfCStr(core::ffi::CStr);

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
