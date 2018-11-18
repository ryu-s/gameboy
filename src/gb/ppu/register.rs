use super::super::bus::Bus;
use super::Mode;

#[derive(Debug, Copy, Clone)]
pub enum Register {
    LCDC, // LCD Control
    STAT, // LCD Control Status
    SCY,  // Scroll Y
    SCX,  // Scroll X
    LY,   // Y-Coordinate
    LYC,  // LY Compare
    DMA,  // DMA Transfer and Start Address
    BGP,  // BG Palette Data
    OBP0, // Object Palette 0 Data
    OBP1, // Object Palette 1 Data
    WY,   // Window Y Position
    WX,   // Window X Position - 7
}

impl Register {
    pub fn read<B: Bus>(&self, bus: &mut B) -> u8 {
        bus.read8(self.address())
    }

    pub fn write<B: Bus>(&self, bus: &mut B, v: u8) {
        bus.write8(self.address(), v);
    }

    fn address(&self) -> u16 {
        use self::Register::*;

        match *self {
            LCDC => 0xFF40,
            STAT => 0xFF41,
            SCY => 0xFF42,
            SCX => 0xFF43,
            LY => 0xFF44,
            LYC => 0xFF45,
            DMA => 0xFF46,
            BGP => 0xFF47,
            OBP0 => 0xFF48,
            OBP1 => 0xFF49,
            WY => 0xFF4A,
            WX => 0xFF4B,
        }
    }
}

#[derive(Copy, Clone)]
pub struct LCDStatus(u8);

impl LCDStatus {
    pub fn new(v: u8) -> Self {
        LCDStatus(v)
    }

    pub fn raw(&self) -> u8 {
        self.0
    }

    pub fn mode(&self) -> Mode {
        match self.0 & 0b0000_0011 {
            0 => Mode::HBlank,
            1 => Mode::VBlank,
            2 => Mode::OAMRead,
            3 => Mode::VRAMRead,
            _ => panic!("unreachable"),
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.0 &= 0b1111_1100;
        self.0 |= match mode {
            Mode::HBlank => 0b00,
            Mode::VBlank => 0b01,
            Mode::OAMRead => 0b10,
            Mode::VRAMRead => 0b11,
        };
    }

    pub fn set_lyc_coincidence(&mut self, v: bool) {
        self.0 &= 0b1111_1011;
        if v {
            self.0 |= 0b0100;
        }
    }

    pub fn is_hblank_interrupt_enabled(&self) -> bool {
        self.0 & 0b0000_1000 != 0
    }

    pub fn is_vblank_interrupt_enabled(&self) -> bool {
        self.0 & 0b0001_0000 != 0
    }

    pub fn is_oam_interrupt_enabled(&self) -> bool {
        self.0 & 0b0010_0000 != 0
    }

    pub fn is_lyc_coincidence_interrupt_enabled(&self) -> bool {
        self.0 & 0b0100_0000 != 0
    }
}
