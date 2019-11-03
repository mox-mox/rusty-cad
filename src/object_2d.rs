use std::fmt;
use std::collections::HashMap;

use crate::math::{Is2DObject, HasRefSys2D, Matrix2D};
use crate::IsSerialisableScope;

use std::ops::Index;
use crate::Colour;

//{{{
pub mod anchors
{
	use std::fmt;
	use std::ops::{BitAnd, BitOr};
	use crate::math::HasRefSys2D;
	use crate::math::Is2DObject;
	//pub use crate::math::RefSysExt;
	//{{{ pub struct AnchorConstraint

	#[derive(Debug, Clone, Copy)]
	pub struct AnchorConstraint
	{
		pub x        : bool,
		pub y        : bool,
		pub z        : bool,
		pub relative : bool,
	}

	impl BitAnd for AnchorConstraint
	{
		type Output = Self;

		fn bitand(self, other: Self) -> Self::Output
		{
			AnchorConstraint{x: self.x & other.x, y: self.y & other.y, z: self.z & other.z, relative: self.relative & other.relative}
		}
	}
	impl BitOr for AnchorConstraint
	{
		type Output = Self;

		fn bitor(self, other: Self) -> Self::Output
		{
			AnchorConstraint{x: self.x | other.x, y: self.y | other.y, z: self.z | other.z, relative: self.relative | other.relative}
		}
	}
	//}}}

	//{{{ pub struct Anchor

	#[derive(Debug, Clone)]
	pub struct Anchor
	{
		pub name                  : String,
		pub ref_sys               : crate::Matrix2D,
		pub constrain_rotation    : AnchorConstraint,
		pub constrain_translation : AnchorConstraint,
		pub constrain_scale       : AnchorConstraint,
		pub constrain_shear       : AnchorConstraint,
	}
	//}}}

	//{{{
	impl HasRefSys2D for Anchor
	{
	    fn ref_sys_mut(&mut self) -> &mut crate::Matrix2D
		{
			&mut self.ref_sys
	    }
	    fn ref_sys(&self) -> &crate::Matrix2D
		{
			&self.ref_sys
	    }
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
				ref_sys               : crate::Matrix2D::identity(),
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
		//{{{ 2D-Manipulation

		//{{{ Positions in a MultMatrix
		//
		// [ (0,0) (0,1) (0,2) (0,3) ]
		// [ (1,0) (1,1) (1,2) (1,3) ]
		// [ (2,0) (2,1) (2,2) (2,3) ]
		// [ (3,0) (3,1) (3,2) (3,3) ]
		//}}}

