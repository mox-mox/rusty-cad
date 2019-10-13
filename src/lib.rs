#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

extern crate vecmath;
pub mod math; // Use 'pub mod' if you want it to be visible outside library.

//pub use math::RefSysExt;
pub use math::*;








extern crate ndarray;
use std::fmt;
//use crate::refsys::RefSys;
pub use colour::colour_none;
pub use colour::colour_named;
pub use colour::colour_rgb;
pub use colour::colour_rgba;

//{{{ helper_traits

//{{{
trait IsSerialisableScope
{
	fn serialise(&self, indentation: usize, child : &str) -> String;
}
//}}}

//}}}

//{{{
impl IsSerialisableScope for math::Matrix3D
{
	fn serialise(&self, indentation: usize, child : &str) -> String
	{
		let indent   = "\t".repeat(indentation as usize);
		//                              "multmatrix(m = [";
		let shift_in = indent.clone() + "                ";

		let mut retval = indent.clone();
		retval += "multmatrix(m = \n";
		retval += &(self.display(indentation+1) + ")\n");
		retval += &("\t".repeat(indentation as usize) + "{\n");
		retval += &child;
		retval += "\n";
		retval += &("\t".repeat(indentation as usize) + "};\n");
		retval
	}
}
//}}}

//{{{
mod colour
{
	use std::fmt;
	// Numeric representation of a colour
	type ColourVec = vecmath::Vector4<f64>;

	//{{{ pub enum Colour

	#[derive(Debug)]
	#[derive(Clone)]
	pub enum Colour
	{
		Unset,
		Numeric(ColourVec),
		Named(String),
	}
	//}}}

	//{{{ Colour constructors

	pub fn colour_none(name: String) -> Colour
	{
		Colour::Unset
	}
	pub fn colour_named(name: &str) -> Colour
	{
		Colour::Named(name.to_string())
	}
	pub fn colour_rgba(r : f64, g : f64, b : f64, a : f64) -> Colour
	{
		Colour::Numeric(ColourVec::from([r,g,b,a]))
	}
	pub fn colour_rgb(r : f64, g : f64, b : f64) -> Colour
	{
		colour_rgba(r,g,b,1.0)
	}
	//}}}

	//{{{
	impl crate::IsSerialisableScope for Colour
	{
		fn serialise(&self, indentation : usize, child : &str) -> String
		{
			let tabs = "\t".repeat(indentation);
			match &self
			{
				Self::Unset        => format!("{}", child),
				Self::Numeric(vec) => format!("{0}color( {1:?} )\n{0}{{\n{2}\n{0} }};\n", tabs,  vec, child),
				Self::Named(name)  => format!("{0}color(\"{1}\")\n{0}{{\n{2}\n{0} }};\n", tabs, name, child),
			}
		}
	}
	//}}}

	//{{{
	impl fmt::Display for Colour
	{
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
		{
			match &self
			{
				Self::Unset        => write!(f, "Unset"),
				Self::Numeric(vec) => write!(f, "{:?}", vec),
				Self::Named(name)  => write!(f, "{}", name),
			}
		}
	}
	//}}}
}
//}}}

//{{{
mod modifiers
{
	use std::fmt;
	// openscad modifiers: #: debug, %: background, !: root, *:disable
	//{{{ pub enum ScadModifier

	#[derive(Debug)]
	#[derive(Clone)]
	pub enum ScadModifier
	{
		Unset,
		Debug,
		Background,
		Root,
		Disable,
	}
	//}}}
	//{{{
	impl fmt::Display for ScadModifier
	{
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
		{
			match &self
			{
				Self::Unset      => write!(f, ""),
				Self::Debug      => write!(f, "#"),
				Self::Background => write!(f, "%"),
				Self::Root       => write!(f, "!"),
				Self::Disable    => write!(f, "*"),
			}
		}
	}
	//}}}

