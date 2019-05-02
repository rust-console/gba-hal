#![no_std]
#![warn(missing_docs)]

//! The bare bones hardware abstraction layer for the GBA.

/// Assists in defining a newtype wrapper over some base type.
///
/// Note that rustdoc and derives are all the "meta" stuff, so you can write all
/// of your docs and derives in front of your newtype in the same way you would
/// for a normal struct. Then the inner type to be wrapped it name.
///
/// The macro _assumes_ that you'll be using it to wrap numeric types and that
/// it's safe to have a `0` value, so it automatically provides a `const fn`
/// method for `new` that just wraps `0`. Also, it derives Debug, Clone, Copy,
/// Default, PartialEq, and Eq. If all this is not desired you can add `, no
/// frills` to the invocation.
///
/// ```no_run
/// # use gba_hal::newtype;
/// newtype! {
///   /// Records a particular key press combination.
///   KeyInput, u16
/// }
/// newtype! {
///   /// You can't derive most stuff above array size 32, so we add
///   /// the `, no frills` modifier to this one.
///   BigArray, [u8; 200], no frills
/// }
/// ```
#[macro_export]
macro_rules! newtype {
  ($(#[$attr:meta])* $new_name:ident, $v:vis $old_name:ty) => {
    $(#[$attr])*
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct $new_name($v $old_name);
    impl $new_name {
      /// A `const` "zero value" constructor
      pub const fn new() -> Self {
        $new_name(0)
      }
    }
  };
  ($(#[$attr:meta])* $new_name:ident, $v:vis $old_name:ty, no frills) => {
    $(#[$attr])*
    #[repr(transparent)]
    pub struct $new_name($v $old_name);
  };
}

/// Assists in defining a newtype that's an enum.
///
/// First give `NewType = OldType,`, then define the tags and their explicit
/// values with zero or more entries of `TagName = base_value,`. In both cases
/// you can place doc comments or other attributes directly on to the type
/// declaration or the tag declaration.
///
/// The generated enum will get an appropriate `repr` attribute as well as
/// Debug, Clone, Copy, PartialEq, and Eq
///
/// ```no_run
/// # use gba_hal::newtype_enum;
/// newtype_enum! {
///   /// The Foo
///   Foo = u16,
///   /// The Bar
///   Bar = 0,
///   /// The Zap
///   Zap = 1,
/// }
/// ```
#[macro_export]
macro_rules! newtype_enum {
  (
    $(#[$struct_attr:meta])*
    $new_name:ident = $old_name:ident,
    $($(#[$tag_attr:meta])* $tag_name:ident = $base_value:expr,)*
  ) => {
    $(#[$struct_attr])*
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr($old_name)]
    pub enum $new_name {
      $(
        $(#[$tag_attr])*
        $tag_name = $base_value,
      )*
    }
  };
}

pub mod data;

#[cfg(any(
  all(target_env = "agb", target_vendor = "nintendo"),
  feature = "unsafe_docs_rs_mmio_listing_override"
))]
pub mod mmio;

