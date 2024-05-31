#include <GLFW/glfw3.h>
#include "../include/Window/Cursor.hpp"

extern "C++" {

    namespace gl {

        Cursor::Cursor() {
            this->data = nullptr;
        }

        Cursor::Cursor(const Type &type) {
            this->data = nullptr;
            this->create(type);
        }

        Cursor::Cursor(const Image &image, const unsigned int &originX, const unsigned int &originY) {
            this->data = nullptr;
            this->create(image, originX, originY);
        }

        bool Cursor::create(const Type &type) {
            this->destroy();
            switch (type) {
                case ResizeHorizontal:
                    this->data = (void *)glfwCreateStandardCursor(GLFW_HRESIZE_CURSOR);
                    break;
                case ResizeVertical:
                    this->data = (void *)glfwCreateStandardCursor(GLFW_VRESIZE_CURSOR);
                    break;
                case Arrow:
                    this->data = (void *)glfwCreateStandardCursor(GLFW_ARROW_CURSOR);
                    break;
                case Aim:
                    this->data = (void *)glfwCreateStandardCursor(GLFW_CROSSHAIR_CURSOR);
                    break;
                case Hand:
                    this->data = (void *)glfwCreateStandardCursor(GLFW_HAND_CURSOR);
                    break;
                case Column:
                    this->data = (void *)glfwCreateStandardCursor(GLFW_IBEAM_CURSOR);
                    break;
            }
            return this->data != nullptr;
        }

        bool Cursor::create(const Image &image, const unsigned int &originX, const unsigned int &originY) {
            this->destroy();
            GLFWimage i{int(image.width()), int(image.height()), image.data()};
            this->data = (void *)glfwCreateCursor(&i, int(originX), int(originY));
            return this->data != nullptr;
        }

        void Cursor::destroy() {
            if (this->data != nullptr) {
                glfwDestroyCursor((GLFWcursor *)this->data);
                this->data = nullptr;
            }
        }

    }

}
