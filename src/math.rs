pub use core::borrow::{Borrow, BorrowMut};
pub use std::convert::{AsRef, AsMut};
pub use std::ops::{Deref, DerefMut, Not, Add, Sub, Mul, BitXor};
use std::fmt;


//{{{ 2D Stuff

//{{{ pub struct Matrix2D

#[derive(Default, Debug,  Clone, Copy)]
pub struct Matrix2D(M2D);
pub type M2D = vecmath::Matrix3<f64>;
//{{{
impl Deref for Matrix2D
{
    type Target = M2D;

    #[inline]
    fn deref(&self) -> &M2D
	{
        &self.0
    }
}
//}}}
//{{{
impl DerefMut for Matrix2D
{
    #[inline]
    fn deref_mut(&mut self) -> &mut M2D
	{
        &mut self.0
    }
}
//}}}
//{{{
impl Borrow<M2D> for Matrix2D
{
    #[inline]
    fn borrow(&self) -> &M2D
	{
        &self.0
    }
}
//}}}
//{{{
impl BorrowMut<M2D> for Matrix2D
{
    #[inline]
    fn borrow_mut(&mut self) -> &mut M2D
	{
        &mut self.0
    }
}
//}}}
//{{{
impl AsRef<M2D> for Matrix2D
{
    #[inline]
    fn as_ref(&self) -> &M2D
	{
        &self.0
    }
}
//}}}
//{{{
impl AsMut<M2D> for Matrix2D
{
    fn as_mut(&mut self) -> &mut M2D {
        &mut self.0
    }
}
//}}}

//{{{
impl fmt::Display for Matrix2D
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		let indentation = if let Some(width) = f.width() { width } else { 0 as usize };
		write!(f, "{0}[{1:16.10?},\n{0} {2:16.10?},\n{0} {3:16.10?}]", "\t".repeat(indentation as usize), self[0], self[1], self[2])
	}
}
//}}}

//{{{
impl Add<&Matrix2D> for Matrix2D
{
    type Output = Self;

    fn add(self, other: &Matrix2D) -> Matrix2D
	{
		Self(vecmath::mat3_add(**other, *self))
    }
}
//}}}
//{{{
impl Sub<&Matrix2D> for Matrix2D
{
    type Output = Self;

    fn sub(self, other: &Matrix2D) -> Matrix2D
	{
		Self(vecmath::mat3_sub(**other, *self))
    }
}
//}}}
//{{{
impl Mul<f64> for Matrix2D
{
    type Output = Self;

    fn mul(self, a: f64) -> Matrix2D
	{
		Self(
		[[ a*self[0][0], a*self[0][1], a*self[0][2]],
		 [ a*self[1][0], a*self[1][1], a*self[1][2]],
		 [ a*self[2][0], a*self[2][1], a*self[2][2]]])
    }
}
//}}}
//{{{
impl Mul<&Matrix2D> for Matrix2D
{
    type Output = Self;

    fn mul(self, other: &Matrix2D) -> Matrix2D
	{
		Self(vecmath::row_mat3_mul(**other, *self))
    }
}
//}}}
//{{{
impl Mul<Matrix2D> for Matrix2D
{
    type Output = Self;

    fn mul(self, other: Matrix2D) -> Matrix2D
	{
		Self(vecmath::row_mat3_mul(*other, *self))
    }
}
//}}}
//{{{
impl Not for Matrix2D
{
    type Output = Self;

    fn not(self) -> Self
	{
		Self(vecmath::mat3_inv(*self))
    }
}
//}}}

