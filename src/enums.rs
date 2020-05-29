use std::fmt::{Display, Error, Formatter};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum OutputFormat {
    PostScript,
    SVG,
    XFIG,
    FrameMaker,
    HPPenPlotter,
    LaserjetPrinter,
    PNG,
    GIF,
    GTK,
    IMap,
    CMapX,
    Dot,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                OutputFormat::PostScript => "ps",
                OutputFormat::SVG => "svg",
                OutputFormat::XFIG => "fig",
                OutputFormat::FrameMaker => "mif",
                OutputFormat::HPPenPlotter => "hpgl",
                OutputFormat::LaserjetPrinter => "pcl",
                OutputFormat::PNG => "png",
                OutputFormat::GIF => "gif",
                OutputFormat::GTK => "dia",
                OutputFormat::IMap => "imap",
                OutputFormat::CMapX => "cmapx",
                OutputFormat::Dot => "dot",
            }
        )
    }
}

#[derive(Debug)]
pub enum OutputLocation {
    File(PathBuf),
    Stdout,
}

impl Display for OutputLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            OutputLocation::File(path) => write!(f, "{}", path.to_string_lossy()),
            OutputLocation::Stdout => write!(f, ""),
        };
        Ok(())
    }
}

#[derive(Debug)]
pub enum Engine {
    Dot,
    Neato,
    Fdp,
    Sfdp,
    Twopi,
    Circo
}

impl Display for Engine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Engine::Dot => write!(f, "dot"),
            Engine::Neato => write!(f, "neato"),
            Engine::Fdp => write!(f, "fdp"),
            Engine::Sfdp => write!(f, "sfdp"),
            Engine::Twopi => write!(f, "twopi"),
            Engine::Circo => write!(f, "circo"),
        };
        Ok(())
    }
}

#[derive(Debug)]
pub enum Args {
    OutputFormat(OutputFormat),
    OutputLocation(OutputLocation),
    Engine(Engine),
}
impl Display for Args {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Args::OutputFormat(of) => write!(f, "{}", of),
            Args::OutputLocation(ol) => write!(f, "{}", ol),
            Args::Engine(en) => write!(f, "{}", en),
        };
        Ok(())
    }
}
