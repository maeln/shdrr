# SHDRR: Live shader compiler 

[![Crates.io](https://img.shields.io/crates/v/shdrr.svg)](https://crates.io/crates/shdrr)
[![Build Status](https://travis-ci.org/maeln/shdrr.svg?branch=master)](https://travis-ci.org/maeln/shdrr)

SHDRR is a live shader compiler, as in, it will automatically detect if a file changed within a directory and recompile it to SPIR-V using [shaderc](https://github.com/google/shaderc).

Right now, it detect the shader type (compute, vertex, fragment) using the file extension:
- *.cs -> compute
- *.fs -> fragment
- *.vs -> vertex


## Usage

See `shdrr --help` for usage:
```
SHDRR: Live compiler for SPIRV based on shaderc. 

USAGE:
    shdrr [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       
            Prints help information

    -r               
            Look for shader file recursively

    -V, --version    
            Prints version information

    -v               
            Verbose output


OPTIONS:
    -d <dir>                 
            Directory with the shaders

    -e <env>                 
            This option let you choose the target environnement for Shaderc, 
                        
                        Accepted value are: 
                        vulkan, opengl, opengl_compat [default: vulkan]
    -O <optimization>        
            The optimization level follow the ones used by shaderc: 
                            1 or nothing is performance optimization, 
                            0 is no optimization for debugging, 
                            s is optimization for size. [default: 1]
    -o <output>              
            Output directory

    -s <spirv>               
            This option let you choose the SPIR-V version to be used for the compiled shader,
                        
                        Accepted value are: 1.0, 1.1, 1.2, 1.3, 1.4 [default: 1.0]
```

Exemple: To compile every shader contained in the directory `./shaders/src` and its sub-directory to `.shaders/bin`, with verbosity:

`shdrr -rv -d shaders/src -o shaders/bin`

## Installation

With `cargo`:
```
cargo install shdrr
```