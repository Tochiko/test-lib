use std::{f32::consts::PI, fmt::Display};

use super::Trig3D;



struct Sphere {
	radius: u32
}

impl Trig3D for Sphere {
	fn volume(&self) -> u32 {
	    self.radius*self.radius*self.radius*PI*(4.0/3.0)
	}
	fn is_empty(&self) -> u32 {
	    self.radius > 0
	}
}

impl Display for Sphere {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	    f.write_str("Radius: " + self.radius.to_string());
	    Ok(())
	}
}