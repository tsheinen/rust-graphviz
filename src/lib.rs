pub mod enums;
mod error;
mod utils;

use crate::enums::{Args, Engine, OutputFormat, OutputLocation};
use crate::error::GenericError;
use crate::utils::Pipe;
use graphviz_sys as gv;
use std::ffi::{CStr, CString};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::mem::ManuallyDrop;
use std::os::raw::{c_char, c_int, c_void};
use std::os::unix::io::FromRawFd;
use std::path::{Path, PathBuf};

pub struct GVC {
    context: *mut gv::GVC_t,
    engine: Option<Engine>,
    output_location: Option<OutputLocation>,
    output_format: Option<OutputFormat>,
}

impl Drop for GVC {
    fn drop(&mut self) {
        unsafe {
            gv::gvFreeContext(self.context);
        }
    }
}

impl GVC {
    pub fn new(engine: Engine, output_format: OutputFormat) -> GVC {
        GVC {
            context: unsafe { gv::gvContext() },
            engine: Some(engine),
            output_location: None,
            output_format: Some(output_format),
        }
    }


    pub fn output_format(&mut self, format: OutputFormat) -> &mut Self {
        self.output_format = Some(format);
        self
    }

    pub fn output_location(&mut self, location: OutputLocation) -> &mut Self {
        self.output_location = Some(location);
        self
    }

    pub fn engine(&mut self, engine: Engine) -> &mut Self {
        self.engine = Some(engine);
        self
    }

    pub fn render(&self, dotfile: &str) -> Result<Vec<u8>, GenericError> {
        let engine = CString::new(match self.engine {
            Some(Engine::Dot) => "dot",
            _ => "dot", // not specified, assume dot
        })?;
        let output_format = CString::new(match self.output_format {
            Some(OutputFormat::PNG) => "png",
            _ => "dot",
        })?;

        let mut pipe: Pipe = Pipe::new()?;
        unsafe {
            let agraph = gv::agmemread(dotfile.as_ptr() as *const i8);
            if gv::gvLayout(self.context, agraph, engine.as_ptr()) == -1 {
                return Err(GenericError::new("gvLayout failed"));
            }
            if gv::gvRender(
                self.context,
                agraph,
                output_format.as_ptr(),
                pipe.write_file,
            ) == -1
            {
                return Err(GenericError::new("gvRender failed"));
            }
            gv::gvFreeLayout(self.context, agraph);
            gv::agclose(agraph);
        }
        pipe.close();

        let mut response: Vec<u8> = Vec::new();
        pipe.read_file.read_to_end(&mut response);

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use crate::GVC;
    use std::fs;

    #[test]
    fn it_works() {
        let mut gvc = GVC::new();
        gvc.render(&fs::read_to_string("graph.dot").unwrap());
        assert!(true);
    }
}
