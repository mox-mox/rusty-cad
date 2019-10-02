#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]



extern crate ndarray;
type ColourVec  = ndarray::Array1<f64>;
//type Measures   = ndarray::Array1<f64>;
type RefSys     = ndarray::Array2<f64>;


//{{{pub struct Colour

#[derive(Debug)]
pub enum Colour
{
	Unset,
	Numeric(ColourVec),
	Named(String),
}

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

//}}}

//{{{pub struct Measures

#[derive(Debug)]
pub struct Measures
{
	x : f64,
	y : f64,
	z : f64,
}
//}}}

//{{{ Traits

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
pub trait HasRefSys
{
	fn ref_sys_mut(&mut self) -> &mut RefSys;
	fn ref_sys(&self) -> &RefSys;
}
//}}}

//{{{
pub trait HasColour
{
	fn colour_mut(&mut self) -> &mut Colour;
	fn colour(&self) -> &Colour;
	fn set_colour(&mut self, colour : Colour);
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

//{{{
impl IsSerialisableScope for RefSys
{
	fn serialise(&self, child : String) -> String
	{
		String::from("multmatrix(m = ") + &self.to_string() + ")\n{\n" + &child + "\n};\n"
	}
}
//}}}

//{{{
impl IsSerialisableScope for Colour
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

//{{{
impl<T> Is3DObject for T where T: HasRefSys
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

//{{{ Cube

//{{{pub struct Cube

#[derive(Debug)]
pub struct Cube
{
	pub ref_sys  : RefSys,
	pub measures : Measures,
	pub colour   : Colour,
}
//}}}

//{{{
impl HasRefSys for Cube
{

    fn ref_sys_mut(&mut self) -> &mut RefSys
	{
		&mut self.ref_sys
    }
    fn ref_sys(&self) -> &RefSys
	{
		&self.ref_sys
    }
}
//}}}

//{{{
impl HasColour for Cube
{

    fn colour_mut(&mut self) -> &mut Colour
	{
		&mut self.colour
    }
    fn colour(&self) -> &Colour
	{
		&self.colour
    }
	fn set_colour(&mut self, colour : Colour)
	{
		self.colour = colour;
		eprintln!("set_colourXXXXXXXXXXXXXXXXXXXXXXX");
	}
}
//}}}

//{{{
impl IsSerialisableObject for Cube
{
	fn serialise(&self) -> String
	{
		self.colour().serialise(
		self.ref_sys().serialise(
			String::from("cube([") + &self.measures.x.to_string() + ", " + &self.measures.y.to_string() + ", " + &self.measures.z.to_string() + "]);"
			)
		)
	}
}
//}}}

//{{{
pub fn cube(x: f64, y: f64, z: f64) -> Cube
{
	//let retval = Cube{ ref_sys : RefSys::eye(4), measures : Measures{x, y, z}, colour : Colour::from_vec(vec![1.0, 0.0, 0.0])};
	let retval = Cube{ ref_sys : RefSys::eye(4), measures : Measures{x, y, z}, colour : Colour::Unset};
	return retval;
}
//}}}

//}}}






fn main()
{
	//println!("Hello World");


	let mut cube = cube(5.0, 1.0, 1.0);
	cube.rotate_x(45.0);
	//cube.set_colour(colour_rgba(1.0, 0.0, 1.0, 0.5));
	cube.set_colour(colour_named("green"));


	eprintln!("cube = {:?}\n\n\n\n", cube);
	println!("{}", cube.serialise());

}
