#include "../include/Window/View.hpp"

#include <cmath>
#include "glad/glad.h"

extern "C++" {

    namespace gl {

        namespace priv { extern unsigned int shaderProgram; }

        View::View(const unsigned int &screenWidth, const unsigned int &screenHeight,
                const float &speed) noexcept : need_flush(true), position(0.f, 0.f, 4.f),
            direction(),
            up(0.f, 1.f, 0.f),
            view(glm::lookAt(position, position + direction, up)), speed(speed),
            fov(glm::radians(45.f)),
            aspect(float(screenWidth) / float(screenHeight)),
            nearPlane(0.1f),
            farPlane(100.f),
            projection(glm::perspective(fov,
                    aspect, nearPlane, farPlane)),
            yaw(-90.f), pitch(), roll() { }

        void View::move(const Motion &motion, int times) noexcept {
            switch (motion) {
                case Motion::Up:
                    for (; times >= 0; --times) this->position.y += this->speed;
                    this->need_flush = true;
                    break;
                case Motion::Down:
                    for (; times >= 0; --times) this->position.y -= this->speed;
                    this->need_flush = true;
                    break;
                case Motion::Left:
                    for (; times >= 0; --times) this->position.x -= this->speed;
                    this->need_flush = true;
                    break;
                case Motion::Right:
                    for (; times >= 0; --times) this->position.x += this->speed;
                    this->need_flush = true;
                    break;
                case Motion::Front:
                    for (; times >= 0; --times) this->position.z -= this->speed;
                    this->need_flush = true;
                    break;
                case Motion::Back:
                    for (; times >= 0; --times) this->position.z += this->speed;
                    this->need_flush = true;
                    break;
                case Motion::Wait:
                    break;
            }
        }

        void View::setProjection(const float &fov, const float &aspect, const float &nearPlane,
                const float &farPlane) noexcept {
            this->projection = glm::perspective(glm::radians(this->fov = fov),
                    this->aspect = aspect, this->nearPlane = nearPlane,
                    this->farPlane = farPlane);
            this->need_flush = true;
        }

        const glm::mat4x4& View::getViewMatrix() const noexcept {
            return this->view;
        }

        glm::mat4x4& View::getViewMatrix() noexcept {
            return this->view;
        }

        void View::setSpeed(const float &speed) noexcept {
            this->speed = speed;
        }

        const float& View::getSpeed() const noexcept {
            return this->speed;
        }

        void View::flush() noexcept {
            if (!this->need_flush) return;
            this->need_flush = false;
            this->direction.x = std::cos(glm::radians(this->yaw)) * std::cos(glm::radians(this->pitch));
            this->direction.y = std::sin(glm::radians(this->pitch));
            this->direction.z = std::sin(glm::radians(this->yaw)) * std::cos(glm::radians(this->pitch));
            this->direction = glm::normalize(this->direction);
            this->view = glm::lookAt(this->position, this->position + this->direction, this->up);
            this->projection = glm::perspective(this->fov, this->aspect, this->nearPlane, this->farPlane);
            glUniformMatrix4fv(glGetUniformLocation(priv::shaderProgram, "view"), 1, GL_FALSE, glm::value_ptr(this->view));
            glUniformMatrix4fv(glGetUniformLocation(priv::shaderProgram, "projection"), 1, GL_FALSE, glm::value_ptr(this->projection));
        }

        void View::rotateYaw(const float &angle) {
            this->yaw += angle;
            if (this->yaw >= 360.f) this->yaw -= 360.f;
            else if (this->yaw < 0.f) this->yaw += 360.f;
            this->need_flush = true;
        }

        void View::rotatePitch(const float &angle) {
            this->pitch += angle;
            if (this->pitch >= 360.f) this->pitch -= 360.f;
            else if (this->pitch < 0.f) this->pitch += 360.f;
            this->need_flush = true;
        }

        void View::rotateRoll(float angle) {
            this->roll += angle;
            if (this->roll >= 360.f) this->roll -= 360.f;
            else if (this->roll < 0.f) this->roll += 360.f;
            glm::vec3 copy = this->up;
            angle = glm::radians(angle);
            this->up.x = copy.x * std::cos(angle) - copy.y * std::sin(angle);
            this->up.y = copy.x * std::sin(angle) + copy.y * std::cos(angle);
            this->need_flush = true;
        }

        void View::zoom(const float &fov) {
            this->fov -= fov;
            if (this->fov < 0.f) this->fov = 0.f;
            else if (this->fov > 180.f) this->fov = 180.f;
            this->need_flush = true;
        }

        void View::moveToSight(const Motion &motion, const SightIgnore &mask) {
            glm::vec3 dir = this->direction;
            if ((mask & SightIgnore::X) != SightIgnore::None) dir.x = 0.f;
            if ((mask & SightIgnore::Y) != SightIgnore::None) dir.y = 0.f;
            if ((mask & SightIgnore::Z) != SightIgnore::None) dir.z = 0.f;
            switch (motion) {
                case Motion::Wait:
                    break;
                case Motion::Front:
                    this->position += dir * this->speed;
                    this->need_flush = true;
                    break;
                case Motion::Back:
                    this->position -= dir * this->speed;
                    this->need_flush = true;
                    break;
                case Motion::Left:
                    this->position -= glm::normalize(glm::cross(dir, this->up)) * this->speed;
                    this->need_flush = true;
                    break;
                case Motion::Right:
                    this->position += glm::normalize(glm::cross(dir, this->up)) * this->speed;
                    this->need_flush = true;
                    break;
                default: break;
//                case Motion::Up:
//                    this->position += glm::normalize(glm::cross(this->up, this->direction)) * this->speed;
//                    this->need_flush = true;
//                    break;
//                case Motion::Down:
//                    this->position -= glm::normalize(glm::cross(this->up, this->direction)) * this->speed;
//                    this->need_flush = true;
//                    break;
            }
        }

    }

    gl::View::SightIgnore operator | (const gl::View::SightIgnore &l, const gl::View::SightIgnore &r) {
        return static_cast <gl::View::SightIgnore> (static_cast <unsigned int> (l) | static_cast <unsigned int> (r));
    }

    gl::View::SightIgnore operator & (const gl::View::SightIgnore &l, const gl::View::SightIgnore &r) {
        return static_cast <gl::View::SightIgnore> (static_cast <unsigned int> (l) & static_cast <unsigned int> (r));
    }

    gl::View::SightIgnore operator ^ (const gl::View::SightIgnore &l, const gl::View::SightIgnore &r) {
        return static_cast <gl::View::SightIgnore> (static_cast <unsigned int> (l) ^ static_cast <unsigned int> (r));
    }

}
