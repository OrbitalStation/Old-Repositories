#include "../inc/App.h"
#include "../inc/Map.h"
#include "../inc/Interface.h"

constexpr uint WIDTH  = 1200;
constexpr uint HEIGHT = 800;
constexpr float PROPORTIONAL = float(WIDTH) / float(HEIGHT);

int main() {

    App::init(WIDTH, HEIGHT);
    Interface::init(WIDTH, HEIGHT);
    Map::init("../img/textures.png");

    Map::add(Map::Wire, sf::Vector2f());

    sf::Vector2f mousePos = App::app.mapPixelToCoords(sf::Mouse::getPosition(App::app)), prevMousePos;
    sf::Vector2f mapCamLimitX, mapCamLimitY;
    //float mapCamLimitYInterface;
    bool isMouseOnMap;//, isMouseOnInterface;

    while (App::app.isOpen()) {

        prevMousePos = mousePos;
        mousePos = App::app.mapPixelToCoords(sf::Mouse::getPosition(App::app));

        isMouseOnMap = mousePos.x > mapCamLimitX.x && mousePos.x < mapCamLimitX.y;
        //isMouseOnInterface = isMouseOnMap;
        isMouseOnMap = isMouseOnMap && mousePos.y > mapCamLimitY.x && mousePos.y < mapCamLimitY.y;
 //       isMouseOnInterface = isMouseOnInterface && mousePos.y > mapCamLimitY.y && mousePos.y < mapCamLimitYInterface;

        while (App::app.pollEvent(App::event)) {
            switch (App::event.type) {
                case sf::Event::Closed:
                    App::terminate();
                    break;
                case sf::Event::Resized: {
                    App::updateCamera(App::cam,sf::FloatRect(0, 0, App::event.size.width, App::event.size.height));
                    Interface::updateCams();
                    //App::updateCamera(Interface::cam, sf::FloatRect(0, App::event.size.height / 4 * 3, App::event.size.width, App::event.size.height / 4));
                    auto zeroPos = App::app.mapPixelToCoords(sf::Vector2i());
                    mapCamLimitY = { 0 + zeroPos.y, (HEIGHT / 4 * 3) + zeroPos.y };
                    mapCamLimitX = { 0 + zeroPos.x, WIDTH + zeroPos.x };
                    //mapCamLimitYInterface = HEIGHT + zeroPos.y;
                    break;
                }
                case sf::Event::MouseWheelScrolled:
                    if (isMouseOnMap) App::cam.zoom((App::event.mouseWheelScroll.delta == 1 ? 0.98f : 1.02f));
                    break;
                case sf::Event::MouseButtonPressed:
                    if (isMouseOnMap) {
                        if (App::event.mouseButton.button == sf::Mouse::Middle) setCursor(App::Cursor::Hand);
                    } else {
                        if (App::event.mouseButton.button == sf::Mouse::Left) {

                        }
                    }
                    break;
                case sf::Event::MouseButtonReleased:
                    if (isMouseOnMap && App::event.mouseButton.button == sf::Mouse::Middle) setCursor(App::Cursor::Arrow);
                    break;
                default: break;
            }
        }

        if (sf::Keyboard::isKeyPressed(sf::Keyboard::Escape)) App::terminate();
        else if (sf::Mouse::isButtonPressed(sf::Mouse::Middle)) {
            if (isMouseOnMap) {
                App::cam.move((prevMousePos - mousePos) / PROPORTIONAL);
                App::app.setView(App::cam);
            }
        }

        App::app.clear(sf::Color::White);

        App::app.setView(App::cam);
        Map::draw();
        Interface::draw();

        App::app.display();
    }

}
