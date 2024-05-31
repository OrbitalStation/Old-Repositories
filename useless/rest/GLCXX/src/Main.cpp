#include <GLFW/glfw3.h>
#include "../include/Main.hpp"

void gl::init() {
    if (glfwInit() == 0) throw error("Cannot initialize the 'GLCXX' library");
    int minor, major;
    glfwGetVersion(&major, &minor, nullptr);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, major);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, minor);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
}

void gl::terminate() { glfwTerminate(); }

double gl::getTime() { return glfwGetTime(); }
