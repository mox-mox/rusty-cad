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
	s2.translate_x(l);

	hull(s1, s2)
}
//}}}



fn main()
{
	let mut z = zeppelin(5.0, 1.0);
	z.set_fn(100);
	z.set_colour(colour_rgba(1.0, 0.0, 1.0, 0.5));

	println!("{}", z);
}
