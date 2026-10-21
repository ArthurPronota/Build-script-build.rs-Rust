/*
    Этот код определяет структуру Point и функцию для вычисления 
    евклидова расстояния между двумя точками.
*/

// подключает заголовочный файл стандартной математической библиотеки.
#include <math.h>

// создаёт псевдоним для типа которой сразу создаётся имя Point
typedef struct {
    double x;
    double y;
} Point;

double distance(Point a, Point b) {
    double dx = a.x - b.x;
    double dy = a.y - b.y;

    return sqrt(dx * dx + dy * dy);
}
