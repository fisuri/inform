use std::time::Instant;
use turtle::{Drawing, Point, Turtle};

fn main() {
    const DELTA_TIME: f64 = 0.016;
    const TIME_LIMIT: f64 = 10.0;
    const N: usize = (TIME_LIMIT / DELTA_TIME) as usize;
    const MASS: f64 = 0.7;
    const WIND_SPEED: f64 = 16.0; // cкорость ветра x
    const COEFFICIENT: f64 = 10.0; // массштабирующий каофициент в пиксель/метр
    const COEFFICIENT_AIR: f64 = 0.7; // каофициент трения воздуха
    const G: f64 = 9.8 * COEFFICIENT; // ускорение свободного падения в пиксель/c^2

    let mut v_x: f64 = 20.0; // скорость в пикселях по оси х
    let mut v_y: f64 = 20.0; // скорость в пикселях по оси y
    let mut f_x: f64 = 0.0; // сила действующая по оси х
    let mut f_y: f64 = -MASS * G; // сила действующая по оси y
    let mut a_x: f64 = 0.0; // ускорение в пикселях по оси х
    let mut a_y: f64 = 0.0; // ускорение в пикселях по оис у

    let mut turtle_drawing = TurtleDrawing::new(200.0, 100.0, 0.0, 0.0);

    for _ in 0..N {
        let time = Instant::now();

        let x = turtle_drawing.x + v_x * DELTA_TIME;
        let y = turtle_drawing.y + v_y * DELTA_TIME;

        turtle_drawing.draw_walls();
        turtle_drawing.draw(x, y);

        f_x = -COEFFICIENT_AIR * (v_x - WIND_SPEED);
        a_x = f_x / MASS * COEFFICIENT;
        a_y = f_y / MASS * COEFFICIENT;
        v_x = a_x * DELTA_TIME;
        v_y = a_y * DELTA_TIME;

        if turtle_drawing.is_wall(x, y) == Some("Left".parse().unwrap())
            || turtle_drawing.is_wall(x, y) == Some("Right".parse().unwrap())
        {
            v_x = -0.5 * v_x;
        } else if turtle_drawing.is_wall(x, y) == Some("Top".parse().unwrap())
            || turtle_drawing.is_wall(x, y) == Some("Bottom".parse().unwrap())
        {
            v_y = -0.5 * v_y;
        }
        while time.elapsed().as_secs_f64() < DELTA_TIME {
            // Wait
        }
    }
}

struct TurtleDrawing {
    x: f64,
    y: f64,
    length: f64,
    width: f64,
    left_edge: f64,
    right_edge: f64,
    top_edge: f64,
    bottom_edge: f64,
    line: Turtle,
}

impl TurtleDrawing {
    fn new(length: f64, width: f64, x: f64, y: f64) -> TurtleDrawing {
        let mut line = Turtle::new();

        let left_edge = x - length / 2.0;
        let right_edge = x + length / 2.0;
        let top_edge = y + width / 2.0;
        let bottom_edge = y - width / 2.0;

        line.set_pen_size(3.0);

        line.hide();
        TurtleDrawing {
            x,
            y,
            length,
            width,
            left_edge,
            right_edge,
            top_edge,
            bottom_edge,
            line,
        }
    }

    fn draw(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
        self.line.clear();
        self.line.set_pen_color("red");
        self.line.set_speed(25);
        self.line.pen_up();
        self.line.go_to(Point {
            x: x + 0.0,
            y: y + 0.0,
        });
        self.line.pen_down();
        self.line.go_to(Point {
            x: x + 10.0,
            y: y + 0.0,
        });
        self.line.go_to(Point {
            x: x + 10.0,
            y: y + 10.0,
        });
        self.line.go_to(Point {
            x: x + 0.0,
            y: y + 10.0,
        });
        self.line.go_to(Point {
            x: x + 0.0,
            y: y + 0.0,
        });
    }

    fn draw_walls(&mut self) {
        let left_edge = self.left_edge;
        let right_edge = self.right_edge;
        let top_edge = self.top_edge;
        let bottom_edge = self.bottom_edge;

        self.line.set_pen_color("black");
        self.line.set_speed(25);
        self.line.pen_up();
        self.line.go_to(Point {
            x: left_edge,
            y: bottom_edge,
        });
        self.line.pen_down();
        self.line.go_to(Point {
            x: left_edge,
            y: top_edge,
        });
        self.line.go_to(Point {
            x: right_edge,
            y: top_edge,
        });
        self.line.go_to(Point {
            x: right_edge,
            y: bottom_edge,
        });
        self.line.go_to(Point {
            x: left_edge,
            y: bottom_edge,
        })
    }

    fn is_wall(&mut self, x: f64, y: f64) -> Option<String> {
        let left_edge = self.left_edge;
        let right_edge = self.right_edge;
        let top_edge = self.top_edge;
        let bottom_edge = self.bottom_edge;
        if x <= left_edge {
            Some(String::from("left"))
        } else if x >= right_edge {
            Some(String::from("right"))
        } else if y <= bottom_edge {
            Some(String::from("down"))
        } else if y >= top_edge {
            Some(String::from("up"))
        } else {
            None
        }
    }
}
