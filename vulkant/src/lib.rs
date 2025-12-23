use core::{fmt, marker::PhantomData, ptr::NonNull};
use std::{ffi::CStr, sync::Arc};

mod version;
mod instance;

pub use version::Version;
pub use instance::Instance;

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
pub struct UtfCStr(core::ffi::CStr);

#[repr(transparent)]
pub struct Queue<'instance, 'device> {
    handle: NonNull<vulkant_sys::VkQueue_T>,
    phantom: PhantomData<&'device Device<'instance>>,
}

#[repr(transparent)]
pub struct Device<'instance> {
    handle: NonNull<vulkant_sys::VkDevice_T>,
    phantom: PhantomData<&'instance Instance>,
}

impl<'instance> Device<'instance> {
    pub fn get_queue<'device>(&'device self, family_index: u32, index: u32) -> Queue<'instance, 'device> {
        let mut handle = core::ptr::null_mut();

        unsafe { vulkant_sys::vkGetDeviceQueue(
            self.handle.as_ptr(),
            family_index,
            index,
            &mut handle,
        ) };

        return Queue {
            handle: NonNull::new(handle).unwrap(),
            phantom: PhantomData,
        };
    }
}

impl Drop for Device<'_> {
    fn drop(&mut self) {
        let vk_allocator = core::ptr::null();
        unsafe { vulkant_sys::vkDestroyDevice(self.handle.as_ptr(), vk_allocator) }
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

impl CombinedProperties {
    pub fn init(&mut self) {
        self.0.sType = vulkant_sys::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2;
        self.1.sType = vulkant_sys::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES;
        self.2.sType = vulkant_sys::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES;
        self.3.sType = vulkant_sys::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES;
        self.4.sType = vulkant_sys::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_4_PROPERTIES;
        self.0.pNext = core::ptr::from_mut(&mut self.1).cast();
        self.1.pNext = core::ptr::from_mut(&mut self.2).cast();
        self.2.pNext = core::ptr::from_mut(&mut self.3).cast();
        self.3.pNext = core::ptr::from_mut(&mut self.4).cast();
    }
}

#[derive(Debug)]
pub enum PhysicalDeviceType {
    Other = 0,
    IntegratedGpu = 1,
    DiscreteGpu = 2,
    VirtualGpu = 3,
    Cpu = 4,
}

impl PhysicalDeviceType {
    pub const fn from_int(x: u32) -> Option<Self> {
        Some(match x {
            0 => Self::Other,
            1 => Self::IntegratedGpu,
            2 => Self::DiscreteGpu,
            3 => Self::VirtualGpu,
            4 => Self::Cpu,
            _ => return None,
        })
    }
}

#[derive(Debug)]
pub struct Properties {
    pub api_version: Version,
    pub conformance_version: vulkant_sys::VkConformanceVersion,
    pub device_type: PhysicalDeviceType,
    pub device_name: Arc<str>,
    pub driver_name: Arc<str>,
    pub driver_info: Arc<str>,
    pub has_geometry_shader: bool,
}

#[derive(Debug)]
pub struct QueueFamilyProperties {
    pub queue_count: u32,
    pub has_graphics: bool,
    pub has_compute: bool,
    pub has_transfer: bool,
    pub has_sparse_binding: bool,
}

impl<'instance> PhysicalDevice<'instance> {
    pub fn id(&self) -> usize {
        self.handle.addr().get()
    }

    pub fn create_logical(&self, info: &vulkant_sys::VkDeviceCreateInfo) -> Device<'instance> {
        let vk_allocator = core::ptr::null();

        let mut handle = core::ptr::null_mut();
        let result = unsafe { vulkant_sys::vkCreateDevice(
            self.handle.as_ptr(),
            info,
            vk_allocator,
            &mut handle,
        ) };

        assert_eq!(result, 0);
        return Device {
            handle: NonNull::new(handle).unwrap(),
            phantom: self.phantom,
        };
    }

    pub fn get_queue_family_properties(&self) -> Vec<QueueFamilyProperties> {
        let mut buf = Vec::new();
        buf.resize(1000, Default::default());

        let mut count = buf.len() as u32;
        unsafe { vulkant_sys::vkGetPhysicalDeviceQueueFamilyProperties(
            self.handle.as_ptr(),
            &mut count,
            buf.as_mut_ptr(),
        ) };

        buf.resize(count as usize, Default::default());
        return buf
            .into_iter()
            .map(|x| QueueFamilyProperties {
                queue_count: x.queueCount,
                has_graphics: (x.queueFlags & (1 << 0)) != 0,
                has_compute: (x.queueFlags & (1 << 1)) != 0,
                has_transfer: (x.queueFlags & (1 << 2)) != 0,
                has_sparse_binding: (x.queueFlags & (1 << 3)) != 0,
            })
            .collect();
    }

    pub fn get_properties(&self) -> Properties {
        let mut properties = CombinedProperties::default();
        properties.init();

        unsafe { vulkant_sys::vkGetPhysicalDeviceProperties2(
            self.handle.as_ptr(),
            &mut properties.0,
        ) };

        return Properties {
            api_version: Version(properties.0.properties.apiVersion),
            conformance_version: properties.2.conformanceVersion,
            device_type: PhysicalDeviceType::from_int(properties.0.properties.deviceType).unwrap(),
            device_name: buf_to_str(&properties.0.properties.deviceName),
            driver_name: buf_to_str(&properties.2.driverName),
            driver_info: buf_to_str(&properties.2.driverInfo),
            has_geometry_shader: properties.0.properties.limits.maxGeometryShaderInvocations != 0,
        };
    }
}

impl fmt::Debug for PhysicalDevice<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PhysicalDevice({:X})", self.id())
    }
}

fn buf_to_str(s: &[i8]) -> Arc<str> {
    let s = unsafe { core::slice::from_raw_parts(s.as_ptr().cast(), s.len()) };
    return CStr::from_bytes_until_nul(s).unwrap().to_str().unwrap().into();
}