	// custom modifiers: show anchors, show origin
	//{{{ pub enum CustomModifier

	#[derive(Debug)]
	#[derive(Clone)]
	pub enum CustomModifier
	{
		Unset,
		ShowOrigin,
		ShowAnchors,
	}
	//}}}
}
//}}}

//{{{ TODO:
pub mod anchors
{
	//{{{
	pub struct AnchorConstraint
	{
		pub x        : bool,
		pub y        : bool,
		pub z        : bool,
		pub relative : bool,
	}
	//}}}

	//{{{
	pub struct Anchor
	{
		pub ref_sys               : crate::Matrix3D,
		pub constrain_rotation    : AnchorConstraint,
		pub constrain_translation : AnchorConstraint,
		pub constrain_scale       : AnchorConstraint,
		pub constrain_shear       : AnchorConstraint,
	}
	//}}}

	//{{{
	impl Anchor
	{
	}
	//}}}
}
//}}}

//// TODO: Polyhedron
//// TODO: text
//// TODO: Measure::Length
//// TODO: Measure::Angle
//// TODO: Measure::Triangle

//{{{ Define Object

//{{{ pub enum BooleanOp

#[derive(Debug)]
#[derive(Clone)]
#[allow(non_camel_case_types)] // We use the debug output to create the OpenSCad code. And that code requires the names to be lower-case.
pub enum BooleanOp
{
	union,
	difference,
	intersection,
	hull,
	minkowski,
}
//}}}

//{{{ pub enum Shape

#[derive(Debug)]
#[derive(Clone)]
pub enum Shape
{
	Cube      { x: f64, y: f64, z: f64},
	Sphere    { r: f64, face_number: Option<i32>, face_angle: Option<f64>, face_size: Option<f64> },
	Cylinder  { h: f64, r1 : f64, r2 : f64, face_angle  : Option<f64>, face_size   : Option<f64>, face_number : Option<i32> },

	//Composite { op: BooleanOp, c1: Box<Object>, c2: Box<Object> },
	Composite { op: BooleanOp, children: Vec<Object> },
}

//{{{
impl Shape
{
	//{{{
	fn serialise(&self, indentation : usize) -> String
	{
		let tabs = "\t".repeat(indentation);
		let indent = indentation + 1;
		match &self
		{
			//{{{
			Shape::Cube{x,y,z}                                        =>
			{
				tabs + "cube([" + &x.to_string() + ", " + &y.to_string() + ", " + &z.to_string() + "]);"
			}
			//}}}
			//{{{
			Shape::Sphere{r,face_number,face_angle,face_size}         =>
			{
				let fan = if let Some(x) = face_number { String::from(", $fn=") + &x.to_string() } else { String::from("") };
				let faa = if let Some(x) = face_angle  { String::from(", $fa=") + &x.to_string() } else { String::from("") };
				let fas = if let Some(x) = face_size   { String::from(", $fs=") + &x.to_string() } else { String::from("") };

				tabs + "sphere( r=" + &r.to_string() + &fan + &faa + &fas + ");"
			}
			//}}}
			//{{{
			Shape::Cylinder{h,r1,r2,face_number,face_angle,face_size} =>
			{
				let fan = if let Some(x) = face_number { String::from(", $fn=") + &x.to_string() } else { String::from("") };
				let faa = if let Some(x) = face_angle  { String::from(", $fa=") + &x.to_string() } else { String::from("") };
				let fas = if let Some(x) = face_size   { String::from(", $fs=") + &x.to_string() } else { String::from("") };

				tabs + "cylinder( h=" + &h.to_string() + ", r1=" + &r1.to_string() + ", r2=" + &r2.to_string() + &fan + &faa + &fas + ");"
			}
			//}}}
			////{{{
			//Shape::Composite{op, c1, c2} =>
			//{
			//	//tabs + &format!("{:?}", &op) + "() {\n" + &(*c1.to_string()) + "\n" + &(*c2.to_string()) + "\n};"
			//	format!("{0}{1:?}()\n{0}{{\n{2:>indent$}\n{3:>indent$}\n{0} }};", tabs, &op, c1, c2, indent=indent)
			//}
			////}}}
			//{{{
			Shape::Composite{op, children} =>
			{
				//retval = format!("{0}{1:?}()\n{0}{{\n{2:>indent$}\n{3:>indent$}\n{0} }};", tabs, &op, c1, c2, indent=indent);
				let mut retval = format!("{0}{1:?}()\n{0}{{\n", tabs, &op);
					for child in children
					{
						retval += &format!("{:>indent$}\n", child, indent=indent);
					}
				retval += &format!("{} }};", tabs);
				retval
			}
			//}}}
		}
	}
	//}}}
}
//}}}

