pub use vecmath::mat4_id as identity3D;
pub use vecmath::row_mat4_mul as multiply;

pub type Vector3D = vecmath::Vector4<f64>;
pub type Point3D  = vecmath::Vector4<f64>;
pub type Matrix3D = vecmath::Matrix4<f64>;

//{{{
pub trait RefSysExt
{
	fn display(&self, indentation: usize) -> String;

	fn     rotate_x(&mut self, x: f64);
	fn     rotate_y(&mut self, y: f64);
	fn     rotate_z(&mut self, z: f64);
	fn     rotate(  &mut self, x: f64, y: f64, z: f64);
	fn rel_rotate_x(&mut self, x: f64);
	fn rel_rotate_y(&mut self, y: f64);
	fn rel_rotate_z(&mut self, z: f64);
	fn rel_rotate(  &mut self, x: f64, y: f64, z: f64);

	fn     translate_x(&mut self, x: f64);
	fn     translate_y(&mut self, y: f64);
	fn     translate_z(&mut self, z: f64);
	fn     translate(  &mut self, x: f64, y:f64, z: f64);
	fn rel_translate_x(&mut self, x: f64);
	fn rel_translate_y(&mut self, y: f64);
	fn rel_translate_z(&mut self, z: f64);
	fn rel_translate(  &mut self, x: f64, y:f64, z: f64);

	fn     scale_x(&mut self, x: f64);
	fn     scale_y(&mut self, y: f64);
	fn     scale_z(&mut self, z: f64);
	fn     scale(  &mut self, x: f64, y:f64, z: f64);
	fn rel_scale_x(&mut self, x: f64);
	fn rel_scale_y(&mut self, y: f64);
	fn rel_scale_z(&mut self, z: f64);
	fn rel_scale(  &mut self, x: f64, y:f64, z: f64);
}
//}}}

//{{{
impl RefSysExt for Matrix3D
{
	//{{{
	fn display(&self, indentation: usize) -> String
	{
		let indent   = "\t".repeat(indentation as usize);
		( format!("{}[{:16.10?},\n", indent, self[0])
		+&format!("{} {:16.10?},\n", indent, self[1])
		+&format!("{} {:16.10?},\n", indent, self[2])
		+&format!("{} {:16.10?}]",   indent, self[3]))
	}
	//}}}
	//{{{ Positions in a MultMatrix
	//
	// [ (0,0) (0,1) (0,2) (0,3) ]
	// [ (1,0) (1,1) (1,2) (1,3) ]
	// [ (2,0) (2,1) (2,2) (2,3) ]
	// [ (3,0) (3,1) (3,2) (3,3) ]
	//}}}

	//{{{Rotations
	//{{{
	fn rotate_x(&mut self, x: f64)
	{
		let x = x.to_radians();
		let mut rotation=identity3D();
		rotation[1][1] =  x.cos();
		rotation[1][2] = -x.sin();
		rotation[2][1] =  x.sin();
		rotation[2][2] =  x.cos();

		*self = multiply(*self, rotation);
	}
	//}}}
	//{{{
	fn rotate_y(&mut self, y: f64)
	{
		let y = y.to_radians();
		let mut rotation=identity3D();
		rotation[0][0] =  y.cos();
		rotation[0][2] =  y.sin();
		rotation[2][0] = -y.sin();
		rotation[2][2] =  y.cos();

		*self = multiply(*self, rotation);
	}
	//}}}
	//{{{
	fn rotate_z(&mut self, z: f64)
	{
		let z = z.to_radians();
		let mut rotation=identity3D();
		rotation[0][0] =  z.cos();
		rotation[0][1] = -z.sin();
		rotation[1][0] =  z.sin();
		rotation[1][1] =  z.cos();

		*self = multiply(*self, rotation);
	}
	//}}}
	//{{{
	fn rotate(&mut self, x: f64, y: f64, z: f64)
	{
		self.rotate_x(x);
		self.rotate_y(y);
		self.rotate_z(z);
	}
	//}}}

