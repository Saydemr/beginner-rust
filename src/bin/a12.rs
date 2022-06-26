// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
#![allow(dead_code)]

struct Dimension {
    x : f32,
    y : f32,
    z : f32
}

impl Dimension {
    fn print(&self)
    {
        print!("X : {} ", self.x);
        print!("Y : {} ", self.y);
        println!("Z : {} ", self.z);
    }

    fn new(x : f32, y : f32, z : f32) -> Self
    {
        Self { x : x, y : y, z : z}
    }
}

enum Color {
    RED,
    BLACK,
    BLUE,
    GRAY
}

impl Color {
    fn print(&self)
    {
        match self {
            Color::RED => println!("RED"),
            _ => println!("Not RED")
        }
    }
}

struct Box {
    dimensions : Dimension,
    weight : f32,
    color : Color
}

impl Box {
    fn new() -> Self {
        Self {
            dimensions : Dimension::new(1.0, 2.0, 3.3),
            weight : 10.2f32,
            color : Color::RED
        }
    }

    fn new_with_params(dimensions : Dimension, weight : f32, color : Color) -> Self
    {
        Self {
            dimensions : dimensions,
            weight : weight,
            color : color
        }
    }

    fn print_box(&self)
    {
        self.color.print();
        self.dimensions.print();
        println!("Weight : {} ", self.weight);
    }
}

fn main() 
{
    let my_box2 = Box::new_with_params(Dimension::new(1.0, 2.0, 3.0), 2.2, Color::BLACK);
    let my_box = Box::new();
    my_box.print_box();
    my_box2.print_box();
}
