//! Examples based on AutoFac 'getting started' example
//! (http://autofac.readthedocs.io/en/latest/getting-started/index.html)
extern crate he_di;

use he_di::{ ContainerBuilder };
use std::error::Error;

trait IOutput {
    fn write(&self, content: String);
}

/// This implementation of the IOutput interface
/// is actually how we write to the Console. Technically
/// we could also implement IOutput to write to Debug
/// or Trace... or anywhere else.
struct ConsoleOutput {}

impl ConsoleOutput {
    fn new() -> ConsoleOutput {
        ConsoleOutput {}
    }
}

impl IOutput for ConsoleOutput {
    fn write(&self, content: String) {
        println!("{}", content);
    }
}

// This interface decouples the notion of writing
// a date from the actual mechanism that performs
// the writing. Like with IOutput, the process
// is abstracted behind an interface.
trait IDateWriter {
    fn write_date(&self);
}

// This TodayWriter is where it all comes together.
// Notice it takes a constructor parameter of type
// IOutput - that lets the writer write to anywhere
// based on the implementation. Further, it implements
// WriteDate such that today's date is written out;
// you could have one that writes in a different format
// or a different date.
struct BoxedTodayWriter {
    output: Box<IOutput>,
}

impl BoxedTodayWriter {
    pub fn new<O: IOutput + 'static>(output: O) -> Self {
        BoxedTodayWriter { output: Box::new(output) }
    }
}

impl IDateWriter for BoxedTodayWriter {
    fn write_date(&self) {
        self.output.write("BoxedTodayWriter says that today is June 5th".to_string());
    }
}

struct GenericTodayWriter<O: IOutput> {
    output: O,
}

impl<O: IOutput> GenericTodayWriter<O> {
    pub fn new(output: O) -> Self {
        GenericTodayWriter { output: output }
    }
}

impl<O: IOutput> IDateWriter for GenericTodayWriter<O> {
    fn write_date(&self) {
        self.output.write("GenericTodayWriter says that today is June 5th".to_string());
    }
}

// =======================================================================
// Code to be generated when calling `#[derive(Component)]`
use std::collections::BTreeMap;

use he_di::component::{ Component, ComponentIndex };
use he_di::Object;
use he_di::util::*;

extern crate unsafe_any;
use unsafe_any::UnsafeAnyExt;

impl Component for ConsoleOutput {
    fn get_index(&self) -> ComponentIndex {
        unimplemented!()
    }

    fn get_arguments(&self) -> &[ComponentIndex] {
        unimplemented!()
    }
}

struct BoxedTodayWriter__DI_Builder {}

impl BoxedTodayWriter__DI_Builder {
    #[allow(non_snake_case)]
    fn __di_build_BoxTodayWriter() -> BoxedTodayWriter {
        let mut param_map : BTreeMap<ComponentIndex, Object> = BTreeMap::new(); // generated
        let mut argument_map : BTreeMap<String, ComponentIndex> = BTreeMap::new();

        // for each param of the struct
        param_map.insert(ComponentIndex::String(get_type::<IOutput>().to_string()), Object::Component( Box::new(ConsoleOutput::new()) ) );
        // param_map.insert(ComponentIndex::String(get_type::<IOutput>().to_string()), Object::Component( Box::new(Container::get_scope().resolve::<IOutput>()) ) );
        argument_map.insert("output".to_string(), ComponentIndex::String(get_type::<IOutput>().to_string())); // generated from parameter name
        
        let tmp = param_map.remove(&argument_map["output"]).unwrap().as_component_owned().unwrap(); //Box<Object::Component>

        BoxedTodayWriter {
            output: unsafe { Box::new(*tmp.downcast_unchecked::<ConsoleOutput>()) },
        }
    }
}
// =======================================================================


fn write_date() {
    // Create the scope, resolve your IDateWriter,
    // use it, then dispose of the scope.
    // let writer = Container::get_scope().resolve::<IDateWriter>();

    let writer = BoxedTodayWriter__DI_Builder::__di_build_BoxTodayWriter();
    writer.write_date();
}

pub fn run(args: Vec<String>) -> Result<(), Box<Error>> {
    // Create your builder.
    let mut builder = ContainerBuilder::new();

    builder.register_type::<ConsoleOutput>().as_type::<IOutput>();
    builder.register_type::<BoxedTodayWriter>().as_type::<IDateWriter>();
    builder.register_type::<GenericTodayWriter<ConsoleOutput>>().as_type::<IDateWriter>();
    builder.build();

    // The WriteDate method is where we'll make use
    // of our dependency injection. We'll define that
    // in a bit.
    write_date();
    Ok(())
}
