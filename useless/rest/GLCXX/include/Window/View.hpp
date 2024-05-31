#ifndef GLCXX_VIEW
#define GLCXX_VIEW

#include "../System/Geometry.hpp"

extern "C++" {

    namespace gl {

        class View {

        public:

            explicit View(const unsigned int &screenWidth, const unsigned int &screenHeight, const float &speed = 1.f) noexcept;

        public:

            enum class Motion {
                Wait, /* Why not? */
                Up, Down,
                Left, Right,
                Front, Back
            };

            enum class SightIgnore {
                X = 1u << 0u,
                Y = 1u << 1u,
                Z = 1u << 2u,
                XY = X | Y,
                XZ = X | Z,
                YZ = Y | Z,
                All = X | Y | Z,
                XYZ = All,
                None = 0u
            };

            void move(const Motion &motion, int times = 1) noexcept;

            void moveToSight(const Motion &motion, const SightIgnore &mask = SightIgnore::None);

            const glm::mat4x4& getViewMatrix() const noexcept;

            glm::mat4x4& getViewMatrix() noexcept;

            void setSpeed(const float &speed) noexcept;

            const float& getSpeed() const noexcept;

            void setProjection(const float &fov, const float &aspect, const float &nearPlane, const float &farPlane) noexcept;

            void rotateYaw(const float &angle);

            void rotatePitch(const float &angle);

            void rotateRoll(float angle);

            void zoom(const float &fov);

        private:

#ifdef GLCXX_WINDOW
            friend class Window;
#endif
            /* only for class Window */
            void flush() noexcept;

            float speed, yaw, pitch, roll, fov, aspect, nearPlane, farPlane;

            glm::vec3 position, direction, up;

            glm::mat4x4 view, projection;

            bool need_flush;

        };

        /* Some Syntax Sugar */
        using Camera = View;

    }

    gl::View::SightIgnore operator | (const gl::View::SightIgnore &l, const gl::View::SightIgnore &r);

    gl::View::SightIgnore operator & (const gl::View::SightIgnore &l, const gl::View::SightIgnore &r);

    gl::View::SightIgnore operator ^ (const gl::View::SightIgnore &l, const gl::View::SightIgnore &r);

}

#endif /* GLCXX_VIEW */
