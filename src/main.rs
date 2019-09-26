#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]



extern crate ndarray;
//type Colour     = ndarray::Array1<f64>;
//type Point      = ndarray::Array1<f64>;
type RefSys     = ndarray::Array2<f64>;

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
impl IsSerialisableObject for Cube
{
	fn serialise(&self) -> String
	{
		self.ref_sys().serialise(
			String::from("cube([") + &self.measures.x.to_string() + ", " + &self.measures.y.to_string() + ", " + &self.measures.z.to_string() + "]);"
		)
	}
}
//}}}

//{{{
pub fn cube(x: f64, y: f64, z: f64) -> Cube
{
	let retval = Cube{ ref_sys : RefSys::eye(4), measures : Measures{x, y, z}};
	return retval;
}
//}}}

//}}}






fn main()
{
	//println!("Hello World");

	let mut cube = cube(5.0, 1.0, 1.0);
	//cube.rotate_x(45.0);


	eprintln!("cube = {:?}\n\n\n\n", cube);
	println!("{}", cube.serialise());
}
