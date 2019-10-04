#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

extern crate ndarray;
//use std::fmt;
//pub use crate::shapes::primitive;

//{{{ helper_traits

//{{{
pub trait IsSerialisableShape
{
	fn serialise(&self) -> String;
}
//}}}

//{{{
pub trait IsSerialisableObject
{
	fn serialise(&self) -> String;
}

//impl<T> fmt::Display for T where T: IsSerialisableObject // Printing for all 3D-Objects
//{
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
//	{
//        write!(f, "{}", self.serialise())
//    }
//}



//}}}

//{{{
pub trait IsSerialisableScope
{
	fn serialise(&self, child : String) -> String;
}
//}}}

//{{{
pub trait Is3DObject
{
	fn rotate_x(&mut self, x: f64);
	//fn rotate_y(&mut self, y: f64);
	//fn rotate_z(&mut self, z: f64);
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

	//{{{
	pub trait HasColour
	{
		fn colour_mut(&mut self) -> &mut Colour;
		fn colour(&self) -> &Colour;
		fn set_colour(&mut self, colour : Colour);
	}
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

	//{{{
	impl<T> crate::Is3DObject for T where T: HasRefSys // All the spatial transformations
	{
		fn rotate_x(&mut self, x: f64)
		{
			let x = x.to_radians();
			let mut rotation=RefSys::eye(4);
			rotation[[1,1]] =  x.cos();
			rotation[[1,2]] = -x.sin();
			rotation[[2,1]] =  x.sin();
			rotation[[2,2]] =  x.cos();

			*self.ref_sys_mut() = rotation.dot(self.ref_sys_mut());
		}
	}
	//}}}
	//}}}

	//{{{
	pub trait HasRefSys
	{
		fn ref_sys_mut(&mut self) -> &mut RefSys;
		fn ref_sys(&self) -> &RefSys;
	}
	//}}}
}
//}}}

////{{{
//pub mod shapes
//{
	//// TODO: Cube
	//// TODO: Sphere
	//// TODO: Cylinder
	// TODO: Polyhedron
	// TODO: Composite ( Object x Object )
	// TODO: text
	// TODO: Measure::Length
	// TODO: Measure::Angle




	//{{{
	pub mod primitive
	{
		//{{{ Define Primitive

		//{{{ pub enum Shape

		#[derive(Debug)]
		pub enum Shape
		{
			Cube      { x: f64, y: f64, z: f64},
			Sphere    { r: f64, face_number: Option<i32>, face_angle: Option<f64>, face_size: Option<f64> },
			Cylinder  { h: f64, r1 : f64, r2 : f64, face_angle  : Option<f64>, face_size   : Option<f64>, face_number : Option<i32> },

			///Composite { c1: Object, c2: Object },
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
				}
			}
			//}}}
		}


		//{{{
		impl crate::refsys::HasRefSys for Object
		{

			fn ref_sys_mut(&mut self) -> &mut crate::refsys::RefSys
			{
				&mut self.ref_sys
			}
			fn ref_sys(&self) -> &crate::refsys::RefSys
			{
				&self.ref_sys
			}
		}
		//}}}

		//{{{
		impl crate::colour::HasColour for Object
		{

			fn colour_mut(&mut self) -> &mut crate::colour::Colour
			{
				&mut self.colour
			}
			fn colour(&self) -> &crate::colour::Colour
			{
				&self.colour
			}
			fn set_colour(&mut self, colour : crate::colour::Colour)
			{
				self.colour = colour;
			}
		}
		//}}}

		//{{{
		impl crate::IsSerialisableObject for Object
		{
			fn serialise(&self) -> String
			{
				use crate::IsSerialisableScope;
				self.colour.serialise(
					self.ref_sys.serialise(
						self.shape.serialise()
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
		//}}}
	}
	//}}}
	
//}
////}}}