//}}}

//{{{pub struct Object

#[derive(Debug)]
#[derive(Clone)]
pub struct Object
{
	pub shape       : Shape,
	pub ref_sys     : crate::Matrix3D,
	pub colour      : crate::colour::Colour,
	scad_modifier   : crate::modifiers::ScadModifier,
	custom_modifier : crate::modifiers::CustomModifier,
}


//{{{
impl Object
{
	//{{{ Positions in a MultMatrix
	//
	// [ (0,0) (0,1) (0,2) (0,3) ]
	// [ (1,0) (1,1) (1,2) (1,3) ]
	// [ (2,0) (2,1) (2,2) (2,3) ]
	// [ (3,0) (3,1) (3,2) (3,3) ]
	//}}}

	//{{{
	fn new(shape : Shape) -> Self
	{
		Self{
			shape           : shape,
			ref_sys         : crate::identity3D(),
			colour          : crate::colour::Colour::Unset, 
			scad_modifier   : crate::modifiers::ScadModifier::Unset, 
			custom_modifier : crate::modifiers::CustomModifier::Unset, 
		}
	}
	//}}}

	//{{{ Helpers
	
	// TODO: Iterator over compound object tree


	//}}}

	//{{{ Rendering
	//{{{
	pub fn set_fn(&mut self, num : i32)
	{
		//{{{ Commeted
		//
		//if let Shape::Sphere{r,ref mut face_number,face_angle,face_size} = self.shape {
		//	*face_number = Some(num);
		//}
		//if let Shape::Cylinder{h,r1,r2,ref mut face_number,face_angle,face_size} = self.shape {
		//	*face_number = Some(num);
		//}
		//}}}

		match self.shape
		{
			Shape::Cube{x,y,z}                                                => {},
			Shape::Sphere{r,ref mut face_number,face_angle,face_size}         => *face_number = Some(num),
			Shape::Cylinder{h,r1,r2,ref mut face_number,face_angle,face_size} => *face_number = Some(num),
			//Shape::Composite{ref op,ref mut c1,ref mut c2}                  => { c1.set_fn(num); c2.set_fn(num); },
			Shape::Composite{ref op,ref mut children}                         => { for child in children { child.set_fn(num) } },
		}
	}
	//}}}
	//{{{
	pub fn set_fa(&mut self, num : f64)
	{
		match self.shape
		{
			Shape::Cube{x,y,z}                                                => {},
			Shape::Sphere{r,face_number,ref mut face_angle,face_size}         => *face_angle = Some(num),
			Shape::Cylinder{h,r1,r2,face_number,ref mut face_angle,face_size} => *face_angle = Some(num),
			//Shape::Composite{ref op,ref mut c1,ref mut c2}                    => { c1.set_fa(num); c2.set_fa(num); },
			Shape::Composite{ref op,ref mut children}                         => { for child in children { child.set_fa(num) } },
		}
	}
	//}}}
	//{{{
	pub fn set_fs(&mut self, num : f64)
	{
		match self.shape
		{
			Shape::Cube{x,y,z}                                                => {},
			Shape::Sphere{r,face_number,face_angle,ref mut face_size}         => *face_size = Some(num),
			Shape::Cylinder{h,r1,r2,face_number,face_angle,ref mut face_size} => *face_size = Some(num),
			//Shape::Composite{ref op,ref mut c1,ref mut c2}                    => { c1.set_fs(num); c2.set_fs(num); },
			Shape::Composite{ref op,ref mut children}                         => { for child in children { child.set_fs(num) } },
		}
	}
	//}}}
	//{{{
	pub fn set_colour(&mut self, colour : crate::colour::Colour)
	{
		self.colour = colour.clone();

		if let Shape::Composite{ref op,ref mut children} = self.shape { for child in children { child.set_colour(colour.clone()) } };
	}
	//}}}
	

