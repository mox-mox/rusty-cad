#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

extern crate ndarray;
use std::fmt;

//{{{ helper_traits

//{{{
pub trait IsSerialisableScope
{
	fn serialise(&self, child : String) -> String;
}
//}}}

//}}}

//{{{
pub mod colour
{

	//{{{ Define Colour Object

	// Numeric representation of a colour
	type ColourVec  = ndarray::Array1<f64>;

	//{{{ pub enum Colour

	#[derive(Debug)]
	#[derive(Clone)] // we add the Clone trait to Morpheus struct
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
		Colour::Numeric(ColourVec::from_vec(vec![r,g,b,a]))
	}
	pub fn colour_rgb(r : f64, g : f64, b : f64) -> Colour
	{
		colour_rgba(r,g,b,1.0)
	}
	//}}}

	//{{{
	impl crate::IsSerialisableScope for Colour
	{
		fn serialise(&self, child : String) -> String
		{
			match &self
			{
				Self::Unset        => String::from("")                                      + &child,
				Self::Numeric(vec) => String::from("color( ") + &vec.to_string() + ")\n{\n" + &child + "\n};\n",
				Self::Named(name)  => String::from("color( \"") + &name            + "\")\n{\n" + &child + "\n};\n",
			}
		}
	}
	//}}}
	//}}}

}
//}}}

//{{{ TODO:
pub mod modifiers
{
	// modifiers: #: debug, %: background, !: root, *:disable
}
//}}}

//{{{ TODO:
pub mod anchors
{

}
//}}}

//{{{
pub mod refsys
{
	//{{{ Define RefSys

	pub type RefSys     = ndarray::Array2<f64>;

	//{{{
	impl crate::IsSerialisableScope for RefSys
	{
		fn serialise(&self, child : String) -> String
		{
			String::from("multmatrix(m = ") + &self.to_string() + ")\n{\n" + &child + "\n};\n"
		}
	}
	//}}}
	//}}}
}
//}}}

//// TODO: Cube
//// TODO: Sphere
//// TODO: Cylinder
// TODO: Polyhedron
// TODO: Composite ( Object x Object )
// TODO: text
// TODO: Measure::Length
// TODO: Measure::Angle

use crate::refsys::RefSys;
//use std::fmt;
//{{{ Define Object

//{{{ pub enum BooleanOp

#[derive(Debug)]
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
pub enum Shape
{
	Cube      { x: f64, y: f64, z: f64},
	Sphere    { r: f64, face_number: Option<i32>, face_angle: Option<f64>, face_size: Option<f64> },
	Cylinder  { h: f64, r1 : f64, r2 : f64, face_angle  : Option<f64>, face_size   : Option<f64>, face_number : Option<i32> },

	Composite { op: BooleanOp, c1: Box<Object>, c2: Box<Object> },
}

//{{{
impl Shape
{
	//{{{
	fn serialise(&self) -> String
	{
		match &self
		{
			//{{{
			Shape::Cube{x,y,z}                                        =>
			{
				String::from("cube([") + &x.to_string() + ", " + &y.to_string() + ", " + &z.to_string() + "]);"
			}
			//}}}
			//{{{
			Shape::Sphere{r,face_number,face_angle,face_size}         =>
			{
				let fan = if let Some(x) = face_number { String::from(", $fn=") + &x.to_string() } else { String::from("") };
				let faa = if let Some(x) = face_angle  { String::from(", $fa=") + &x.to_string() } else { String::from("") };
				let fas = if let Some(x) = face_size   { String::from(", $fs=") + &x.to_string() } else { String::from("") };

				String::from("sphere( r=") + &r.to_string() + &fan + &faa + &fas + ");"
			}
			//}}}
			//{{{
			Shape::Cylinder{h,r1,r2,face_number,face_angle,face_size} =>
			{
				let fan = if let Some(x) = face_number { String::from(", $fn=") + &x.to_string() } else { String::from("") };
				let faa = if let Some(x) = face_angle  { String::from(", $fa=") + &x.to_string() } else { String::from("") };
				let fas = if let Some(x) = face_size   { String::from(", $fs=") + &x.to_string() } else { String::from("") };

				String::from("sphere( h=") + &h.to_string() + ", r1=" + &r1.to_string() + ", r2=" + &r2.to_string() + &fan + &faa + &fas + ");"
			}
			//}}}
			//{{{
			Shape::Composite{op, c1, c2} =>
			{
				let mut name : String = format!("{:?}", self);
				//String::from(format!("{:?}", &op)) + "() {\n" + &(*c1.serialise_object()) + "\n" + &(*c2.serialise_object()) + "\n};"
				String::from(format!("{:?}", &op)) + "() {\n" + &(*c1.to_string()) + "\n" + &(*c2.to_string()) + "\n};"
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
pub struct Object
{
	pub shape    : Shape,
	pub ref_sys  : crate::refsys::RefSys,
	pub colour   : crate::colour::Colour,
}

impl Object
{
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
			Shape::Cube{x,y,z}                                                => unreachable!(),
			Shape::Sphere{r,ref mut face_number,face_angle,face_size}         => *face_number = Some(num),
			Shape::Cylinder{h,r1,r2,ref mut face_number,face_angle,face_size} => *face_number = Some(num),
			Shape::Composite{ref op,ref c1,ref c2}                                        => unreachable!(), // TODO
		}
	}
	//}}}
	//{{{
	pub fn set_fa(&mut self, num : f64)
	{
		match self.shape
		{
			Shape::Cube{x,y,z}                                                => unreachable!(),
			Shape::Sphere{r,face_number,ref mut face_angle,face_size}         => *face_angle = Some(num),
			Shape::Cylinder{h,r1,r2,face_number,ref mut face_angle,face_size} => *face_angle = Some(num),
			Shape::Composite{ref op,ref c1,ref c2}                                        => unreachable!(), // TODO
		}
	}
	//}}}
	//{{{
	pub fn set_fs(&mut self, num : f64)
	{
		match self.shape
		{
			Shape::Cube{x,y,z}                                                => unreachable!(),
			Shape::Sphere{r,face_number,face_angle,ref mut face_size}         => *face_size = Some(num),
			Shape::Cylinder{h,r1,r2,face_number,face_angle,ref mut face_size} => *face_size = Some(num),
			Shape::Composite{ref op,ref c1,ref c2}                                        => unreachable!(), // TODO
		}
	}
	//}}}
	//{{{
	pub fn set_colour(&mut self, colour : crate::colour::Colour)
	{
		self.colour = colour;
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
		let x = x.to_radians();
		let mut rotation=RefSys::eye(4);
		rotation[[1,1]] =  x.cos();
		rotation[[1,2]] = -x.sin();
		rotation[[2,1]] =  x.sin();
		rotation[[2,2]] =  x.cos();

		self.ref_sys = rotation.dot(&self.ref_sys);
	}
	//}}}
	//{{{
	pub fn rotate_y(&mut self, y: f64)
	{
		let y = y.to_radians();
		let mut rotation=RefSys::eye(4);
		rotation[[0,0]] =  y.cos();
		rotation[[0,2]] =  y.sin();
		rotation[[2,0]] = -y.sin();
		rotation[[2,2]] =  y.cos();

		self.ref_sys = rotation.dot(&self.ref_sys);
	}
	//}}}
	//{{{
	pub fn rotate_z(&mut self, z: f64)
	{
		let z = z.to_radians();
		let mut rotation=RefSys::eye(4);
		rotation[[0,0]] =  z.cos();
		rotation[[0,1]] = -z.sin();
		rotation[[1,0]] =  z.sin();
		rotation[[1,1]] =  z.cos();

		self.ref_sys = rotation.dot(&self.ref_sys);
	}
	//}}}
	//}}}
}



//{{{
impl fmt::Display for Object
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		//use crate::IsSerialisableScope;
		write!(f, "{}",
			self.colour.serialise(
				self.ref_sys.serialise(
					self.shape.serialise()
				)
			) 
		)
	}
}
//}}}



