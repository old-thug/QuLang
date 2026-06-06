cmake_minimum_required(VERSION 4.2)
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_EXPERIMENTAL_CXX_IMPORT_STD "451f2fe2-a8a2-47c3-bc32-94786d8fc91b")
set(CMAKE_CXX_STANDARD_REQUIRED OFF)
set(CMAKE_CXX_MODULE_STD 1)

project(QuLang VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_EXPORT_COMPILE_COMMANDS On)

add_executable(quc "${CMAKE_SOURCE_DIR}/src/main.cpp")

target_compile_features(quc
  PRIVATE cxx_std_26
  INTERFACE cxx_std_26
)

file(GLOB_RECURSE MODULE_SOURCES "${CMAKE_SOURCE_DIR}/src/*.cppm")

target_sources(quc
  PUBLIC
  FILE_SET CXX_MODULES FILES ${MODULE_SOURCES}
)

set_target_properties(quc
  PROPERTIES
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_SOURCE_DIR}/bin"
)
