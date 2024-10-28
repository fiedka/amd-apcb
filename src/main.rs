use amd_apcb::{Apcb, ApcbIoOptions};
use clap::Parser;
use std::fs;

/// Parse a PSP binary's header
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Print verbosely
    #[arg(required = false, short, long)]
    verbose: bool,

    /// File to read
    #[arg(index = 1)]
    file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file = args.file;
    // let verbose = args.verbose;
    let data: &[u8] = &fs::read(file).unwrap();

    let buf = std::borrow::Cow::Borrowed(data);

    let apcb = Apcb::load(buf, &ApcbIoOptions::default())?;
    let h = apcb.header().unwrap();
    let platform = match h.version.get() {
        0x20 => "Fam 17h Model 00h to 1fh",
        0x30 => "Fam 17h Model 30h and later",
        _ => "unknown platform",
    };
    println!("version 0x{:x} ({platform})", h.version);

    for g in apcb.groups()? {
        println!("{:?}", g.id());
        for e in g.entries() {
            let id = format!("{:?}", e.id());
            let sz = e.body_as_buf().unwrap_or(&[]).len();
            println!(" - {id:36} size: {sz}");
        }
    }
    Ok(())
}
