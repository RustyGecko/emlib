
pub struct Rectangle {
    dx: u16,
    dy: u16,
    width: u16,
    height: u16,
}

static rect1: Rectangle = Rectangle {
    dx: 76,
    dy: 76,
    width: 51,
    height: 51,
};

static rect2: Rectangle = Rectangle {
    dx: 150,
    dy: 150,
    width: 51,
    height: 51,
};

static rectObst: Rectangle = Rectangle {
    dx: 0,
    dy: 0,
    width: 320,
    height: 5,
};

pub struct Circle {
    radius: u16,
    points: [u16; 35],
}

static circle_y: Circle = Circle {
    radius: 25,
    points: [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,20,21,21,22,22,23,23,23,23,24,24,24,24,24],
};

static circle_x: Circle = Circle {
    radius: 25,
    points: [24,24,24,24,24,23,23,23,23,22,22,21,21,20,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0],
};