//{{{
impl Matrix2D
{
	//{{{
	pub fn identity() -> Matrix2D
	{
		Self(
		[[ 1.0, 0.0, 0.0],
		 [ 0.0, 1.0, 0.0],
		 [ 0.0, 0.0, 1.0]])
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
	pub fn row(&self, i: i32) -> Vector2D
	{
		let i = i as usize;
		Vector2D([self[i][0], self[i][1], self[i][2]])
	}
	//}}}

	//{{{
	pub fn column(&self, i: i32) -> Vector2D
	{
		let i = i as usize;
		Vector2D([self[0][i], self[1][i], self[2][i]])
	}
	//}}}

	//{{{ Positions in a MultMatrix
	//
	// [ (0,0) (0,1) (0,2) (0,3) ]
	// [ (1,0) (1,1) (1,2) (1,3) ]
	// [ (2,0) (2,1) (2,2) (2,3) ]
	// [ (3,0) (3,1) (3,2) (3,3) ]
	//}}}

	//{{{ 2D Manipulations

	//{{{Rotations
	//{{{
	pub fn rotate(&mut self, x: f64)
	{
		let x = x.to_radians();
		let mut rotation=Self::identity();
		rotation[0][0] =  x.cos();
		rotation[0][1] = -x.sin();
		rotation[1][0] =  x.sin();
		rotation[1][1] =  x.cos();

		*self = *self * rotation;
	}
	//}}}

	//{{{
	pub fn rel_rotate(&mut self, x: f64)
	{
		let x = x.to_radians();
		let mut rotation=Self::identity();
		rotation[0][0] =  x.cos();
		rotation[0][1] = -x.sin();
		rotation[1][0] =  x.sin();
		rotation[1][1] =  x.cos();

		*self = rotation * *self;
	}
	//}}}
	//}}}

	//{{{Translations
	//{{{
	pub fn translate_x(&mut self, x: f64)
	{
		let mut translation=Self::identity();
		translation[0][2] =  x;

		*self = *self * translation;
	}
	//}}}
	//{{{
	pub fn translate_y(&mut self, y: f64)
	{
		let mut translation=Self::identity();
		translation[1][2] =  y;

		*self = *self * translation;
	}
	//}}}
	//{{{
	pub fn translate(&mut self, x: f64, y:f64)
	{
		let mut translation=Self::identity();
		translation[0][2] =  x;
		translation[1][2] =  y;

		*self = *self * translation;
	}
	//}}}

	//{{{
	pub fn rel_translate_x(&mut self, x: f64)
	{
		let mut translation=Self::identity();
		translation[0][2] =  x;

		*self = translation * *self;
	}
	//}}}
	//{{{
	pub fn rel_translate_y(&mut self, y: f64)
	{
		let mut translation=Self::identity();
		translation[1][2] =  y;

		*self = translation * *self;
	}
	//}}}
	//{{{
	pub fn rel_translate(&mut self, x: f64, y:f64)
	{
		let mut translation=Self::identity();
		translation[0][2] =  x;
		translation[1][2] =  y;

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
	pub fn scale(&mut self, x: f64, y:f64)
	{
		let mut scale=Self::identity();
		scale[0][0] =  x;
		scale[1][1] =  y;

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
	pub fn rel_scale(&mut self, x: f64, y:f64)
	{
		let mut scale=Self::identity();
		scale[0][0] =  x;
		scale[1][1] =  y;

		*self = scale * *self;
	}
	//}}}
	//}}}
	//}}}


	//{{{ Get 2D Coordinates, Rotation, Scale, Shear
	
	// See: https://math.stackexchange.com/questions/237369/given-this-transformation-matrix-how-do-i-decompose-it-into-translation-rotati
	//{{{ Positions in a MultMatrix
	//
	// [ (0,0) (0,1) (0,2) (0,3) ]
	// [ (1,0) (1,1) (1,2) (1,3) ]
	// [ (2,0) (2,1) (2,2) (2,3) ]
	// [ (3,0) (3,1) (3,2) (3,3) ]
	//}}}


	//{{{
	pub fn get_rotate(&self) -> f64
	{
		(self[0][0]/self.get_scale_x()).acos().to_degrees()
	}
	//}}}

	//{{{
	pub fn get_translate_x(&self) -> f64
	{
		self[0][2]
	}
	//}}}
	//{{{
	pub fn get_translate_y(&self) -> f64
	{
		self[1][2]
	}
	//}}}
	//{{{
	pub fn get_translate(&self) -> (f64, f64)
	{
		(self.get_translate_x(), self.get_translate_y())
	}
	//}}}

	//{{{
	pub fn get_scale_x(&self) -> f64
	{
		self.column(0).l2_norm().sqrt()
	}
	//}}}
	//{{{
	pub fn get_scale_y(&self) -> f64
	{
		self.column(1).l2_norm().sqrt()
	}
	//}}}
	//{{{
	pub fn get_scale(&self) -> (f64, f64)
	{
		(self.get_scale_x(), self.get_scale_y())
	}
	//}}}
	//}}}
}
//}}}
//}}}

//{{{ pub struct Vector2D

#[derive(Clone, Copy, Debug)]
pub struct Vector2D(pub V2D);
pub type Point2D=Vector2D; 
pub type V2D = vecmath::Vector3<f64>;
//{{{
impl Deref for Vector2D
{
    type Target = V2D;

    #[inline]
    fn deref(&self) -> &V2D
	{
        &self.0
    }
}
//}}}
//{{{
impl Borrow<V2D> for Vector2D
{
    #[inline]
    fn borrow(&self) -> &V2D
	{
        &self.0
    }
}
//}}}
//{{{
impl BorrowMut<V2D> for Vector2D
{
    #[inline]
    fn borrow_mut(&mut self) -> &mut V2D
	{
        &mut self.0
    }
}
//}}}
//{{{
impl AsRef<V2D> for Vector2D
{
    #[inline]
    fn as_ref(&self) -> &V2D
	{
        &self.0
    }
}
//}}}
//{{{
impl AsMut<V2D> for Vector2D
{
    fn as_mut(&mut self) -> &mut V2D {
        &mut self.0
    }
}
//}}}

//{{{
impl fmt::Display for Vector2D
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		let indentation = if let Some(width) = f.width() { width } else { 0 as usize };
		if f.alternate()
		{
			write!(f, "{0}[{1:16.10?}, {2:16.10?}, {3:16.10?}]", "\t".repeat(indentation as usize), self[0], self[1], self[2])
		} else {
			write!(f, "{0}[{1:16.10?}, {2:16.10?}]",             "\t".repeat(indentation as usize), self[0], self[1])
		}

	}
}
//}}}

//{{{
impl Add<&Vector2D> for Vector2D
{
    type Output = Self;

