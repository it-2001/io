/**
 * Looks intimidating, ik
 *
 * to find the actual code, look for the match statement
 * or just ctrl+f for "std::print" or whatever you want to find
 *
 * there is no official documentation for writing Rusty danda libraries at the time of writing this
 * for more information, please refer to the main repository www.github.com/it-2001/Rusty-compiler
 *
 */
extern crate runtime;

use runtime::runtime_types::*;
use runtime::*;

pub struct Foo {
    file_handles: Vec<Option<std::fs::File>>,
    id: usize,
}

impl lib::Library for Foo {
    fn call(
        &mut self,
        id: usize,
        mem: PublicData,
    ) -> Result<Types, runtime_error::ErrTypes> {
        let m = mem.memory;
        match id {
            // std::print
            0 => {
                if let Types::Pointer(u_size, PointerTypes::String) =
                    m.registers[runtime_types::POINTER_REG]
                {
                    let mut string = String::new();
                    for i in m.strings.pool[u_size].iter() {
                        string.push(*i);
                    }
                    print!("{}", string);
                } else {
                    return Err(runtime_error::ErrTypes::Message(
                        "Invalid argument".to_owned(),
                    ));
                }
            }
            // std::println
            1 => {
                if let Types::Pointer(u_size, PointerTypes::String) =
                    m.registers[runtime_types::POINTER_REG]
                {
                    let mut string = String::new();
                    for i in m.strings.pool[u_size].iter() {
                        string.push(*i);
                    }
                    println!("{}", string);
                } else {
                    return Err(runtime_error::ErrTypes::Message(
                        "Invalid argument".to_owned(),
                    ));
                }
            }
            // std::read
            2 => {
                let mut input = String::new();
                match std::io::stdin().read_line(&mut input) {
                    Err(why) => {
                        return Err(runtime_error::ErrTypes::Message(format!(
                            "Couldn't read line: {}",
                            why
                        )))
                    }
                    Ok(_) => (),
                }
                m.strings.pool.push(input.chars().collect());
                return Ok(Types::Pointer(
                    m.strings.pool.len() - 1,
                    PointerTypes::String,
                ));
            }
            // std::file_read
            3 => {
                use std::fs::File;
                use std::io::prelude::*;
                if let Types::Pointer(u_size, PointerTypes::String) =
                    m.registers[runtime_types::POINTER_REG]
                {
                    let string = m.strings.to_string(u_size);
                    let mut file = match File::open(string) {
                        Err(why) => {
                            return Err(runtime_error::ErrTypes::Message(format!(
                                "Couldn't open file: {}",
                                why
                            )))
                        }
                        Ok(file) => file,
                    };
                    let mut contents = String::new();
                    match file.read_to_string(&mut contents) {
                        Err(why) => {
                            return Err(runtime_error::ErrTypes::Message(format!(
                                "Couldn't read file: {}",
                                why
                            )))
                        }
                        Ok(_) => (),
                    }
                    m.strings.pool.push(contents.chars().collect());
                    return Ok(Types::Pointer(
                        m.strings.pool.len() - 1,
                        PointerTypes::String,
                    ));
                } else {
                    return Err(runtime_error::ErrTypes::Message(
                        "Invalid argument".to_owned(),
                    ));
                }
            }
            // std::file_write
            4 => {
                use std::fs::File;
                use std::io::prelude::*;
                if let Types::Pointer(u_size, PointerTypes::String) =
                    m.registers[runtime_types::POINTER_REG]
                {
                    let string = m.strings.to_string(u_size);
                    let mut file = match File::create(string) {
                        Err(why) => {
                            return Err(runtime_error::ErrTypes::Message(format!(
                                "Couldn't create file: {}",
                                why
                            )))
                        }
                        Ok(file) => file,
                    };
                    if let Types::Pointer(u_size, PointerTypes::String) =
                        m.registers[runtime_types::GENERAL_REG1]
                    {
                        let string = m.strings.to_string(u_size);
                        match file.write_all(string.as_bytes()) {
                            Err(why) => {
                                return Err(runtime_error::ErrTypes::Message(format!(
                                    "Couldn't write to file: {}",
                                    why
                                )))
                            }
                            Ok(_) => (),
                        }
                    } else {
                        return Err(runtime_error::ErrTypes::Message(
                            "Invalid argument".to_owned(),
                        ));
                    }
                } else {
                    return Err(runtime_error::ErrTypes::Message(
                        "Invalid argument".to_owned(),
                    ));
                }
            }
            // std::file_append
            5 => {
                use std::fs::OpenOptions;
                use std::io::prelude::*;
                if let Types::Pointer(u_size, PointerTypes::String) =
                    m.registers[runtime_types::POINTER_REG]
                {
                    let string = m.strings.to_string(u_size);
                    let mut file = match OpenOptions::new().append(true).open(string) {
                        Err(why) => {
                            return Err(runtime_error::ErrTypes::Message(format!(
                                "Couldn't open file: {}",
                                why
                            )))
                        }
                        Ok(file) => file,
                    };
                    if let Types::Pointer(u_size, PointerTypes::String) =
                        m.registers[runtime_types::GENERAL_REG1]
                    {
                        let string = m.strings.to_string(u_size);
                        match file.write_all(string.as_bytes()) {
                            Err(why) => {
                                return Err(runtime_error::ErrTypes::Message(format!(
                                    "Couldn't write to file: {}",
                                    why
                                )))
                            }
                            Ok(_) => (),
                        }
                    } else {
                        return Err(runtime_error::ErrTypes::Message(format!(
                            "String pointer expected, got {:#}",
                            m.registers[runtime_types::GENERAL_REG1]
                        )));
                    }
                } else {
                    return Err(runtime_error::ErrTypes::Message(
                        "Invalid argument".to_owned(),
                    ));
                }
            }
            // std::file_open
            // returns index of file handle
            6 => {
                use std::fs::File;
                if let Types::Pointer(u_size, PointerTypes::String) =
                    m.registers[runtime_types::POINTER_REG]
                {
                    let string = m.strings.to_string(u_size);
                    let file = match File::open(string) {
                        Err(why) => {
                            return Err(runtime_error::ErrTypes::Message(format!(
                                "Couldn't open file: {}",
                                why
                            )))
                        }
                        Ok(file) => file,
                    };
                    self.file_handles.push(Some(file));
                    return Ok(Types::Usize(self.file_handles.len() - 1));
                } else {
                    return Err(runtime_error::ErrTypes::Message(
                        "Invalid argument".to_owned(),
                    ));
                }
            }
            // std::file_close
            // takes index of file handle
            // returns bool
            7 => {
                if let Types::Usize(u_size) = m.registers[runtime_types::POINTER_REG] {
                    if u_size >= self.file_handles.len() {
                        return Err(runtime_error::ErrTypes::Message(
                            "Invalid file handle".to_owned(),
                        ));
                    }
                    if self.file_handles[u_size].is_none() {
                        return Err(runtime_error::ErrTypes::Message(
                            "File handle already closed".to_owned(),
                        ));
                    }
                    self.file_handles[u_size] = None;
                } else {
                    return Err(runtime_error::ErrTypes::Message(format!(
                        "File handle must be usize, got {:#}",
                        m.registers[runtime_types::POINTER_REG]
                    )));
                }
            }
            // std::handle_read
            // takes index of file handle
            // returns string
            8 => {
                use std::io::prelude::*;
                if let Types::Usize(u_size) = m.registers[runtime_types::POINTER_REG] {
                    if u_size >= self.file_handles.len() {
                        return Err(runtime_error::ErrTypes::Message(
                            "Invalid file handle".to_owned(),
                        ));
                    }
                    if self.file_handles[u_size].is_none() {
                        return Err(runtime_error::ErrTypes::Message(
                            "File handle already closed".to_owned(),
                        ));
                    }
                    let mut file = self.file_handles[u_size].as_ref().unwrap();
                    let mut contents = String::new();
                    match file.read_to_string(&mut contents) {
                        Err(why) => {
                            return Err(runtime_error::ErrTypes::Message(format!(
                                "Couldn't read file: {}",
                                why
                            )))
                        }
                        Ok(_) => (),
                    }
                    m.strings.pool.push(contents.chars().collect());
                    return Ok(Types::Pointer(
                        m.strings.pool.len() - 1,
                        PointerTypes::String,
                    ));
                } else {
                    return Err(runtime_error::ErrTypes::Message(format!(
                        "File handle must be usize, got {:#}",
                        m.registers[runtime_types::POINTER_REG]
                    )));
                }
            }
            // std::handle_write
            // takes index of file handle
            // writes to file from register 1
            9 => {
                use std::io::prelude::*;
                if let Types::Usize(u_size) = m.registers[runtime_types::POINTER_REG] {
                    if u_size >= self.file_handles.len() {
                        return Err(runtime_error::ErrTypes::Message(
                            "Invalid file handle".to_owned(),
                        ));
                    }
                    if self.file_handles[u_size].is_none() {
                        return Err(runtime_error::ErrTypes::Message(
                            "File handle already closed".to_owned(),
                        ));
                    }
                    let mut file = self.file_handles[u_size].as_ref().unwrap();
                    if let Types::Pointer(u_size, PointerTypes::String) =
                        m.registers[runtime_types::GENERAL_REG1]
                    {
                        let string = m.strings.to_string(u_size);
                        match file.write_all(string.as_bytes()) {
                            Err(why) => {
                                return Err(runtime_error::ErrTypes::Message(format!(
                                    "Couldn't write to file: {}",
                                    why
                                )))
                            }
                            Ok(_) => (),
                        }
                    } else {
                        return Err(runtime_error::ErrTypes::Message(format!(
                            "String pointer expected, got {:#}",
                            m.registers[runtime_types::GENERAL_REG1]
                        )));
                    }
                } else {
                    return Err(runtime_error::ErrTypes::Message(format!(
                        "File handle must be usize, got {:#}",
                        m.registers[runtime_types::POINTER_REG]
                    )));
                }
            }
            // std::handle_append
            // takes index of file handle
            // appends to file from register 1
            10 => {
                use std::io::prelude::*;
                if let Types::Usize(u_size) = m.registers[runtime_types::POINTER_REG] {
                    if u_size >= self.file_handles.len() {
                        return Err(runtime_error::ErrTypes::Message(
                            "Invalid file handle".to_owned(),
                        ));
                    }
                    if self.file_handles[u_size].is_none() {
                        return Err(runtime_error::ErrTypes::Message(
                            "File handle already closed".to_owned(),
                        ));
                    }
                    let mut file = self.file_handles[u_size].as_ref().unwrap();
                    if let Types::Pointer(u_size, PointerTypes::String) =
                        m.registers[runtime_types::GENERAL_REG1]
                    {
                        let string = m.strings.to_string(u_size);
                        match file.write_all(string.as_bytes()) {
                            Err(why) => {
                                return Err(runtime_error::ErrTypes::Message(format!(
                                    "Couldn't write to file: {}",
                                    why
                                )))
                            }
                            Ok(_) => (),
                        }
                    } else {
                        return Err(runtime_error::ErrTypes::Message(format!(
                            "String pointer expected, got {:#}",
                            m.registers[runtime_types::GENERAL_REG1]
                        )));
                    }
                } else {
                    return Err(runtime_error::ErrTypes::Message(format!(
                        "File handle must be usize, got {:#}",
                        m.registers[runtime_types::POINTER_REG]
                    )));
                }
            }
            // std::args
            // returns array of strings
            11 => {
                // first get a vector of args
                let args: Vec<String> = std::env::args().collect();
                // allocate enough space for the array on the heap
                let obj = m.allocate_obj(args.len() + 1);
                // set the first element to the length of the array
                m.heap.data[obj][0] = Types::NonPrimitive(0);
                // iterate over the args
                for (i, arg) in args.iter().enumerate() {
                    // push the string to the string pool
                    let str = m.strings.from(arg.to_string().chars().collect());
                    // set the element in the array to the index of the string in the string pool
                    m.heap.data[obj][i + 1] = Types::Pointer(str, PointerTypes::String);
                }
                // return the pointer to the array
                return Ok(Types::Pointer(obj, PointerTypes::Object));
            }
            _ => {
                unreachable!("Invalid function id")
            }
        }
        return Ok(runtime_types::Types::Void);
    }
    fn name(&self) -> String {
        return "io".to_owned();
    }
    fn register(&self) -> lib::RegisterData {
        return lib::RegisterData::new().set_rest(r#"
        type File = usize

        impl File {
            fun read(&self=reg.ptr): string > 8
            fun write(&self=reg.ptr, data=reg.G1:string)! > 9
            fun append(&self=reg.ptr, data=reg.G1:string)! > 10
            fun close(&self=reg.ptr)! > 7
        }

        fun print(msg=reg.ptr: string) > 0
        fun println(msg=reg.ptr: string) > 1
        fun input(): string > 2
        fun fileRead(fileName=reg.ptr: string): string > 3
        fun fileWrite(fileName=reg.ptr: string, data=reg.G1: string)! > 4
        fun fileAppend(fileName=reg.ptr: string, data=reg.G1: string)! > 5
        fun fileOpen(fileName=reg.ptr: string)!: File > 6
        fun args(): &[string: _] > 11

        "#.to_string())
    }
}

#[no_mangle]
pub fn init(ctx: &mut Context, my_id: usize) -> Box<dyn lib::Library> {
    return Box::new(Foo {
        file_handles: Vec::new(),
        id: my_id,
    });
}
