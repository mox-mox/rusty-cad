#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

extern crate ndarray;

//{{{
pub mod objects
{

	//{{{ helper_traits

	//{{{
	pub trait IsSerialisableObject
	{
		fn serialise(&self) -> String;
	}
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
		impl crate::objects::IsSerialisableScope for Colour
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
		impl crate::objects::IsSerialisableScope for RefSys
		{
			fn serialise(&self, child : String) -> String
			{
				String::from("multmatrix(m = ") + &self.to_string() + ")\n{\n" + &child + "\n};\n"
			}
		}
		//}}}

		//{{{
		impl<T> crate::objects::Is3DObject for T where T: HasRefSys // All the spatial transformations
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

	//{{{
	pub mod shapes
	{
		// TODO: Sphere
		// TODO: Cylinder
		// TODO: Polyhedron
		// TODO: Composite ( Object x Object )
		// TODO: text
		// TODO: Measure::Length
		// TODO: Measure::Angle

		//{{{
		pub mod cube
		{
			use crate::objects::IsSerialisableScope;
			//{{{ Define Cube

			//{{{pub struct Cube

#[derive(Debug)]
			pub struct Cube
			{
				pub ref_sys  : crate::objects::refsys::RefSys,
				pub measures : Measures,
				pub colour   : crate::objects::colour::Colour,
			}

			//{{{pub struct Measures

			#[derive(Debug)]
			pub struct Measures
			{
				x : f64,
				y : f64,
				z : f64,
			}
			//}}}
			//}}}

			//{{{
			impl crate::objects::refsys::HasRefSys for Cube
			{

				fn ref_sys_mut(&mut self) -> &mut crate::objects::refsys::RefSys
				{
					&mut self.ref_sys
				}
				fn ref_sys(&self) -> &crate::objects::refsys::RefSys
				{
					&self.ref_sys
				}
			}
			//}}}

			//{{{
			impl crate::objects::colour::HasColour for Cube
			{

				fn colour_mut(&mut self) -> &mut crate::objects::colour::Colour
				{
					&mut self.colour
				}
				fn colour(&self) -> &crate::objects::colour::Colour
				{
					&self.colour
				}
				fn set_colour(&mut self, colour : crate::objects::colour::Colour)
				{
					self.colour = colour;
				}
			}
			//}}}

			//{{{
			impl crate::objects::IsSerialisableObject for Cube
			{
				fn serialise(&self) -> String
				{
					//self.colour().serialise(
					self.colour.serialise(
						//self.ref_sys().serialise(
						self.ref_sys.serialise(
							String::from("cube([") + &self.measures.x.to_string() + ", " + &self.measures.y.to_string() + ", " + &self.measures.z.to_string() + "]);"
							)
						)
				}
			}
			//}}}
			//}}}

			//{{{ Use Cube

			//{{{
			pub fn cube(x: f64, y: f64, z: f64) -> Cube
			{
				let retval = Cube{ ref_sys : crate::objects::refsys::RefSys::eye(4), measures : Measures{x, y, z}, colour : crate::objects::colour::Colour::Unset};
				return retval;
			}
			//}}}
			//}}}
		}
		//}}}

	}
	//}}}
}
//}}}

