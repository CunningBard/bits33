# Bits33
A register based virtual machine, made for programming languages that need a runtime.

# File Structure
```
bits33: the main folder/workspace
| --> bits33asm : assembler for bits33asm, turns .bit33asm files into .bits33exe files
| --> bits33core: shared library for internal parts, used for programs that need to use the bits33vm, e.g. compiling your language into bits33exe
| --> bits33vm  : the virtual machine, runs .bits33exe files
```

# Usage
## bits33asm
```
bits33asm <input file> | -o <output file>

input file: the file to assemble, not implemented yet
output file: the file to output to, not implemented yet
```

## bits33vm
```
bits33vm <input file> | -debug

input file: the file to run
-debug: enables debug mode, not implemented yet
```