# SHDRR: Live shader compiler 

SHDRR is a live shader compiler, as in, it will automatically detect if a file changed within a directory and recompile it to SPIR-V using [shaderc](https://github.com/google/shaderc).

Right now, it detect the shader type (compute, vertex, fragment) using the file extension:
- *.cs -> compute
- *.fs -> fragment
- *.vs -> vertex


## Usage

See `shadrr --help` for usage.

## Installation

With `cargo`:
```
cargo install shdrr
```