		//{{{
		pub fn set_ref_sys(&mut self, ref_sys: crate::Matrix2D)
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
	Text      { text: String, font: String, size: i32, spacing: f64 },

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
				tabs + "cube([" + &x.to_string() + ", " + &y.to_string() + ", " + &z.to_string() + "], center=true);"
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
			Shape::Text{ref text, ref font, size, spacing} =>
			{
				format!("{0}text(\"{1}\", font=\"{2}\", spacing={3}, size={4});", tabs, &text, &font, spacing, size)
			}
			//}}}
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
	pub ref_sys     : crate::Matrix2D,
	pub colour      : Colour,
	pub anchors     : HashMap<String, anchors::Anchor>,
	scad_modifier   : crate::ScadModifier,
	custom_modifier : crate::CustomModifier,
	snap_parent     : bool,
}

//{{{
impl HasRefSys2D for Object
{
    fn ref_sys_mut(&mut self) -> &mut Matrix2D
	{
		&mut self.ref_sys
    }
    fn ref_sys(&self) -> &Matrix2D
	{
		&self.ref_sys
    }
}
//}}}

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
			ref_sys         : crate::Matrix2D::identity(),
			colour          : Colour::Unset, 
			anchors         : HashMap::new(),
			scad_modifier   : crate::ScadModifier::Unset, 
			custom_modifier : crate::CustomModifier::Unset, 
			snap_parent     : false,
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
			Shape::Text{ref text, ref font, size, spacing}                    => {},
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
			Shape::Text{ref text, ref font, size, spacing}                    => {},
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
			Shape::Text{ref text, ref font, size, spacing}                    => {},
			Shape::Composite{ref op,ref mut children}                         => { for child in children { child.set_fs(num) } },
		}
	}
	//}}}
	//{{{
	pub fn set_colour(&mut self, colour : Colour)
	{
		self.colour = colour.clone();

		if let Shape::Composite{ref op,ref mut children} = self.shape { for child in children { child.set_colour(colour.clone()) } };
	}
	//}}}
	

	//{{{
	pub fn set_debug(&mut self)
	{
		self.scad_modifier = crate::ScadModifier::Debug;
		// TODO: Iterate over compound object tree and delete colour
	}
	//}}}
	//{{{
	pub fn set_background(&mut self)
	{
		self.scad_modifier = crate::ScadModifier::Background;
		// TODO: Iterate over compound object tree and delete colour
	}
	//}}}
	//{{{
	pub fn set_root(&mut self)
	{
		self.scad_modifier = crate::ScadModifier::Root;
	}
	//}}}
	//{{{
	pub fn set_disable(&mut self)
	{
		self.scad_modifier = crate::ScadModifier::Disable;
	}
	//}}}

	//{{{
	pub fn set_show_origin(&mut self)
	{
		self.custom_modifier = crate::CustomModifier::ShowOrigin;
	}
	//}}}
	//{{{
	pub fn set_show_anchors(&mut self)
	{
		self.custom_modifier = crate::CustomModifier::ShowAnchors;
	}
	//}}}
	//}}}

	//{{{
	pub fn set_ref_sys(&mut self, ref_sys: crate::Matrix2D)
	{
		self.ref_sys = ref_sys;
	}
	//}}}


	//{{{
	pub fn add_anchor(&mut self, anchor: anchors::Anchor)
	{
		self.anchors.insert(String::from(&anchor.name), anchor);
	}
	//}}}
	//{{{
	pub fn anchor<'a>(&'a mut self, index: &'a str) -> ObjectIndexHelper<'a>
	{
		//ObjectIndexHelper{ anchor: &self.anchors[index], object: self }
		ObjectIndexHelper{ anchor_name: index, object: self }
	}
	//}}}
	////{{{
	//pub fn snap_to_anchor(&mut self, own_anchor: &str, other: &Self, other_anchor: &crate::anchors::Anchor)
	//{

	//	//{{{ Generate inverse own anchor

	//	//}}}
	//	//{{{ Generate other anchor

	//	//}}}

	//	self.ref_sys = !self[own_anchor].ref_sys * other_anchor.ref_sys * other.ref_sys;
	//}
	////}}}
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

		if let crate::CustomModifier::ShowAnchors = self.custom_modifier
		{
			for anchor in self.anchors.values()
			{
				custom_mods += &format!("{:>indent$}\n", anchor, indent=indentation+1);
			}
			custom_mods += &format!("{:>indent$}\n", object_origin(&(String::from("Object Origin for ")+&self.name)), indent=indentation+1);
		}
		if let crate::CustomModifier::ShowOrigin = self.custom_modifier
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



//{{{ Snapping

pub struct ObjectIndexHelper<'a>
{
	//pub anchor: &'a crate::anchors::Anchor,
	pub anchor_name: &'a str,
	pub object:      &'a mut Object,
}

