// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimeter {
    fn calculate(&self);
}

struct Triangle {
    l1 : f32,
    l2 : f32,
    l3 : f32,
}

impl Perimeter for Triangle {
    fn calculate(&self) {
        println!("Perimeter is : {:?}", self.l1 + self.l2 + self.l3);
    }
}

struct Square {
    side : f32,
}


impl Perimeter for Square {
    fn calculate(&self) {
        println!("Perimeter is : {:?}", self.side * 4.0);
    }
}

fn calculation(shape: impl Perimeter) {
    shape.calculate();
}


fn main()
{
    calculation(Square { side: 5.0 });
    calculation(Triangle {l1 : 3.0, l2: 4.0, l3: 5.0});
}
