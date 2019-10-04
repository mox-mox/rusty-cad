#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

extern crate rusty_scad;
use rusty_scad::primitive::cube;
use rusty_scad::colour::colour_named;
use rusty_scad::primitive::sphere;
use crate::rusty_scad::Is3DObject;
use crate::rusty_scad::colour::HasColour;
use crate::rusty_scad::IsSerialisableObject;

fn main()
{
	let mut cube = cube(5.0, 1.0, 1.0);
	cube.rotate_x(45.0);
	//cube.set_colour(colour_rgba(1.0, 0.0, 1.0, 0.5));
	cube.set_colour(colour_named("yellow"));

	let mut sphere = sphere(2.0);
	sphere.rotate_x(45.0);
	sphere.set_colour(colour_named("red"));
	sphere.set_fn(100);

	eprintln!("cube = {:?}\n\n\n\n", cube);
	println!("{}\n{}", cube.serialise(), sphere.serialise());

}