impl ObjectIndexHelper<'_>
{
	//{{{
	pub fn snap_to(&mut self, other: &mut Self)
	{
		// Usage: child_object.anchor("anchor2").snap_to(parent_object.anchor("anchor1"));
		let child_anchor = self.object.anchors[self.anchor_name].clone();
		let child_object = &mut self.object;

		let parent_anchor  = &other.object.anchors[other.anchor_name];
		let parent_object  = &other.object;

		//let child_anchor : &mut anchors::Anchor = &mut other.anchor;
		//let child_object : &mut Object = &mut other.object;

		//let constrain_rotation     = parent_anchor.constrain_rotation     & child_anchor.constrain_rotation;
		//let constrain_translation  = parent_anchor.constrain_translation  & child_anchor.constrain_translation;
		//let constrain_scale        = parent_anchor.constrain_scale        & child_anchor.constrain_scale;
		//let constrain_shear        = parent_anchor.constrain_shear        & child_anchor.constrain_shear;

		////{{{ Generate inverse own anchor

		//let mut own_anchor=Matrix2D::identity();
		//if constrain_rotation.x || constrain_rotation.y || constrain_rotation.z
		//{
		//	own_anchor.rotate(
		//		-child_anchor.get_rotate_x()*((constrain_rotation.x as i32) as f64),
		//		-child_anchor.get_rotate_y()*((constrain_rotation.y as i32) as f64),
		//		-child_anchor.get_rotate_z()*((constrain_rotation.z as i32) as f64));
		//}

		//if constrain_translation.x || constrain_translation.y || constrain_translation.z
		//{
		//	own_anchor.translate(
		//		-child_anchor.get_translate_x()*((constrain_translation.x as i32) as f64),
		//		-child_anchor.get_translate_y()*((constrain_translation.y as i32) as f64),
		//		-child_anchor.get_translate_z()*((constrain_translation.z as i32) as f64));
		//}
		////}}}
		////{{{ Generate other anchor

		////}}}


		let new_origin = !child_anchor.ref_sys * parent_anchor.ref_sys * parent_object.ref_sys;
		eprintln!("new_origin = {}", new_origin);

		child_object.ref_sys = !child_anchor.ref_sys * parent_anchor.ref_sys * parent_object.ref_sys;
		//child_object.set_ref_sys(!child_anchor.ref_sys * parent_anchor.ref_sys * parent_object.ref_sys);
	}
	//}}}

}


//}}}








//{{{ Index Operator for Object



//{{{ Index -> (Object, Anchor) Does not work // <<=== This is the problematic part
//impl Index<&str> for Object
//{
//    type Output = ObjectIndexHelper;
//
//    fn index(&self, index: &str) -> &ObjectIndexHelper
//	{
//        eprintln!("Accessing {}-anchor of {}-object immutably", index, self.name);
//
//		ObjectIndexHelper{ anchor: Box::new(self.anchors[index]), object: Box::new(self) }
//    }
//}

//}}}

// This would allow calling functions that need an objects and an anchors like
// do_something_with_object_and_anchor(my_object["my_anchor_name"]);


//{{{ Normal Index operator: Works but is insufficient for my use case.
impl Index<&str> for Object
{
    type Output = crate::anchors::Anchor;

    fn index(&self, index: &str) -> &Self::Output
	{
        eprintln!("Accessing {}-anchor of {}-object immutably", index, self.name);

		&self.anchors[index]
    }
}
//}}}

//{{{ IndexMut
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
//}}}


//}}}
//}}}

//{{{ Create Objects

