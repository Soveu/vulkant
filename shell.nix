{ pkgs ? (import <nixpkgs> {}) }:
with pkgs;
mkShell {
  buildInputs = [
    # For vulkan bindings
    libclang
    rustPlatform.bindgenHook

    # For glfw crate
    # ninja
    cmake
    glfw
    wayland-scanner
    wayland
    # wayland-client
    pkg-config

    # glm
    # tinyobjloader
    # stb
    # tinygltf
    # nlohmann_json
    # glslang
    # shaderc
    # shader-slang
    vulkan-tools
    vulkan-headers
    vulkan-utility-libraries
    vulkan-helper
    vulkan-loader
    vulkan-validation-layers
  ];

  VK_LAYER_PATH = "${vulkan-validation-layers}/share/vulkan/explicit_layer.d";
  LIBCLANG_PATH = "${libclang.lib}/lib";
}
