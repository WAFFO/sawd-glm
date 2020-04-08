
/*! # Homebrew-glm - Lightweight, Easy to Use, Graphics Focused Linear Algebra Library

  ## Quick facts

  This library is based on the C OpenGL library [GLM](https://glm.g-truc.net/). That means:

  - Matrices are Column-Major
  - Math follows the right-hand rule
  - Z points towards the screen (up until the projection matrix)

  ## Goals for this library

  This library was created with these 4 goals in mind:

  ### Straight Forward

  This library should be _simple_ and _easy to use_. Types aren't heavily abstracted, so any
  compilation errors encountered will be easy to understand, and what a types a function requires
  can't be misinterpreted. By default, each type has the copy trait, and no functions take a
  reference, removing the small road-bumps a developer might have forgetting to pass by reference.
  This way we leverage the power of Rust and let the compiler optimize the memory allocation.

  ### IDE Friendly

  Important functionality should not be hidden behind macros or aliases. We want IDEs to be able to
  easily identify the outcome of operators and what functions are available to them.

  ### Complete Documentation

  This goes beyond just giving descriptions for functions, but explaining the finer details of
  complicated functionality. Giving examples where necessary. Concepts here are available all over
  the internet, but the examples aren't executed with this library. The examples might be following
  the left handed rule, the matrices might be row-major, or the axis may be pointing in different
  directions. It could be any combination of those, and it's an obligation of the library to provide
  relevant examples.

  ### Lightweight

  No dependencies, what you see here is what you get. Code is fast and efficient.

*/

#[macro_use]
mod macros;

pub(crate) mod glm;

pub use glm::*;
pub use glm::vec3::Vec3;
pub use glm::vec4::Vec4;
pub use glm::mat3::Mat3;
pub use glm::mat4::Mat4;
pub use glm::quat::Quat;
