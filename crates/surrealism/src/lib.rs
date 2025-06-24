pub mod memory;
pub mod controller;
pub mod registry;
pub mod err;
pub use surrealism_macros::surrealism;
pub use registry::SurrealismFunction;
pub use surrealism_types as types;
pub use controller::Controller;