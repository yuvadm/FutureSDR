mod delay;
pub use delay::Delay;

mod frame_equalizer;
pub use frame_equalizer::FrameEqualizer;

mod moving_average;
pub use moving_average::MovingAverage;

mod sync_long;
pub use sync_long::SyncLong;

mod sync_short;
pub use sync_short::SyncShort;

#[derive(Clone, Copy)]
pub enum Modulation {
    Bpsk,
    Qpsk,
    Qam16,
    Qam64,
}

#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Mcs {
    Bpsk_1_2,
    Bpsk_3_4,
    Qpsk_1_2,
    Qpsk_3_4,
    Qam16_1_2,
    Qam16_3_4,
    Qam64_2_3,
    Qam64_3_4,
}

impl Mcs {
    pub fn modulation(&self) -> Modulation {
        match self {
            Mcs::Bpsk_1_2 => Modulation::Bpsk,
            Mcs::Bpsk_3_4 => Modulation::Bpsk,
            Mcs::Qpsk_1_2 => Modulation::Qpsk,
            Mcs::Qpsk_3_4 => Modulation::Qpsk,
            Mcs::Qam16_1_2 => Modulation::Qam16,
            Mcs::Qam16_3_4 => Modulation::Qam16,
            Mcs::Qam64_2_3 => Modulation::Qam64,
            Mcs::Qam64_3_4 => Modulation::Qam64,
        }
    }

    // coded bits per symbol
    pub fn cbps(&self) -> usize {
        match self {
            Mcs::Bpsk_1_2 => 48,
            Mcs::Bpsk_3_4 => 48,
            Mcs::Qpsk_1_2 => 96,
            Mcs::Qpsk_3_4 => 96,
            Mcs::Qam16_1_2 => 192,
            Mcs::Qam16_3_4 => 192,
            Mcs::Qam64_2_3 => 288,
            Mcs::Qam64_3_4 => 288,
        }
    }

    // data bits per symbol
    pub fn dbps(&self) -> usize {
        match self {
            Mcs::Bpsk_1_2 => 24,
            Mcs::Bpsk_3_4 => 36,
            Mcs::Qpsk_1_2 => 48,
            Mcs::Qpsk_3_4 => 72,
            Mcs::Qam16_1_2 => 96,
            Mcs::Qam16_3_4 => 144,
            Mcs::Qam64_2_3 => 192,
            Mcs::Qam64_3_4 => 216,
        }
    }
}

pub struct FrameParam {
    mcs: Mcs,
    bytes: usize,
}

impl FrameParam {
    pub fn psdu_size(&self) -> usize {
        self.bytes
    }

    pub fn mcs(&self) -> Mcs {
        self.mcs
    }

    pub fn modulation(&self) -> Modulation {
        self.mcs.modulation()
    }

    pub fn symbols(&self) -> usize {
        let bits = 16 + 8 * self.bytes + 6;

        let mut syms = bits / self.mcs.dbps();
        if bits % self.mcs.dbps() > 0 {
            syms += 1;
        }

        syms
    }
}
