#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

extern crate ndarray;
use std::fmt;
use crate::refsys::RefSys;
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
mod refsys
{

	pub type RefSys     = ndarray::Array2<f64>;

	//{{{
	impl crate::IsSerialisableScope for RefSys
	{
		fn serialise(&self, indentation: usize, child : &str) -> String
		{
			let indent   = "\t".repeat(indentation as usize);
			//                              "multmatrix(m = [";
			let shift_in = indent.clone() + "                ";

			let mut retval = indent.clone();
			retval += "multmatrix(m = [";

			//for i in 0..(self.nrows())
			for i in 0..(self.len_of(ndarray::Axis(1)))
			{
				retval += "\n";
				//retval += &("\t".repeat((indentation+1) as usize));
				retval += &shift_in.clone();
				retval += "[";

				//for j in 0..(self.ncols()-1)
				for j in 0..(self.len_of(ndarray::Axis(0))-1)
				{
					retval += &format!("{:16.10}, ", self[(i,j)]);
				}
				retval += &format!("{:16.10}],", self[(i,self.len_of(ndarray::Axis(0))-1)]);
			}
			retval += "])\n";
			retval += &("\t".repeat(indentation as usize) + "{\n");
			retval += &child;
			retval += "\n";
			retval += &("\t".repeat(indentation as usize) + "};\n");
			retval
		}
	}
	//}}}
}
//}}}

//{{{
mod colour
{
	use std::fmt;
	// Numeric representation of a colour
	type ColourVec  = ndarray::Array1<f64>;

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
		Colour::Numeric(ColourVec::from(vec![r,g,b,a]))
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
				Self::Unset        => String::from("")                                      + &child,
				Self::Numeric(vec) => tabs.clone() + "color( "   + &vec.to_string() +   ")\n" + &tabs + "{\n" + &child + "\n" + &tabs + "};\n",
				Self::Named(name)  => tabs.clone() + "color( \"" + &name            + "\")\n" + &tabs + "{\n" + &child + "\n" + &tabs + "};\n",
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
				Self::Numeric(vec) => write!(f, "{}", vec),
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

}
//}}}

// TODO: Polyhedron
// TODO: text
// TODO: Measure::Length
// TODO: Measure::Angle
// TODO: Measure::Triangle

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
			//{{{
			Shape::Composite{op, c1, c2} =>
			{
				//tabs + &format!("{:?}", &op) + "() {\n" + &(*c1.to_string()) + "\n" + &(*c2.to_string()) + "\n};"
				format!("{0}{1:?}()\n{0}{{\n{2:>indent$}\n{3:>indent$}\n{0} }};", tabs, &op, c1, c2, indent=indent)
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
	pub shape       : Shape,
	pub ref_sys     : crate::refsys::RefSys,
	pub colour      : crate::colour::Colour,
	scad_modifier   : crate::modifiers::ScadModifier,
	custom_modifier : crate::modifiers::CustomModifier,
}

//{{{
//fn l2_norm(x: ndarray::ArrayBase<ndarray::ViewRepr<&f64>, _>) -> f64
fn l2_norm(x: ndarray::ArrayView1<f64>) -> f64
{
	// Taken from:  https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/linear_algebra.html
	//x.dot(&x).sqrt()
	0.0
}
//}}}

//{{{
impl Object
{
	//{{{
	fn new(shape : Shape) -> Self
	{
		Self{
			shape           : shape,
			ref_sys         : crate::refsys::RefSys::eye(4),
			colour          : crate::colour::Colour::Unset, 
			scad_modifier   : crate::modifiers::ScadModifier::Unset, 
			custom_modifier : crate::modifiers::CustomModifier::Unset, 
		}
	}
	//}}}


	//{{{ Helpers
	
	// TODO: Iterator over compound object tree

	////{{{
	//pub fn invert(&self) -> Self
	//{
	//	// TODO
	//}
	////}}}


