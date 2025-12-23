use std::collections::BTreeSet;
use std::ffi::CStr;

fn cstr_from_buf(s: &[i8]) -> &CStr {
    let s = unsafe { core::slice::from_raw_parts(
        s.as_ptr() as *const u8,
        s.len(),
    ) };
    return CStr::from_bytes_until_nul(s).unwrap();
}

fn enumerate_instance_layer_properties() -> BTreeSet<Box<CStr>> {
    let mut buf = Vec::new();
    buf.resize(1000, Default::default());

    let mut count = buf.len() as u32;
    let result = unsafe { vulkant_sys::vkEnumerateInstanceLayerProperties(
        &mut count,
        buf.as_mut_ptr(),
    ) };

    assert_eq!(result, 0);
    buf.resize(count as usize, Default::default());

    return buf
        .iter()
        .map(|x| cstr_from_buf(&x.layerName).into())
        .collect();
}

fn enumerate_instance_extension_properties() -> BTreeSet<Box<str>> {
    let mut buf = Vec::new();
    buf.resize(1000, Default::default());

    let mut count = buf.len() as u32;
    let result = unsafe { vulkant_sys::vkEnumerateInstanceExtensionProperties(
        std::ptr::null(),
        &mut count,
        buf.as_mut_ptr(),
    ) };

    assert_eq!(result, 0);
    buf.resize(count as usize, Default::default());

    return buf
        .iter()
        .map(|x| cstr_from_buf(&x.extensionName).to_str().unwrap().into())
        .collect();
}

fn create_instance(glfw_ext: &[String]) -> vulkant::Instance {
    let ext_as_cstr: Vec<String> = glfw_ext.iter().map(|x| x.clone() + "\0").collect();
    let pointer_array: Vec<*const i8> = ext_as_cstr.iter().map(|x| x.as_ptr() as *const i8).collect();

    let layer_pointer_array = [
        c"VK_LAYER_KHRONOS_validation".as_ptr()
    ];

    let app_info = vulkant_sys::VkApplicationInfo {
        sType: vulkant_sys::VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO,
        pNext: std::ptr::null(),
        pApplicationName: c"Vulkant".as_ptr(),
        applicationVersion: 1,
        pEngineName: c"None".as_ptr(),
        engineVersion: 0,
        apiVersion: vulkant::Version::new(0, 1, 4, 0).0,
    };

    let create_info = vulkant_sys::VkInstanceCreateInfo {
        sType: vulkant_sys::VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        pNext: std::ptr::null(),
        flags: 0,
        pApplicationInfo: &app_info,
        enabledLayerCount: layer_pointer_array.len().try_into().unwrap(),
        ppEnabledLayerNames: layer_pointer_array.as_ptr(),
        enabledExtensionCount: pointer_array.len().try_into().unwrap(),
        ppEnabledExtensionNames: pointer_array.as_ptr(),
    };

    return unsafe { vulkant::Instance::create(&create_info) };
}

fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
    glfw.window_hint(glfw::WindowHint::Resizable(false));

    let (window, _events) = glfw.create_window(800, 600, "Vulkant", glfw::WindowMode::Windowed).unwrap();

    let vk_ext = enumerate_instance_extension_properties();
    let glfw_ext = glfw.get_required_instance_extensions().unwrap();
    for req in glfw_ext.iter() {
        assert!(vk_ext.contains(req.as_str()));
    }

    let vk_layers = enumerate_instance_layer_properties();
    assert!(vk_layers.contains(c"VK_LAYER_KHRONOS_validation"));

    let instance = create_instance(&glfw_ext);
    let devices = instance.enumerate_physical_devices();

    let device = &devices[0];
    let device_prop = device.get_properties();
    assert_eq!(device_prop.conformance_version.major, 1);
    assert_eq!(device_prop.conformance_version.minor, 4);
    assert!(device_prop.has_geometry_shader);
    println!("{:#?}", device_prop);

    let queues = device.get_queue_family_properties();
    println!("{:#?}", queues);
    assert!(queues[0].has_graphics);

    let mut queue_create_info = vulkant_sys::VkDeviceQueueCreateInfo::default();
    queue_create_info.sType = vulkant_sys::VkStructureType_VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
    queue_create_info.queueFamilyIndex = 0;
    queue_create_info.queueCount = 1;
    queue_create_info.pQueuePriorities = [0.5f32].as_ptr();

    #[derive(Clone, Copy, Debug, Default)]
    struct CombinedFeatures(
        pub vulkant_sys::VkPhysicalDeviceFeatures2,
        pub vulkant_sys::VkPhysicalDeviceVulkan11Features,
        pub vulkant_sys::VkPhysicalDeviceVulkan12Features,
        pub vulkant_sys::VkPhysicalDeviceVulkan13Features,
        pub vulkant_sys::VkPhysicalDeviceVulkan14Features,
        pub vulkant_sys::VkPhysicalDeviceExtendedDynamicStateFeaturesEXT,
    );

    let mut combined = CombinedFeatures::default();
    combined.0.sType = vulkant_sys::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2;
    combined.1.sType = vulkant_sys::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES;
    combined.2.sType = vulkant_sys::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES;
    combined.3.sType = vulkant_sys::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_FEATURES;
    combined.4.sType = vulkant_sys::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_4_FEATURES;
    combined.5.sType = vulkant_sys::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT;
    combined.0.pNext = core::ptr::from_mut(&mut combined.1).cast();
    combined.1.pNext = core::ptr::from_mut(&mut combined.2).cast();
    combined.2.pNext = core::ptr::from_mut(&mut combined.3).cast();
    combined.3.pNext = core::ptr::from_mut(&mut combined.4).cast();
    combined.4.pNext = core::ptr::from_mut(&mut combined.5).cast();

    combined.3.dynamicRendering = 1;
    combined.5.extendedDynamicState = 1;

    let extensions = [
        vulkant_sys::VK_KHR_SWAPCHAIN_EXTENSION_NAME.as_ptr(),
        vulkant_sys::VK_KHR_SPIRV_1_4_EXTENSION_NAME.as_ptr(),
        vulkant_sys::VK_KHR_SYNCHRONIZATION_2_EXTENSION_NAME.as_ptr(),
        vulkant_sys::VK_KHR_CREATE_RENDERPASS_2_EXTENSION_NAME.as_ptr(),
    ];

    let mut device_create_info = vulkant_sys::VkDeviceCreateInfo::default();
    device_create_info.sType = vulkant_sys::VkStructureType_VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
    device_create_info.pNext = core::ptr::from_ref(&combined).cast();
    device_create_info.queueCreateInfoCount = 1;
    device_create_info.pQueueCreateInfos = &queue_create_info;
    device_create_info.enabledExtensionCount = extensions.len().try_into().unwrap();
    device_create_info.ppEnabledExtensionNames = extensions.as_ptr();

    let actual_device = device.create_logical(&device_create_info);
    let actual_queue = actual_device.get_queue(0, 0);

    // while !window.should_close() {
    //     glfw.poll_events();
    // }
}
