cmake_minimum_required(VERSION 2.8)
project(GLLibrary)

find_package(X11)

add_library(${PROJECT_NAME} STATIC src/GL.cpp)

target_link_libraries(${PROJECT_NAME} X11)
