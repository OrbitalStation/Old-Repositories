cmake_minimum_required(VERSION 3.11)

project(libGLCXX)

find_package(OpenGL REQUIRED)

add_library(${PROJECT_NAME} STATIC src/Window.cpp src/Event.cpp
        src/Main.cpp src/Color.cpp src/Triangle.cpp src/Quadrangle.cpp
        src/View.cpp src/Keyboard.cpp src/Low.cpp src/Image.cpp
        src/Cursor.cpp src/Monitor.cpp
        src/stb_image/implementation.c src/glad/glad.c)

target_link_libraries(${PROJECT_NAME} glfw ${GLFW_LIBRARIES} ${OPENGL_LIBRARY} dl)
