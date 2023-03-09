use std::{f32::consts::PI, fmt::Display};

use super::Trig2D;



struct RoundPlane {
	radius: u32
}

impl Trig2D for RoundPlane {
	fn area(&self) -> u32 {
	    PI*self.radius*self.radius
	}
	fn is_emtpy(&self) -> bool {
	    self.radius > 0
	}
}

impl Display for RoundPlane {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	    f.write_str("Radius: "+self.radius.to_string());
	    Ok(())
	}
}