//}}}
//}}}

//{{{ Use Primitives

//{{{
pub fn cube(x: f64, y: f64, z: f64) -> Object
{
	Object{
		shape: Shape::Cube{ x: x,y: y,z: z },
		ref_sys: crate::refsys::RefSys::eye(4),
		colour : crate::colour::Colour::Unset
	}
}
//}}}
//{{{
pub fn sphere(r: f64) -> Object
{
	Object{
		shape: Shape::Sphere{ r: r, face_number: None::<i32>, face_angle: None::<f64>, face_size: None::<f64> },
		ref_sys: crate::refsys::RefSys::eye(4),
		colour : crate::colour::Colour::Unset
	}
}
//}}}
//{{{
pub fn cylinder(h: f64, r1: f64, r2: f64) -> Object
{
	Object{
		shape: Shape::Cylinder{ h: h, r1: r1, r2: r2, face_number: None::<i32>, face_angle: None::<f64>, face_size: None::<f64> },
		ref_sys: crate::refsys::RefSys::eye(4),
		colour : crate::colour::Colour::Unset
	}
}
//}}}
//{{{
pub fn union(c1: Object, c2: Object) -> Object
{
	let colour = c1.colour.clone();
	Object{
		shape: Shape::Composite{ op: BooleanOp::union, c1: Box::new(c1), c2: Box::new(c2) },
		ref_sys: crate::refsys::RefSys::eye(4),
		colour : colour,
	}
}
//}}}
//{{{
pub fn difference(c1: Object, c2: Object) -> Object
{
	let colour = c1.colour.clone();
	Object{
		shape: Shape::Composite{ op: BooleanOp::difference, c1: Box::new(c1), c2: Box::new(c2) },
		ref_sys: crate::refsys::RefSys::eye(4),
		colour : colour,
	}
}
//}}}
//{{{
pub fn intersection(c1: Object, c2: Object) -> Object
{
	let colour = c1.colour.clone();
	Object{
		shape: Shape::Composite{ op: BooleanOp::intersection, c1: Box::new(c1), c2: Box::new(c2) },
		ref_sys: crate::refsys::RefSys::eye(4),
		colour : colour,
	}
}
//}}}
//{{{
pub fn hull(c1: Object, c2: Object) -> Object
{
	let colour = c1.colour.clone();
	Object{
		shape: Shape::Composite{ op: BooleanOp::hull, c1: Box::new(c1), c2: Box::new(c2) },
		ref_sys: crate::refsys::RefSys::eye(4),
		colour : colour,
	}
}
//}}}
//{{{
pub fn minkowski(c1: Object, c2: Object) -> Object
{
	let colour = c1.colour.clone();
	Object{
		shape: Shape::Composite{ op: BooleanOp::minkowski, c1: Box::new(c1), c2: Box::new(c2) },
		ref_sys: crate::refsys::RefSys::eye(4),
		colour : colour,
	}
}
//}}}
//}}}