    fn add(self, other: &Self) -> Self
	{
		Self(vecmath::vec3_add(**other, *self))
    }
}
//}}}
//{{{
impl Sub<&Vector2D> for Vector2D
{
    type Output = Self;

    fn sub(self, other: &Self) -> Self
	{
		Self(vecmath::vec3_sub(**other, *self))
    }
}
//}}}
//{{{
impl Mul<f64> for Vector2D
{
    type Output = Self;

    fn mul(self, a: f64) -> Vector2D
	{
		Self([ a*self[0], a*self[1], a*self[2]])
    }
}
//}}}
//{{{
impl Mul<Vector2D> for Vector2D
{
    type Output = f64;

    fn mul(self, other: Vector2D) -> f64
	{
		self[0]*other[0] + self[1]*other[1] + self[2]*other[2]
    }
}
//}}}

//{{{
impl Vector2D
{
	//{{{
	pub fn l2_norm(&self) -> f64
	{
		vecmath::vec3_dot(**self, **self)
	}
	//}}}
}
//}}}
//{{{
//pub fn point2D(x: f64, y: f64) -> Point2D
#[allow(non_snake_case)]
pub fn point2D(x: f64, y: f64) -> Point2D
{
	Vector2D([x, y, 1.0])
}
//}}}
//{{{
//pub fn vector2D(x: f64, y: f64) -> Point2D
#[allow(non_snake_case)]
pub fn vector2D(x: f64, y: f64) -> Point2D
{
	Vector2D([x, y, 0.0])
}
//}}}
//}}}

//{{{
pub trait HasRefSys2D
{
	fn ref_sys_mut(&mut self) -> &mut Matrix2D;
	fn ref_sys(&self) -> &Matrix2D;
	fn set_ref_sys(&mut self, ref_sys: Matrix2D);
}
//}}}

//{{{
pub trait Is2DObject
{
	fn rotate(&mut self, x: f64);
	fn rel_rotate(&mut self, x: f64);
	fn translate_x(&mut self, x: f64);
	fn translate_y(&mut self, y: f64);
	fn translate(&mut self, x: f64, y:f64);
	fn rel_translate_x(&mut self, x: f64);
	fn rel_translate_y(&mut self, y: f64);
	fn rel_translate(&mut self, x: f64, y:f64);
	fn scale_x(&mut self, x: f64);
	fn scale_y(&mut self, y: f64);
	fn scale(&mut self, x: f64, y:f64);
	fn rel_scale_x(&mut self, x: f64);
	fn rel_scale_y(&mut self, y: f64);
	fn rel_scale(&mut self, x: f64, y:f64);