	#[inline]
	//{{{ TODO
	fn try_invert(&self) -> Option<crate::refsys::RefSys>
	{
		//use ndarray::arr2;
		//use num_traits::identities::Zero;
		let orig=&self.ref_sys;
		//{{{ Early termination
		if orig.ndim() != 2 || orig.dim().0 != orig.dim().1
		{
			eprintln!("Cannot invert a not two-dimensional or not quadratic matxi: {}", orig);
			return None::<crate::refsys::RefSys>
		}
		//}}}


		//{{{
		match orig.dim().0 // Equals orig.dim().1
		{
			0 => Some(self.ref_sys.clone()),
			//1 => Some(crate::refsys::RefSys::from(vec![1.0/orig[(0,0)]])),
			2 =>
			{
				let m11 = orig[(0, 0)];
				let m12 = orig[(0, 1)];
				let m21 = orig[(1, 0)];
				let m22 = orig[(1, 1)];

				let determinant = m11 * m22 - m21 * m12;

				//if determinant.is_zero()
				if determinant == 0.0
				{
					None::<crate::refsys::RefSys>
				}
				else
				{
					let m11 = m11 / determinant;
					let m12 = -m12 / determinant;
					let m21 = -m21 / determinant;
					let m22 = m22 / determinant;

					Some(ndarray::arr2(&[[ m11, m12], [m21, m22 ]]))
				}
			}
			//3 => {
			//	let m11 = *self.get_unchecked((0, 0));
			//	let m12 = *self.get_unchecked((0, 1));
			//	let m13 = *self.get_unchecked((0, 2));

			//	let m21 = *self.get_unchecked((1, 0));
			//	let m22 = *self.get_unchecked((1, 1));
			//	let m23 = *self.get_unchecked((1, 2));

			//	let m31 = *self.get_unchecked((2, 0));
			//	let m32 = *self.get_unchecked((2, 1));
			//	let m33 = *self.get_unchecked((2, 2));

			//	let minor_m12_m23 = m22 * m33 - m32 * m23;
			//	let minor_m11_m23 = m21 * m33 - m31 * m23;
			//	let minor_m11_m22 = m21 * m32 - m31 * m22;

			//	let determinant =
			//		m11 * minor_m12_m23 - m12 * minor_m11_m23 + m13 * minor_m11_m22;

			//	if determinant.is_zero() {
			//		false
			//	} else {
			//		*self.get_unchecked_mut((0, 0)) = minor_m12_m23 / determinant;
			//		*self.get_unchecked_mut((0, 1)) = (m13 * m32 - m33 * m12) / determinant;
			//		*self.get_unchecked_mut((0, 2)) = (m12 * m23 - m22 * m13) / determinant;

			//		*self.get_unchecked_mut((1, 0)) = -minor_m11_m23 / determinant;
			//		*self.get_unchecked_mut((1, 1)) = (m11 * m33 - m31 * m13) / determinant;
			//		*self.get_unchecked_mut((1, 2)) = (m13 * m21 - m23 * m11) / determinant;

			//		*self.get_unchecked_mut((2, 0)) = minor_m11_m22 / determinant;
			//		*self.get_unchecked_mut((2, 1)) = (m12 * m31 - m32 * m11) / determinant;
			//		*self.get_unchecked_mut((2, 2)) = (m11 * m22 - m21 * m12) / determinant;

			//		true
			//	}
			//}
			//4 => {
			//	let oself = self.clone_owned();
			//	do_inverse4(&oself, self)
			//}
			_ => {
				Some(self.ref_sys.clone())
				//let oself = self.clone_owned();
				//lu::try_invert_to(oself, self)
			}
		}
		//}}}
	}
	//}}}

	//{{{ Commented
	//
	///// Attempts to invert this matrix in-place. Returns `false` and leaves `self` untouched if
	///// inversion fails.
	//pub fn try_inverse_mut(&mut self) -> bool
	//	where DefaultAllocator: Allocator<N, D, D> {
	//assert!(self.is_square(), "Unable to invert a non-square matrix.");

	//let dim = self.shape().0;

	//unsafe {
	//	match dim {
	//		0 => true,
	//		1 => {
	//			let determinant = self.get_unchecked((0, 0)).clone();
	//			if determinant.is_zero() {
	//				false
	//			} else {
	//				*self.get_unchecked_mut((0, 0)) = N::one() / determinant;
	//				true
	//			}
	//		}
	//		2 => {
	//			let m11 = *self.get_unchecked((0, 0));
	//			let m12 = *self.get_unchecked((0, 1));
	//			let m21 = *self.get_unchecked((1, 0));
	//			let m22 = *self.get_unchecked((1, 1));

	//			let determinant = m11 * m22 - m21 * m12;

	//			if determinant.is_zero() {
	//				false
	//			} else {
	//				*self.get_unchecked_mut((0, 0)) = m22 / determinant;
	//				*self.get_unchecked_mut((0, 1)) = -m12 / determinant;

