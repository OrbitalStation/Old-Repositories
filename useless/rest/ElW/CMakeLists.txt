cmake_minimum_required(VERSION 3.17)
project(ElW)

find_package(OpenGL REQUIRED)

set(CMAKE_CXX_STANDARD 14)

file(GLOB_RECURSE SOURCE src/*.cpp)

add_executable(${PROJECT_NAME} ${SOURCE})

target_link_libraries(${PROJECT_NAME} sfml-graphics sfml-audio sfml-window sfml-system glfw X11 GLX)
