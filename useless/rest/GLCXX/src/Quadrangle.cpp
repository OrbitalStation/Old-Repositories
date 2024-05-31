#include "../include/Figures/Quadrangle.hpp"
#include "glad/glad.h"

extern "C++" {

    namespace gl {

        namespace priv { extern unsigned int shaderProgram; }

        QuadrangleWithoutRotation::QuadrangleWithoutRotation() noexcept : vao(), vbo(), need_flush(true), vertices{} {
            if (glGenVertexArrays != nullptr) {
                glGenVertexArrays(1, &this->vao);
                glGenBuffers(1, &this->vbo);
            }
            float *pointer = this->vertices;

            *pointer++ = -1.0f;
            *pointer++ = +1.0f;
            *pointer++ = +0.0f;
            *pointer++ = +1.0f;
            *pointer++ = +1.0f;
            *pointer++ = +1.0f;
            *pointer++ = +1.0f;

            *pointer++ = +1.0f;
            *pointer++ = +1.0f;
            *pointer++ = +0.0f;
            *pointer++ = +1.0f;
            *pointer++ = +1.0f;
            *pointer++ = +1.0f;
            *pointer++ = +1.0f;

            *pointer++ = +1.0f;
            *pointer++ = -1.0f;
            *pointer++ = +0.0f;
            *pointer++ = +1.0f;
            *pointer++ = +1.0f;
            *pointer++ = +1.0f;
            *pointer++ = +1.0f;

            *pointer++ = -1.0f;
            *pointer++ = -1.0f;
            *pointer++ = +0.0f;
            *pointer++ = +1.0f;
            *pointer++ = +1.0f;
            *pointer++ = +1.0f;
            *pointer   = +1.0f;
        }

        Quadrangle::Quadrangle() noexcept : QuadrangleWithoutRotation(), model(1.f), rotationX(), rotationY(), rotationZ() { }

        void QuadrangleWithoutRotation::destroy() noexcept {
            if (this->vao == 0u) return;
            if (glGenVertexArrays != nullptr) {
                glDeleteBuffers(1, &this->vbo);
                glDeleteVertexArrays(1, &this->vao);
            }
            this->vao = 0u;
        }

        void QuadrangleWithoutRotation::flush() const noexcept {
            if (!this->need_flush) return;
            this->need_flush = false;
            glBindVertexArray(this->vao);
            glBindBuffer(GL_ARRAY_BUFFER, this->vbo);
            glBufferData(GL_ARRAY_BUFFER, sizeof(this->vertices), this->vertices, GL_DYNAMIC_DRAW);
            glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 7 * sizeof(float), nullptr);
            glEnableVertexAttribArray(0);
            glVertexAttribPointer(1, 4, GL_FLOAT, GL_FALSE, 7 * sizeof(float), (void *)(sizeof(float) * 3));
            glEnableVertexAttribArray(1);
        }

        void QuadrangleWithoutRotation::draw() const noexcept {
            this->flush();
            glm::mat4x4 model(1.f);
            glUniformMatrix4fv(glGetUniformLocation(priv::shaderProgram, "model"), 1, GL_FALSE, glm::value_ptr(model));
            glDrawArrays(GL_TRIANGLE_FAN, 0, 4);
        }

        void Quadrangle::draw() const noexcept {
            this->flush();
            glBindVertexArray(this->vao);
            glUniformMatrix4fv(glGetUniformLocation(priv::shaderProgram, "model"), 1, GL_FALSE, glm::value_ptr(this->model));
            glDrawArrays(GL_TRIANGLE_FAN, 0, 4);
        }

        void QuadrangleWithoutRotation::setColor(const Uint8 &vertex, const Color &color) noexcept {
            if (vertex > 3) return;
            float *pointer = this->vertices + vertex * 7 + 3;
            *pointer++ = Color::toFloat(color.r);
            *pointer++ = Color::toFloat(color.g);
            *pointer++ = Color::toFloat(color.b);
            *pointer   = Color::toFloat(color.a);
            this->need_flush = true;
        }

        Color QuadrangleWithoutRotation::getColor(const Uint8 &vertex) const noexcept {
            if (vertex > 3) return Color::Black;
            const float *pointer = this->vertices + vertex * 7 + 3;
            return Color(Color::toUint8(*pointer), Color::toUint8(*(pointer + 1)),
                         Color::toUint8(*(pointer + 2)), Color::toUint8(*(pointer + 3)));
        }

        void QuadrangleWithoutRotation::setPosition(const Uint8 &vertex, const float &x,
                const float &y, const float &z) noexcept {
            if (vertex > 3) return;
            float *pointer = this->vertices + vertex * 7;
            *pointer++ = x;
            *pointer++ = y;
            *pointer   = z;
            this->need_flush = true;
        }

        void QuadrangleWithoutRotation::setPosition(const Uint8 &vertex, const glm::vec3 &position) noexcept {
            this->setPosition(vertex, position.x, position.y, position.z);
        }

        glm::vec3 QuadrangleWithoutRotation::getPosition(const Uint8 &vertex) noexcept {
            if (vertex > 3) return glm::vec3();
            const float *pointer = this->vertices + vertex * 7;
            return glm::vec3(*pointer, *(pointer + 1), *(pointer + 2));
        }

        void QuadrangleWithoutRotation::getPosition(const Uint8 &vertex, float &x, float &y, float &z) const noexcept {
            if (vertex > 3) return;
            const float *pointer = this->vertices + vertex * 7;
            x = *pointer++;
            y = *pointer++;
            z = *pointer;
        }

        void QuadrangleWithoutRotation::getPosition(const Uint8 &vertex, float * const &x,
                float * const &y, float * const &z) const noexcept {
            if (vertex > 3) return;
            const float *pointer = this->vertices + vertex * 7;
            if (x != nullptr) *x = *pointer;
            ++pointer;
            if (y != nullptr) *y = *pointer;
            ++pointer;
            if (z != nullptr) *z = *pointer;
        }

        void Quadrangle::rotateX(const float &angle) noexcept {
            this->rotationX += angle;
            if (this->rotationX >= 360.f) this->rotationX -= 360.f;
            else if (this->rotationX < 0.f) this->rotationX += 360.f;
            this->model = glm::rotate(this->model, glm::radians(angle),glm::vec3(1.f, 0.f, 0.f));
            this->need_flush = true;
        }

        void Quadrangle::setRotationX(const float &angle) noexcept {
            this->model = glm::rotate(this->model, glm::radians(angle - this->rotationX),
            glm::vec3(1.f, 0.f, 0.f));
            this->rotationX = angle;
            this->need_flush = true;
        }

        void Quadrangle::rotateY(const float &angle) noexcept {
            this->rotationY += angle;
            if (this->rotationY >= 360.f) this->rotationY -= 360.f;
            else if (this->rotationY < 0.f) this->rotationY += 360.f;
            this->model = glm::rotate(this->model, glm::radians(angle),glm::vec3(0.f, 1.f, 0.f));
            this->need_flush = true;
        }

        void Quadrangle::setRotationY(const float &angle) noexcept {
            this->model = glm::rotate(this->model, glm::radians(angle - this->rotationY),
            glm::vec3(0.f, 1.f, 0.f));
            this->rotationY = angle;
            this->need_flush = true;
        }

        void Quadrangle::rotateZ(const float &angle) noexcept {
            this->rotationZ += angle;
            if (this->rotationZ >= 360.f) this->rotationZ -= 360.f;
            else if (this->rotationZ < 0.f) this->rotationZ += 360.f;
            this->model = glm::rotate(this->model, glm::radians(angle),glm::vec3(0.f, 0.f, 1.f));
            this->need_flush = true;
        }

        void Quadrangle::setRotationZ(const float &angle) noexcept {
            this->model = glm::rotate(this->model, glm::radians(angle - this->rotationZ),
            glm::vec3(0.f, 0.f, 1.f));
            this->rotationZ = angle;
            this->need_flush = true;
        }

        glm::vec3 QuadrangleWithoutRotation::getCenterPosition() noexcept {
            return (this->getPosition('\0') + this->getPosition('\1') +
                this->getPosition('\2') + this->getPosition('\3')) / 4.f;
        }

        void QuadrangleWithoutRotation::moveTo(const glm::vec3 &position) noexcept {
            glm::vec3 difference = position - this->getCenterPosition();
            this->setPosition('\0', this->getPosition('\0') + difference);
            this->setPosition('\1', this->getPosition('\1') + difference);
            this->setPosition('\2', this->getPosition('\2') + difference);
            this->setPosition('\3', this->getPosition('\3') + difference);
        }

        const float& Quadrangle::getRotationX() const noexcept { return this->rotationX; }

        const float& Quadrangle::getRotationY() const noexcept { return this->rotationY; }

        const float& Quadrangle::getRotationZ() const noexcept { return this->rotationZ; }

    }

}