	fn get_rotate(&self) -> f64;
	fn get_translate_x(&self) -> f64;
	fn get_translate_y(&self) -> f64;
	fn get_translate(&self) -> (f64, f64);
	fn get_scale_x(&self) -> f64;
	fn get_scale_y(&self) -> f64;
	fn get_scale(&self) -> (f64, f64);
}
//}}}

//{{{
impl<T> Is2DObject for T where T: HasRefSys2D
{
	//{{{ 2D-Manipulation

	//{{{
	fn rotate(&mut self, x: f64)
	{
		self.ref_sys_mut().rotate(x);
	}
	//}}}

	//{{{
	fn rel_rotate(&mut self, x: f64)
	{
		self.ref_sys_mut().rel_rotate(x);
	}
	//}}}


	//{{{
	fn translate_x(&mut self, x: f64)
	{
		self.ref_sys_mut().translate_x(x);
	}
	//}}}
	//{{{
	fn translate_y(&mut self, y: f64)
	{
		self.ref_sys_mut().translate_y(y);
	}
	//}}}
	//{{{
	fn translate(&mut self, x: f64, y:f64)
	{
		self.ref_sys_mut().translate(x, y);
	}
	//}}}

	//{{{
	fn rel_translate_x(&mut self, x: f64)
	{
		self.ref_sys_mut().rel_translate_x(x);
	}
	//}}}
	//{{{
	fn rel_translate_y(&mut self, y: f64)
	{
		self.ref_sys_mut().rel_translate_y(y);
	}
	//}}}
	//{{{
	fn rel_translate(&mut self, x: f64, y:f64)
	{
		self.ref_sys_mut().rel_translate(x, y);
	}
	//}}}


	//{{{
	fn scale_x(&mut self, x: f64)
	{
		self.ref_sys_mut().scale_x(x);
	}
	//}}}
	//{{{
	fn scale_y(&mut self, y: f64)
	{
		self.ref_sys_mut().scale_y(y);
	}
	//}}}
	//{{{
	fn scale(&mut self, x: f64, y:f64)
	{
		self.ref_sys_mut().scale(x, y);
	}
	//}}}
	//{{{
	fn rel_scale_x(&mut self, x: f64)
	{
		self.ref_sys_mut().rel_scale_x(x);
	}
	//}}}
	//{{{
	fn rel_scale_y(&mut self, y: f64)
	{
		self.ref_sys_mut().rel_scale_y(y);
	}
	//}}}
	//{{{
	fn rel_scale(&mut self, x: f64, y:f64)
	{
		self.ref_sys_mut().rel_scale(x, y);
	}
	//}}}
	//}}}

	//{{{ Get 2D Coordinates, Rotation, Scale, Shear
	
	// See: https://math.stackexchange.com/questions/237369/given-this-transformation-matrix-how-do-i-decompose-it-into-translation-rotati
	//{{{ Positions in a MultMatrix
	//
	// [ (0,0) (0,1) (0,2) (0,3) ]
	// [ (1,0) (1,1) (1,2) (1,3) ]
	// [ (2,0) (2,1) (2,2) (2,3) ]
	// [ (3,0) (3,1) (3,2) (3,3) ]
	//}}}


	//{{{
	fn get_rotate(&self) -> f64
	{
		self.ref_sys().get_rotate()
	}
	//}}}

	//{{{
	fn get_translate_x(&self) -> f64
	{
		self.ref_sys().get_translate_x()
	}
	//}}}
	//{{{
	fn get_translate_y(&self) -> f64
	{
		self.ref_sys().get_translate_y()
	}
	//}}}
	//{{{
	fn get_translate(&self) -> (f64, f64)
	{
		self.ref_sys().get_translate()
	}
	//}}}

