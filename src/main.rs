#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

extern crate rusty_scad;
use rusty_scad::*;


//{{{
pub fn zeppelin(l: f64, r: f64) -> Object
{
	let mut s1 = sphere(r);
	let mut s2 = sphere(r);
	s1.translate_x(-l/2.0);
	s2.translate_x( l/2.0);

	hull(s1, s2)
}
//}}}



fn main()
{
	//let mut z = zeppelin(5.0, 1.0);
	//z.set_fn(100);
	//z.set_colour(colour_rgba(1.0, 0.0, 1.0, 0.5));

	//z.rotate_z(45.0);
	//z.translate_x(4.0);

	//println!("{}", z);


	let mut c = object_origin();
	//let mut c = cube(1.0, 1.0, 1.0);
	//c.set_colour(colour_named("blue"));
	//c.set_colour(colour_rgba(1.0, 0.0, 1.0, 0.5));
	c.translate_x(4.0);

	//c.rotate_z(45.0);
	//c.rotate_y(45.0);
	//c.rel_translate_x(2.0);


	println!("{}", c);



	//let mut cube = cube(5.0, 1.0, 1.0);
	//cube.rotate_z(45.0);
	//cube.set_colour(colour_rgba(1.0, 0.0, 1.0, 0.5));
	////cube.set_colour(colour_named("blue"));

	////let mut sphere = sphere(2.0);
	////sphere.rotate_x(45.0);
	////sphere.set_colour(colour_named("red"));
	////sphere.set_fn(100);

	////eprintln!("cube = {:?}\n\n\n\n", cube);
	////eprintln!("{}\n{}", cube.serialise(), sphere.serialise());
	////println!("{}", cube.serialise());
	//println!("{}", cube);

	//eprintln!("{}", cube.ref_sys);


	////let mut merged = hull(cube, sphere);
	//////merged.set_colour(colour_named("green"));


	////println!("{}", merged.serialise());



}
