#ifndef GLCXX_WINDOW
#define GLCXX_WINDOW

#include "../Color.hpp"
#include "../System/Drawable.hpp"
#include "Cursor.hpp"
#include "Event.hpp"
#include "View.hpp"
#include "Monitor.hpp"

extern "C++" {

    namespace gl {

        class Window {

        public:

            enum Style {
                Fullscreen,
                NotResizeable,
                NotDecorated,
                Default
            };

            Window(const VideoMode &vm, const char * const &title, const Style &style = Default);

            void setView(View &view);

            void setCursor(Cursor &cursor);

            void setCursorMode(const Cursor::Mode &mode);

            void setCursorPos(const glm::vec3 &pos);

            glm::vec2 getCursorPos();

            bool isOpen();

            void clear(const Color &color = Color::Black);

            void refresh();

            bool pollEvents(Event &event);

            void close();

            void destroy();

            void draw(Drawable &drawable);

            void setIcon(const Image &image);

            void resetIcon();

            void iconify();

        private:

            static void loadAndConfigureOpenGL();

#ifdef GLCXX_LOW
            friend void low::setCursorMoveCallback(const Window &, const low::CursorMoveCallback &);
            friend void low::setWindowResizeCallback(const Window &, const low::WindowResizeCallback &);
            friend void low::setMouseWheelScrollCallback(const Window &, const low::MouseWheelScroll &);
            friend void low::setKeyCallback(const Window &, const low::KeyCallback &);
            friend void low::setWindowCloseCallback(const Window &, const low::WindowCloseCallback &);
#endif

            void *data;

            Cursor *cursor;

            View *camera;

        };

    }

}

#endif /* GLCXX_WINDOW */
