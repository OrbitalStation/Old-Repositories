#include "glad/glad.h"
#include <GLFW/glfw3.h>
#include "../include/Window/Window.hpp"
#include "../include/Window/VideoMode.hpp"
#include "../include/Window/Monitor.hpp"
#include "stb_image/read.h"
#include "stb_image/write.h"

extern "C++" {

    namespace gl {

        VideoMode::VideoMode() : width(), height(), redBits(), greenBits(), blueBits(), refreshRate() { }

        VideoMode::VideoMode(const int &width, const int &height, const int &r, const int &g, const int &b,
                const int &refresh)  : width(width), height(height), redBits(r), greenBits(g), blueBits(b),
                refreshRate(refresh) { }

        VideoMode::VideoMode(const int &width, const int &height) {
            auto primary = Monitor::primary().videoMode();
            this->width = width;
            this->height = height;
            this->redBits = primary.redBits;
            this->blueBits = primary.blueBits;
            this->greenBits = primary.greenBits;
            this->refreshRate = primary.refreshRate;
        }

        VideoMode::VideoMode(const Monitor &monitor) {
            auto primary = monitor.videoMode();
            this->width = primary.width;
            this->height = primary.height;
            this->redBits = primary.redBits;
            this->blueBits = primary.blueBits;
            this->greenBits = primary.greenBits;
            this->refreshRate = primary.refreshRate;
        }

        VideoMode& VideoMode::operator=(const void * const &data) {
            if (data == nullptr) return *this;
            this->width = ((GLFWvidmode *)data)->width;
            this->height = ((GLFWvidmode *)data)->height;
            this->redBits = ((GLFWvidmode *)data)->redBits;
            this->greenBits = ((GLFWvidmode *)data)->greenBits;
            this->blueBits = ((GLFWvidmode *)data)->blueBits;
            this->refreshRate = ((GLFWvidmode *)data)->refreshRate;
            return *this;
        }

        namespace priv { unsigned int shaderProgram; }

        void Window::loadAndConfigureOpenGL() {
            if (gladLoadGLLoader((GLADloadproc)glfwGetProcAddress) == 0)
                throw error("Cannot load OpenGL functions");

            stbi_set_flip_vertically_on_load(1);
            stbi_flip_vertically_on_write(1);

            int success;

            char log[512];

            const char *vertexShaderSource = "#version 330 core\n"
                                             "layout (location = 0) in vec3 pos;\n"
                                             "layout (location = 1) in vec4 color;\n"
                                             "out vec4 _color;\n"
                                             "uniform mat4 projection;\n"
                                             "uniform mat4 view;\n"
                                             "uniform mat4 model;\n"
                                             "void main() {\n"
                                             "    gl_Position = projection * view * model * vec4(pos, 1.f);\n"
                                             "    _color = color;\n"
                                             "}\n\0";

            unsigned int vertexShader = glCreateShader(GL_VERTEX_SHADER);

            glShaderSource(vertexShader, 1, &vertexShaderSource, nullptr);

            glCompileShader(vertexShader);

            glGetShaderiv(vertexShader, GL_COMPILE_STATUS, &success);

            if (success == 0) {
                glGetShaderInfoLog(vertexShader, 512, nullptr, log);
                throw error((std::string("Cannot create vertex shader: ") + log).c_str());
            }

            const char *fragmentShaderSource = "#version 330 core\n"
                                               "in vec4 _color;\n"
                                               "out vec4 FragColor;\n"
                                               "void main() {\n"
                                               "    FragColor = _color;\n"
                                               "}\n\0";

            unsigned int fragmentShader = glCreateShader(GL_FRAGMENT_SHADER);

            glShaderSource(fragmentShader, 1, &fragmentShaderSource, nullptr);

            glCompileShader(fragmentShader);

            glGetShaderiv(fragmentShader, GL_COMPILE_STATUS, &success);

            if (success == 0) {
                glGetShaderInfoLog(fragmentShader, 512, nullptr, log);
                throw error((std::string("Cannot create fragment shader: ") + log).c_str());
            }

            priv::shaderProgram = glCreateProgram();

            glAttachShader(priv::shaderProgram, vertexShader);

            glAttachShader(priv::shaderProgram, fragmentShader);

            glLinkProgram(priv::shaderProgram);

            glGetProgramiv(priv::shaderProgram, GL_LINK_STATUS, &success);

            if (success == 0) {
                glGetProgramInfoLog(priv::shaderProgram, 512, nullptr, log);
                throw error((std::string("Cannot create shader program: ") + log).c_str());
            }

            glDeleteShader(vertexShader);

            glDeleteShader(fragmentShader);

            glEnable(GL_DEPTH_TEST);
        }

        Window::Window(const VideoMode &vm, const char * const &title, const Style &style) :
                camera(nullptr), cursor(nullptr) {
            glfwWindowHint(GLFW_RED_BITS, vm.redBits);
            glfwWindowHint(GLFW_GREEN_BITS, vm.greenBits);
            glfwWindowHint(GLFW_BLUE_BITS, vm.blueBits);
            glfwWindowHint(GLFW_REFRESH_RATE, vm.refreshRate);
            glfwWindowHint(GLFW_RESIZABLE, style != NotResizeable);
            glfwWindowHint(GLFW_DECORATED, style != NotDecorated);
            this->data = (void *)(glfwCreateWindow(int(vm.width), int(vm.height),
                    title, style == Fullscreen ? glfwGetPrimaryMonitor() : nullptr, nullptr));
            if (this->data == nullptr) throw error("Cannot create window");
            glfwMakeContextCurrent((GLFWwindow *)this->data);
            glfwSetWindowUserPointer((GLFWwindow *)this->data, (void *)this);
            static bool is_first_time = false;
            if (!is_first_time) {
                this->loadAndConfigureOpenGL();
                is_first_time = true;
            }
        }

        void Window::setView(View &view) {
            this->camera = &view;
        }

        void Window::setCursor(Cursor &cursor) {
            this->cursor = &cursor;
            glfwSetCursor((GLFWwindow *)this->data, (GLFWcursor *)this->cursor->data);
        }

        void Window::setCursorMode(const Cursor::Mode &mode) {
            glfwSetInputMode((GLFWwindow *)this->data, GLFW_CURSOR,
                    mode == Cursor::Normal ? GLFW_CURSOR_NORMAL :
                    (mode == Cursor::Hidden ? GLFW_CURSOR_HIDDEN : GLFW_CURSOR_DISABLED));
        }

        void Window::setCursorPos(const glm::vec3 &pos) {
            glfwSetCursorPos((GLFWwindow *)this->data, double(pos.x), double(pos.y));
        }

        glm::vec2 Window::getCursorPos() {
            double x, y;
            glfwGetCursorPos((GLFWwindow *)this->data, &x, &y);
            return { float(x), float(y) };
        }

        bool Window::isOpen() { return !glfwWindowShouldClose((GLFWwindow *)this->data); }

        void Window::clear(const Color &color) {
            if (glfwGetCurrentContext() != (GLFWwindow *)this->data) return;
            glClearColor(Color::toFloat(color.r), Color::toFloat(color.g), Color::toFloat(color.b), Color::toFloat(color.a));
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        }

        void Window::refresh() {
            this->camera->flush();
            glfwSwapBuffers((GLFWwindow *)this->data);
        }

        bool Window::pollEvents(Event &event) {
            event.clear();
            if (glfwGetCurrentContext() != (GLFWwindow *)this->data || glfwWindowShouldClose((GLFWwindow *)this->data)) return false;
            glfwPollEvents();
            if (glfwWindowShouldClose((GLFWwindow *)this->data)) {
                glfwSetWindowShouldClose((GLFWwindow *)this->data, GLFW_FALSE);
                event.type = Event::windowClosed;
                return true;
            }
            return false;
        }

        void Window::close() { glfwSetWindowShouldClose((GLFWwindow *)this->data, GLFW_TRUE); }

        void Window::destroy() { glfwDestroyWindow((GLFWwindow *)this->data); }

        void Window::draw(Drawable &drawable) {
            if (this->data == nullptr || this->camera == nullptr) return;
            if (glfwGetCurrentContext() != (GLFWwindow *)this->data) return;
            if (glfwWindowShouldClose((GLFWwindow *)this->data)) return;
            glUseProgram(priv::shaderProgram);
            drawable.draw();
        }

        void Window::setIcon(const Image &image) {
            if (image.format() == Image::none) {
                this->resetIcon();
            } else if (image.format() == Image::png) {
                GLFWimage i{int(image.width()), int(image.height()), image.data()};
                glfwSetWindowIcon((GLFWwindow *)this->data, 1, &i);
            } else if (image.format() == Image::jpg) {
                int i = int(image.width() * image.height()) / 3 * 4;
                auto *src = new unsigned char[i];
                auto d = image.data();
                for (int j = --i; i >= 0; --i) {
                    if (i % 4 == 0) src[i] = '\255';
                    else src[i] = d[j++];
                }
                GLFWimage img{int(image.width()) / 3 * 4, int(image.height()) / 3 * 4, src};
                glfwSetWindowIcon((GLFWwindow *)this->data, 1, &img);
                delete[] src;
            }
        }

        void Window::resetIcon() {
            glfwSetWindowIcon((GLFWwindow *)this->data, 0, nullptr);
        }

        void Window::iconify() {
            glfwIconifyWindow((GLFWwindow *)this->data);
        }

    }

}
