mod structs;
mod utils;

use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use std::error::Error;

use clap::Parser;
use uesave::{PropertyInner, Save, StructValue};
use crate::structs::UserArgs;

const BP_MAX: i16 = 999;
const CP_MAX: i64 = 999_999_999_999;

fn parse_args() -> Result<UserArgs, Box<dyn Error>> {
    let args = UserArgs::parse();
    if let Some(bp) = args.bp {
        if !(bp >= 0 && bp <= BP_MAX) {
            return Err("bp value is out of range".into());
        }
    }

    if let Some(cp) = args.cp {
        if !(cp >= 0 && cp <= CP_MAX) {
            return Err("cp value is out of range".into());
        }
    }

    Ok(args)
}

fn read_and_write_props(save: &mut Save, args: &UserArgs) -> Result<(), Box<dyn Error>> {
    if let PropertyInner::Struct(user_info) = &mut save.root.properties["user_info"].inner {
        if let StructValue::Struct(properties) = user_info {

            if let Some(cp) = args.cp {
                if let PropertyInner::Int64(ref mut cur_cp) = properties["Cp"].inner {
                    println!("CP:\n{} -> {}", cur_cp, cp);
                    *cur_cp = cp;
                } else {
                    return Err("cp value key is missing".into());
                }
            }

            if let Some(bp) = args.bp {
                if let PropertyInner::Int(ref mut cur_bp) = properties["PP"].inner {
                    println!("BP:\n{} -> {}", cur_bp, bp);
                    *cur_bp = bp as i32;
                } else {
                    return Err("bp value key is missing".into());
                }
            }

        } else {
            return Err("properties struct is missing".into());
        }
    } else {
        return Err("user info struct is missing".into());
    }

    Ok(())
}

fn open_save(path: &PathBuf) -> Result<Save, Box<dyn Error>> {
    let mut f = File::open(path)?;
    let save = Save::read(&mut f)?;
    Ok(save)
}

fn write_save(save: &Save, out_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut f = File::create(out_path)?;
    let mut bw = BufWriter::new(&mut f);
    save.write(&mut bw)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args()?;
    let temp_out_path = utils::get_temp_path();
    let in_path = &args.in_path;

    {
        let mut save = open_save(in_path)?;
        read_and_write_props(&mut save, &args)?;
        write_save(&save, &temp_out_path)?;
    }

    fs::remove_file(in_path)?;
    fs::copy(&temp_out_path, in_path)?;
    fs::remove_file(&temp_out_path)?;

    Ok(())
}