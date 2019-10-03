#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

extern crate rusty_scad;
//use rusty_scad::objects::shapes::cube;
//use rusty_scad::objects::shapes::cube;

use crate::rusty_scad::objects::Is3DObject;
use crate::rusty_scad::objects::colour::HasColour;
use crate::rusty_scad::objects::IsSerialisableObject;

fn main()
{
	//println!("Hello World");


	let mut cube = rusty_scad::objects::shapes::cube::cube(5.0, 1.0, 1.0);
	cube.rotate_x(45.0);
	//cube.set_colour(colour_rgba(1.0, 0.0, 1.0, 0.5));
	cube.set_colour(rusty_scad::objects::colour::colour_named("cyan"));


	eprintln!("cube = {:?}\n\n\n\n", cube);
	println!("{}", cube.serialise());

}