//// TODO: Polyhedron
//// TODO: text
//// TODO: Measure::Length
//// TODO: Measure::Angle
//// TODO: Measure::Triangle
//{{{
pub fn cube(name: &str, x: f64, y: f64, z: f64) -> Object
{
	Object::new(name, Shape::Cube{ x: x,y: y,z: z })
}
//}}}
//{{{
pub fn cube_coords(name: &str, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> Object
{
	let x = (x1 - x2).abs();
	let y = (y1 - y2).abs();
	let z = (z1 - z2).abs();

	let x_shift = if x1<x2 { x1 } else { x2 };
	let y_shift = if y1<y2 { y1 } else { y2 };
	let z_shift = if z1<z2 { z1 } else { z2 };

	let mut cube = Object::new(name, Shape::Cube{ x: x,y: y,z: z });
	cube.translate(x/2.0+x_shift, y/2.0+y_shift,z/2.0+z_shift);
	cube
}
//}}}
//{{{
pub fn sphere(name: &str, r: f64) -> Object
{
	Object::new(name, Shape::Sphere{ r: r, face_number: None::<i32>, face_angle: None::<f64>, face_size: None::<f64> })
}
//}}}
//{{{
pub fn cylinder(name: &str, h: f64, r1: f64, r2: f64) -> Object // Stands along the z-axis, r1 is at the orgigin, r2 is h away from the origin
{
	Object::new(name, Shape::Cylinder{ h: h, r1: r1, r2: r2, face_number: None::<i32>, face_angle: None::<f64>, face_size: None::<f64> })
}
//}}}
//{{{
pub fn text(name: &str, text: &str, font: &str, size: i32, spacing: f64) -> Object
{
	Object::new(name, Shape::Text{ text: String::from(text), font: String::from(font), size: size, spacing: spacing })
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
pub fn pipe(name: &str, l: f64, r_outer: f64, r_inner: f64) -> Object // Pipe of length l with inner radius r_inner and outer radius r_outer
{
	let     outer = cylinder(&(String::from("base cylinder for ")+name), l, r_outer, r_outer);
	let mut inner = cylinder(&(String::from("base cylinder for ")+name), l+2.0, r_inner, r_inner);
	inner.translate_z(-1.0);
	difference(name, [outer, inner])
}
//}}}
//{{{
pub fn wedge(name: &str, x: f64, y: f64, angle: f64) -> Object
{
	let     lower = cube_coords(&(String::from("base cylinder for ")+name), 0.0, 0.0, 0.0, x, y, 0.000000000000001);
	let mut upper = cube_coords(&(String::from("base cylinder for ")+name), 0.0, 0.0, 0.0, x, y, 0.000000000000001);
	upper.rotate_y(-angle);
	hull(name, [lower, upper])
}
//}}}
//{{{
pub fn pipe_cut(name: &str, l: f64, r_outer: f64, r_inner: f64, angle: f64) -> Object
{
	let     pipe    = pipe(&(String::from("base pipe for ")+name), l, r_outer, r_inner);
	let mut stencil = wedge(&(String::from("wedge for ")+name), 10.0*r_outer+2.0, -l, angle);
	stencil.rotate_x(-90.0);
	//inner.translate_z(-1.0);
	intersection(name, [pipe, stencil])
}
//}}}


//{{{
pub fn arrow(name: &str, length: f64, width: f64) -> Object
{
	let mut shaft = cylinder(&(String::from(name)+"::arrow::shaft"), 0.9*length,     width, width);
	let mut tip   = cylinder(&(String::from(name)+"::arrow::tip"),   0.1*length, 2.0*width,   0.0);
	tip.translate_z(0.9*length);
	union(name, [shaft, tip])
}
//}}}

//{{{
pub fn coordinate_system(name: &str) -> Object
{
	use crate::colour_named;
	//{{{
	let mut x = arrow("coordinate_system::x_axis", 1.0, 0.05);
	x.rotate_y(90.0);
	x.set_colour(colour_named("red"));
	//}}}
	
	//{{{
	let mut y = arrow("coordinate_system::y_axis", 1.0, 0.05);
	y.rotate_x(-90.0);
	y.set_colour(colour_named("green"));
	//}}}

	//{{{
	let mut z = arrow("coordinate_system::z_axis", 1.0, 0.05);
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
	use crate::colour_named;
	//{{{
	let mut x = arrow("object_origin::x_axis", 1.0, 0.05);
	x.rotate_y(90.0);
	x.set_colour(colour_named("red"));
	//}}}
	
	//{{{
	let mut y = arrow("object_origin::y_axis", 1.0, 0.05);
	y.rotate_x(-90.0);
	y.set_colour(colour_named("green"));
	//}}}

	//{{{
	let mut z = arrow("object_origin::z_axis", 1.0, 0.05);
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
	use crate::colour_named;
	//{{{
	let mut x = arrow("object_anchor::x_axis", 1.0, 0.05);
	x.rotate_y(90.0);
	x.set_colour(colour_named("red"));
	//}}}
	
	//{{{
	let mut y = arrow("object_anchor::y_axis", 1.0, 0.05);
	y.rotate_x(-90.0);
	y.set_colour(colour_named("green"));
	//}}}

	//{{{
	let mut z = arrow("object_anchor::z_axis", 1.0, 0.05);
	z.set_colour(colour_named("blue"));
	//}}}

	let mut base = sphere("object_anchor::origin", 0.5);
	base.set_colour(colour_named("blue"));
	let mut coord_sys = union("object anchor", [x, y, z, base]);

	coord_sys.set_fn(10);
	coord_sys.scale(0.3, 0.3, 0.3);

	coord_sys
}
//}}}



//}}}


