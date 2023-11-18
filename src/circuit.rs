use mpz_circuits::{Circuit, CircuitBuilder};

pub fn build_le_circuit() -> Circuit {
    let builder = CircuitBuilder::new();

    let a = builder.add_input::<u16>();
    let b = builder.add_input::<u16>();

    // subtract a - b
    let c = a.wrapping_sub(b);

    // check if zero
    let is_zero = c.bitand(0u16);

    // check if negative
    let is_neg = c.bitand(0x8000u16);

    // check if not zero and not negative
    let is_pos = is_zero.bitor(is_neg);

    builder.add_output(res);

    builder.build().unwrap()
}

pub fn build_battle_circuit() -> Circuit {
    let builder = CircuitBuilder::new();

    let jmp_1 = builder.add_input::<u8>();
    let jmp_2 = builder.add_input::<u8>();

    let res = jmp_1.wrapping_add(jmp_2);
}

#[cfg(test)]

mod test {
    use mpz_circuits_macros::evaluate;

    #[test]
    fn test_le() {
        let circ = build_le_circuit();

        let a = 0x0001u16;
        let b = 0x0002u16;

        let output = evaluate!(circ, fn(a, b) -> u8).unwrap();

        assert_eq!(output, 0x0001u16);
    }
}
