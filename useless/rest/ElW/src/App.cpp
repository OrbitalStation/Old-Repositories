#pragma ide diagnostic ignored "cert-err58-cpp"

#include "../inc/App.h"

#define Cursor XCursor
#include <X11/Xlib.h>
#undef Cursor

#include <X11/cursorfont.h>
#include <GL/glx.h>

namespace App {

    sf::RenderWindow app;
    sf::View cam;
    sf::Event event{ };

    static XCursor currentXCursor = 0;

    void setCursor(Cursor cursor) {
        deleteCursor();
        currentXCursor = XCreateFontCursor(glXGetCurrentDisplay(), cursor == Cursor::Arrow ? XC_left_ptr : XC_hand2);
        XDefineCursor(glXGetCurrentDisplay(), app.getSystemHandle(), currentXCursor);
    }

    void deleteCursor() {
        if (currentXCursor != 0) {
            XUndefineCursor(glXGetCurrentDisplay(), app.getSystemHandle());
            XFreeCursor(glXGetCurrentDisplay(), currentXCursor);
        }
    }

    void init(uint width, uint height) {
        App::app.create(sf::VideoMode(width, height), "ElW");
        App::cam.reset(sf::FloatRect(0, 0, width,height));
        App::setCursor(App::Cursor::Arrow);
    }

    void terminate() {
        deleteCursor();
        app.close();
    }

    sf::Vector2f updateCamera(sf::View &camera, const sf::FloatRect &rect) {
        auto lastCenter = camera.getCenter();
        camera.reset(rect);
        camera.move(lastCenter -= camera.getCenter());
        return lastCenter;
    }

}
