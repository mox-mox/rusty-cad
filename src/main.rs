#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

extern crate rusty_scad;
use rusty_scad::*;
//use rusty_scad::IsSerialisableScope;
use vecmath::mat4_inv;
use rusty_scad::math::*;

//{{{
pub fn zeppelin(l: f64, r: f64) -> Object
{
	let mut s1 = sphere("zeppelin::front", r);
	let mut s2 = sphere("zeppelin::rear", r);
	s1.translate_y(-l/2.0);
	s2.translate_y( l/2.0);

	let mut z = hull("zeppelin", [s1, s2]);

	let mut a = rusty_scad::anchors::Anchor::new("right");
	a.translate_x(1.0);
	z.add_anchor(a);

	let mut a = rusty_scad::anchors::Anchor::new("left");
	a.translate_x(-1.0);
	z.add_anchor(a);

	let mut a = rusty_scad::anchors::Anchor::new("front");
	a.translate_y(l/2.0+r);
	a.rel_rotate_x(-45.0);
	z.add_anchor(a);

	let mut a = rusty_scad::anchors::Anchor::new("rear");
	a.translate_y(-l/2.0-r);
	a.rel_rotate_z(45.0);
	z.add_anchor(a);

	z
}
//}}}



fn main()
{





	let mut c1 = zeppelin(5.0, 1.0);
	c1.set_fn(100);
	c1.set_debug();
	//c1.set_show_origin();
	c1.set_show_anchors();
	//c1.rel_rotate_z(45.0);
	c1.rotate_z(45.0);
	//c1.rel_translate_x(2.0);
	//c1.translate_x(2.0);



	let mut c2 = zeppelin(5.0, 1.0);
	c2.set_fn(100);
	c2.set_debug();
	c2.set_show_anchors();
	//c2.rotate_z(45.0);
	//c2.rel_translate_x(2.0);


	// This code is ugly.
	c2.snap_to_anchor("front", &c1, &c1["rear"].clone());
	// I would like to to something like this:
	//m1["front"].snap_to(z["rear"]);


	println!("{}{}", c1, c2);
}
