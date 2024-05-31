#include "../include/Window/Monitor.hpp"
#include <GLFW/glfw3.h>

extern "C++" {

    namespace gl {

        Monitor::Monitor() : data(nullptr) { }

        Monitor::Monitor(void * const &data) : data(data) { }

        Monitor Monitor::primary() {
            Monitor monitor((void *)glfwGetPrimaryMonitor());
            if (monitor.data == nullptr) throw error("cannot find any monitors");
            return monitor;
        }

        Monitor* Monitor::all() {
            int count;
            GLFWmonitor **monitors = glfwGetMonitors(&count);
            if (monitors == nullptr || count == 0) return nullptr;
            auto *result = new Monitor[count];
            for (--count; count >= 0; --count) result[count].data = (void *)monitors[count];
            return result;
        }

        Monitor* Monitor::all(int &count) {
            GLFWmonitor **monitors = glfwGetMonitors(&count);
            if (monitors == nullptr || count == 0) return nullptr;
            auto *result = new Monitor[count];
            for (int i = count - 1; i >= 0; --i) result[i].data = (void *)monitors[i];
            return result;
        }

        int Monitor::count() {
            int count;
            glfwGetMonitors(&count);
            return count;
        }

        VideoMode* Monitor::videoModes(int &count) {
            const GLFWvidmode *modes = glfwGetVideoModes((GLFWmonitor *)this->data, &count);
            if (modes == nullptr) {
                count = 0;
                return nullptr;
            }
            auto *result = new VideoMode[count];
            for (int i = count - 1; i >= 0; --i) result[i] = (const void *)&modes[i];
            return result;
        }

        VideoMode Monitor::videoMode() const {
            VideoMode vm;
            vm = (void *)glfwGetVideoMode((GLFWmonitor *)this->data);
            return vm;
        }

        void Monitor::size(int &x, int &y) {
            glfwGetMonitorPhysicalSize((GLFWmonitor *)this->data, &x, &y);
        }

        void Monitor::position(int &x, int &y) {
            glfwGetMonitorPos((GLFWmonitor *)this->data, &x, &y);
        }

        const char* Monitor::name() {
            return glfwGetMonitorName((GLFWmonitor *)this->data);
        }

        std::string Monitor::info() {
            std::string result = "Monitor ";
            VideoMode vm = this->videoMode();
            result.append(std::string(this->name()) + ":\n\tWidth: " + std::to_string(vm.width) + ";\n\tHeight: "
                + std::to_string(vm.height) + ";\n\tRedBits: " + std::to_string(vm.redBits)
                + ";\n\tGreenBits: " + std::to_string(vm.greenBits) + ";\n\tBlueBits: " + std::to_string(vm.blueBits)
                + ";\n\tRefreshRate: " + std::to_string(vm.refreshRate) + " Hz;\n");
            return result;
        }

    }

}
