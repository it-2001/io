extern crate runtime;

use runtime::runtime::runtime_types::Context;
use runtime::runtime::runtime_types::PointerTypes;
use runtime::runtime::runtime_types::PublicData;
use runtime::runtime::runtime_types::Types;
use runtime::runtime::runtime_types::*;
use runtime::runtime::*;

pub struct Foo {
    pub a: i32,
    pub b: i32,
}

impl runtime::runtime::Library for Foo {
    fn init(&mut self, ctx: &mut Context) -> Result<Box<Self>, String> {
        return Ok(Box::new(Foo { a: 3, b: 0 }));
    }
    fn call(
        &mut self,
        id: usize,
        mem: PublicData,
    ) -> Result<runtime_types::Types, runtime_error::ErrTypes> {
        let m = mem.memory;
        match id {
            // std::print
            0 => {
                if let Types::Pointer(u_size, PointerTypes::String) = m.registers[runtime_types::POINTER_REG] {
                    let mut string = String::new();
                    for i in m.strings.pool[u_size].iter() {
                        string.push(*i);
                    }
                    print!("{}", string);
                } else {
                    return Err(runtime_error::ErrTypes::Message("Invalid argument".to_owned()));
                }
            }
            // std::println
            1 => {
                if let Types::Pointer(u_size, PointerTypes::String) = m.registers[runtime_types::POINTER_REG] {
                    let mut string = String::new();
                    for i in m.strings.pool[u_size].iter() {
                        string.push(*i);
                    }
                    println!("{}", string);
                } else {
                    return Err(runtime_error::ErrTypes::Message("Invalid argument".to_owned()));
                }
            }
            // std::read
            2 => {
                let mut input = String::new();
                match std::io::stdin().read_line(&mut input) {
                    Err(why) => return Err(runtime_error::ErrTypes::Message(format!("Couldn't read line: {}", why))),
                    Ok(_) => (),
                }
                m.strings.pool.push(input.chars().collect());
                return Ok(Types::Pointer(m.strings.pool.len() - 1, PointerTypes::String));
            }
            // std::file_read
            3 => {
                use std::io::prelude::*;
                use std::fs::File;
                if let Types::Pointer(u_size, PointerTypes::String) = m.registers[runtime_types::POINTER_REG] {
                    let mut string = String::new();
                    for i in m.strings.pool[u_size].iter() {
                        string.push(*i);
                    }
                    let mut file = match File::open(string) {
                        Err(why) => return Err(runtime_error::ErrTypes::Message(format!("Couldn't open file: {}", why))),
                        Ok(file) => file,
                    };
                    let mut contents = String::new();
                    match file.read_to_string(&mut contents) {
                        Err(why) => return Err(runtime_error::ErrTypes::Message(format!("Couldn't read file: {}", why))),
                        Ok(_) => (),
                    }
                    m.strings.pool.push(contents.chars().collect());
                    return Ok(Types::Pointer(m.strings.pool.len() - 1, PointerTypes::String));
                } else {
                    return Err(runtime_error::ErrTypes::Message("Invalid argument".to_owned()));
                }
            }
            // std::file_write
            4 => {
                use std::io::prelude::*;
                use std::fs::File;
                if let Types::Pointer(u_size, PointerTypes::String) = m.registers[runtime_types::POINTER_REG] {
                    let mut string = String::new();
                    for i in m.strings.pool[u_size].iter() {
                        string.push(*i);
                    }
                    let mut file = match File::create(string) {
                        Err(why) => return Err(runtime_error::ErrTypes::Message(format!("Couldn't create file: {}", why))),
                        Ok(file) => file,
                    };
                    if let Types::Pointer(u_size, PointerTypes::String) = m.registers[runtime_types::GENERAL_REG1] {
                        let mut string = String::new();
                        for i in m.strings.pool[u_size].iter() {
                            string.push(*i);
                        }
                        match file.write_all(string.as_bytes()) {
                            Err(why) => return Err(runtime_error::ErrTypes::Message(format!("Couldn't write to file: {}", why))),
                            Ok(_) => (),
                        }
                    } else {
                        return Err(runtime_error::ErrTypes::Message("Invalid argument".to_owned()));
                    }
                } else {
                    return Err(runtime_error::ErrTypes::Message("Invalid argument".to_owned()));
                }
            }
            _ => {
                unreachable!("Invalid function id")
            }
        }
        return Ok(runtime_types::Types::Void);
    }
    fn name(&self) -> String {
        return "Foo".to_owned();
    }
    fn register(&self) -> Vec<(String, usize)> {
        return vec![];
    }
}

#[no_mangle]
pub fn init(ctx: &mut Context) -> Box<dyn Library> {
    return Box::new(Foo { a: 3, b: 0 });
}
