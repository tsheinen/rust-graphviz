pub mod enums;
mod error;
mod plugins;
mod utils;

use crate::enums::{Args, Engine, OutputFormat, OutputLocation};
use crate::error::GenericError;
// use crate::plugins::*;
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
use lib::Symbol;

extern crate libloading as lib;
pub struct GVC {
    context: *mut gv::GVC_t,
    engine: Engine,
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
            context: unsafe {
                // use libloading - find a way to yoink address from the Symbol object Symbol->into_raw()->into_raw()
                let dot_layout_lib = lib::Library::new("/usr/lib/graphviz/libgvplugin_dot_layout.so").unwrap();
                let neato_layout_lib = lib::Library::new("/usr/lib/graphviz/libgvplugin_neato_layout.so").unwrap();
                let pango_lib = lib::Library::new("/usr/lib/graphviz/libgvplugin_pango.so").unwrap();
                let webp_lib = lib::Library::new("/usr/lib/graphviz/libgvplugin_webp.so").unwrap();
                let gd_lib = lib::Library::new("/usr/lib/graphviz/libgvplugin_gd.so").unwrap();
                let core_lib = lib::Library::new("/usr/lib/graphviz/libgvplugin_core.so").unwrap();

                let gvplugin_dot_layout_LTX_library: Symbol<*mut core::ffi::c_void> = dot_layout_lib.get(b"gvplugin_dot_layout_LTX_library").unwrap();
                let gvplugin_neato_layout_LTX_library: Symbol<*mut core::ffi::c_void> = neato_layout_lib.get(b"gvplugin_neato_layout_LTX_library").unwrap();
                let gvplugin_pango_LTX_library: Symbol<*mut core::ffi::c_void> = pango_lib.get(b"gvplugin_pango_LTX_library").unwrap();
                let gvplugin_webp_LTX_library: Symbol<*mut core::ffi::c_void> = webp_lib.get(b"gvplugin_webp_LTX_library").unwrap();
                let gvplugin_gd_LTX_library: Symbol<*mut core::ffi::c_void> = gd_lib.get(b"gvplugin_gd_LTX_library").unwrap();
                let gvplugin_core_LTX_library: Symbol<*mut core::ffi::c_void> = core_lib.get(b"gvplugin_core_LTX_library").unwrap();

                let plugins: [gv::lt_symlist_t; 7] = [
                    gv::lt_symlist_t {
                        name: "gvplugin_dot_layout_LTX_library".as_ptr() as *const i8,
                        address: *gvplugin_dot_layout_LTX_library,
                    },
                    gv::lt_symlist_t {
                        name: "gvplugin_neato_layout_LTX_library".as_ptr() as *const i8,
                        address: *gvplugin_neato_layout_LTX_library,
                    },
                    gv::lt_symlist_t {
                        name: "gvplugin_pango_LTX_library".as_ptr() as *const i8,
                        address: *gvplugin_pango_LTX_library,
                    },
                    gv::lt_symlist_t {
                        name: "gvplugin_webp_LTX_library".as_ptr() as *const i8,
                        address: *gvplugin_webp_LTX_library,
                    },
                    gv::lt_symlist_t {
                        name: "gvplugin_gd_LTX_library".as_ptr() as *const i8,
                        address: *gvplugin_gd_LTX_library,
                    },
                    gv::lt_symlist_t {
                        name: "gvplugin_core_LTX_library".as_ptr() as *const i8,
                        address: *gvplugin_core_LTX_library,
                    },
                    gv::lt_symlist_t {
                        name: std::ptr::null_mut(),
                        address: std::ptr::null_mut(),
                    },
                ];
                gv::gvContextPlugins(plugins.as_ptr(), 1)
                // gv::gvContext()
            },
            engine: engine,
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
        self.engine = engine;
        self
    }

    pub fn render(&self, dotfile: &str) -> Result<Vec<u8>, GenericError> {
        let engine = CString::new(format!("{}", self.engine))?;
        let output_format = CString::new(match self.output_format {
            Some(OutputFormat::PNG) => "png",
            _ => "dot",
        })?;

        let mut pipe: Pipe = Pipe::new()?;
        unsafe {
            let agraph = gv::agmemread(dotfile.as_ptr() as *const i8);
            if agraph == std::ptr::null_mut() {
                return Err(GenericError::new(
                    "agmemread failed. try adding a newline after the closing } of the graph",
                ));
            }
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
    use crate::enums::Engine;
    use crate::enums::OutputFormat;
    use crate::error::GenericError;
    use crate::GVC;
    use std::fs;

    #[test]
    fn it_works() -> Result<(), GenericError> {
        let mut gvc_png = GVC::new(Engine::Dot, OutputFormat::PNG);
        let mut gvc_svg = GVC::new(Engine::Dot, OutputFormat::SVG);
        let mut gvc_gif = GVC::new(Engine::Dot, OutputFormat::GIF);
        let dot = include_str!("../test_files/graph.dot");
        let graph_png = include_bytes!("../test_files/graph.png").to_vec();
        let graph_svg = include_bytes!("../test_files/graph.svg").to_vec();
        let graph_gif = include_bytes!("../test_files/graph.gif").to_vec();

        assert_eq!(gvc_png.render(dot)?, graph_png);

        // svg and gif don't work - maybe it isn't loading plugins??
        assert_eq!(gvc_svg.render(dot)?, graph_svg);
        assert_eq!(gvc_gif.render(dot)?, graph_gif);
        Ok(())
    }
}
