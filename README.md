# AsmShellCliRust

An interactive assembly shell written in Rust. This project is a Rust command-line interface (CLI) application that provides a machine emulator and assembler functionality. It allows you to emulate and assemble instructions for different CPU architectures.

## Overview

`AsmShellCliRust` provides an interactive shell for working with assembly language. It supports both x86 and x64 instruction sets, allowing you to write and execute assembly instructions directly from the command line.
![e02a29d22e9a3d870222751b82cba89f](https://github.com/Shahupdates/AsmShellCliRust/assets/120000782/d416321e-78eb-4b07-9981-5dd424df33b9)
## Files

The project consists of the following files:

- `main.rs`: The main entry point of the CLI application. It contains the `main` function and the command-line argument parsing logic.
- `machine.rs`: Contains the implementation of the `Machine` struct, which represents the machine emulator. It provides functions for initializing the machine, manipulating registers and memory, and executing instructions.
- `cpu.rs`: Defines the CPU architecture and mode enums (`Arch` and `Mode`) and provides implementations for converting between different CPU modes and architectures. It also includes the `ArchMeta` trait and its implementations (`X32` and `X64`), which define the meta information and operations for specific CPU architectures.
- `mod.rs`: A module file that includes the `cpu` and `machine` modules.

## Usage

To use this project and start the interactive assembly shell, allowing you to enter assembly instructions and execute them, you can run the following command:

```sh
cargo run --release
```

## `main.rs`

The `main.rs` file is the entry point of the CLI application. It uses the `clap` crate for command-line argument parsing. The `Args` struct defines the available command-line options and arguments.

The `main` function performs the following steps:

1. Parses the command-line arguments using the `Args::parse` method.
2. Retrieves the desired architecture and assembler syntax from the parsed arguments.
3. Initializes the machine emulator by calling the appropriate functions in the `Machine` module.
4. Sets up the initial state of the machine, including stack pointer, stack frame, and memory mapping.
5. Enters a loop to read user input from the command line using the `rustyline` crate.
6. Assembles the user input using the machine's `asm` method and executes the assembled instructions.
7. Prints the mnemonic, hex representation, and modifies the machine's state accordingly.
8. The loop continues until the user interrupts it (e.g., by pressing Ctrl-C or Ctrl-D).

## `machine.rs`

The `machine.rs` file contains the implementation of the `Machine` struct, which represents the machine emulator. The struct includes fields for the register map, assembler, Unicorn engine, CPU information, stack pointer, and stack frame.

The `Machine` struct provides several methods:

- `new_from_arch`: Initializes a new `Machine` instance based on the specified architecture.
- `init_unicorn`: Initializes the Unicorn engine for the specified architecture and mode.
- `init_keystone`: Initializes the Keystone assembler for the specified architecture and mode.
- `get_arch_name`: Retrieves the architecture name based on the provided architecture and mode.
- `get_arch_meta`: Retrieves the architecture metadata based on the provided architecture and mode.
- `set_sp`: Sets the stack pointer value for the machine.
- `set_fp`: Sets the stack frame value for the machine.
- `print_machine`: Prints the current machine architecture and mode.
- `print_register`: Prints the current register values of the machine.
- `asm`: Assembles the provided string instruction using the machine's assembler.
- `write_instruction`: Writes the assembled instruction to memory and executes it using the Unicorn engine.
- `print_stack`: Prints the current stack context of the machine.

## `cpu.rs`

The `cpu.rs` file defines the CPU architecture and mode enums (`Arch` and `Mode`) and provides implementations for converting between different CPU modes and architectures. It also includes the `ArchMeta` trait and its implementations (`X32` and `X64`), which define the meta information and operations for specific CPU architectures.

The `Arch` enum represents different CPU architectures, and the `Mode` enum represents different CPU modes (32-bit or 64-bit). The file provides implementations to convert between these enums and the corresponding enums used by the Unicorn and Keystone libraries.

The `ArchMeta` trait defines the common operations and metadata for different CPU architectures. It includes methods such as `cpu` (returns the CPU architecture and mode), `sp_reg` (returns the stack pointer register value), `fp_reg` (returns the stack frame register value), `int_size` (returns the integer size in bytes), `sorted_reg_names` (returns the sorted names of the registers), `register_map` (returns the mapping of register names to register values), and `dump_registers` (dumps the register values for the machine).

The file also provides two implementations of the `ArchMeta` trait: `X32` for 32-bit x86 architecture and `X64` for 64-bit x86 architecture.

## `mod.rs`

The `mod.rs` file acts as a module file that includes the `cpu` and `machine` modules. It helps organize and manage the project's modules.

## Dependencies

The project depends on the following external crates:

- `ansi_term`: A crate for coloring and styling terminal output.
- `clap`: A crate for command-line argument parsing.
- `keystone`: A crate for the Keystone assembler.
- `rustyline`: A crate for readline-like functionality.
- `unicorn_engine`: A crate for the Unicorn CPU emulator.

Make sure to include these dependencies in your project's `Cargo.toml` file to ensure that the application compiles and runs correctly.

## Contributing

Contributions to `AsmShellCliRust` are welcome! If you encounter any issues or have suggestions for improvements, please open an issue on the GitHub repository.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/Shahupdates/AsmShellCliRust/blob/main/LICENSE) file for more information.
