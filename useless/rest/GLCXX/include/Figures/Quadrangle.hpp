#ifndef GLCXX_RECTANGLE
#define GLCXX_RECTANGLE

#include "../Color.hpp"
#include "../System/Drawable.hpp"
#include "../System/Geometry.hpp"

extern "C++" {

    namespace gl {

        class QuadrangleWithoutRotation : public Drawable {

        public:

            QuadrangleWithoutRotation() noexcept;

            void destroy() noexcept;

            void setColor(const Uint8 &vertex, const Color &color) noexcept;

            Color getColor(const Uint8 &vertex) const noexcept;

            void setPosition(const Uint8 &vertex, const float &x, const float &y, const float &z) noexcept;

            void setPosition(const Uint8 &vertex, const glm::vec3 &position) noexcept;

            glm::vec3 getPosition(const Uint8 &vertex) noexcept;

            void getPosition(const Uint8 &vertex, float &x, float &y, float &z) const noexcept;

            void getPosition(const Uint8 &vertex, float * const &x, float * const &y, float * const &z) const noexcept;

            void moveTo(const glm::vec3 &position) noexcept;

            glm::vec3 getCenterPosition() noexcept;

        protected:

            void draw() const noexcept override;

            void flush() const noexcept;

            float vertices[28];

            unsigned int vao, vbo;

            mutable bool need_flush;

        };

        class Quadrangle : public QuadrangleWithoutRotation {

        public:

            Quadrangle() noexcept;

            void rotateX(const float &angle) noexcept;

            void setRotationX(const float &angle) noexcept;

            void rotateY(const float &angle) noexcept;

            void setRotationY(const float &angle) noexcept;

            void rotateZ(const float &angle) noexcept;

            void setRotationZ(const float &angle) noexcept;

            const float& getRotationX() const noexcept;

            const float& getRotationY() const noexcept;

            const float& getRotationZ() const noexcept;

        private:

            void draw() const noexcept override;

            float rotationX, rotationY, rotationZ;

            glm::mat4x4 model;

        };

    }

}

#endif /* GLCXX_RECTANGLE */
