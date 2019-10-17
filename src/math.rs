//pub use vecmath::mat4_id as identity3D;
//pub use vecmath::mat4_inv as invert3D;
//pub use vecmath::row_mat4_mul as multiply;

//pub type Vector3D = vecmath::Vector4<f64>;
//pub type Point3D  = vecmath::Vector4<f64>;
//pub type Matrix3D = vecmath::Matrix4<f64>;


pub use core::borrow::{Borrow, BorrowMut};
pub use std::convert::{AsRef, AsMut};
pub use std::ops::{Deref, DerefMut, Not, Add, Sub, Mul, BitXor};
use std::fmt;


//{{{ pub struct Matrix4D

#[derive(Clone, Copy, Debug)]
pub struct Matrix4D(M3D);
pub type M3D = vecmath::Matrix4<f64>;
//{{{
impl Deref for Matrix4D
{
    type Target = M3D;

    #[inline]
    fn deref(&self) -> &M3D
	{
        &self.0
    }
}
//}}}
//{{{
impl DerefMut for Matrix4D
{
    #[inline]
    fn deref_mut(&mut self) -> &mut M3D
	{
        &mut self.0
    }
}
//}}}
//{{{
impl Borrow<M3D> for Matrix4D
{
    #[inline]
    fn borrow(&self) -> &M3D
	{
        &self.0
    }
}
//}}}
//{{{
impl BorrowMut<M3D> for Matrix4D
{
    #[inline]
    fn borrow_mut(&mut self) -> &mut M3D
	{
        &mut self.0
    }
}
//}}}
//{{{
impl AsRef<M3D> for Matrix4D
{
    #[inline]
    fn as_ref(&self) -> &M3D
	{
        &self.0
    }
}
//}}}
//{{{
impl AsMut<M3D> for Matrix4D {
    fn as_mut(&mut self) -> &mut M3D {
        &mut self.0
    }
}
//}}}
//{{{
impl fmt::Display for Matrix4D
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		let indentation = if let Some(width) = f.width() { width } else { 0 as usize };
		write!(f, "{0}[{1:16.10?},\n{0} {2:16.10?},\n{0} {3:16.10?},\n{0} {4:16.10?}]", "\t".repeat(indentation as usize), self[0], self[1], self[2], self[3])
	}
}
//}}}

//{{{
impl Add<&Matrix4D> for Matrix4D
{
    type Output = Self;

    fn add(self, other: &Matrix4D) -> Matrix4D
	{
		Self(vecmath::mat4_add(**other, *self))
    }
}
//}}}
//{{{
impl Sub<&Matrix4D> for Matrix4D
{
    type Output = Self;

    fn sub(self, other: &Matrix4D) -> Matrix4D
	{
		Self(vecmath::mat4_sub(**other, *self))
    }
}
//}}}
//{{{
impl Mul<f64> for Matrix4D
{
    type Output = Self;

    fn mul(self, a: f64) -> Matrix4D
	{
		Self(
		[[ a*self[0][0], a*self[0][1], a*self[0][2], a*self[0][3]],
		 [ a*self[1][0], a*self[1][1], a*self[1][2], a*self[1][3]],
		 [ a*self[2][0], a*self[2][1], a*self[2][2], a*self[2][3]],
		 [ a*self[3][0], a*self[3][1], a*self[3][2], a*self[3][3]]])
    }
}
//}}}
//{{{
impl Mul<&Matrix4D> for Matrix4D
{
    type Output = Self;

    fn mul(self, other: &Matrix4D) -> Matrix4D
	{
		Self(vecmath::row_mat4_mul(**other, *self))
    }
}
//}}}
//{{{
impl Mul<Matrix4D> for Matrix4D
{
    type Output = Self;

    fn mul(self, other: Matrix4D) -> Matrix4D
	{
		Self(vecmath::row_mat4_mul(*other, *self))
    }
}
//}}}
//{{{
impl Not for Matrix4D
{
    type Output = Self;

    fn not(self) -> Self
	{
		Self(vecmath::mat4_inv(*self))
    }
}
//}}}

//{{{
impl Matrix4D
{
	//{{{
	pub fn identity() -> Matrix4D
	{
		Self(
		[[ 1.0, 0.0, 0.0, 0.0],
		 [ 0.0, 1.0, 0.0, 0.0],
		 [ 0.0, 0.0, 1.0, 0.0],
		 [ 0.0, 0.0, 0.0, 1.0]])
	}
	//}}}

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

	//{{{
	pub fn row(&self, i: i32) -> Vector4D
	{
		let i = i as usize;
		Vector4D([self[i][0], self[i][1], self[i][2], self[i][3]])
	}
	//}}}

	//{{{
	pub fn column(&self, i: i32) -> Vector4D
	{
		let i = i as usize;
		Vector4D([self[0][i], self[1][i], self[2][i], self[3][i]])
	}
	//}}}

