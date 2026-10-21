// данная структура должна иметь такое же расположение в памяти (layout), 
// как если бы она была объявлена на языке C
#[repr(C)]
struct Point {
    x:  f64,
    y:  f64,
}

// блок, который объявляет функции, реализованные вне текущего крейта 
// (обычно на языке C или в любой библиотеке, использующей C ABI 
//   [Application Binary Interface]).
// функция создана в файле native\distance.c
unsafe extern "C" {
    fn distance(a: Point, b: Point) -> f64;
}

fn main() {
    let pt1 = Point{x: 10.0, y: 10.0} ;
    let pt2 = Point{x: 5.0, y: 6.0} ;

    let d = unsafe {
        distance(pt1, pt2)
    } ;

    println!("Distance: {d}") ;
} /* Out:
Distance: 6.4031242374328485
*/
