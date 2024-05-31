#include "../include/Low.hpp"
#include "../include/Window/Window.hpp"
#include "../include/Window/Monitor.hpp"
#include "glad/glad.h"
#include <GLFW/glfw3.h>

extern "C++" {

    namespace gl {

        namespace priv {
            void *temp;
            extern Keyboard::Key glfwToGL(const int &key);
        }

        namespace low {

            void viewport(const int &x, const int &y, const int &width,
                          const int &height) {
                glViewport(x, y, width, height);
            }

            void setCursorMoveCallback(const Window &window, const CursorMoveCallback &callback) {
                priv::temp = (void *)callback;
                glfwSetCursorPosCallback((GLFWwindow *)window.data, [](GLFWwindow *w, double x, double y) {
                    static auto cb = (CursorMoveCallback)priv::temp;
                    (*cb)(*((Window *)glfwGetWindowUserPointer(w)), float(x), float(y));
                });
            }

            void setWindowResizeCallback(const Window &window, const WindowResizeCallback &callback) {
                priv::temp = (void *)callback;
                glfwSetFramebufferSizeCallback((GLFWwindow *)window.data, [](GLFWwindow *w, int width, int height) {
                    static auto cb = (WindowResizeCallback)priv::temp;
                    (*cb)(*((Window *)glfwGetWindowUserPointer(w)), (unsigned int)(width), (unsigned int)(height));
                });
            }

            void setMouseWheelScrollCallback(const Window &window, const MouseWheelScroll &callback) {
                priv::temp = (void *)callback;
                glfwSetScrollCallback((GLFWwindow *)window.data, [](GLFWwindow *w, double offsetX, double offsetY) {
                    static auto cb = (MouseWheelScroll)priv::temp;
                    (*cb)(*((Window *)glfwGetWindowUserPointer(w)), float(offsetX), float(offsetY));
                });
            }

            void setKeyCallback(const Window &window, const KeyCallback &callback) {
                priv::temp = (void *)callback;
                glfwSetKeyCallback((GLFWwindow *)window.data, [](GLFWwindow *w, int key, int scancode, int action, int) {
                    static auto cb = (KeyCallback)priv::temp;
                    (*cb)(*((Window *)glfwGetWindowUserPointer(w)), priv::glfwToGL(key), scancode,
                            (action == GLFW_PRESS ? Keyboard::KeyType::Pressed : (action == GLFW_RELEASE ?
                            Keyboard::KeyType::Released : Keyboard::KeyType::Repeated)));
                });
            }

            void setMonitorCallback(const MonitorCallback &callback) {
                priv::temp = (void *)callback;
                glfwSetMonitorCallback([](GLFWmonitor *m, int event) {
                    static auto cb = (MonitorCallback)priv::temp;
                    Monitor monitor(m);
                    (*cb)(monitor, event == GLFW_CONNECTED);
                });
            }

            void setWindowCloseCallback(const Window &window, const WindowCloseCallback &callback) {
                priv::temp = (void *)callback;
                glfwSetWindowCloseCallback((GLFWwindow *)window.data, [](GLFWwindow *w) {
                    static auto cb = (WindowCloseCallback)priv::temp;
                    (*cb)(*((Window *)glfwGetWindowUserPointer(w)));
                });
            }

        }

    }

}
