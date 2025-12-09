use std::collections::BTreeSet;

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
        .map(|x| unsafe { std::ffi::CStr::from_ptr(x.extensionName.as_ptr() as *const i8) }.to_str().unwrap().into())
        .collect();
}

fn create_instance(glfw_ext: &[String]) -> vulkant::Instance {
    let ext_as_cstr: Vec<String> = glfw_ext.iter().map(|x| x.clone() + "\0").collect();
    let pointer_array: Vec<*const i8> = ext_as_cstr.iter().map(|x| x.as_ptr() as *const i8).collect();

    let app_info = vulkant_sys::VkApplicationInfo {
        sType: vulkant_sys::VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO,
        pNext: std::ptr::null(),
        pApplicationName: c"Vulkant".as_ptr(),
        applicationVersion: 1,
        pEngineName: c"None".as_ptr(),
        engineVersion: 0,
        apiVersion: vulkant::make_version(0, 1, 4, 0),
    };

    let create_info = vulkant_sys::VkInstanceCreateInfo {
        sType: vulkant_sys::VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        pNext: std::ptr::null(),
        flags: 0,
        pApplicationInfo: &app_info,
        enabledLayerCount: 0,
        ppEnabledLayerNames: std::ptr::null(),
        enabledExtensionCount: pointer_array.len().try_into().unwrap(),
        ppEnabledExtensionNames: pointer_array.as_ptr(),
    };

    let mut instance = std::ptr::null_mut();
    let result = unsafe { vulkant_sys::vkCreateInstance(
        &createInfo,
        std::ptr::null(),
        &mut instance,
    ) };

    assert_eq!(result, 0);
    todo!();
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

    let instance = create_instance(&glfw_ext);

    while !window.should_close() {
        glfw.poll_events();
    }
}
