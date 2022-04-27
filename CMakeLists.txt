cmake_minimum_required(VERSION 3.21)
project(ffi_demo C)

set(CMAKE_C_STANDARD 99)

add_executable(ffi_demo library.c)
add_library(ffi_demoLIB STATIC library.c)
target_link_libraries(ffi_demo PRIVATE d3d11.lib)
target_link_libraries(ffi_demoLIB PRIVATE d3d11.lib)