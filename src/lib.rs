#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

extern crate vecmath;
pub mod math; // Use 'pub mod' if you want it to be visible outside library.

//pub use math::RefSysExt;
pub use math::*;
use std::collections::HashMap;
use std::ops::{Index,IndexMut,ShlAssign,Fn};








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
impl IsSerialisableScope for math::Matrix4D
{
	fn serialise(&self, indentation: usize, child : &str) -> String
	{
		let indent   = "\t".repeat(indentation as usize);
		//                              "multmatrix(m = [";
		let shift_in = indent.clone() + "                ";

		let mut retval = indent.clone();
		retval += "multmatrix(m = \n";
		retval += &format!("{:>indent$}\n{})\n", self, indent, indent=indentation+1);
		retval += &format!("{}{{\n", indent);
		retval += &child;
		retval += &format!("{} }};\n", indent);
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
	use std::fmt;
	//pub use crate::math::RefSysExt;
	//{{{ pub struct AnchorConstraint

	#[derive(Debug)]
	#[derive(Clone)]
	pub struct AnchorConstraint
	{
		pub x        : bool,
		pub y        : bool,
		pub z        : bool,
		pub relative : bool,
	}
	//}}}

	//{{{ pub struct Anchor

	#[derive(Debug, Clone)]
	pub struct Anchor
	{
		pub name                  : String,
		pub ref_sys               : crate::Matrix4D,
		pub constrain_rotation    : AnchorConstraint,
		pub constrain_translation : AnchorConstraint,
		pub constrain_scale       : AnchorConstraint,
		pub constrain_shear       : AnchorConstraint,
	}
	//}}}

	//{{{
	impl Anchor
	{
		//{{{
		pub fn new(name: &str) -> Self
		{
			Self
			{
				name                  : String::from(name),
				ref_sys               : crate::Matrix4D::identity(),
				constrain_rotation    : AnchorConstraint{x: false, y: false, z: false, relative: false},
				constrain_translation : AnchorConstraint{x: false, y: false, z: false, relative: false},
				constrain_scale       : AnchorConstraint{x: false, y: false, z: false, relative: false},
				constrain_shear       : AnchorConstraint{x: false, y: false, z: false, relative: false},
			}
		}
		//}}}
		//{{{
		fn get_object_anchor(&self) -> crate::Object
		{
			let mut object_anchor = crate::object_anchor(&self.name);
			object_anchor.set_ref_sys(self.ref_sys);
			object_anchor.rel_scale(0.3, 0.3, 0.3);
			object_anchor
		}
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
		pub fn set_ref_sys(&mut self, ref_sys: crate::Matrix4D)
		{
			self.ref_sys = ref_sys;
		}
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
	}
	//}}}

	//{{{
	impl fmt::Display for Anchor // TODO
	{
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
		{
			use crate::IsSerialisableScope;
			let indentation = if let Some(width) = f.width() { width } else { 0 as usize };

			write!(f, "{:>indent$}", &self.get_object_anchor(), indent=indentation+1)
		}
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
	pub name        : String,
	pub shape       : Shape,
	pub ref_sys     : crate::Matrix4D,
	pub colour      : crate::colour::Colour,
	pub anchors     : HashMap<String, anchors::Anchor>,
	scad_modifier   : crate::modifiers::ScadModifier,
	custom_modifier : crate::modifiers::CustomModifier,
	snap_parent     : bool,
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
	fn new(name: &str, shape : Shape) -> Self
	{
		Self{
			name            : String::from(name),
			shape           : shape,
			ref_sys         : crate::Matrix4D::identity(),
			colour          : crate::colour::Colour::Unset, 
			anchors         : HashMap::new(),
			scad_modifier   : crate::modifiers::ScadModifier::Unset, 
			custom_modifier : crate::modifiers::CustomModifier::Unset, 
			snap_parent     : false,
		}
	}
	//}}}

	//{{{
	pub fn add_anchor(&mut self, anchor: anchors::Anchor)
	{
		self.anchors.insert(String::from(&anchor.name), anchor);
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
	pub fn set_ref_sys(&mut self, ref_sys: crate::Matrix4D)
	{
		self.ref_sys = ref_sys;
	}
	//}}}
	//{{{
	pub fn snap_to_anchor(&mut self, own_anchor: &str, other: &Self, other_anchor: &crate::anchors::Anchor)
	{
		//TODO
		//self.ref_sys = !self[own_anchor].ref_sys * other_anchor.ref_sys * other.ref_sys;
	}
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


	//{{{ Get 3D Coordinates, Rotation, Scale, Shear
	
	// See: https://math.stackexchange.com/questions/237369/given-this-transformation-matrix-how-do-i-decompose-it-into-translation-rotati
	//{{{ Positions in a MultMatrix
	//
	// [ (0,0) (0,1) (0,2) (0,3) ]
	// [ (1,0) (1,1) (1,2) (1,3) ]
	// [ (2,0) (2,1) (2,2) (2,3) ]
	// [ (3,0) (3,1) (3,2) (3,3) ]
	//}}}


	//{{{
	pub fn get_rotate_x(&mut self) -> f64
	{
		self.ref_sys.get_rotate_x()
	}
	//}}}
	//{{{
	pub fn get_rotate_y(&mut self) -> f64
	{
		self.ref_sys.get_rotate_x()
	}
	//}}}
	//{{{
	pub fn get_rotate_z(&mut self) -> f64
	{
		self.ref_sys.get_rotate_x()
	}
	//}}}
	//{{{
	pub fn get_rotate(&mut self) -> ( f64, f64, f64)
	{
		self.ref_sys.get_rotate()
	}
	//}}}

	//{{{
	pub fn get_translate_x(&mut self) -> f64
	{
		self.ref_sys.get_translate_x()
	}
	//}}}
	//{{{
	pub fn get_translate_y(&mut self) -> f64
	{
		self.ref_sys.get_translate_y()
	}
	//}}}
	//{{{
	pub fn get_translate_z(&mut self) -> f64
	{
		self.ref_sys.get_translate_z()
	}
	//}}}
	//{{{
	pub fn get_translate(&mut self) -> (f64, f64, f64)
	{
		self.ref_sys.get_translate()
	}
	//}}}

	//{{{
	pub fn get_scale_x(&mut self) -> f64
	{
		self.ref_sys.get_scale_x()
	}
	//}}}
	//{{{
	pub fn get_scale_y(&mut self) -> f64
	{
		self.ref_sys.get_scale_y()
	}
	//}}}
	//{{{
	pub fn get_scale_z(&mut self) -> f64
	{
		self.ref_sys.get_scale_z()
	}
	//}}}
	//{{{
	pub fn get_scale(&mut self) -> (f64, f64, f64)
	{
		self.ref_sys.get_scale()
	}
	//}}}
	//}}}


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

		//{{{ Custom Modifiers

		let mut custom_mods = String::from("");

		if let crate::modifiers::CustomModifier::ShowAnchors = self.custom_modifier
		{
			for anchor in self.anchors.values()
			{
				custom_mods += &format!("{:>indent$}\n", anchor, indent=indentation+1);
			}
			custom_mods += &format!("{:>indent$}\n", object_origin(&(String::from("Object Origin for ")+&self.name)), indent=indentation+1);
		}
		if let crate::modifiers::CustomModifier::ShowOrigin = self.custom_modifier
		{
			custom_mods += &format!("{:>indent$}\n", object_origin(&(String::from("Object Origin for ")+&self.name)), indent=indentation+1);
		}
		//}}}

		write!(f, "\n{}//{}\n{}", "\t".repeat(indentation), &self.name,
			&self.colour.serialise(indentation,
				&self.ref_sys.serialise(indentation+1,
					&(	scad_mod + &self.shape.serialise(indentation+2) + &custom_mods)
				)
			) 
		)
	}
}
//}}}


//{{{ Index Operator for Object

struct ObjectIndexHelper<'a>
{
	pub anchor: &'a crate::anchors::Anchor,
	pub object: &'a Object,
}

//impl Index<&str> for Object {
//    type Output = crate::anchors::Anchor;
//
//    fn index(&self, index: &str) -> &Self::Output
//	{
//        eprintln!("Accessing {}-anchor of {}-object immutably", index, self.name);
//
//		&self.anchors[index]
//    }
//}




// Normal Index operator
//impl Index<&str> for Object {
//    type Output = crate::anchors::Anchor;
//
//    fn index(&self, index: &str) -> &Self::Output
//	{
//        eprintln!("Accessing {}-anchor of {}-object immutably", index, self.name);
//
//		&self.anchors[index]
//    }
//}

// Does not work because HashMap dows not support mutable indexing
//impl IndexMut<&str> for Object
//{
//    //type Output = crate::anchors::Anchor;
//
//    fn index_mut(&mut self, index: &str) -> &mut Self::Output
//	{
//        eprintln!("Accessing {}-anchor of {}-object immutably", index, self.name);
//
//		&mut self.anchors[index]
//    }
//}
//}}}

////{{{ The Snapping operator
//
//// parent["anchor_1"] <<= child["anchor_2"]; // Child's coordinates will be changed so that anchor_2
//// will reside exactly where anchor_1 is.
//impl ShlAssign<&mut Self> for Object
//{
//    fn shl_assign(&mut self, rhs: &mut Self)
//	{
//		// TODO
//    }
//}
////}}}

//{{{




//}}}





//}}}
//}}}

//{{{ Create Objects

//{{{
pub fn cube(name: &str, x: f64, y: f64, z: f64) -> Object
{
	Object::new(name, Shape::Cube{ x: x,y: y,z: z })
}
//}}}
//{{{
pub fn sphere(name: &str, r: f64) -> Object
{
	Object::new(name, Shape::Sphere{ r: r, face_number: None::<i32>, face_angle: None::<f64>, face_size: None::<f64> })
}
//}}}
//{{{
pub fn cylinder(name: &str, h: f64, r1: f64, r2: f64) -> Object
{
	Object::new(name, Shape::Cylinder{ h: h, r1: r1, r2: r2, face_number: None::<i32>, face_angle: None::<f64>, face_size: None::<f64> })
}
//}}}

//{{{
pub fn union<T: AsRef<[Object]>>(name: &str, children: T) -> Object
{
	Object::new(name, Shape::Composite{ op: BooleanOp::union, children: children.as_ref().to_vec() })
}
//}}}
//{{{
pub fn difference<T: AsRef<[Object]>>(name: &str, children: T) -> Object
{
	Object::new(name, Shape::Composite{ op: BooleanOp::difference, children: children.as_ref().to_vec() })
}
//}}}
//{{{
pub fn intersection<T: AsRef<[Object]>>(name: &str, children: T) -> Object
{
	Object::new(name, Shape::Composite{ op: BooleanOp::intersection, children: children.as_ref().to_vec() })
}
//}}}
//{{{
pub fn hull<T: AsRef<[Object]>>(name: &str, children: T) -> Object
{
	Object::new(name, Shape::Composite{ op: BooleanOp::hull, children: children.as_ref().to_vec() })
}
//}}}
//{{{
pub fn minkowski<T: AsRef<[Object]>>(name: &str, children: T) -> Object
{
	Object::new(name, Shape::Composite{ op: BooleanOp::minkowski, children: children.as_ref().to_vec() })
}
//}}}


//{{{
pub fn coordinate_system(name: &str) -> Object
{
	//{{{
	let mut x1 = cylinder("coordinate_system::x_shaft", 1.0, 0.05, 0.05);
	let mut x2 = cylinder("coordinate_system::x_arrow", 0.1, 0.1,  0.0);
	x2.translate_z(1.0);
	let mut x = union("coordinate_system::x_axis", [x1, x2]);
	x.rotate_y(90.0);
	x.set_colour(colour_named("red"));
	//}}}
	
	//{{{
	let mut y1 = cylinder("coordinate_system::y_shaft", 1.0, 0.05, 0.05);
	let mut y2 = cylinder("coordinate_system::y_arrow", 0.1, 0.1,  0.0);
	y2.translate_z(1.0);
	let mut y = union("coordinate_system::y_axis", [y1, y2]);
	y.rotate_x(-90.0);
	y.set_colour(colour_named("green"));
	//}}}

	//{{{
	let mut z1 = cylinder("coordinate_system::z_shaft", 1.0, 0.05, 0.05);
	let mut z2 = cylinder("coordinate_system::z_arrow", 0.1, 0.1,  0.0);
	z2.translate_z(1.0);
	let mut z = union("coordinate_system::z_axis", [z1, z2]);
	z.set_colour(colour_named("blue"));
	//}}}

	let mut base = sphere("coordinate_system::origin", 0.05);
	let mut coord_sys = union("coordinate_system", [x, y, z, base]);

	coord_sys.set_fn(10);

	coord_sys
}
//}}}
//{{{
pub fn object_origin(name: &str) -> Object
{
	//{{{
	let mut x1 = cylinder("object_origin::x_shaft", 1.0, 0.05, 0.05);
	let mut x2 = cylinder("object_origin::x_arrow", 0.1, 0.1,  0.0);
	x2.translate_z(1.0);
	let mut x = union("object_origin::x_axis", [x1, x2]);
	x.rotate_y(90.0);
	x.set_colour(colour_named("red"));
	//}}}
	
	//{{{
	let mut y1 = cylinder("object_origin::y_shaft", 1.0, 0.05, 0.05);
	let mut y2 = cylinder("object_origin::y_arrow", 0.1, 0.1,  0.0);
	y2.translate_z(1.0);
	let mut y = union("object_origin::y_axis", [y1, y2]);
	y.rotate_x(-90.0);
	y.set_colour(colour_named("green"));
	//}}}

	//{{{
	let mut z1 = cylinder("object_origin::z_shaft", 1.0, 0.05, 0.05);
	let mut z2 = cylinder("object_origin::z_arrow", 0.1, 0.1,  0.0);
	z2.translate_z(1.0);
	let mut z = union("object_origin::z_axis", [z1, z2]);
	z.set_colour(colour_named("blue"));
	//}}}

	let mut base = cube("object_origin::origin", 0.5, 0.5, 0.5);
	base.translate(-0.25, -0.25, -0.25);
	base.set_colour(colour_named("red"));
	let mut coord_sys = union("object_origin", [x, y, z, base]);

	coord_sys.set_fn(10);
	coord_sys.scale(0.6, 0.6, 0.6);


	coord_sys
}
//}}}
//{{{
pub fn object_anchor(name: &str) -> Object
{
	//{{{
	let mut x1 = cylinder("object_anchor::x_shaft", 1.0, 0.05, 0.05);
	let mut x2 = cylinder("object_anchor::x_arrow", 0.1, 0.1,  0.0);
	x2.translate_z(1.0);
	let mut x = union("object_anchor::x_axis", [x1, x2]);
	x.rotate_y(90.0);
	x.set_colour(colour_named("red"));
	//}}}
	
	//{{{
	let mut y1 = cylinder("object_anchor::x_shaft", 1.0, 0.05, 0.05);
	let mut y2 = cylinder("object_anchor::x_arrow", 0.1, 0.1,  0.0);
	y2.translate_z(1.0);
	let mut y = union("object_anchor::x_axis", [y1, y2]);
	y.rotate_x(-90.0);
	y.set_colour(colour_named("green"));
	//}}}

	//{{{
	let mut z1 = cylinder("object_anchor::x_shaft", 1.0, 0.05, 0.05);
	let mut z2 = cylinder("object_anchor::x_arrow", 0.1, 0.1,  0.0);
	z2.translate_z(1.0);
	let mut z = union("object_anchor::x_axis", [z1, z2]);
	z.set_colour(colour_named("blue"));
	//}}}

	//let mut xy = union(x, y);
	//let mut xyz = union(xy, z);

	let mut base = sphere("object_anchor::origin", 0.5);
	base.set_colour(colour_named("blue"));
	let mut coord_sys = union("object anchor", [x, y, z, base]);

	coord_sys.set_fn(10);
	coord_sys.scale(0.3, 0.3, 0.3);

	coord_sys
}
//}}}



//}}}




