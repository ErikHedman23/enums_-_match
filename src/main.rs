/*
Enums:

Defines a data type with multiple possible variants.


*/

/*
The match operator:
Compares a given value to a series of patterns to determine which code to execute.

This is similar to a switch statement in languages like C# or Go

The match operator can also be used with things other than enums...

if you were to use the _ wildcard, this is used as a default for the program to fall on when it has not found a match yet.

If it has still not found a match, once it hits that wild card, it will return the result of that wildcard, even if the next
pattern in the match was the correct one.  If the wild card comes before it, it will select the wild card always.

enums can also have implementations assigned to them as seen below:



*/
#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

impl Shape {
    fn get_perimeter(&self) -> f64 {
        match *self {
            // You no longer need to add the dereference operator (*) now, but it is helpful to see at times
            Shape::Circle(r) => r * 2.0 * std::f64::consts::PI,
            Shape::Rectangle(w, h) => (2.0 * w) + (2.0 * h),
            Shape::Triangle(a, b, c) => a + b + c,
        }
    }
}
fn main() {
    let my_shape = Shape::Rectangle(2.90, 8.8);
    println!("This is my shape a {:?}", my_shape);

    match my_shape {
        Shape::Circle(r) => println!("Circle with radius {}", r),
        Shape::Rectangle(w, h) => println!("{} x {} Rectangle", w, h),
        Shape::Triangle(a, b, c) => println!("Triangle with sides {}, {}, {}", a, b, c),
    };

    let my_num = 1u8;

    let result = match my_num {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        _ => {
            println!("{} did not match", my_num);
            "something else"
        }
    };

    println!("result is {}", result);

    let perimeter = my_shape.get_perimeter();
    println!("The perimeter is {}", perimeter)
}
