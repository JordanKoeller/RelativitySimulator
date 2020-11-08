use gl::types::*;

use std::sync::mpsc::Receiver;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::path::Path;
use std::ffi::CStr;

pub fn glCheckError_(file: &str, line: u32) -> u32 {
  unsafe {
    let mut errorCode = gl::GetError();
    while errorCode != gl::NO_ERROR {
        let error = match errorCode {
            gl::INVALID_ENUM => "INVALID_ENUM",
            gl::INVALID_VALUE => "INVALID_VALUE",
            gl::INVALID_OPERATION => "INVALID_OPERATION",
            gl::STACK_OVERFLOW => "STACK_OVERFLOW",
            gl::STACK_UNDERFLOW => "STACK_UNDERFLOW",
            gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
            gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
            _ => "unknown GL error code"
        };

        println!("{} | {} ({})", error, file, line);

        errorCode = gl::GetError();
    }
    errorCode
  }
}

macro_rules! glCheckError {
    () => (
        glCheckError_(file!(), line!())
    )
}

#[allow(dead_code)]
pub extern "system" fn gl_debug_output(source: gl::types::GLenum,
                                 type_: gl::types::GLenum,
                                 id: gl::types::GLuint,
                                 severity: gl::types::GLenum,
                                 _length: gl::types::GLsizei,
                                 message: *const gl::types::GLchar,
                                 _user_param: *mut c_void)
{
    if id == 131_169 || id == 131_185 || id == 131_218 || id == 131_204 {
        // ignore these non-significant error codes
        return
    }

    println!("---------------");
    let message = unsafe { CStr::from_ptr(message).to_str().unwrap() };
    println!("Debug message ({}): {}", id, message);
    match source {
        gl::DEBUG_SOURCE_API =>             println!("Source: API"),
        gl::DEBUG_SOURCE_WINDOW_SYSTEM =>   println!("Source: Window System"),
        gl::DEBUG_SOURCE_SHADER_COMPILER => println!("Source: Shader Compiler"),
        gl::DEBUG_SOURCE_THIRD_PARTY =>     println!("Source: Third Party"),
        gl::DEBUG_SOURCE_APPLICATION =>     println!("Source: Application"),
        gl::DEBUG_SOURCE_OTHER =>           println!("Source: Other"),
        _ =>                                println!("Source: Unknown enum value")
    }

    match type_ {
       gl::DEBUG_TYPE_ERROR =>               println!("Type: Error"),
       gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => println!("Type: Deprecated Behaviour"),
       gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR =>  println!("Type: Undefined Behaviour"),
       gl::DEBUG_TYPE_PORTABILITY =>         println!("Type: Portability"),
       gl::DEBUG_TYPE_PERFORMANCE =>         println!("Type: Performance"),
       gl::DEBUG_TYPE_MARKER =>              println!("Type: Marker"),
       gl::DEBUG_TYPE_PUSH_GROUP =>          println!("Type: Push Group"),
       gl::DEBUG_TYPE_POP_GROUP =>           println!("Type: Pop Group"),
       gl::DEBUG_TYPE_OTHER =>               println!("Type: Other"),
       _ =>                                  println!("Type: Unknown enum value")
    }

    match severity {
       gl::DEBUG_SEVERITY_HIGH =>         println!("Severity: high"),
       gl::DEBUG_SEVERITY_MEDIUM =>       println!("Severity: medium"),
       gl::DEBUG_SEVERITY_LOW =>          println!("Severity: low"),
       gl::DEBUG_SEVERITY_NOTIFICATION => println!("Severity: notification"),
       _ =>                               println!("Severity: Unknown enum value")
    }
}