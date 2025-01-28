use clap::{Parser, ArgGroup};
use std::path::PathBuf;

#[derive(Parser)]
#[command(group(
    ArgGroup::new("params")
    .args(&["bp", "cp"])
    .multiple(true)
    .required(true)
))]
#[command(name = "TXR Save Editor")]
pub struct UserArgs {
    #[clap(short, long, required = true, help="Input path of save file.")]
    pub in_path: PathBuf,

    #[clap(long, help="Set BP. Max: 999")]
    pub bp: Option<i16>,

    #[clap(long, help="Set CP. Max: 999,999,999,999")]
    pub cp: Option<i64>,
}