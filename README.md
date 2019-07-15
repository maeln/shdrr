# SHDRR: Live shader compiler 

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
    -h, --help       Prints help information
    -r               Look for shader file recursively
    -V, --version    Prints version information
    -v               Verbose output

OPTIONS:
    -d <dir>           Directory with the shaders
    -o <output>        Output directory
```

Exemple: To compile every shader contained in the directory `./shaders/src` and its sub-directory to `.shaders/bin`, with verbosity:

`shdrr -rv -d shaders/src -o shaders/bin`

## Installation

With `cargo`:
```
cargo install shdrr
```