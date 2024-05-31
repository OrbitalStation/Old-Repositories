#ifndef GLCXX_TRIANGLE
#define GLCXX_TRIANGLE

#include "../Color.hpp"
#include "../System/Drawable.hpp"

extern "C++" {

    namespace gl {

        class Triangle : public Drawable {

        public:

            Triangle();

            void destroy();

            void setColor(const Uint8 &vertex, const Color &color);

            void setPosition(const Uint8 &vertex, const float &x, const float &y, const float &z);

        private:

            void draw() const noexcept override;

            void flush() const;

            float vertices[21];

            unsigned int vao, vbo;

            mutable bool need_flush;

        };

    }

}

#endif /* GLCXX_TRIANGLE */
