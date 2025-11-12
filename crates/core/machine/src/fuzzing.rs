use std::ops::ControlFlow;

use arbitrary::Arbitrary;
use p3_air::{AirBuilder, AirBuilderWithPublicValues};
use p3_field::FieldAlgebra;
use p3_koala_bear::KoalaBear;
use p3_matrix::dense::DenseMatrix;
use zkm_stark::{AirLookup, MessageBuilder};

pub trait DiffFuzzingTarget: Sized {
    type Input: for<'a> Arbitrary<'a>;
    type Result: Eq + std::fmt::Debug;

    fn create() -> Self;

    fn fuzz(&mut self, input: &Self::Input) -> ControlFlow<(), Self::Result>;

    fn oracle(input: &Self::Input) -> ControlFlow<(), Self::Result>;

    fn check(&self, input: &Self::Input);
}

#[derive(Default)]
pub struct FuzzingAirBuilder;

impl AirBuilderWithPublicValues for FuzzingAirBuilder {
    type PublicVar = KoalaBear;

    fn public_values(&self) -> &[Self::PublicVar] {
        todo!()
    }
}

impl<F: std::fmt::Debug> MessageBuilder<AirLookup<F>> for FuzzingAirBuilder {
    fn send(&mut self, message: AirLookup<F>, scope: zkm_stark::LookupScope) {
        match message.kind {
            zkm_stark::LookupKind::Memory => todo!(),
            zkm_stark::LookupKind::Program => todo!(),
            zkm_stark::LookupKind::Instruction => todo!(),
            zkm_stark::LookupKind::Alu => todo!(),
            zkm_stark::LookupKind::Byte => {
                for value in message.values {
                    format!("{value:?}").parse::<u8>().unwrap();
                }
            }
            zkm_stark::LookupKind::Range => todo!(),
            zkm_stark::LookupKind::Field => todo!(),
            zkm_stark::LookupKind::Syscall => todo!(),
            zkm_stark::LookupKind::Global => todo!(),
        }
    }

    fn receive(&mut self, message: AirLookup<F>, scope: zkm_stark::LookupScope) {
        todo!()
    }
}

impl AirBuilder for FuzzingAirBuilder {
    type F = KoalaBear;

    type Expr = KoalaBear;

    type Var = KoalaBear;

    type M = DenseMatrix<KoalaBear>;

    fn main(&self) -> Self::M {
        todo!()
    }

    fn is_first_row(&self) -> Self::Expr {
        todo!()
    }

    fn is_last_row(&self) -> Self::Expr {
        todo!()
    }

    fn is_transition_window(&self, size: usize) -> Self::Expr {
        todo!()
    }

    fn assert_zero<I: Into<Self::Expr>>(&mut self, x: I) {
        assert_eq!(x.into(), Self::F::ZERO);
    }
}