	//				*self.get_unchecked_mut((1, 0)) = -m21 / determinant;
	//				*self.get_unchecked_mut((1, 1)) = m11 / determinant;

	//				true
	//			}
	//		}
	//		3 => {
	//			let m11 = *self.get_unchecked((0, 0));
	//			let m12 = *self.get_unchecked((0, 1));
	//			let m13 = *self.get_unchecked((0, 2));

	//			let m21 = *self.get_unchecked((1, 0));
	//			let m22 = *self.get_unchecked((1, 1));
	//			let m23 = *self.get_unchecked((1, 2));

	//			let m31 = *self.get_unchecked((2, 0));
	//			let m32 = *self.get_unchecked((2, 1));
	//			let m33 = *self.get_unchecked((2, 2));

	//			let minor_m12_m23 = m22 * m33 - m32 * m23;
	//			let minor_m11_m23 = m21 * m33 - m31 * m23;
	//			let minor_m11_m22 = m21 * m32 - m31 * m22;

	//			let determinant =
	//				m11 * minor_m12_m23 - m12 * minor_m11_m23 + m13 * minor_m11_m22;

	//			if determinant.is_zero() {
	//				false
	//			} else {
	//				*self.get_unchecked_mut((0, 0)) = minor_m12_m23 / determinant;
	//				*self.get_unchecked_mut((0, 1)) = (m13 * m32 - m33 * m12) / determinant;
	//				*self.get_unchecked_mut((0, 2)) = (m12 * m23 - m22 * m13) / determinant;

	//				*self.get_unchecked_mut((1, 0)) = -minor_m11_m23 / determinant;
	//				*self.get_unchecked_mut((1, 1)) = (m11 * m33 - m31 * m13) / determinant;
	//				*self.get_unchecked_mut((1, 2)) = (m13 * m21 - m23 * m11) / determinant;

	//				*self.get_unchecked_mut((2, 0)) = minor_m11_m22 / determinant;
	//				*self.get_unchecked_mut((2, 1)) = (m12 * m31 - m32 * m11) / determinant;
	//				*self.get_unchecked_mut((2, 2)) = (m11 * m22 - m21 * m12) / determinant;

