mod bits;
mod sbc;

const SBC_HEADER_SIZE: usize = 4;
const SBC_PROBE_SIZE: usize = SBC_HEADER_SIZE;

const SBC_MAX_SUBBANDS: usize = 8;
const SBC_MAX_BLOCKS: usize = 16;
const SBC_MAX_SAMPLES: usize = SBC_MAX_BLOCKS * SBC_MAX_SUBBANDS;

const SBC_MSBC_SAMPLES: usize = 120;
const SBC_MSBC_SIZE: usize = 57;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = std::fs::read("../bluefang/target/sbc/output.sbc").unwrap();
        assert!(data.len() > SBC_PROBE_SIZE);

    }
}
