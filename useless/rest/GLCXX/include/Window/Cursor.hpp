#ifndef GLCXX_CURSOR
#define GLCXX_CURSOR

#include "../Image.hpp"
#include "../System/Geometry.hpp"

extern "C++" {

    namespace gl {

        class Window;

        class Cursor {

            friend class Window;

        public:

            enum Mode {
                Normal,
                Hidden,
                Disabled
            };

            enum Type {
                ResizeHorizontal,
                ResizeVertical,
                Arrow,
                Aim,
                Hand,
                Column
            };

        public:

            Cursor();

            Cursor(const Type &type);

            Cursor(const Image &image, const unsigned int &originX = 0u, const unsigned int &originY = 0u);

            bool create(const Type &type);

            bool create(const Image &image, const unsigned int &originX = 0u, const unsigned int &originY = 0u);

            void destroy();

        private:

            void *data;

        };

    }

}

#endif /* GLCXX_CURSOR */