	//{{{
	fn get_scale_x(&self) -> f64
	{
		self.ref_sys().get_scale_x()
	}
	//}}}
	//{{{
	fn get_scale_y(&self) -> f64
	{
		self.ref_sys().get_scale_y()
	}
	//}}}
	//{{{
	fn get_scale(&self) -> (f64, f64)
	{
		self.ref_sys().get_scale()
	}
	//}}}
	//}}}
}
//}}}

//}}}



//{{{ 3D Stuff

//{{{ pub struct Matrix3D

#[derive(Default, Debug, Clone, Copy)]
pub struct Matrix3D(M3D);
pub type M3D = vecmath::Matrix4<f64>;
//{{{
impl Deref for Matrix3D
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
impl DerefMut for Matrix3D
{
    #[inline]
    fn deref_mut(&mut self) -> &mut M3D
	{
        &mut self.0
    }
}
//}}}
//{{{
impl Borrow<M3D> for Matrix3D
{
    #[inline]
    fn borrow(&self) -> &M3D
	{
        &self.0
    }
}
//}}}
//{{{
impl BorrowMut<M3D> for Matrix3D
{
    #[inline]
    fn borrow_mut(&mut self) -> &mut M3D
	{
        &mut self.0
    }
}
//}}}
//{{{
impl AsRef<M3D> for Matrix3D
{
    #[inline]
    fn as_ref(&self) -> &M3D
	{
        &self.0
    }
}
//}}}
//{{{
impl AsMut<M3D> for Matrix3D {
    fn as_mut(&mut self) -> &mut M3D {
        &mut self.0
    }
}
//}}}

//{{{
impl fmt::Display for Matrix3D
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		let indentation = if let Some(width) = f.width() { width } else { 0 as usize };
		write!(f, "{0}[{1:16.10?},\n{0} {2:16.10?},\n{0} {3:16.10?},\n{0} {4:16.10?}]", "\t".repeat(indentation as usize), self[0], self[1], self[2], self[3])
	}
}
//}}}

//{{{
impl Add<&Matrix3D> for Matrix3D
{
    type Output = Self;

    fn add(self, other: &Matrix3D) -> Matrix3D
	{
		Self(vecmath::mat4_add(**other, *self))
    }
}
//}}}
//{{{
impl Sub<&Matrix3D> for Matrix3D
{
    type Output = Self;

    fn sub(self, other: &Matrix3D) -> Matrix3D
	{
		Self(vecmath::mat4_sub(**other, *self))
    }
}
//}}}
//{{{
impl Mul<f64> for Matrix3D
{
    type Output = Self;

    fn mul(self, a: f64) -> Matrix3D
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
impl Mul<&Matrix3D> for Matrix3D
{
    type Output = Self;

    fn mul(self, other: &Matrix3D) -> Matrix3D
	{
		Self(vecmath::row_mat4_mul(**other, *self))
    }
}
//}}}
//{{{
impl Mul<Matrix3D> for Matrix3D
{
    type Output = Self;

    fn mul(self, other: Matrix3D) -> Matrix3D
	{
		Self(vecmath::row_mat4_mul(*other, *self))
    }
}
//}}}
//{{{
impl Not for Matrix3D
{
    type Output = Self;

    fn not(self) -> Self
	{
		Self(vecmath::mat4_inv(*self))
    }
}
//}}}

//{{{
impl Matrix3D
{
	//{{{
	pub fn identity() -> Matrix3D
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
	pub fn row(&self, i: i32) -> Vector3D
	{
		let i = i as usize;
		Vector3D([self[i][0], self[i][1], self[i][2], self[i][3]])
	}
	//}}}

