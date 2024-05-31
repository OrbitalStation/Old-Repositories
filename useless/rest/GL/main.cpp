#include <zconf.h>
#include <iostream>
#include "GL/include/GL.hpp"

int main() {

    if (!gl::Init()) return -1;

    gl::Keyboard::setDefaultKeymap();

    gl::Window window(200, 100, "Window");

    window.show();

    sleep(1);

    window.destroy();

    gl::Keyboard::pollEvents();

    std::cout << gl::Keyboard::isKeyPressed(gl::Keyboard::A);

    gl::Terminate();

}

//#define X 100
//#define Y 100
//#define WIDTH 200
//#define HEIGHT 200
//#define WIDTH_MIN 50
//#define HEIGHT_MIN 50
//#define BORDER_WIDTH 0
//#define TITLE "TITLE"
//#define ICON_TITLE "ICON_TITLE"
//#define PRG_CLASS "PRG_CLASS"
//
//
//* SetWindowManagerHints - функция, которая передает информацию о
//* свойствах программы менеджеру окон.
//
//
//static void SetWindowManagerHints (
//        Display * display, 		Указатель на структуру Display
//        char * PClass, 		Класс программы
//        char * argv[],   		Аргументы программы
//        int argc,    			Число аргументов
//        Window window,    		Идентификатор окна
//        int x,     			Координаты левого верхнего
//        int y,	   				угла окна
//        int win_wdt,			Ширина  окна
//        int win_hgt,  			Высота окна
//        int win_wdt_min,			Минимальная ширина окна
//        int win_hgt_min, 		Минимальная высота окна
//        char * ptrTitle,  		Заголовок окна
//        char * ptrITitle,	Заголовок пиктограммы окна
//        Pixmap pixmap 	Рисунок пиктограммы
//)
//{
//    XSizeHints size_hints; Рекомендации о размерах окна
//
//    XWMHints wm_hints;
//    XClassHint class_hint;
//    XTextProperty windowname, iconname;
//
//    if ( !XStringListToTextProperty (&ptrTitle, 1, &windowname ) ||
//         !XStringListToTextProperty (&ptrITitle, 1, &iconname ) ) {
//        puts ( "No memory!\n");
//        exit ( 1 );
//    }
//
//    size_hints.flags = PPosition | PSize | PMinSize;
//    size_hints.min_width = win_wdt_min;
//    size_hints.min_height = win_hgt_min;
//    wm_hints.flags = StateHint | IconPixmapHint | InputHint;
//    wm_hints.initial_state = NormalState;
//    wm_hints.input = True;
//    wm_hints.icon_pixmap = pixmap;
//    class_hint.res_name = argv[0];
//    class_hint.res_class = PClass;
//
//    XSetWMProperties ( display, window, &windowname,
//                       &iconname, nullptr, 0, &size_hints, &wm_hints,
//                       &class_hint );
//}

//    Display *display;  /* Указатель на структуру Display */
//    int ScreenNumber;    /* Номер экрана */
//    GC gc;				/* Графический контекст */
//    XEvent report;
//    Window window;
//
//    /* Устанавливаем связь с сервером */
//    if ( ( display = XOpenDisplay ( nullptr ) ) == nullptr ) {
//        puts ("Can not connect to the X server!\n");
//        exit ( 1 );
//    }
//
//    /* Получаем номер основного экрана */
//    ScreenNumber = DefaultScreen ( display );
//
//    /* Создаем окно */
//    window = XCreateSimpleWindow ( display,
//                                   RootWindow ( display, ScreenNumber ),
//                                   X, Y, WIDTH, HEIGHT, BORDER_WIDTH,
//                                   BlackPixel ( display, ScreenNumber ),
//                                   WhitePixel ( display, ScreenNumber ) );
//
//
//
//    /* Задаем рекомендации для менеджера окон */
///*    SetWindowManagerHints ( display, PRG_CLASS, argv, argc,
//                            window, X, Y, WIDTH, HEIGHT, WIDTH_MIN,
//                            HEIGHT_MIN, TITLE, ICON_TITLE, 0 );*/
//
//    /* Выбираем события,  которые будет обрабатывать программа */
//    XSelectInput ( display, window, ExposureMask | KeyPressMask );
//
//    /* Покажем окно */
//    XMapWindow ( display, window );
//
//    //XIconifyWindow(display, window, ScreenNumber);
//
//    XUnmapWindow(display, window);
//
//
//
//    /* Создадим цикл получения и обработки ошибок */
//    while ( true ) {
//        XNextEvent ( display, &report );
//
//
//
//        switch ( report.type ) {
//            case Expose :
//                /* Запрос на перерисовку */
//                if ( report.xexpose.count != 0 )
//                    break;
//
//                gc = XCreateGC ( display, window, 0 , nullptr );
//
//                //XSetForeground ( display, gc, BlackPixel ( display, 0) );
//                XDrawString ( display, window, gc, 20,50,
//                              "First example", strlen ( "First example" ) );
//                XFreeGC ( display, gc );
//
//                XFlush(display);
//                break;
//
//            case KeyPress :
//                /* Выход нажатием клавиши клавиатуры */
//                XCloseDisplay ( display );
//                exit ( 0 );
//        }
//    }

