use clap::{Parser, ValueEnum};
use p3_koala_bear::KoalaBear;
use zkm_core_machine::operations::{
    Add4Operation, Add5Operation, AddDoubleOperation, AddOperation, AndOperation, AssertLtColsBits,
    AssertLtColsBytes, FixedRotateRightOperation, FixedShiftRightOperation, GtColsBytes,
    IsEqualWordOperation, IsZeroOperation, IsZeroWordOperation, KoalaBearBitDecomposition,
    KoalaBearWordRangeChecker, NotOperation, OrOperation, XorOperation,
};

#[derive(Debug, Copy, Clone, ValueEnum)]
enum Mode {
    OpAdd,
    OpAdd4,
    OpAdd5,
    OpAddDouble,
    OpAnd,
    OpFixedRotateRight,
    OpFixedShiftRight,
    OpIsEqualWord,
    OpIsZero,
    OpIsZeroWord,
    OpNot,
    OpOr,
    OpXor,
    KbBitDecomposition,
    KbRangeChecker,
    GtBytes,
    AssertLtBytes,
    AssertLtBits,
}

fn main() {
    let opt = Opt::parse();
    match opt.mode {
        Mode::OpAdd => diff_fuzz::<AddOperation<KoalaBear>>(),
        Mode::OpAdd4 => diff_fuzz::<Add4Operation<KoalaBear>>(),
        Mode::OpAdd5 => diff_fuzz::<Add5Operation<KoalaBear>>(),
        Mode::OpAddDouble => diff_fuzz::<AddDoubleOperation<KoalaBear>>(),
        Mode::OpAnd => diff_fuzz::<AndOperation<KoalaBear>>(),
        Mode::OpFixedRotateRight => diff_fuzz::<FixedRotateRightOperation<KoalaBear>>(),
        Mode::OpFixedShiftRight => diff_fuzz::<FixedShiftRightOperation<KoalaBear>>(),
        Mode::OpIsEqualWord => diff_fuzz::<IsEqualWordOperation<KoalaBear>>(),
        Mode::OpIsZero => diff_fuzz::<IsZeroOperation<KoalaBear>>(),
        Mode::OpIsZeroWord => diff_fuzz::<IsZeroWordOperation<KoalaBear>>(),
        Mode::OpNot => diff_fuzz::<NotOperation<KoalaBear>>(),
        Mode::OpOr => diff_fuzz::<OrOperation<KoalaBear>>(),
        Mode::OpXor => diff_fuzz::<XorOperation<KoalaBear>>(),
        Mode::KbBitDecomposition => diff_fuzz::<KoalaBearBitDecomposition<KoalaBear>>(),
        Mode::KbRangeChecker => diff_fuzz::<KoalaBearWordRangeChecker<KoalaBear>>(),
        Mode::GtBytes => diff_fuzz::<GtColsBytes<KoalaBear>>(),
        Mode::AssertLtBytes => diff_fuzz::<AssertLtColsBytes<KoalaBear, 4>>(),
        Mode::AssertLtBits => diff_fuzz::<AssertLtColsBits<KoalaBear, 32>>(),
    }
}

fn diff_fuzz<T>()
where
    T: zkm_core_machine::fuzzing::DiffFuzzingTarget,
{
    afl::fuzz!(|data: T::Input| {
        let oracle = T::oracle(&data);
        if matches!(oracle, std::ops::ControlFlow::Break(_)) {
            return;
        }
        let mut op = T::create();
        let output = op.fuzz(&data);
        if matches!(output, std::ops::ControlFlow::Break(_)) {
            return;
        }
        assert_eq!(output.continue_value().unwrap(), oracle.continue_value().unwrap());
        op.check(&data);
    });
    println!("Fuzzing target finished without issues!");
}

#[derive(Parser)]
struct Opt {
    #[arg(short = 'm')]
    mode: Mode,
}