	//{{{
	pub fn column(&self, i: i32) -> Vector3D
	{
		let i = i as usize;
		Vector3D([self[0][i], self[1][i], self[2][i], self[3][i]])
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
	pub fn get_rotate_x(&self) -> f64
	{
		(self[1][2]/self.get_scale_y()).asin().to_degrees()
	}
	//}}}
	//{{{
	pub fn get_rotate_y(&self) -> f64
	{
		(self[2][0]/self.get_scale_z()).asin().to_degrees()
	}
	//}}}
	//{{{
	pub fn get_rotate_z(&self) -> f64
	{
		(self[0][1]/self.get_scale_x()).asin().to_degrees()
	}
	//}}}
	//{{{
	pub fn get_rotate(&self) -> ( f64, f64, f64)
	{
		(self.get_rotate_x(), self.get_rotate_y(), self.get_rotate_z())
	}
	//}}}

	//{{{
	pub fn get_translate_x(&self) -> f64
	{
		self[0][3]
	}
	//}}}
	//{{{
	pub fn get_translate_y(&self) -> f64
	{
		self[1][3]
	}
	//}}}
	//{{{
	pub fn get_translate_z(&self) -> f64
	{
		self[2][3]
	}
	//}}}
	//{{{
	pub fn get_translate(&self) -> (f64, f64, f64)
	{
		(self.get_translate_x(), self.get_translate_y(), self.get_translate_z())
	}
	//}}}

	//{{{
	pub fn get_scale_x(&self) -> f64
	{
		self.column(0).l2_norm().sqrt()
	}
	//}}}
	//{{{
	pub fn get_scale_y(&self) -> f64
	{
		self.column(1).l2_norm().sqrt()
	}
	//}}}
	//{{{
	pub fn get_scale_z(&self) -> f64
	{
		self.column(2).l2_norm().sqrt()
	}
	//}}}
	//{{{
	pub fn get_scale(&self) -> (f64, f64, f64)
	{
		(self.get_scale_x(), self.get_scale_y(), self.get_scale_z())
	}
	//}}}
	//}}}
}
//}}}
//}}}

//{{{ pub struct Vector3D

#[derive(Clone, Copy, Debug)]
pub struct Vector3D(V3D);
pub type Point3D=Vector3D; 
pub type V3D = vecmath::Vector4<f64>;
//{{{
impl Deref for Vector3D
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
impl Borrow<V3D> for Vector3D
{
    #[inline]
    fn borrow(&self) -> &V3D
	{
        &self.0
    }
}
//}}}
//{{{
impl BorrowMut<V3D> for Vector3D
{
    #[inline]
    fn borrow_mut(&mut self) -> &mut V3D
	{
        &mut self.0
    }
}
//}}}
//{{{
impl AsRef<V3D> for Vector3D
{
    #[inline]
    fn as_ref(&self) -> &V3D
	{
        &self.0
    }
}
//}}}
//{{{
impl AsMut<V3D> for Vector3D {
    fn as_mut(&mut self) -> &mut V3D {
        &mut self.0
    }
}
//}}}

//{{{
impl fmt::Display for Vector3D
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		let indentation = if let Some(width) = f.width() { width } else { 0 as usize };
		//write!(f, "{0}[{1:16.10?},\n{0} {2:16.10?},\n{0} {3:16.10?},\n{0} {4:16.10?}]", "\t".repeat(indentation as usize), self[0], self[1], self[2], self[3])
		if !f.alternate() {
			write!(f, "{0}[{1:16.10?}, {2:16.10?}, {3:16.10?},\n{0} {4:16.10?}]", "\t".repeat(indentation as usize), self[0], self[1], self[2], self[3])
		} else {
			write!(f, "{0}[{1:16.10?}, {2:16.10?}, {3:16.10?}]",                  "\t".repeat(indentation as usize), self[0], self[1], self[2])
		}
	}
}
//}}}

//{{{
impl Add<&Vector3D> for Vector3D
{
    type Output = Self;

    fn add(self, other: &Self) -> Self
	{
		Self(vecmath::vec4_add(**other, *self))
    }
}
//}}}
//{{{
impl Sub<&Vector3D> for Vector3D
{
    type Output = Self;

    fn sub(self, other: &Self) -> Self
	{
		Self(vecmath::vec4_sub(**other, *self))
    }
}
//}}}
//{{{
impl Mul<f64> for Vector3D
{
    type Output = Self;

    fn mul(self, a: f64) -> Vector3D
	{
		Self([ a*self[0], a*self[1], a*self[2], a*self[3]])
    }
}
//}}}
//{{{
impl Mul<Vector3D> for Vector3D
{
    type Output = f64;

    fn mul(self, other: Vector3D) -> f64
	{
		self[0]*other[0] + self[1]*other[1] + self[2]*other[2] + self[3]*other[3]
    }
}
//}}}

