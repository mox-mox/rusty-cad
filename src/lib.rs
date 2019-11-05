#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

/// Idea:
/// There are a number of functions (see last section "create objects") to create 3D objects.
/// They are all of type 'Object' and you can call methods on them to translate, rotate, scale, etc.
/// For CSG, you use the union(), difference(), intersection(), hull() and minkowski() functions which take an array of Objects and return an Object.
/// 
/// For nicer placement, you can define 'Anchors' on Objects. They work by defining a point and direction relative to the origin of an Object and allow snapping objects together (bit of matrix math).

/// TODO: Much more documentation...


//mod object_2d;
//pub use crate::object_2d::*;
mod object_3d;
pub use crate::object_3d::*;

//extern crate vecmath;:
pub mod math; // Use 'pub mod' if you want it to be visible outside library.

pub use math::*;
use std::collections::HashMap;
use std::ops::{Index,IndexMut,ShlAssign,Fn,BitAnd};

extern crate ndarray;
use std::fmt;

//{{{ helper_traits

//{{{
trait IsSerialisableScope
{
	fn serialise(&self, indentation: usize, child : &str) -> String;
}
//}}}

//{{{
trait IsObject
{
	type Shape;
	type Matrix;

	fn new(name: &str, shape : Self::Shape) -> Self;
}
//}}}

//}}}


//{{{
//mod colour
//{
	//use std::fmt;
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
//}
//}}}

//{{{
//mod modifiers
//{
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
//}
//}}}




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






////{{{
//pub fn union<Object: IsObject>(c1: Object, c2: Object) -> Object
//{
//	Object::new(name, Shape::Composite{ op: BooleanOp::union, children: children.as_ref().to_vec() })
//	//Object{
//	//	shape: Shape::Composite{ op: BooleanOp::union, c1: Box::new(c1), c2: Box::new(c2) },
//	//	ref_sys: crate::refsys::RefSys::eye(4),
//	//	colour : crate::colour::Colour::Unset,
//	//}
//}
////}}}


////{{{
//pub fn union_generic<T, R>(name: &str, children: AsRef<[T]>) -> Object where T: IsObject, 
////pub fn union_generic<T: AsRef<[Object]>>(name: &str, children: T) -> Object where Object: IsObject
//{
//	T::new(name, Shape::Composite{ op: BooleanOp::union, children: children.as_ref().to_vec() })
//}
////}}}


//
////{{{
//pub fn difference<T: AsRef<[Object2D]>>(name: &str, children: T) -> Object2D
//{
//	Object2D::new(name, Shape::Composite{ op: BooleanOp::difference, children: children.as_ref().to_vec() })
//}
////}}}
////{{{
//pub fn intersection<T: AsRef<[Object2D]>>(name: &str, children: T) -> Object2D
//{
//	Object2D::new(name, Shape::Composite{ op: BooleanOp::intersection, children: children.as_ref().to_vec() })
//}
////}}}
////{{{
//pub fn hull<T: AsRef<[Object2D]>>(name: &str, children: T) -> Object2D
//{
//	Object2D::new(name, Shape::Composite{ op: BooleanOp::hull, children: children.as_ref().to_vec() })
//}
////}}}
////{{{
//pub fn minkowski<T: AsRef<[Object2D]>>(name: &str, children: T) -> Object2D
//{
//	Object2D::new(name, Shape::Composite{ op: BooleanOp::minkowski, children: children.as_ref().to_vec() })
//}
////}}}
//