	//{{{
	fn rel_rotate_x(&mut self, x: f64)
	{
		let x = x.to_radians();
		let mut rotation=identity3D();
		rotation[1][1] =  x.cos();
		rotation[1][2] = -x.sin();
		rotation[2][1] =  x.sin();
		rotation[2][2] =  x.cos();

		*self = multiply(rotation, *self);
	}
	//}}}
	//{{{
	fn rel_rotate_y(&mut self, y: f64)
	{
		let y = y.to_radians();
		let mut rotation=identity3D();
		rotation[0][0] =  y.cos();
		rotation[0][2] =  y.sin();
		rotation[2][0] = -y.sin();
		rotation[2][2] =  y.cos();

		*self = multiply(rotation, *self);
	}
	//}}}
	//{{{
	fn rel_rotate_z(&mut self, z: f64)
	{
		let z = z.to_radians();
		let mut rotation=identity3D();
		rotation[0][0] =  z.cos();
		rotation[0][1] = -z.sin();
		rotation[1][0] =  z.sin();
		rotation[1][1] =  z.cos();

		*self = multiply(rotation, *self);
	}
	//}}}
	//{{{
	fn rel_rotate(&mut self, x: f64, y: f64, z: f64)
	{
		self.rel_rotate_x(x);
		self.rel_rotate_y(y);
		self.rel_rotate_z(z);
	}
	//}}}
	//}}}

	//{{{Translations
	//{{{
	fn translate_x(&mut self, x: f64)
	{
		let mut translation=identity3D();
		translation[0][3] =  x;

		*self = multiply(translation, *self);
	}
	//}}}
	//{{{
	fn translate_y(&mut self, y: f64)
	{
		let mut translation=identity3D();
		translation[1][3] =  y;

		*self = multiply(translation, *self);
	}
	//}}}
	//{{{
	fn translate_z(&mut self, z: f64)
	{
		let mut translation=identity3D();
		translation[2][3] =  z;

		*self = multiply(translation, *self);
	}
	//}}}
	//{{{
	fn translate(&mut self, x: f64, y:f64, z: f64)
	{
		let mut translation=identity3D();
		translation[0][3] =  x;
		translation[1][3] =  y;
		translation[2][3] =  z;

		*self = multiply(*self, translation);
	}
	//}}}

	//{{{
	fn rel_translate_x(&mut self, x: f64)
	{
		let mut translation=identity3D();
		translation[0][3] =  x;

		*self = multiply(*self, translation);
	}
	//}}}
	//{{{
	fn rel_translate_y(&mut self, y: f64)
	{
		let mut translation=identity3D();
		translation[1][3] =  y;

		*self = multiply(*self, translation);
	}
	//}}}
	//{{{
	fn rel_translate_z(&mut self, z: f64)
	{
		let mut translation=identity3D();
		translation[2][3] =  z;

		*self = multiply(*self, translation);
	}
	//}}}
	//{{{
	fn rel_translate(&mut self, x: f64, y:f64, z: f64)
	{
		let mut translation=identity3D();
		translation[0][3] =  x;
		translation[1][3] =  y;
		translation[2][3] =  z;

		*self = multiply(translation, *self);
	}
	//}}}
	//}}}

	//{{{ Scale
	//{{{
	fn scale_x(&mut self, x: f64)
	{
		let mut scale=identity3D();
		scale[0][0] =  x;

		*self = multiply(*self, scale);
	}
	//}}}
	//{{{
	fn scale_y(&mut self, y: f64)
	{
		let mut scale=identity3D();
		scale[1][1] =  y;

		*self = multiply(*self, scale);
	}
	//}}}
	//{{{
	fn scale_z(&mut self, z: f64)
	{
		let mut scale=identity3D();
		scale[2][2] =  z;

		*self = multiply(*self, scale);
	}
	//}}}
	//{{{
	fn scale(&mut self, x: f64, y:f64, z: f64)
	{
		let mut scale=identity3D();
		scale[0][0] =  x;
		scale[1][1] =  y;
		scale[2][2] =  z;

		*self = multiply(*self, scale);
	}
	//}}}

	//{{{
	fn rel_scale_x(&mut self, x: f64)
	{
		let mut scale=identity3D();
		scale[0][0] =  x;

		*self = multiply(scale, *self);
	}
	//}}}
	//{{{
	fn rel_scale_y(&mut self, y: f64)
	{
		let mut scale=identity3D();
		scale[1][1] =  y;

		*self = multiply(scale, *self);
	}
	//}}}
	//{{{
	fn rel_scale_z(&mut self, z: f64)
	{
		let mut scale=identity3D();
		scale[2][2] =  z;

		*self = multiply(scale, *self);
	}
	//}}}
	//{{{
	fn rel_scale(&mut self, x: f64, y:f64, z: f64)
	{
		let mut scale=identity3D();
		scale[0][0] =  x;
		scale[1][1] =  y;
		scale[2][2] =  z;

		*self = multiply(scale, *self);
	}
	//}}}
	//}}}
}
//}}}


