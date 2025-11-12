use clap::{Parser, ValueEnum};
use zkm_core_executor::{ExecutionError, Executor, Instruction, Opcode, Program};
use zkm_stark::ZKMCoreOpts;

fn main() {
    let opt = Opt::parse();
    fuzz(match opt.mode {
        Mode::Raw => raw_mode,
        Mode::Elf => elf_mode,
    })
}

fn fuzz(mode: fn(&[u8]) -> ()) {
    afl::fuzz!(|data: &[u8]| { mode(data) });
    println!("Fuzzing target finished without issues!");
}

fn raw_mode(data: &[u8]) {
    let (insns, _): (&[[_; 4]], _) = data.as_chunks();
    let insns: Vec<_> = insns
        .iter()
        .map(|data| u32::from_le_bytes(*data))
        .flat_map(Instruction::decode_from)
        .filter(|i| i.opcode != Opcode::UNIMPL)
        .collect();
    if insns.is_empty() {
        return;
    }
    let program = Program::new(insns, 0, 0);

    let mut runtime = Executor::new(program, ZKMCoreOpts::default());
    use ExecutionError::*;
    match runtime.run() {
        Ok(_)
        | Err(
            HaltWithNonZeroExitCode(_)
            | InvalidMemoryAccess(_, _)
            | UnsupportedSyscall(_)
            | UnsupportedInstruction(_)
            | Breakpoint()
            | InvalidSyscallUsage(_),
        ) => {}
        Err(err) => panic!("{err}"),
    }
}

fn elf_mode(data: &[u8]) {
    todo!();
}

#[derive(Parser)]
struct Opt {
    #[arg(short = 'm')]
    mode: Mode,
}

#[derive(Debug, Copy, Clone, ValueEnum)]
enum Mode {
    Raw,
    Elf,
}
