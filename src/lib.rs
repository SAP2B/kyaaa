#![no_std]
#![no_main]

pub mod macros;
pub mod math;
pub mod std;
pub mod types;

pub mod prelude {
    //pub use crate::math::prelude::*;
    pub use crate::std::prelude::*;
    pub use crate::types::prelude::*;
    pub use crate::*;
}