	//{{{
	pub fn set_debug(&mut self)
	{
		self.scad_modifier = crate::modifiers::ScadModifier::Debug;
		// TODO: Iterate over compound object tree and delete colour
	}
	//}}}
	//{{{
	pub fn set_background(&mut self)
	{
		self.scad_modifier = crate::modifiers::ScadModifier::Background;
		// TODO: Iterate over compound object tree and delete colour
	}
	//}}}
	//{{{
	pub fn set_root(&mut self)
	{
		self.scad_modifier = crate::modifiers::ScadModifier::Root;
	}
	//}}}
	//{{{
	pub fn set_disable(&mut self)
	{
		self.scad_modifier = crate::modifiers::ScadModifier::Disable;
	}
	//}}}

	//{{{
	pub fn set_show_origin(&mut self)
	{
		self.custom_modifier = crate::modifiers::CustomModifier::ShowOrigin;
	}
	//}}}
	//{{{
	pub fn set_show_anchors(&mut self)
	{
		self.custom_modifier = crate::modifiers::CustomModifier::ShowAnchors;
	}
	//}}}
	//}}}

	//{{{ 3D-Manipulation

	//{{{ Positions in a MultMatrix
	//
	// [ (0,0) (0,1) (0,2) (0,3) ]
	// [ (1,0) (1,1) (1,2) (1,3) ]
	// [ (2,0) (2,1) (2,2) (2,3) ]
	// [ (3,0) (3,1) (3,2) (3,3) ]
	//}}}

	//{{{
	pub fn rotate_x(&mut self, x: f64)
	{
		self.ref_sys.rotate_x(x);
	}
	//}}}
	//{{{
	pub fn rotate_y(&mut self, y: f64)
	{
		self.ref_sys.rotate_y(y);
	}
	//}}}
	//{{{
	pub fn rotate_z(&mut self, z: f64)
	{
		self.ref_sys.rotate_z(z);
	}
	//}}}
	//{{{
	pub fn rotate(&mut self, x: f64, y: f64, z: f64)
	{
		self.ref_sys.rotate(x, y, z);
	}
	//}}}

	//{{{
	pub fn rel_rotate_x(&mut self, x: f64)
	{
		self.ref_sys.rel_rotate_x(x);
	}
	//}}}
	//{{{
	pub fn rel_rotate_y(&mut self, y: f64)
	{
		self.ref_sys.rel_rotate_y(y);
	}
	//}}}
	//{{{
	pub fn rel_rotate_z(&mut self, z: f64)
	{
		self.ref_sys.rel_rotate_z(z);
	}
	//}}}
	//{{{
	pub fn rel_rotate(&mut self, x: f64, y: f64, z: f64)
	{
		self.ref_sys.rel_rotate(x, y, z);
	}
	//}}}


	//{{{
	pub fn translate_x(&mut self, x: f64)
	{
		self.ref_sys.translate_x(x);
	}
	//}}}
	//{{{
	pub fn translate_y(&mut self, y: f64)
	{
		self.ref_sys.translate_y(y);
	}
	//}}}
	//{{{
	pub fn translate_z(&mut self, z: f64)
	{
		self.ref_sys.translate_z(z);
	}
	//}}}
	//{{{
	pub fn translate(&mut self, x: f64, y:f64, z: f64)
	{
		self.ref_sys.translate(x, y, z);
	}
	//}}}

