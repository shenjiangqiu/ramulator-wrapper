project(dramsim2_wrapper)
cmake_minimum_required(VERSION 3.20)
set(CMAKE_CXX_STANDARD 17)

add_subdirectory(HBM)

add_library(dramsim2_wrapper_c SHARED dramsim2_wrapper.cc)
target_link_libraries(dramsim2_wrapper_c PUBLIC dramsim2)
target_include_directories(dramsim2_wrapper_c PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

add_executable(main_test test.cc)
target_link_libraries(main_test PUBLIC dramsim2_wrapper_c)