	//{{{ Positions in a MultMatrix
	//
	// [ (0,0) (0,1) (0,2) (0,3) ]
	// [ (1,0) (1,1) (1,2) (1,3) ]
	// [ (2,0) (2,1) (2,2) (2,3) ]
	// [ (3,0) (3,1) (3,2) (3,3) ]
	//}}}

	//{{{ 3D Manipulations
	//{{{Rotations
	//{{{
	pub fn rotate_x(&mut self, x: f64)
	{
		let x = x.to_radians();
		let mut rotation=Self::identity();
		rotation[1][1] =  x.cos();
		rotation[1][2] = -x.sin();
		rotation[2][1] =  x.sin();
		rotation[2][2] =  x.cos();

		*self = *self * rotation;
	}
	//}}}
	//{{{
	pub fn rotate_y(&mut self, y: f64)
	{
		let y = y.to_radians();
		let mut rotation=Self::identity();
		rotation[0][0] =  y.cos();
		rotation[0][2] =  y.sin();
		rotation[2][0] = -y.sin();
		rotation[2][2] =  y.cos();

		*self = *self * rotation;
	}
	//}}}
	//{{{
	pub fn rotate_z(&mut self, z: f64)
	{
		let z = z.to_radians();
		let mut rotation=Self::identity();
		rotation[0][0] =  z.cos();
		rotation[0][1] = -z.sin();
		rotation[1][0] =  z.sin();
		rotation[1][1] =  z.cos();

		*self = *self * rotation;
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
		let mut rotation=Self::identity();
		rotation[1][1] =  x.cos();
		rotation[1][2] = -x.sin();
		rotation[2][1] =  x.sin();
		rotation[2][2] =  x.cos();

		*self = rotation * *self;
	}
	//}}}
	//{{{
	pub fn rel_rotate_y(&mut self, y: f64)
	{
		let y = y.to_radians();
		let mut rotation=Self::identity();
		rotation[0][0] =  y.cos();
		rotation[0][2] =  y.sin();
		rotation[2][0] = -y.sin();
		rotation[2][2] =  y.cos();

		*self = rotation * *self;
	}
	//}}}
	//{{{
	pub fn rel_rotate_z(&mut self, z: f64)
	{
		let z = z.to_radians();
		let mut rotation=Self::identity();
		rotation[0][0] =  z.cos();
		rotation[0][1] = -z.sin();
		rotation[1][0] =  z.sin();
		rotation[1][1] =  z.cos();

		*self = rotation * *self;
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
	//}}}

	//{{{Translations
	//{{{
	pub fn translate_x(&mut self, x: f64)
	{
		let mut translation=Self::identity();
		translation[0][3] =  x;

		*self = *self * translation;
	}
	//}}}
	//{{{
	pub fn translate_y(&mut self, y: f64)
	{
		let mut translation=Self::identity();
		translation[1][3] =  y;

		*self = *self * translation;
	}
	//}}}
	//{{{
	pub fn translate_z(&mut self, z: f64)
	{
		let mut translation=Self::identity();
		translation[2][3] =  z;

		*self = *self * translation;
	}
	//}}}
	//{{{
	pub fn translate(&mut self, x: f64, y:f64, z: f64)
	{
		let mut translation=Self::identity();
		translation[0][3] =  x;
		translation[1][3] =  y;
		translation[2][3] =  z;

		*self = *self * translation;
	}
	//}}}

	//{{{
	pub fn rel_translate_x(&mut self, x: f64)
	{
		let mut translation=Self::identity();
		translation[0][3] =  x;

		*self = translation * *self;
	}
	//}}}
	//{{{
	pub fn rel_translate_y(&mut self, y: f64)
	{
		let mut translation=Self::identity();
		translation[1][3] =  y;

		*self = translation * *self;
	}
	//}}}
	//{{{
	pub fn rel_translate_z(&mut self, z: f64)
	{
		let mut translation=Self::identity();
		translation[2][3] =  z;

		*self = translation * *self;
	}
	//}}}
	//{{{
	pub fn rel_translate(&mut self, x: f64, y:f64, z: f64)
	{
		let mut translation=Self::identity();
		translation[0][3] =  x;
		translation[1][3] =  y;
		translation[2][3] =  z;

		*self = translation * *self;
	}
	//}}}
	//}}}

	//{{{ Scale
	//{{{
	pub fn scale_x(&mut self, x: f64)
	{
		let mut scale=Self::identity();
		scale[0][0] =  x;

		*self = *self * scale;
	}
	//}}}
	//{{{
	pub fn scale_y(&mut self, y: f64)
	{
		let mut scale=Self::identity();
		scale[1][1] =  y;

		*self = *self * scale;
	}
	//}}}
	//{{{
	pub fn scale_z(&mut self, z: f64)
	{
		let mut scale=Self::identity();
		scale[2][2] =  z;

		*self = *self * scale;
	}
	//}}}
	//{{{
	pub fn scale(&mut self, x: f64, y:f64, z: f64)
	{
		let mut scale=Self::identity();
		scale[0][0] =  x;
		scale[1][1] =  y;
		scale[2][2] =  z;

		*self = *self * scale;
	}
	//}}}

	//{{{
	pub fn rel_scale_x(&mut self, x: f64)
	{
		let mut scale=Self::identity();
		scale[0][0] =  x;

		*self = scale * *self;
	}
	//}}}
	//{{{
	pub fn rel_scale_y(&mut self, y: f64)
	{
		let mut scale=Self::identity();
		scale[1][1] =  y;

		*self = scale * *self;
	}
	//}}}
	//{{{
	pub fn rel_scale_z(&mut self, z: f64)
	{
		let mut scale=Self::identity();
		scale[2][2] =  z;

		*self = scale * *self;
	}
	//}}}
	//{{{
	pub fn rel_scale(&mut self, x: f64, y:f64, z: f64)
	{
		let mut scale=Self::identity();
		scale[0][0] =  x;
		scale[1][1] =  y;
		scale[2][2] =  z;

		*self = scale * *self;
	}
	//}}}
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


	//{{{ Positions in an Affine Matrix
	//
	// [    Sx+cox(y)+cos(z)             -sin(z)              sin(y)              deltaX ]
	// [              sin(z)    Sy+cos(x)+cos(z)             -sin(x)              deltaX ]
	// [             -sin(y)              sin(x)    Sz+cos(x)+cos(y)              deltaX ]
	// [                   0                   0                   0                   1 ]
	//}}}
	//{{{ Positions in a Rotation Matrix
	//
	// [       cox(y)+cos(z)             -sin(z)              sin(y)               (0,3) ]
	// [              sin(z)       cos(x)+cos(z)             -sin(x)               (1,3) ]
	// [             -sin(y)              sin(x)       cos(x)+cos(y)               (2,3) ]
	// [               (3,0)               (3,1)               (3,2)               (3,3) ]
	//}}}
	//{{{
	pub fn get_rotate_x(&mut self) -> f64
	{
		(self[1][2]/self.get_scale_y()).asin().to_degrees()
	}
	//}}}
	//{{{
	pub fn get_rotate_y(&mut self) -> f64
	{
		(self[2][0]/self.get_scale_z()).asin().to_degrees()
	}
	//}}}
	//{{{
	pub fn get_rotate_z(&mut self) -> f64
	{
		(self[0][1]/self.get_scale_x()).asin().to_degrees()
	}
	//}}}
	//{{{
	pub fn get_rotate(&mut self) -> ( f64, f64, f64)
	{
		(self.get_rotate_x(), self.get_rotate_y(), self.get_rotate_z())
	}
	//}}}

	//{{{
	pub fn get_translate_x(&mut self) -> f64
	{
		self[0][3]
	}
	//}}}
	//{{{
	pub fn get_translate_y(&mut self) -> f64
	{
		self[1][3]
	}
	//}}}
	//{{{
	pub fn get_translate_z(&mut self) -> f64
	{
		self[2][3]
	}
	//}}}
	//{{{
	pub fn get_translate(&mut self) -> (f64, f64, f64)
	{
		(self.get_translate_x(), self.get_translate_y(), self.get_translate_z())
	}
	//}}}

	//{{{
	pub fn get_scale_x(&mut self) -> f64
	{
		self.column(0).l2_norm().sqrt()
	}
	//}}}
	//{{{
	pub fn get_scale_y(&mut self) -> f64
	{
		self.column(1).l2_norm().sqrt()
	}
	//}}}
	//{{{
	pub fn get_scale_z(&mut self) -> f64
	{
		self.column(2).l2_norm().sqrt()
	}
	//}}}
	//{{{
	pub fn get_scale(&mut self) -> (f64, f64, f64)
	{
		(self.get_scale_x(), self.get_scale_y(), self.get_scale_z())
	}
	//}}}
	//}}}
}
//}}}
//}}}

//{{{ pub struct Vector4D

#[derive(Clone, Copy, Debug)]
pub struct Vector4D(V3D);
pub type V3D = vecmath::Vector4<f64>;
//{{{
impl Deref for Vector4D
{
    type Target = V3D;