	//{{{
	pub fn rel_translate_x(&mut self, x: f64)
	{
		self.ref_sys.rel_translate_x(x);
	}
	//}}}
	//{{{
	pub fn rel_translate_y(&mut self, y: f64)
	{
		self.ref_sys.rel_translate_y(y);
	}
	//}}}
	//{{{
	pub fn rel_translate_z(&mut self, z: f64)
	{
		self.ref_sys.rel_translate_z(z);
	}
	//}}}
	//{{{
	pub fn rel_translate(&mut self, x: f64, y:f64, z: f64)
	{
		self.ref_sys.rel_translate(x, y, z);
	}
	//}}}


	//{{{
	pub fn scale_x(&mut self, x: f64)
	{
		self.ref_sys.scale_x(x);
	}
	//}}}
	//{{{
	pub fn scale_y(&mut self, y: f64)
	{
		self.ref_sys.scale_y(y);
	}
	//}}}
	//{{{
	pub fn scale_z(&mut self, z: f64)
	{
		self.ref_sys.scale_z(z);
	}
	//}}}
	//{{{
	pub fn scale(&mut self, x: f64, y:f64, z: f64)
	{
		self.ref_sys.scale(x, y, z);
	}
	//}}}
	//{{{
	pub fn rel_scale_x(&mut self, x: f64)
	{
		self.ref_sys.rel_scale_x(x);
	}
	//}}}
	//{{{
	pub fn rel_scale_y(&mut self, y: f64)
	{
		self.ref_sys.rel_scale_y(y);
	}
	//}}}
	//{{{
	pub fn rel_scale_z(&mut self, z: f64)
	{
		self.ref_sys.rel_scale_z(z);
	}
	//}}}
	//{{{
	pub fn rel_scale(&mut self, x: f64, y:f64, z: f64)
	{
		self.ref_sys.rel_scale(x, y, z);
	}
	//}}}


	//}}}

//
//	//{{{ Get 3D Coordinates, Rotation, Scale, Shear
//	
//	// See: https://math.stackexchange.com/questions/237369/given-this-transformation-matrix-how-do-i-decompose-it-into-translation-rotati
//	//{{{ Positions in a MultMatrix
//	//
//	// [ (0,0) (0,1) (0,2) (0,3) ]
//	// [ (1,0) (1,1) (1,2) (1,3) ]
//	// [ (2,0) (2,1) (2,2) (2,3) ]
//	// [ (3,0) (3,1) (3,2) (3,3) ]
//	//}}}
//
//
//	//{{{
//	pub fn get_rotate_x(&mut self) -> f64
//	{
//		0.0
//	}
//	//}}}
//	//{{{
//	pub fn get_rotate_y(&mut self) -> f64
//	{
//		0.0
//	}
//	//}}}
//	//{{{
//	pub fn get_rotate_z(&mut self) -> f64
//	{
//		0.0
//	}
//	//}}}
//	//{{{
//	pub fn get_rotate(&mut self) -> ( f64, f64, f64)
//	{
//		(0.0, 0.0, 0.0)
//	}
//	//}}}
//
//	//{{{
//	pub fn get_translate_x(&mut self) -> f64
//	{
//		self.ref_sys[[0,3]]
//	}
//	//}}}
//	//{{{
//	pub fn get_translate_y(&mut self) -> f64
//	{
//		self.ref_sys[[1,3]]
//	}
//	//}}}
//	//{{{
//	pub fn get_translate_z(&mut self) -> f64
//	{
//		self.ref_sys[[2,3]]
//	}
//	//}}}
//	//{{{
//	pub fn get_translate(&mut self) -> (f64, f64, f64)
//	{
//		(self.get_translate_x(), self.get_translate_y(), self.get_translate_z())
//	}
//	//}}}
//
//	//{{{
//	pub fn get_scale_x(&mut self) -> f64
//	{
//		use ndarray::s;
//		l2_norm(self.ref_sys.slice(s![0, ..]))
//	}
//	//}}}
//	//{{{
//	pub fn get_scale_y(&mut self) -> f64
//	{
//		use ndarray::s;
//		l2_norm(self.ref_sys.slice(s![1, ..]))
//	}
//	//}}}
//	//{{{
//	pub fn get_scale_z(&mut self) -> f64
//	{
//		use ndarray::s;
//		l2_norm(self.ref_sys.slice(s![2, ..]))
//	}
//	//}}}
//	//{{{
//	pub fn get_scale(&mut self) -> (f64, f64, f64)
//	{
//		(self.get_scale_x(), self.get_scale_y(), self.get_scale_z())
//	}
//	//}}}
//	//}}}
//

}
//}}}

