use ansi_term::Colour::Red;
use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::{Cmd, Editor, KeyCode, KeyEvent, Modifiers};

pub mod engine;

use crate::engine::machine::Machine;

#[derive(clap::ValueEnum, Clone, Debug, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
enum AssemblerSyntax {
    INTEL,
    ATT,
    GAS,
    MASM,
    NASM,
}

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(short, long)]
    arch: Option<String>,

    #[arg(short, long, value_enum)]
    syntax: Option<AssemblerSyntax>,
}

fn get_machine(arch_name: String) -> Machine<'static> {
    Machine::new_from_arch(arch_name.as_str()).unwrap()
}

fn main() {
    let args = Args::parse();

    let arch_name = match args.arch {
        Some(r) => r,
        None => "x64".to_string(),
    };
    let ass_syntax = match args.syntax {
        Some(syntax) => match syntax {
            AssemblerSyntax::INTEL => OptionValue::SYNTAX_INTEL,
            AssemblerSyntax::ATT => OptionValue::SYNTAX_ATT,
            AssemblerSyntax::GAS => OptionValue::SYNTAX_GAS,
            AssemblerSyntax::MASM => OptionValue::SYNTAX_MASM,
            AssemblerSyntax::NASM => OptionValue::SYNTAX_NASM,
        },
        None => OptionValue::SYNTAX_INTEL,
    };

    let mut m: Machine = get_machine(arch_name);

    // Machine initialization
    m.assembler
        .option(OptionType::SYNTAX, ass_syntax)
        .expect("failed to change assembler syntax");

    m.print_machine();
    m.print_register();
    m.print_stack();

    let mut rl = Editor::<()>::new();
    rl.bind_sequence(KeyEvent(KeyCode::Down, Modifiers::NONE), Cmd::NextHistory);
    rl.bind_sequence(KeyEvent(KeyCode::Up, Modifiers::NONE), Cmd::PreviousHistory);

    loop {
        let input = rl.readline(Red.paint(">> ").to_string().as_str());
        match input {
            Ok(line) => {
                let result = m.asm(line.to_string(), 0);
                match result {
                    Ok(r) => {
                        rl.add_history_entry(line.as_str());
                        println!(
                            "{} : {} {} : {}",
                            Red.paint("mnemonic"),
                            line.trim(),
                            Red.paint("hex"),
                            r
                        );
                        m.write_instruction(r.bytes);
                        m.print_register();
                        m.print_stack();
                    }
                    Err(e) => println!("failed to assemble, err: {:?}", e),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
