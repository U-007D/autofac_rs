Example build on the WIP he_di package

This version contains 2 possible implementation of the `IDateWriter` interface/trait 

- `BoxTodayWriter` is an implementation of `TodayWriter` using Rust's trait objects
- `GenericTodayWriter` is an implementation of `TodayWriter` using Rust's generics

Both these implementations expect an object implementing the `IOutput` trait. It is written with no knowledge of the `he_di` package. Ultimately, one would be able to tag its components (i.e. types requiring DI) and the package's magic would do the rest. A target 'tagging' is `#[derive(Component)]` using Rust's procedural macros

For now, the code between the comment lines (lines 76-122) is supposed to be what the compiler would generate after a call to that macro to be created.

Few comments
1. this version compiles but contains a lot of unnecessary code. Notably the Component trait and its 2 associated (and unimplemented) methods
2. The ComponentBuilder trait is not needed for now but might come back
3. The current version is not satisfactory as it requires a knowledge of `IOutput`'s implementation name (namely `ConsoleOutput` but I couldn't find a way to downcast to a trait since it's size is unknown)
4. I'm using `UnsafeAny` to be able to downcast a Box<Component>. Otherwise I was receiving the following compile error. Since it's just a WIP code, I didn't overinvest in trying to make it work with a classical Box
```
error: no method named `downcast` found for type `std::boxed::Box<he_di::Component>` in the current scope
   --> examples/autofac.rs:119:25
    |
114 |             output: tmp.downcast::<ConsoleOutput>(),
    |                         ^^^^^^^^
``` 