//{{{ TODO
fn l2_norm(x: ndarray::ArrayView1<f64>) -> f64
{
	// Taken from:  https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/linear_algebra.html
	//x.dot(&x).sqrt()
	0.0
}
//}}}
//{{{
impl fmt::Display for Object
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		let indentation = if let Some(width) = f.width() { width } else { 0 as usize };

		let scad_mod = self.scad_modifier.to_string();
		// TODO: Add Anchors here
		let additional_stuff = if let crate::modifiers::CustomModifier::ShowOrigin = self.custom_modifier {object_origin().to_string()} else {String::from("")};
		//let additional_stuff = String::from("");

		write!(f, "{}",
			&self.colour.serialise(indentation,
				&self.ref_sys.serialise(indentation+1,
					&(	scad_mod + &self.shape.serialise(indentation+2) + &additional_stuff)
				)
			) 
		)
	}
}
//}}}
//}}}
//}}}

//{{{ Create Objects

//{{{
pub fn cube(x: f64, y: f64, z: f64) -> Object
{
	Object::new(Shape::Cube{ x: x,y: y,z: z })
}
//}}}
//{{{
pub fn sphere(r: f64) -> Object
{
	Object::new(Shape::Sphere{ r: r, face_number: None::<i32>, face_angle: None::<f64>, face_size: None::<f64> })
}
//}}}
//{{{
pub fn cylinder(h: f64, r1: f64, r2: f64) -> Object
{
	Object::new(Shape::Cylinder{ h: h, r1: r1, r2: r2, face_number: None::<i32>, face_angle: None::<f64>, face_size: None::<f64> })
}
//}}}

//{{{
pub fn union<T: AsRef<[Object]>>(children: T) -> Object
{
	Object::new(Shape::Composite{ op: BooleanOp::union, children: children.as_ref().to_vec() })
}
//}}}
//{{{
pub fn difference<T: AsRef<[Object]>>(children: T) -> Object
{
	Object::new(Shape::Composite{ op: BooleanOp::difference, children: children.as_ref().to_vec() })
}
//}}}
//{{{
pub fn intersection<T: AsRef<[Object]>>(children: T) -> Object
{
	Object::new(Shape::Composite{ op: BooleanOp::intersection, children: children.as_ref().to_vec() })
}
//}}}
//{{{
pub fn hull<T: AsRef<[Object]>>(children: T) -> Object
{
	Object::new(Shape::Composite{ op: BooleanOp::hull, children: children.as_ref().to_vec() })
}
//}}}
//{{{
pub fn minkowski<T: AsRef<[Object]>>(children: T) -> Object
{
	Object::new(Shape::Composite{ op: BooleanOp::minkowski, children: children.as_ref().to_vec() })
}
//}}}