//{{{
impl Vector3D
{
	//{{{
	pub fn l2_norm(&self) -> f64
	{
		vecmath::vec4_dot(**self, **self)
	}
	//}}}
}
//}}}
//{{{
//pub fn point3D(x: f64, y: f64, z: f64) -> Point3D
#[allow(non_snake_case)]
pub fn point3D(x: f64, y: f64, z: f64) -> Point3D
{
	Vector3D([x, y, z, 1.0])
}
//}}}
//{{{
//pub fn vector3D(x: f64, y: f64, z: f64) -> Point3D
#[allow(non_snake_case)]
pub fn vector3D(x: f64, y: f64, z: f64) -> Point3D
{
	Vector3D([x, y, z, 0.0])
}
//}}}
//}}}

//{{{
pub trait HasRefSys3D
{
	fn ref_sys_mut(&mut self) -> &mut Matrix3D;
	fn ref_sys(&self) -> &Matrix3D;
	fn set_ref_sys(&mut self, ref_sys: Matrix3D);
}
//}}}

//{{{
pub trait Is3DObject
{
	fn rotate_x(&mut self, x: f64);
	fn rotate_y(&mut self, y: f64);
	fn rotate_z(&mut self, z: f64);
	fn rotate(&mut self, x: f64, y: f64, z: f64);
	fn rel_rotate_x(&mut self, x: f64);
	fn rel_rotate_y(&mut self, y: f64);
	fn rel_rotate_z(&mut self, z: f64);
	fn rel_rotate(&mut self, x: f64, y: f64, z: f64);
	fn translate_x(&mut self, x: f64);
	fn translate_y(&mut self, y: f64);
	fn translate_z(&mut self, z: f64);
	fn translate(&mut self, x: f64, y:f64, z: f64);
	fn rel_translate_x(&mut self, x: f64);
	fn rel_translate_y(&mut self, y: f64);
	fn rel_translate_z(&mut self, z: f64);
	fn rel_translate(&mut self, x: f64, y:f64, z: f64);
	fn scale_x(&mut self, x: f64);
	fn scale_y(&mut self, y: f64);
	fn scale_z(&mut self, z: f64);
	fn scale(&mut self, x: f64, y:f64, z: f64);
	fn rel_scale_x(&mut self, x: f64);
	fn rel_scale_y(&mut self, y: f64);
	fn rel_scale_z(&mut self, z: f64);
	fn rel_scale(&mut self, x: f64, y:f64, z: f64);

	fn get_rotate_x(&self) -> f64;
	fn get_rotate_y(&self) -> f64;
	fn get_rotate_z(&self) -> f64;
	fn get_rotate(&self) -> ( f64, f64, f64);
	fn get_translate_x(&self) -> f64;
	fn get_translate_y(&self) -> f64;
	fn get_translate_z(&self) -> f64;
	fn get_translate(&self) -> (f64, f64, f64);
	fn get_scale_x(&self) -> f64;
	fn get_scale_y(&self) -> f64;
	fn get_scale_z(&self) -> f64;
	fn get_scale(&self) -> (f64, f64, f64);
}
//}}}

//{{{
impl<T> Is3DObject for T where T: HasRefSys3D
{
	//{{{ 3D-Manipulation

	//{{{
	fn rotate_x(&mut self, x: f64)
	{
		self.ref_sys_mut().rotate_x(x);
	}
	//}}}
	//{{{
	fn rotate_y(&mut self, y: f64)
	{
		self.ref_sys_mut().rotate_y(y);
	}
	//}}}
	//{{{
	fn rotate_z(&mut self, z: f64)
	{
		self.ref_sys_mut().rotate_z(z);
	}
	//}}}
	//{{{
	fn rotate(&mut self, x: f64, y: f64, z: f64)
	{
		self.ref_sys_mut().rotate(x, y, z);
	}
	//}}}

	//{{{
	fn rel_rotate_x(&mut self, x: f64)
	{
		self.ref_sys_mut().rel_rotate_x(x);
	}
	//}}}
	//{{{
	fn rel_rotate_y(&mut self, y: f64)
	{
		self.ref_sys_mut().rel_rotate_y(y);
	}
	//}}}
	//{{{
	fn rel_rotate_z(&mut self, z: f64)
	{
		self.ref_sys_mut().rel_rotate_z(z);
	}
	//}}}
	//{{{
	fn rel_rotate(&mut self, x: f64, y: f64, z: f64)
	{
		self.ref_sys_mut().rel_rotate(x, y, z);
	}
	//}}}


