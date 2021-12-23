use ultraviolet::{Vec3, Isometry3, Rotor3, Mat4, projection::rh_yup::perspective_wgpu_dx};

#[derive(Debug)]
struct Camera {
	pub eye: Vec3,
	pub rotation: Rotor3,
}

impl Camera {
	pub fn new(eye: Vec3, pitch: f32, yaw: f32) -> Self {
		Camera {
			eye,
			rotation: Rotor3::from_euler_angles(0., pitch, yaw),
		}
	}

	pub fn txform(&self) -> Isometry3 {
		Isometry3::new(self.eye, self.rotation)
	}
}

#[derive(Debug)]
struct Projection {
	aspect: f32,
	fovy: f32,
	znear: f32,
	zfar: f32,
}

impl Projection {
	pub fn new(width: u32, height: u32, fovy: f32, znear: f32, zfar: f32) -> Self {
		Self {
			aspect: (width as f32)/(height as f32),
			fovy, znear, zfar,
		}
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		self.aspect = (width as f32)/(height as f32);
	}

	pub fn txform(&self) -> Mat4 {
		perspective_wgpu_dx(
			self.fovy,
			self.aspect,
			self.znear,
			self.zfar,
		)
	}
}