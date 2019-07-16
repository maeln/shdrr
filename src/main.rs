extern crate clap;
extern crate notify;
extern crate shaderc;

use clap::{App, Arg};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::env;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

#[derive(Debug, Clone)]
struct ShdrrConf {
    dir: PathBuf,
    output: PathBuf,
    verbose: bool,
    recursive: bool,
    optimization: String,
    target: String,
}

fn main() {
    let matches = App::new("SHDRR: Live compiler for SPIRV based on shaderc.")
        .arg(
            Arg::with_name("dir")
                .short("d")
                .help("Directory with the shaders")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("recursive")
                .short("r")
                .help("Look for shader file recursively"),
        )
        .arg(Arg::with_name("verbose").short("v").help("Verbose output"))
        .arg(
            Arg::with_name("output")
                .short("o")
                .help("Output directory")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("optimization")
                .short("O")
                .help("Optimization level")
                .long_help(
                    "The optimization level follow the ones used by shaderc: 
                1 or nothing is performance optimization (default value), 
                0 is no optimization for debugging, 
                s is optimization for size.",
                )
                .takes_value(true),
        )
        .get_matches();
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();

    let rec = matches.is_present("recursive");
    let mut dir: PathBuf;
    if matches.is_present("dir") {
        dir = PathBuf::from(matches.value_of("dir").unwrap());
        if rec {
            watcher
                .watch(dir.clone(), RecursiveMode::Recursive)
                .unwrap();
        } else {
            watcher
                .watch(dir.clone(), RecursiveMode::NonRecursive)
                .unwrap();
        }
    } else {
        dir = env::current_dir().unwrap();
        if rec {
            watcher
                .watch(dir.clone(), RecursiveMode::Recursive)
                .unwrap();
        } else {
            watcher
                .watch(dir.clone(), RecursiveMode::NonRecursive)
                .unwrap();
        }
    }

    let output_dir = matches
        .value_of("output")
        .map_or(dir.clone(), |o| PathBuf::from(o));
    let conf = ShdrrConf {
        dir,
        output: output_dir,
        verbose: matches.is_present("verbose"),
        recursive: rec,
        optimization: matches.value_of("optimization").unwrap_or("1").to_string(),
        target: String::new(),
    };

    loop {
        match rx.recv() {
            Ok(event) => handle_event(event, conf.clone()),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn load_file(file: &PathBuf) -> Option<String> {
    let contents = fs::read_to_string(file);
    match contents {
        Ok(file_str) => Some(file_str),
        Err(err) => {
            eprintln!("[ERR] Impossible to read file {} : {}", file.display(), err);

            None
        }
    }
}

fn get_shader_kind_from_filename(path: &PathBuf) -> Option<shaderc::ShaderKind> {
    let ext = path.extension();
    if ext.is_none() {
        return None;
    }

    let ext = ext.unwrap();
    match ext.to_str().unwrap() {
        "cs" => Some(shaderc::ShaderKind::Compute),
        "fs" => Some(shaderc::ShaderKind::Fragment),
        "vs" => Some(shaderc::ShaderKind::Vertex),
        _ => None,
    }
}

fn compile_shader(path: &PathBuf, conf: ShdrrConf) -> Option<Vec<u8>> {
    let kind = get_shader_kind_from_filename(path);
    if kind.is_none() {
        if conf.verbose {
            println!(
                "[NFO] Could not compile {}: No valid extension.",
                path.display()
            );
        }

        return None;
    }

    let kind = kind.unwrap();
    let src = load_file(path).unwrap();

    let mut compiler = shaderc::Compiler::new().unwrap();
    let mut options = shaderc::CompileOptions::new().unwrap();
    match conf.optimization.as_str() {
        "0" => options.set_optimization_level(shaderc::OptimizationLevel::Zero),
        "s" => options.set_optimization_level(shaderc::OptimizationLevel::Size),
        _ => options.set_optimization_level(shaderc::OptimizationLevel::Performance),
    };

    if conf.verbose {
        println!("[NFO] Compiling file {} ...", path.display());
    }

    let binary_result = compiler.compile_into_spirv(
        &src,
        kind,
        path.file_name().unwrap().to_str().unwrap(),
        "main",
        Some(&options),
    );
    if binary_result.is_err() {
        println!(
            "[ERR] Could not compile file {}: {}",
            path.display(),
            binary_result.err().unwrap()
        );
        return None;
    }

    if conf.verbose {
        println!("[NFO] Successfully compiled {} ...", path.display());
    }
    let bin = binary_result.unwrap().as_binary_u8().to_owned();
    Some(bin)
}

fn compile_and_write(path: &PathBuf, conf: ShdrrConf) {
    let bin = compile_shader(path, conf.clone());
    if bin.is_none() {
        return;
    }

    let bin = bin.unwrap();
    let ext = path.extension().unwrap();
    let filename = path.file_stem().unwrap();
    let new_filename = format!(
        "{}.{}.spriv",
        filename.to_str().unwrap(),
        ext.to_str().unwrap()
    );
    let mut output_path = PathBuf::new();
    output_path.push(conf.output);

    let dir_path_res = path.strip_prefix(conf.dir.canonicalize().unwrap());
    if dir_path_res.is_ok() {
        let dir_path = dir_path_res.unwrap();
        let intermediate_dir = dir_path.parent();
        if intermediate_dir.is_some() {
            output_path.push(intermediate_dir.unwrap());
        }
    }

    output_path.push(new_filename);

    if conf.verbose {
        println!("[NFO] Writing shader to {}", output_path.display());
    }

    let need_mkdir = output_path.parent().map_or(true, |p| !p.exists());
    if need_mkdir {
        fs::create_dir_all(output_path.parent().unwrap()).unwrap();
    }

    let mut file = File::create(output_path.clone()).unwrap();
    let write_res = file.write_all(&bin);
    if write_res.is_err() {
        println!(
            "[ERR] Error while writing shader to {}: {}",
            output_path.display(),
            write_res.err().unwrap()
        );
    } else if conf.verbose {
        println!("[NFO] Finished writing {}", output_path.display());
    }
}

fn handle_event(event: notify::DebouncedEvent, conf: ShdrrConf) {
    match event {
        notify::DebouncedEvent::Create(path) => compile_and_write(&path, conf),
        notify::DebouncedEvent::Write(path) => compile_and_write(&path, conf),
        notify::DebouncedEvent::Rename(_, path) => compile_and_write(&path, conf),
        _ => (),
    }
}