	//				true
	//			}
	//		}
	//		4 => {
	//			let oself = self.clone_owned();
	//			do_inverse4(&oself, self)
	//		}
	//		_ => {
	//			let oself = self.clone_owned();
	//			lu::try_invert_to(oself, self)
	//		}
	//	}
	//}
//}}}

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
			Shape::Composite{ref op,ref mut c1,ref mut c2}                    => { c1.set_fn(num); c2.set_fn(num); },
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
			Shape::Composite{ref op,ref mut c1,ref mut c2}                    => { c1.set_fa(num); c2.set_fa(num); },
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
			Shape::Composite{ref op,ref mut c1,ref mut c2}                    => { c1.set_fs(num); c2.set_fs(num); },
		}
	}
	//}}}
	//{{{
	pub fn set_colour(&mut self, colour : crate::colour::Colour)
	{
		self.colour = colour;
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
	//{{{
	pub fn rotate(&mut self, x: f64, y: f64, z: f64)
	{
		self.rotate_x(x);
		self.rotate_y(y);
		self.rotate_z(z);
	}
	//}}}

	//{{{
	pub fn rel_rotate_x(&mut self, x: f64)
	{
		let x = x.to_radians();
		let mut rotation=RefSys::eye(4);
		rotation[[1,1]] =  x.cos();
		rotation[[1,2]] = -x.sin();
		rotation[[2,1]] =  x.sin();
		rotation[[2,2]] =  x.cos();

		self.ref_sys = self.ref_sys.dot(&rotation);
	}
	//}}}
	//{{{
	pub fn rel_rotate_y(&mut self, y: f64)
	{
		let y = y.to_radians();
		let mut rotation=RefSys::eye(4);
		rotation[[0,0]] =  y.cos();
		rotation[[0,2]] =  y.sin();
		rotation[[2,0]] = -y.sin();
		rotation[[2,2]] =  y.cos();

		self.ref_sys = self.ref_sys.dot(&rotation);
	}
	//}}}
	//{{{
	pub fn rel_rotate_z(&mut self, z: f64)
	{
		let z = z.to_radians();
		let mut rotation=RefSys::eye(4);
		rotation[[0,0]] =  z.cos();
		rotation[[0,1]] = -z.sin();
		rotation[[1,0]] =  z.sin();
		rotation[[1,1]] =  z.cos();

		self.ref_sys = self.ref_sys.dot(&rotation);
	}
	//}}}
	//{{{
	pub fn rel_rotate(&mut self, x: f64, y: f64, z: f64)
	{
		self.rel_rotate_x(x);
		self.rel_rotate_y(y);
		self.rel_rotate_z(z);
	}
	//}}}


	//{{{
	pub fn translate_x(&mut self, x: f64)
	{
		let mut translation=RefSys::eye(4);
		translation[[0,3]] =  x;

		self.ref_sys = translation.dot(&self.ref_sys);
	}
	//}}}
	//{{{
	pub fn translate_y(&mut self, y: f64)
	{
		let mut translation=RefSys::eye(4);
		translation[[1,3]] =  y;

		self.ref_sys = translation.dot(&self.ref_sys);
	}
	//}}}
	//{{{
	pub fn translate_z(&mut self, z: f64)
	{
		let mut translation=RefSys::eye(4);
		translation[[2,3]] =  z;

		self.ref_sys = translation.dot(&self.ref_sys);
	}
	//}}}
	//{{{
	pub fn translate(&mut self, x: f64, y:f64, z: f64)
	{
		let mut translation=RefSys::eye(4);
		translation[[0,3]] =  x;
		translation[[1,3]] =  y;
		translation[[2,3]] =  z;

		self.ref_sys = translation.dot(&self.ref_sys);
	}
	//}}}

	//{{{
	pub fn rel_translate_x(&mut self, x: f64)
	{
		let mut translation=RefSys::eye(4);
		translation[[0,3]] =  x;

		//self.ref_sys = translation.dot(&self.ref_sys);
		self.ref_sys = self.ref_sys.dot(&translation);
	}
	//}}}
	//{{{
	pub fn rel_translate_y(&mut self, y: f64)
	{
		let mut translation=RefSys::eye(4);
		translation[[1,3]] =  y;

		//self.ref_sys = translation.dot(&self.ref_sys);
		self.ref_sys = self.ref_sys.dot(&translation);
	}
	//}}}
	//{{{
	pub fn rel_translate_z(&mut self, z: f64)
	{
		let mut translation=RefSys::eye(4);
		translation[[2,3]] =  z;

		//self.ref_sys = translation.dot(&self.ref_sys);
		self.ref_sys = self.ref_sys.dot(&translation);
	}
	//}}}
	//{{{
	pub fn rel_translate(&mut self, x: f64, y:f64, z: f64)
	{
		let mut translation=RefSys::eye(4);
		translation[[0,3]] =  x;
		translation[[1,3]] =  y;
		translation[[2,3]] =  z;

		//self.ref_sys = translation.dot(&self.ref_sys);
		self.ref_sys = self.ref_sys.dot(&translation);
	}
	//}}}


	//{{{
	pub fn scale_x(&mut self, x: f64)
	{
		let mut scale=RefSys::eye(4);
		scale[[0,0]] =  x;

		self.ref_sys = scale.dot(&self.ref_sys);
	}
	//}}}
	//{{{
	pub fn scale_y(&mut self, y: f64)
	{
		let mut scale=RefSys::eye(4);
		scale[[1,1]] =  y;

		self.ref_sys = scale.dot(&self.ref_sys);
	}
	//}}}
	//{{{
	pub fn scale_z(&mut self, z: f64)
	{
		let mut scale=RefSys::eye(4);
		scale[[3,3]] =  z;

		self.ref_sys = scale.dot(&self.ref_sys);
	}
	//}}}
	//{{{
	pub fn scale(&mut self, x: f64, y:f64, z: f64)
	{
		let mut scale=RefSys::eye(4);
		scale[[0,0]] =  x;
		scale[[1,1]] =  y;
		scale[[2,2]] =  z;

		self.ref_sys = scale.dot(&self.ref_sys);
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

//{{{
impl fmt::Display for Object
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		let indentation = if let Some(width) = f.width() { width } else { 0 as usize };

		// TODO: Add Anchors here
		let additional_stuff = if let crate::modifiers::CustomModifier::ShowOrigin = self.custom_modifier {object_origin().to_string()} else {String::from("")};

		write!(f, "{}{}",&self.scad_modifier,
			&self.colour.serialise(indentation,
				&self.ref_sys.serialise(indentation+1,
					&(	self.shape.serialise(indentation+2) + &additional_stuff)
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
pub fn union(c1: Object, c2: Object) -> Object
{
	Object::new(Shape::Composite{ op: BooleanOp::union, c1: Box::new(c1), c2: Box::new(c2) })
}
//}}}
//{{{
pub fn difference(c1: Object, c2: Object) -> Object
{
	Object::new(Shape::Composite{ op: BooleanOp::difference, c1: Box::new(c1), c2: Box::new(c2) })
}
//}}}
//{{{
pub fn intersection(c1: Object, c2: Object) -> Object
{
	Object::new(Shape::Composite{ op: BooleanOp::intersection, c1: Box::new(c1), c2: Box::new(c2) })
}
//}}}
//{{{
pub fn hull(c1: Object, c2: Object) -> Object
{
	Object::new(Shape::Composite{ op: BooleanOp::hull, c1: Box::new(c1), c2: Box::new(c2) })
}
//}}}
//{{{
pub fn minkowski(c1: Object, c2: Object) -> Object
{
	Object::new(Shape::Composite{ op: BooleanOp::minkowski, c1: Box::new(c1), c2: Box::new(c2) })
}
//}}}

//{{{
pub fn coordinate_system() -> Object
{
	//{{{
	let mut x1 = cylinder(1.0, 0.05, 0.05);
	let mut x2 = cylinder(0.1, 0.1,  0.0);
	x2.translate_z(1.0);
	let mut x = union(x1, x2);
	x.rotate_y(90.0);
	x.set_colour(colour_named("red"));
	//}}}
	
	//{{{
	let mut y1 = cylinder(1.0, 0.05, 0.05);
	let mut y2 = cylinder(0.1, 0.1,  0.0);
	y2.translate_z(1.0);
	let mut y = union(y1, y2);
	y.rotate_x(-90.0);
	y.set_colour(colour_named("green"));
	//}}}

	//{{{
	let mut z1 = cylinder(1.0, 0.05, 0.05);
	let mut z2 = cylinder(0.1, 0.1,  0.0);
	z2.translate_z(1.0);
	let mut z = union(z1, z2);
	z.set_colour(colour_named("blue"));
	//}}}

	let mut xy = union(x, y);
	let mut xyz = union(xy, z);

	let mut base = sphere(0.05);
	let mut coord_sys = union(xyz, base);

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
	let mut x = union(x1, x2);
	x.rotate_y(90.0);
	x.set_colour(colour_named("red"));
	//}}}
	
	//{{{
	let mut y1 = cylinder(1.0, 0.05, 0.05);
	let mut y2 = cylinder(0.1, 0.1,  0.0);
	y2.translate_z(1.0);
	let mut y = union(y1, y2);
	y.rotate_x(-90.0);
	y.set_colour(colour_named("green"));
	//}}}

	//{{{
	let mut z1 = cylinder(1.0, 0.05, 0.05);
	let mut z2 = cylinder(0.1, 0.1,  0.0);
	z2.translate_z(1.0);
	let mut z = union(z1, z2);
	z.set_colour(colour_named("blue"));
	//}}}

	let mut xy = union(x, y);
	let mut xyz = union(xy, z);

	let mut base = cube(0.5, 0.5, 0.5);
	base.translate(-0.25, -0.25, -0.25);
	base.set_colour(colour_named("red"));
	let mut coord_sys = union(xyz, base);

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
	let mut x = union(x1, x2);
	x.rotate_y(90.0);
	x.set_colour(colour_named("red"));
	//}}}
	
	//{{{
	let mut y1 = cylinder(1.0, 0.05, 0.05);
	let mut y2 = cylinder(0.1, 0.1,  0.0);
	y2.translate_z(1.0);
	let mut y = union(y1, y2);
	y.rotate_x(-90.0);
	y.set_colour(colour_named("green"));
	//}}}

	//{{{
	let mut z1 = cylinder(1.0, 0.05, 0.05);
	let mut z2 = cylinder(0.1, 0.1,  0.0);
	z2.translate_z(1.0);
	let mut z = union(z1, z2);
	z.set_colour(colour_named("blue"));
	//}}}

	let mut xy = union(x, y);
	let mut xyz = union(xy, z);

	let mut base = sphere(0.5);
	base.set_colour(colour_named("blue"));
	let mut coord_sys = union(xyz, base);

	coord_sys.set_fn(10);
	coord_sys.scale(0.3, 0.3, 0.3);

	coord_sys
}
//}}}
//}}}