    #[inline]
    fn deref(&self) -> &V3D
	{
        &self.0
    }
}
//}}}
//{{{
impl Borrow<V3D> for Vector4D
{
    #[inline]
    fn borrow(&self) -> &V3D
	{
        &self.0
    }
}
//}}}
//{{{
impl BorrowMut<V3D> for Vector4D
{
    #[inline]
    fn borrow_mut(&mut self) -> &mut V3D
	{
        &mut self.0
    }
}
//}}}
//{{{
impl AsRef<V3D> for Vector4D
{
    #[inline]
    fn as_ref(&self) -> &V3D
	{
        &self.0
    }
}
//}}}
//{{{
impl AsMut<V3D> for Vector4D {
    fn as_mut(&mut self) -> &mut V3D {
        &mut self.0
    }
}
//}}}
//{{{
impl Mul<f64> for Vector4D
{
    type Output = Self;

    fn mul(self, a: f64) -> Vector4D
	{
		Self([ a*self[0], a*self[1], a*self[2], a*self[3]])
    }
}
//}}}
//{{{
impl Mul<Vector4D> for Vector4D
{
    type Output = f64;

    fn mul(self, other: Vector4D) -> f64
	{
		self[0]*other[0] + self[1]*other[1] + self[2]*other[2] + self[3]*other[3]
    }
}
//}}}
////{{{
//impl BitXor<Vector4D> for Vector4D // Vector Product
//{
//    type Output = Self;
//
//    fn bitxor(self, other: Self) -> Self::Output
//	{
//		Self()
//    }
//}
////}}}

//{{{
impl Vector4D
{
	//{{{
	pub fn l2_norm(&self) -> f64
	{
		vecmath::vec4_dot(**self, **self)
	}
	//}}}



//
//	//{{{
//	fn display(&self, indentation: usize) -> String
//	{
//		let indent   = "\t".repeat(indentation as usize);
//		( format!("{}[{:16.10?},\n", indent, self[0])
//		+&format!("{} {:16.10?},\n", indent, self[1])
//		+&format!("{} {:16.10?},\n", indent, self[2])
//		+&format!("{} {:16.10?}]",   indent, self[3]))
//	}
//	//}}}
//
//	////{{{
//	//fn l2_norm(x: ndarray::ArrayView1<f64>) -> f64
//	//{
//	//	// Taken from:  https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/linear_algebra.html
//	//	//x.dot(&x).sqrt()
//	//	0.0
//	//}
//	////}}}
//
//	//{{{ Positions in a MultMatrix
//	//
//	// [ (0,0) (0,1) (0,2) (0,3) ]
//	// [ (1,0) (1,1) (1,2) (1,3) ]
//	// [ (2,0) (2,1) (2,2) (2,3) ]
//	// [ (3,0) (3,1) (3,2) (3,3) ]
//	//}}}
//
//	//{{{ 3D Manipulations
//	//{{{Rotations
//	//{{{
//	fn rotate_x(&mut self, x: f64)
//	{
//		let x = x.to_radians();
//		let mut rotation=identity3D();
//		rotation[1][1] =  x.cos();
//		rotation[1][2] = -x.sin();
//		rotation[2][1] =  x.sin();
//		rotation[2][2] =  x.cos();
//
//		*self = multiply(rotation, *self);
//	}
//	//}}}
//	//{{{
//	fn rotate_y(&mut self, y: f64)
//	{
//		let y = y.to_radians();
//		let mut rotation=identity3D();
//		rotation[0][0] =  y.cos();
//		rotation[0][2] =  y.sin();
//		rotation[2][0] = -y.sin();
//		rotation[2][2] =  y.cos();
//
//		*self = multiply(rotation, *self);
//	}
//	//}}}
//	//{{{
//	fn rotate_z(&mut self, z: f64)
//	{
//		let z = z.to_radians();
//		let mut rotation=identity3D();
//		rotation[0][0] =  z.cos();
//		rotation[0][1] = -z.sin();
//		rotation[1][0] =  z.sin();
//		rotation[1][1] =  z.cos();
//
//		*self = multiply(rotation, *self);
//	}
//	//}}}
//	//{{{
//	fn rotate(&mut self, x: f64, y: f64, z: f64)
//	{
//		self.rotate_x(x);
//		self.rotate_y(y);
//		self.rotate_z(z);
//	}
//	//}}}
//
//	//{{{
//	fn rel_rotate_x(&mut self, x: f64)
//	{
//		let x = x.to_radians();
//		let mut rotation=identity3D();
//		rotation[1][1] =  x.cos();
//		rotation[1][2] = -x.sin();
//		rotation[2][1] =  x.sin();
//		rotation[2][2] =  x.cos();
//
//		*self = multiply(*self, rotation);
//	}
//	//}}}
//	//{{{
//	fn rel_rotate_y(&mut self, y: f64)
//	{
//		let y = y.to_radians();
//		let mut rotation=identity3D();
//		rotation[0][0] =  y.cos();
//		rotation[0][2] =  y.sin();
//		rotation[2][0] = -y.sin();
//		rotation[2][2] =  y.cos();
//
//		*self = multiply(*self, rotation);
//	}
//	//}}}
//	//{{{
//	fn rel_rotate_z(&mut self, z: f64)
//	{
//		let z = z.to_radians();
//		let mut rotation=identity3D();
//		rotation[0][0] =  z.cos();
//		rotation[0][1] = -z.sin();
//		rotation[1][0] =  z.sin();
//		rotation[1][1] =  z.cos();
//
//		*self = multiply(*self, rotation);
//	}
//	//}}}
//	//{{{
//	fn rel_rotate(&mut self, x: f64, y: f64, z: f64)
//	{
//		self.rel_rotate_x(x);
//		self.rel_rotate_y(y);
//		self.rel_rotate_z(z);
//	}
//	//}}}
//	//}}}
//
//	//{{{Translations
//	//{{{
//	fn translate_x(&mut self, x: f64)
//	{
//		let mut translation=identity3D();
//		translation[0][3] =  x;
//
//		*self = multiply(translation, *self);
//	}
//	//}}}
//	//{{{
//	fn translate_y(&mut self, y: f64)
//	{
//		let mut translation=identity3D();
//		translation[1][3] =  y;
//
//		*self = multiply(translation, *self);
//	}
//	//}}}
//	//{{{
//	fn translate_z(&mut self, z: f64)
//	{
//		let mut translation=identity3D();
//		translation[2][3] =  z;
//
//		*self = multiply(translation, *self);
//	}
//	//}}}
//	//{{{
//	fn translate(&mut self, x: f64, y:f64, z: f64)
//	{
//		let mut translation=identity3D();
//		translation[0][3] =  x;
//		translation[1][3] =  y;
//		translation[2][3] =  z;
//
//		*self = multiply(*self, translation);
//	}
//	//}}}
//
//	//{{{
//	fn rel_translate_x(&mut self, x: f64)
//	{
//		let mut translation=identity3D();
//		translation[0][3] =  x;
//
//		*self = multiply(*self, translation);
//	}
//	//}}}
//	//{{{
//	fn rel_translate_y(&mut self, y: f64)
//	{
//		let mut translation=identity3D();
//		translation[1][3] =  y;
//
//		*self = multiply(*self, translation);
//	}
//	//}}}
//	//{{{
//	fn rel_translate_z(&mut self, z: f64)
//	{
//		let mut translation=identity3D();
//		translation[2][3] =  z;
//
//		*self = multiply(*self, translation);
//	}
//	//}}}
//	//{{{
//	fn rel_translate(&mut self, x: f64, y:f64, z: f64)
//	{
//		let mut translation=identity3D();
//		translation[0][3] =  x;
//		translation[1][3] =  y;
//		translation[2][3] =  z;
//
//		*self = multiply(translation, *self);
//	}
//	//}}}
//	//}}}
//
//	//{{{ Scale
//	//{{{
//	fn scale_x(&mut self, x: f64)
//	{
//		let mut scale=identity3D();
//		scale[0][0] =  x;
//
//		*self = multiply(*self, scale);
//	}
//	//}}}
//	//{{{
//	fn scale_y(&mut self, y: f64)
//	{
//		let mut scale=identity3D();
//		scale[1][1] =  y;
//
//		*self = multiply(*self, scale);
//	}
//	//}}}
//	//{{{
//	fn scale_z(&mut self, z: f64)
//	{
//		let mut scale=identity3D();
//		scale[2][2] =  z;
//
//		*self = multiply(*self, scale);
//	}
//	//}}}
//	//{{{
//	fn scale(&mut self, x: f64, y:f64, z: f64)
//	{
//		let mut scale=identity3D();
//		scale[0][0] =  x;
//		scale[1][1] =  y;
//		scale[2][2] =  z;
//
//		*self = multiply(*self, scale);
//	}
//	//}}}
//
//	//{{{
//	fn rel_scale_x(&mut self, x: f64)
//	{
//		let mut scale=identity3D();
//		scale[0][0] =  x;
//
//		*self = multiply(scale, *self);
//	}
//	//}}}
//	//{{{
//	fn rel_scale_y(&mut self, y: f64)
//	{
//		let mut scale=identity3D();
//		scale[1][1] =  y;
//
//		*self = multiply(scale, *self);
//	}
//	//}}}
//	//{{{
//	fn rel_scale_z(&mut self, z: f64)
//	{
//		let mut scale=identity3D();
//		scale[2][2] =  z;
//
//		*self = multiply(scale, *self);
//	}
//	//}}}
//	//{{{
//	fn rel_scale(&mut self, x: f64, y:f64, z: f64)
//	{
//		let mut scale=identity3D();
//		scale[0][0] =  x;
//		scale[1][1] =  y;
//		scale[2][2] =  z;
//
//		*self = multiply(scale, *self);
//	}
//	//}}}
//	//}}}
//	//}}}
//
////
////	//{{{ Get 3D Coordinates, Rotation, Scale, Shear
////	
////	// See: https://math.stackexchange.com/questions/237369/given-this-transformation-matrix-how-do-i-decompose-it-into-translation-rotati
////	//{{{ Positions in a MultMatrix
////	//
////	// [ (0,0) (0,1) (0,2) (0,3) ]
////	// [ (1,0) (1,1) (1,2) (1,3) ]
////	// [ (2,0) (2,1) (2,2) (2,3) ]
////	// [ (3,0) (3,1) (3,2) (3,3) ]
////	//}}}
////
////
////	//{{{
////	pub fn get_rotate_x(&mut self) -> f64
////	{
////		0.0
////	}
////	//}}}
////	//{{{
////	pub fn get_rotate_y(&mut self) -> f64
////	{
////		0.0
////	}
////	//}}}
////	//{{{
////	pub fn get_rotate_z(&mut self) -> f64
////	{
////		0.0
////	}
////	//}}}
////	//{{{
////	pub fn get_rotate(&mut self) -> ( f64, f64, f64)
////	{
////		(0.0, 0.0, 0.0)
////	}
////	//}}}
////
////	//{{{
////	pub fn get_translate_x(&mut self) -> f64
////	{
////		self.ref_sys[[0,3]]
////	}
////	//}}}
////	//{{{
////	pub fn get_translate_y(&mut self) -> f64
////	{
////		self.ref_sys[[1,3]]
////	}
////	//}}}
////	//{{{
////	pub fn get_translate_z(&mut self) -> f64
////	{
////		self.ref_sys[[2,3]]
////	}
////	//}}}
////	//{{{
////	pub fn get_translate(&mut self) -> (f64, f64, f64)
////	{
////		(self.get_translate_x(), self.get_translate_y(), self.get_translate_z())
////	}
////	//}}}
////
////	//{{{
////	pub fn get_scale_x(&mut self) -> f64
////	{
////		use ndarray::s;
////		l2_norm(self.ref_sys.slice(s![0, ..]))
////	}
////	//}}}
////	//{{{
////	pub fn get_scale_y(&mut self) -> f64
////	{
////		use ndarray::s;
////		l2_norm(self.ref_sys.slice(s![1, ..]))
////	}
////	//}}}
////	//{{{
////	pub fn get_scale_z(&mut self) -> f64
////	{
////		use ndarray::s;
////		l2_norm(self.ref_sys.slice(s![2, ..]))
////	}
////	//}}}
////	//{{{
////	pub fn get_scale(&mut self) -> (f64, f64, f64)
////	{
////		(self.get_scale_x(), self.get_scale_y(), self.get_scale_z())
////	}
////	//}}}
////	//}}}
////
//
}
//}}}
//}}}


//
//
////{{{
//pub trait RefSysExt
//{
//	fn display(&self, indentation: usize) -> String;
//
//	fn     rotate_x(&mut self, x: f64);
//	fn     rotate_y(&mut self, y: f64);
//	fn     rotate_z(&mut self, z: f64);
//	fn     rotate(  &mut self, x: f64, y: f64, z: f64);
//	fn rel_rotate_x(&mut self, x: f64);
//	fn rel_rotate_y(&mut self, y: f64);
//	fn rel_rotate_z(&mut self, z: f64);
//	fn rel_rotate(  &mut self, x: f64, y: f64, z: f64);
//
//	fn     translate_x(&mut self, x: f64);
//	fn     translate_y(&mut self, y: f64);
//	fn     translate_z(&mut self, z: f64);
//	fn     translate(  &mut self, x: f64, y:f64, z: f64);
//	fn rel_translate_x(&mut self, x: f64);
//	fn rel_translate_y(&mut self, y: f64);
//	fn rel_translate_z(&mut self, z: f64);
//	fn rel_translate(  &mut self, x: f64, y:f64, z: f64);
//
//	fn     scale_x(&mut self, x: f64);
//	fn     scale_y(&mut self, y: f64);
//	fn     scale_z(&mut self, z: f64);
//	fn     scale(  &mut self, x: f64, y:f64, z: f64);
//	fn rel_scale_x(&mut self, x: f64);
//	fn rel_scale_y(&mut self, y: f64);
//	fn rel_scale_z(&mut self, z: f64);
//	fn rel_scale(  &mut self, x: f64, y:f64, z: f64);
//
//
//	//fn get_rotate_x(&mut self) -> f64;
//	//fn get_rotate_y(&mut self) -> f64;
//	//fn get_rotate_z(&mut self) -> f64;
//	//fn get_rotate(  &mut self) -> ( f64, f64, f64);
//
//	//fn get_translate_x(&mut self) -> f64;
//	//fn get_translate_y(&mut self) -> f64;
//	//fn get_translate_z(&mut self) -> f64;
//	//fn get_translate(&mut self) -> (f64, f64, f64);
//
//	//fn get_scale_x(&mut self) -> f64;
//	//fn get_scale_y(&mut self) -> f64;
//	//fn get_scale_z(&mut self) -> f64;
//	//fn get_scale(  &mut self) -> (f64, f64, f64);
//}
////}}}
//
////{{{
//impl RefSysExt for Matrix3D
//{
//	//{{{
//	fn display(&self, indentation: usize) -> String
//	{
//		let indent   = "\t".repeat(indentation as usize);
//		( format!("{}[{:16.10?},\n", indent, self[0])
//		+&format!("{} {:16.10?},\n", indent, self[1])
//		+&format!("{} {:16.10?},\n", indent, self[2])
//		+&format!("{} {:16.10?}]",   indent, self[3]))
//	}
//	//}}}
//
//	////{{{
//	//fn l2_norm(x: ndarray::ArrayView1<f64>) -> f64
//	//{
//	//	// Taken from:  https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/linear_algebra.html
//	//	//x.dot(&x).sqrt()
//	//	0.0
//	//}
//	////}}}
//
//	//{{{ Positions in a MultMatrix
//	//
//	// [ (0,0) (0,1) (0,2) (0,3) ]
//	// [ (1,0) (1,1) (1,2) (1,3) ]
//	// [ (2,0) (2,1) (2,2) (2,3) ]
//	// [ (3,0) (3,1) (3,2) (3,3) ]
//	//}}}
//
//	//{{{ 3D Manipulations
//	//{{{Rotations
//	//{{{
//	fn rotate_x(&mut self, x: f64)
//	{
//		let x = x.to_radians();
//		let mut rotation=identity3D();
//		rotation[1][1] =  x.cos();
//		rotation[1][2] = -x.sin();
//		rotation[2][1] =  x.sin();
//		rotation[2][2] =  x.cos();
//
//		*self = multiply(rotation, *self);
//	}
//	//}}}
//	//{{{
//	fn rotate_y(&mut self, y: f64)
//	{
//		let y = y.to_radians();
//		let mut rotation=identity3D();
//		rotation[0][0] =  y.cos();
//		rotation[0][2] =  y.sin();
//		rotation[2][0] = -y.sin();
//		rotation[2][2] =  y.cos();
//
//		*self = multiply(rotation, *self);
//	}
//	//}}}
//	//{{{
//	fn rotate_z(&mut self, z: f64)
//	{
//		let z = z.to_radians();
//		let mut rotation=identity3D();
//		rotation[0][0] =  z.cos();
//		rotation[0][1] = -z.sin();
//		rotation[1][0] =  z.sin();
//		rotation[1][1] =  z.cos();
//
//		*self = multiply(rotation, *self);
//	}
//	//}}}
//	//{{{
//	fn rotate(&mut self, x: f64, y: f64, z: f64)
//	{
//		self.rotate_x(x);
//		self.rotate_y(y);
//		self.rotate_z(z);
//	}
//	//}}}
//
//	//{{{
//	fn rel_rotate_x(&mut self, x: f64)
//	{
//		let x = x.to_radians();
//		let mut rotation=identity3D();
//		rotation[1][1] =  x.cos();
//		rotation[1][2] = -x.sin();
//		rotation[2][1] =  x.sin();
//		rotation[2][2] =  x.cos();
//
//		*self = multiply(*self, rotation);
//	}
//	//}}}
//	//{{{
//	fn rel_rotate_y(&mut self, y: f64)
//	{
//		let y = y.to_radians();
//		let mut rotation=identity3D();
//		rotation[0][0] =  y.cos();
//		rotation[0][2] =  y.sin();
//		rotation[2][0] = -y.sin();
//		rotation[2][2] =  y.cos();
//
//		*self = multiply(*self, rotation);
//	}
//	//}}}
//	//{{{
//	fn rel_rotate_z(&mut self, z: f64)
//	{
//		let z = z.to_radians();
//		let mut rotation=identity3D();
//		rotation[0][0] =  z.cos();
//		rotation[0][1] = -z.sin();
//		rotation[1][0] =  z.sin();
//		rotation[1][1] =  z.cos();
//
//		*self = multiply(*self, rotation);
//	}
//	//}}}
//	//{{{
//	fn rel_rotate(&mut self, x: f64, y: f64, z: f64)
//	{
//		self.rel_rotate_x(x);
//		self.rel_rotate_y(y);
//		self.rel_rotate_z(z);
//	}
//	//}}}
//	//}}}
//
//	//{{{Translations
//	//{{{
//	fn translate_x(&mut self, x: f64)
//	{
//		let mut translation=identity3D();
//		translation[0][3] =  x;
//
//		*self = multiply(translation, *self);
//	}
//	//}}}
//	//{{{
//	fn translate_y(&mut self, y: f64)
//	{
//		let mut translation=identity3D();
//		translation[1][3] =  y;
//
//		*self = multiply(translation, *self);
//	}
//	//}}}
//	//{{{
//	fn translate_z(&mut self, z: f64)
//	{
//		let mut translation=identity3D();
//		translation[2][3] =  z;
//
//		*self = multiply(translation, *self);
//	}
//	//}}}
//	//{{{
//	fn translate(&mut self, x: f64, y:f64, z: f64)
//	{
//		let mut translation=identity3D();
//		translation[0][3] =  x;
//		translation[1][3] =  y;
//		translation[2][3] =  z;
//
//		*self = multiply(*self, translation);
//	}
//	//}}}
//
//	//{{{
//	fn rel_translate_x(&mut self, x: f64)
//	{
//		let mut translation=identity3D();
//		translation[0][3] =  x;
//
//		*self = multiply(*self, translation);
//	}
//	//}}}
//	//{{{
//	fn rel_translate_y(&mut self, y: f64)
//	{
//		let mut translation=identity3D();
//		translation[1][3] =  y;
//
//		*self = multiply(*self, translation);
//	}
//	//}}}
//	//{{{
//	fn rel_translate_z(&mut self, z: f64)
//	{
//		let mut translation=identity3D();
//		translation[2][3] =  z;
//
//		*self = multiply(*self, translation);
//	}
//	//}}}
//	//{{{
//	fn rel_translate(&mut self, x: f64, y:f64, z: f64)
//	{
//		let mut translation=identity3D();
//		translation[0][3] =  x;
//		translation[1][3] =  y;
//		translation[2][3] =  z;
//
//		*self = multiply(translation, *self);
//	}
//	//}}}
//	//}}}
//
//	//{{{ Scale
//	//{{{
//	fn scale_x(&mut self, x: f64)
//	{
//		let mut scale=identity3D();
//		scale[0][0] =  x;
//
//		*self = multiply(*self, scale);
//	}
//	//}}}
//	//{{{
//	fn scale_y(&mut self, y: f64)
//	{
//		let mut scale=identity3D();
//		scale[1][1] =  y;
//
//		*self = multiply(*self, scale);
//	}
//	//}}}
//	//{{{
//	fn scale_z(&mut self, z: f64)
//	{
//		let mut scale=identity3D();
//		scale[2][2] =  z;
//
//		*self = multiply(*self, scale);
//	}
//	//}}}
//	//{{{
//	fn scale(&mut self, x: f64, y:f64, z: f64)
//	{
//		let mut scale=identity3D();
//		scale[0][0] =  x;
//		scale[1][1] =  y;
//		scale[2][2] =  z;
//
//		*self = multiply(*self, scale);
//	}
//	//}}}
//
//	//{{{
//	fn rel_scale_x(&mut self, x: f64)
//	{
//		let mut scale=identity3D();
//		scale[0][0] =  x;
//
//		*self = multiply(scale, *self);
//	}
//	//}}}
//	//{{{
//	fn rel_scale_y(&mut self, y: f64)
//	{
//		let mut scale=identity3D();
//		scale[1][1] =  y;
//
//		*self = multiply(scale, *self);
//	}
//	//}}}
//	//{{{
//	fn rel_scale_z(&mut self, z: f64)
//	{
//		let mut scale=identity3D();
//		scale[2][2] =  z;
//
//		*self = multiply(scale, *self);
//	}
//	//}}}
//	//{{{
//	fn rel_scale(&mut self, x: f64, y:f64, z: f64)
//	{
//		let mut scale=identity3D();
//		scale[0][0] =  x;
//		scale[1][1] =  y;
//		scale[2][2] =  z;
//
//		*self = multiply(scale, *self);
//	}
//	//}}}
//	//}}}
//	//}}}
//
////
////	//{{{ Get 3D Coordinates, Rotation, Scale, Shear
////	
////	// See: https://math.stackexchange.com/questions/237369/given-this-transformation-matrix-how-do-i-decompose-it-into-translation-rotati
////	//{{{ Positions in a MultMatrix
////	//
////	// [ (0,0) (0,1) (0,2) (0,3) ]
////	// [ (1,0) (1,1) (1,2) (1,3) ]
////	// [ (2,0) (2,1) (2,2) (2,3) ]
////	// [ (3,0) (3,1) (3,2) (3,3) ]
////	//}}}
////
////
////	//{{{
////	pub fn get_rotate_x(&mut self) -> f64
////	{
////		0.0
////	}
////	//}}}
////	//{{{
////	pub fn get_rotate_y(&mut self) -> f64
////	{
////		0.0
////	}
////	//}}}
////	//{{{
////	pub fn get_rotate_z(&mut self) -> f64
////	{
////		0.0
////	}
////	//}}}
////	//{{{
////	pub fn get_rotate(&mut self) -> ( f64, f64, f64)
////	{
////		(0.0, 0.0, 0.0)
////	}
////	//}}}
////
////	//{{{
////	pub fn get_translate_x(&mut self) -> f64
////	{
////		self.ref_sys[[0,3]]
////	}
////	//}}}
////	//{{{
////	pub fn get_translate_y(&mut self) -> f64
////	{
////		self.ref_sys[[1,3]]
////	}
////	//}}}
////	//{{{
////	pub fn get_translate_z(&mut self) -> f64
////	{
////		self.ref_sys[[2,3]]
////	}
////	//}}}
////	//{{{
////	pub fn get_translate(&mut self) -> (f64, f64, f64)
////	{
////		(self.get_translate_x(), self.get_translate_y(), self.get_translate_z())
////	}
////	//}}}
////
////	//{{{
////	pub fn get_scale_x(&mut self) -> f64
////	{
////		use ndarray::s;
////		l2_norm(self.ref_sys.slice(s![0, ..]))
////	}
////	//}}}
////	//{{{
////	pub fn get_scale_y(&mut self) -> f64
////	{
////		use ndarray::s;
////		l2_norm(self.ref_sys.slice(s![1, ..]))
////	}
////	//}}}
////	//{{{
////	pub fn get_scale_z(&mut self) -> f64
////	{
////		use ndarray::s;
////		l2_norm(self.ref_sys.slice(s![2, ..]))
////	}
////	//}}}
////	//{{{
////	pub fn get_scale(&mut self) -> (f64, f64, f64)
////	{
////		(self.get_scale_x(), self.get_scale_y(), self.get_scale_z())
////	}
////	//}}}
////	//}}}
////
//
//}
////}}}
//
//
//
//
////You can impl Deref, Borrow and AsRef, which should cover all needs at callsite.
