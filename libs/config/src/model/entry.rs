use super::{Array, Number, Str};

#[derive(Debug, Clone, PartialEq)]
/// A value entry in a config file
pub enum Entry {
    /// A string value
    ///
    /// ```cpp
    /// my_string = "Hello World";
    /// ```
    Str(Str),
    /// A number value
    ///
    /// ```cpp
    /// my_number = 1;
    /// ```
    Number(Number),
    /// An array value
    ///
    /// ```cpp
    /// my_array[] = {1,2,3};
    /// ```
    Array(Array),
}
