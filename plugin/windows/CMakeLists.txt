cmake_minimum_required(VERSION 3.14)
set(PROJECT_NAME "plugin")
project(${PROJECT_NAME} LANGUAGES CXX)

# This value is used when generating builds using this plugin, so it must
# not be changed
set(PLUGIN_NAME "plugin_plugin")

add_library(${PLUGIN_NAME} SHARED
  "plugin.cpp"
)
apply_standard_settings(${PLUGIN_NAME})
set_target_properties(${PLUGIN_NAME} PROPERTIES
  CXX_VISIBILITY_PRESET hidden)
target_compile_definitions(${PLUGIN_NAME} PRIVATE FLUTTER_PLUGIN_IMPL)
target_include_directories(${PLUGIN_NAME} INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/include")
target_link_libraries(${PLUGIN_NAME} PRIVATE flutter flutter_wrapper_plugin)

# List of absolute paths to libraries that should be bundled with the plugin
set(plugin_bundled_libraries
  "C:\\Users\\amirb\\rust\\moon\\target\\release\\moon.dll"
  "C:\\Users\\amirb\\rust\\vcpkg\\packages\\openssl_x64-windows\\bin\\libcrypto-1_1-x64.dll"
  PARENT_SCOPE
)
