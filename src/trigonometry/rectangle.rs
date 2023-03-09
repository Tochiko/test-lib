use std::fmt::Display;

use super::Trig2D;



pub struct Rectangle {
	height: u32,
	width: u32,
}

impl Trig2D for Rectangle {
	fn area(&self) -> u32 {
	    self.height*self.width
	}
	fn is_emtpy(&self) -> bool {
	    self.width > 0 && self.height >0 
	}
}

impl Display for Rectangle {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	    f.write_str("Height: "+self.height.to_string()+", Width: "+self.width.to_string());
	    Ok(())
	}
}