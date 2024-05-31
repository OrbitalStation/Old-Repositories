#ifndef GLCXX_DRAWABLE
#define GLCXX_DRAWABLE

extern "C++" {

    namespace gl {

        class Drawable {

#ifdef GLCXX_WINDOW
            friend class Window;
#endif

            virtual void draw() const noexcept = 0;

        };

    }

}

#endif /* GLCXX_DRAWABLE */
