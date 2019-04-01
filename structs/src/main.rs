struct Point {
    x: i32,
    y: i32,
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let origin_x = 0;
    let origin_y = 0;

    let origin = Point { x: 0, y: 0 }; // origin: Point
    println!("The origin is at ({}, {})", origin.x, origin.y);

    let mut point = Point { x: 5, y: 0};
    point.x =5;
    println!("The point is at ({}, {})", point.x, point.y);

    let mut point = Point3d { x: 0, y: 0, z: 0 };
    point = Point3d { y: 1, .. point };

    let origin = Point3d { x: 0, y: 0, z: 0 };
    let point = Point3d { z: 1, x: 2, .. origin };

    // Unit like 構造体
    struct Electron;
    let x = Electron;
}