	//{{{
	fn translate_x(&mut self, x: f64)
	{
		self.ref_sys_mut().translate_x(x);
	}
	//}}}
	//{{{
	fn translate_y(&mut self, y: f64)
	{
		self.ref_sys_mut().translate_y(y);
	}
	//}}}
	//{{{
	fn translate_z(&mut self, z: f64)
	{
		self.ref_sys_mut().translate_z(z);
	}
	//}}}
	//{{{
	fn translate(&mut self, x: f64, y:f64, z: f64)
	{
		self.ref_sys_mut().translate(x, y, z);
	}
	//}}}

	//{{{
	fn rel_translate_x(&mut self, x: f64)
	{
		self.ref_sys_mut().rel_translate_x(x);
	}
	//}}}
	//{{{
	fn rel_translate_y(&mut self, y: f64)
	{
		self.ref_sys_mut().rel_translate_y(y);
	}
	//}}}
	//{{{
	fn rel_translate_z(&mut self, z: f64)
	{
		self.ref_sys_mut().rel_translate_z(z);
	}
	//}}}
	//{{{
	fn rel_translate(&mut self, x: f64, y:f64, z: f64)
	{
		self.ref_sys_mut().rel_translate(x, y, z);
	}
	//}}}


	//{{{
	fn scale_x(&mut self, x: f64)
	{
		self.ref_sys_mut().scale_x(x);
	}
	//}}}
	//{{{
	fn scale_y(&mut self, y: f64)
	{
		self.ref_sys_mut().scale_y(y);
	}
	//}}}
	//{{{
	fn scale_z(&mut self, z: f64)
	{
		self.ref_sys_mut().scale_z(z);
	}
	//}}}
	//{{{
	fn scale(&mut self, x: f64, y:f64, z: f64)
	{
		self.ref_sys_mut().scale(x, y, z);
	}
	//}}}
	//{{{
	fn rel_scale_x(&mut self, x: f64)
	{
		self.ref_sys_mut().rel_scale_x(x);
	}
	//}}}
	//{{{
	fn rel_scale_y(&mut self, y: f64)
	{
		self.ref_sys_mut().rel_scale_y(y);
	}
	//}}}
	//{{{
	fn rel_scale_z(&mut self, z: f64)
	{
		self.ref_sys_mut().rel_scale_z(z);
	}
	//}}}
	//{{{
	fn rel_scale(&mut self, x: f64, y:f64, z: f64)
	{
		self.ref_sys_mut().rel_scale(x, y, z);
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
	fn get_rotate_x(&self) -> f64
	{
		self.ref_sys().get_rotate_x()
	}
	//}}}
	//{{{
	fn get_rotate_y(&self) -> f64
	{
		self.ref_sys().get_rotate_x()
	}
	//}}}
	//{{{
	fn get_rotate_z(&self) -> f64
	{
		self.ref_sys().get_rotate_x()
	}
	//}}}
	//{{{
	fn get_rotate(&self) -> ( f64, f64, f64)
	{
		self.ref_sys().get_rotate()
	}
	//}}}

	//{{{
	fn get_translate_x(&self) -> f64
	{
		self.ref_sys().get_translate_x()
	}
	//}}}
	//{{{
	fn get_translate_y(&self) -> f64
	{
		self.ref_sys().get_translate_y()
	}
	//}}}
	//{{{
	fn get_translate_z(&self) -> f64
	{
		self.ref_sys().get_translate_z()
	}
	//}}}
	//{{{
	fn get_translate(&self) -> (f64, f64, f64)
	{
		self.ref_sys().get_translate()
	}
	//}}}

	//{{{
	fn get_scale_x(&self) -> f64
	{
		self.ref_sys().get_scale_x()
	}
	//}}}
	//{{{
	fn get_scale_y(&self) -> f64
	{
		self.ref_sys().get_scale_y()
	}
	//}}}
	//{{{
	fn get_scale_z(&self) -> f64
	{
		self.ref_sys().get_scale_z()
	}
	//}}}
	//{{{
	fn get_scale(&self) -> (f64, f64, f64)
	{
		self.ref_sys().get_scale()
	}
	//}}}
	//}}}
}
//}}}

//}}}


