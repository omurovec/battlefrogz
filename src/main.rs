use core::ops::{BitAnd, BitOr};
use mpz_circuits::{ops::WrappingSub, Circuit, CircuitBuilder};

pub fn build_le_circuit() -> Circuit {
    let builder = CircuitBuilder::new();

    let a = builder.add_input::<u16>();
    let b = builder.add_input::<u16>();

    let c = a.bitand(0u16);

    builder.add_output(c);

    builder.build().unwrap()
}

pub fn main() {
    build_le_circuit();
}

#[cfg(test)]

mod test {
    use super::*;
    use mpz_circuits_macros::evaluate;

    #[test]
    fn test_le() {
        let circ = build_le_circuit();

        let a = 0x0001u16;
        let b = 0x0002u16;

        let output = evaluate!(circ, fn(a, b) -> u16).unwrap();

        assert_eq!(output, 0x0001u16);
    }
}