//{{{
pub fn coordinate_system() -> Object
{
	//{{{
	let mut x1 = cylinder(1.0, 0.05, 0.05);
	let mut x2 = cylinder(0.1, 0.1,  0.0);
	x2.translate_z(1.0);
	let mut x = union([x1, x2]);
	x.rotate_y(90.0);
	x.set_colour(colour_named("red"));
	//}}}
	
	//{{{
	let mut y1 = cylinder(1.0, 0.05, 0.05);
	let mut y2 = cylinder(0.1, 0.1,  0.0);
	y2.translate_z(1.0);
	let mut y = union([y1, y2]);
	y.rotate_x(-90.0);
	y.set_colour(colour_named("green"));
	//}}}

	//{{{
	let mut z1 = cylinder(1.0, 0.05, 0.05);
	let mut z2 = cylinder(0.1, 0.1,  0.0);
	z2.translate_z(1.0);
	let mut z = union([z1, z2]);
	z.set_colour(colour_named("blue"));
	//}}}

	//let mut xyz = union([x, y, z]);

	let mut base = sphere(0.05);
	let mut coord_sys = union([x, y, z, base]);

	coord_sys.set_fn(10);

	coord_sys
}
//}}}
//{{{
pub fn object_origin() -> Object
{
	//{{{
	let mut x1 = cylinder(1.0, 0.05, 0.05);
	let mut x2 = cylinder(0.1, 0.1,  0.0);
	x2.translate_z(1.0);
	let mut x = union([x1, x2]);
	x.rotate_y(90.0);
	x.set_colour(colour_named("red"));
	//}}}
	
	//{{{
	let mut y1 = cylinder(1.0, 0.05, 0.05);
	let mut y2 = cylinder(0.1, 0.1,  0.0);
	y2.translate_z(1.0);
	let mut y = union([y1, y2]);
	y.rotate_x(-90.0);
	y.set_colour(colour_named("green"));
	//}}}

	//{{{
	let mut z1 = cylinder(1.0, 0.05, 0.05);
	let mut z2 = cylinder(0.1, 0.1,  0.0);
	z2.translate_z(1.0);
	let mut z = union([z1, z2]);
	z.set_colour(colour_named("blue"));
	//}}}

	//let mut xy = union(x, y);
	//let mut xyz = union(xy, z);

	let mut base = cube(0.5, 0.5, 0.5);
	base.translate(-0.25, -0.25, -0.25);
	base.set_colour(colour_named("red"));
	let mut coord_sys = union([x, y, z, base]);

	coord_sys.set_fn(10);
	coord_sys.scale(0.6, 0.6, 0.6);


	coord_sys
}
//}}}
//{{{
pub fn object_anchor() -> Object
{
	//{{{
	let mut x1 = cylinder(1.0, 0.05, 0.05);
	let mut x2 = cylinder(0.1, 0.1,  0.0);
	x2.translate_z(1.0);
	let mut x = union([x1, x2]);
	x.rotate_y(90.0);
	x.set_colour(colour_named("red"));
	//}}}
	
	//{{{
	let mut y1 = cylinder(1.0, 0.05, 0.05);
	let mut y2 = cylinder(0.1, 0.1,  0.0);
	y2.translate_z(1.0);
	let mut y = union([y1, y2]);
	y.rotate_x(-90.0);
	y.set_colour(colour_named("green"));
	//}}}

	//{{{
	let mut z1 = cylinder(1.0, 0.05, 0.05);
	let mut z2 = cylinder(0.1, 0.1,  0.0);
	z2.translate_z(1.0);
	let mut z = union([z1, z2]);
	z.set_colour(colour_named("blue"));
	//}}}

	//let mut xy = union(x, y);
	//let mut xyz = union(xy, z);

	let mut base = sphere(0.5);
	base.set_colour(colour_named("blue"));
	let mut coord_sys = union([x, y, z, base]);

	coord_sys.set_fn(10);
	coord_sys.scale(0.3, 0.3, 0.3);

	coord_sys
}
//}}}



//}}}




