cmake_minimum_required(VERSION 3.16)
project(X11p)

set(CMAKE_CXX_STANDARD 14)

add_subdirectory(GL)

add_executable(${PROJECT_NAME} main.cpp)

target_link_libraries(${PROJECT_NAME} GLLibrary)
