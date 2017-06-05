#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

use std::{result, error};
mod consts;
use consts::msgs;

#[cfg(test)]
mod unit_tests;

// This interface helps decouple the concept of
// "writing output" from the Console class. We
// don't really "care" how the Write operation
// happens, just that we can write.
pub trait IOutput
{
    fn write<T: Into<String>>(&self, content: T);
}

// This implementation of the IOutput interface
// is actually how we write to the Console. Technically
// we could also implement IOutput to write to Debug
// or Trace... or anywhere else.
pub struct ConsoleOutput;

impl IOutput for ConsoleOutput
{
    fn write<T: Into<String>>(&self, content: T) {
        println!("{}", content.into());
    }
}

// This interface decouples the notion of writing
// a date from the actual mechanism that performs
// the writing. Like with IOutput, the process
// is abstracted behind an interface.
pub trait IDateWriter
{
    fn write_date(&self);
}

// This TodayWriter is where it all comes together.
// Notice it takes a constructor parameter of type
// IOutput - that lets the writer write to anywhere
// based on the implementation. Further, it implements
// WriteDate such that today's date is written out;
// you could have one that writes in a different format
// or a different date.
struct TodayWriter<T: IOutput> {
    output: T,
}

impl<T: IOutput> TodayWriter<T> {
    fn new(output: T) -> TodayWriter<T> {
        TodayWriter::<T> { output: output }
    }
}

impl<T: IOutput> IDateWriter for TodayWriter<T> {
    fn write_date(&self) {
        self.output.write("Today is June 4th, 2017")
    }
}

pub fn run(args: Vec<String>) -> result::Result<(), Box<error::Error>> {
    TodayWriter::new(ConsoleOutput).write_date();
    Ok(())
}
