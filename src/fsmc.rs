# ! [ doc = "Flexible static memory controller" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;
use vcell::VolatileCell;

# [ doc = "Flexible static memory controller" ]
pub const FSMC: Peripheral<FSMC> = unsafe { Peripheral::new(2684354560) };

# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - SRAM/NOR-Flash chip-select control register 1" ]
    pub bcr1: BCR1,
    # [ doc = "0x04 - SRAM/NOR-Flash chip-select timing register 1" ]
    pub btr1: BTR1,
    # [ doc = "0x08 - SRAM/NOR-Flash chip-select control register 2" ]
    pub bcr2: BCR2,
    # [ doc = "0x0c - SRAM/NOR-Flash chip-select timing register 2" ]
    pub btr2: BTR2,
    # [ doc = "0x10 - SRAM/NOR-Flash chip-select control register 3" ]
    pub bcr3: BCR3,
    # [ doc = "0x14 - SRAM/NOR-Flash chip-select timing register 3" ]
    pub btr3: BTR3,
    # [ doc = "0x18 - SRAM/NOR-Flash chip-select control register 4" ]
    pub bcr4: BCR4,
    # [ doc = "0x1c - SRAM/NOR-Flash chip-select timing register 4" ]
    pub btr4: BTR4,
    _reserved0: [u8; 64usize],
    # [ doc = "0x60 - PC Card/NAND Flash control register 2" ]
    pub pcr2: PCR2,
    # [ doc = "0x64 - FIFO status and interrupt register 2" ]
    pub sr2: SR2,
    # [ doc = "0x68 - Common memory space timing register 2" ]
    pub pmem2: PMEM2,
    # [ doc = "0x6c - Attribute memory space timing register 2" ]
    pub patt2: PATT2,
    _reserved1: [u8; 4usize],
    # [ doc = "0x74 - ECC result register 2" ]
    pub eccr2: ECCR2,
    _reserved2: [u8; 8usize],
    # [ doc = "0x80 - PC Card/NAND Flash control register 3" ]
    pub pcr3: PCR3,
    # [ doc = "0x84 - FIFO status and interrupt register 3" ]
    pub sr3: SR3,
    # [ doc = "0x88 - Common memory space timing register 3" ]
    pub pmem3: PMEM3,
    # [ doc = "0x8c - Attribute memory space timing register 3" ]
    pub patt3: PATT3,
    _reserved3: [u8; 4usize],
    # [ doc = "0x94 - ECC result register 3" ]
    pub eccr3: ECCR3,
    _reserved4: [u8; 8usize],
    # [ doc = "0xa0 - PC Card/NAND Flash control register 4" ]
    pub pcr4: PCR4,
    # [ doc = "0xa4 - FIFO status and interrupt register 4" ]
    pub sr4: SR4,
    # [ doc = "0xa8 - Common memory space timing register 4" ]
    pub pmem4: PMEM4,
    # [ doc = "0xac - Attribute memory space timing register 4" ]
    pub patt4: PATT4,
    # [ doc = "0xb0 - I/O space timing register 4" ]
    pub pio4: PIO4,
    _reserved5: [u8; 80usize],
    # [ doc = "0x104 - SRAM/NOR-Flash write timing registers 1" ]
    pub bwtr1: BWTR1,
    _reserved6: [u8; 4usize],
    # [ doc = "0x10c - SRAM/NOR-Flash write timing registers 2" ]
    pub bwtr2: BWTR2,
    _reserved7: [u8; 4usize],
    # [ doc = "0x114 - SRAM/NOR-Flash write timing registers 3" ]
    pub bwtr3: BWTR3,
    _reserved8: [u8; 4usize],
    # [ doc = "0x11c - SRAM/NOR-Flash write timing registers 4" ]
    pub bwtr4: BWTR4,
}
# [ doc = "SRAM/NOR-Flash chip-select control register 1" ]
pub struct BCR1 {
    register: VolatileCell<u32>,
}
# [ doc = "SRAM/NOR-Flash chip-select control register 1" ]
pub mod bcr1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::BCR1 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CBURSTRWR {
        bits: bool,
    }
    impl CBURSTRWR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ASYNCWAITR {
        bits: bool,
    }
    impl ASYNCWAITR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EXTMODR {
        bits: bool,
    }
    impl EXTMODR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WAITENR {
        bits: bool,
    }
    impl WAITENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WRENR {
        bits: bool,
    }
    impl WRENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WAITCFGR {
        bits: bool,
    }
    impl WAITCFGR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WAITPOLR {
        bits: bool,
    }
    impl WAITPOLR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct BURSTENR {
        bits: bool,
    }
    impl BURSTENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct FACCENR {
        bits: bool,
    }
    impl FACCENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MWIDR {
        bits: u8,
    }
    impl MWIDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MTYPR {
        bits: u8,
    }
    impl MTYPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MUXENR {
        bits: bool,
    }
    impl MUXENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MBKENR {
        bits: bool,
    }
    impl MBKENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CBURSTRWW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CBURSTRWW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ASYNCWAITW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ASYNCWAITW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EXTMODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTMODW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WAITENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAITENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WRENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WRENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WAITCFGW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAITCFGW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WAITPOLW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAITPOLW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _BURSTENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BURSTENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _FACCENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FACCENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MWIDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MWIDW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MTYPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MTYPW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MUXENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MUXENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MBKENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MBKENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 19 - CBURSTRW" ]
        # [ inline ( always ) ]
        pub fn cburstrw(&self) -> CBURSTRWR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CBURSTRWR { bits }
        }
        # [ doc = "Bit 15 - ASYNCWAIT" ]
        # [ inline ( always ) ]
        pub fn asyncwait(&self) -> ASYNCWAITR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ASYNCWAITR { bits }
        }
        # [ doc = "Bit 14 - EXTMOD" ]
        # [ inline ( always ) ]
        pub fn extmod(&self) -> EXTMODR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTMODR { bits }
        }
        # [ doc = "Bit 13 - WAITEN" ]
        # [ inline ( always ) ]
        pub fn waiten(&self) -> WAITENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAITENR { bits }
        }
        # [ doc = "Bit 12 - WREN" ]
        # [ inline ( always ) ]
        pub fn wren(&self) -> WRENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WRENR { bits }
        }
        # [ doc = "Bit 11 - WAITCFG" ]
        # [ inline ( always ) ]
        pub fn waitcfg(&self) -> WAITCFGR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAITCFGR { bits }
        }
        # [ doc = "Bit 9 - WAITPOL" ]
        # [ inline ( always ) ]
        pub fn waitpol(&self) -> WAITPOLR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAITPOLR { bits }
        }
        # [ doc = "Bit 8 - BURSTEN" ]
        # [ inline ( always ) ]
        pub fn bursten(&self) -> BURSTENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BURSTENR { bits }
        }
        # [ doc = "Bit 6 - FACCEN" ]
        # [ inline ( always ) ]
        pub fn faccen(&self) -> FACCENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FACCENR { bits }
        }
        # [ doc = "Bits 4:5 - MWID" ]
        # [ inline ( always ) ]
        pub fn mwid(&self) -> MWIDR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MWIDR { bits }
        }
        # [ doc = "Bits 2:3 - MTYP" ]
        # [ inline ( always ) ]
        pub fn mtyp(&self) -> MTYPR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MTYPR { bits }
        }
        # [ doc = "Bit 1 - MUXEN" ]
        # [ inline ( always ) ]
        pub fn muxen(&self) -> MUXENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MUXENR { bits }
        }
        # [ doc = "Bit 0 - MBKEN" ]
        # [ inline ( always ) ]
        pub fn mbken(&self) -> MBKENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MBKENR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 12496 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 19 - CBURSTRW" ]
        # [ inline ( always ) ]
        pub fn cburstrw(&mut self) -> _CBURSTRWW {
            _CBURSTRWW { w: self }
        }
        # [ doc = "Bit 15 - ASYNCWAIT" ]
        # [ inline ( always ) ]
        pub fn asyncwait(&mut self) -> _ASYNCWAITW {
            _ASYNCWAITW { w: self }
        }
        # [ doc = "Bit 14 - EXTMOD" ]
        # [ inline ( always ) ]
        pub fn extmod(&mut self) -> _EXTMODW {
            _EXTMODW { w: self }
        }
        # [ doc = "Bit 13 - WAITEN" ]
        # [ inline ( always ) ]
        pub fn waiten(&mut self) -> _WAITENW {
            _WAITENW { w: self }
        }
        # [ doc = "Bit 12 - WREN" ]
        # [ inline ( always ) ]
        pub fn wren(&mut self) -> _WRENW {
            _WRENW { w: self }
        }
        # [ doc = "Bit 11 - WAITCFG" ]
        # [ inline ( always ) ]
        pub fn waitcfg(&mut self) -> _WAITCFGW {
            _WAITCFGW { w: self }
        }
        # [ doc = "Bit 9 - WAITPOL" ]
        # [ inline ( always ) ]
        pub fn waitpol(&mut self) -> _WAITPOLW {
            _WAITPOLW { w: self }
        }
        # [ doc = "Bit 8 - BURSTEN" ]
        # [ inline ( always ) ]
        pub fn bursten(&mut self) -> _BURSTENW {
            _BURSTENW { w: self }
        }
        # [ doc = "Bit 6 - FACCEN" ]
        # [ inline ( always ) ]
        pub fn faccen(&mut self) -> _FACCENW {
            _FACCENW { w: self }
        }
        # [ doc = "Bits 4:5 - MWID" ]
        # [ inline ( always ) ]
        pub fn mwid(&mut self) -> _MWIDW {
            _MWIDW { w: self }
        }
        # [ doc = "Bits 2:3 - MTYP" ]
        # [ inline ( always ) ]
        pub fn mtyp(&mut self) -> _MTYPW {
            _MTYPW { w: self }
        }
        # [ doc = "Bit 1 - MUXEN" ]
        # [ inline ( always ) ]
        pub fn muxen(&mut self) -> _MUXENW {
            _MUXENW { w: self }
        }
        # [ doc = "Bit 0 - MBKEN" ]
        # [ inline ( always ) ]
        pub fn mbken(&mut self) -> _MBKENW {
            _MBKENW { w: self }
        }
    }
}
# [ doc = "SRAM/NOR-Flash chip-select timing register 1" ]
pub struct BTR1 {
    register: VolatileCell<u32>,
}
# [ doc = "SRAM/NOR-Flash chip-select timing register 1" ]
pub mod btr1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::BTR1 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ACCMODR {
        bits: u8,
    }
    impl ACCMODR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATLATR {
        bits: u8,
    }
    impl DATLATR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CLKDIVR {
        bits: u8,
    }
    impl CLKDIVR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct BUSTURNR {
        bits: u8,
    }
    impl BUSTURNR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATASTR {
        bits: u8,
    }
    impl DATASTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ADDHLDR {
        bits: u8,
    }
    impl ADDHLDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ADDSETR {
        bits: u8,
    }
    impl ADDSETR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ACCMODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ACCMODW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DATLATW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATLATW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CLKDIVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CLKDIVW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _BUSTURNW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BUSTURNW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DATASTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATASTW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADDHLDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADDHLDW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADDSETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADDSETW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 28:29 - ACCMOD" ]
        # [ inline ( always ) ]
        pub fn accmod(&self) -> ACCMODR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ACCMODR { bits }
        }
        # [ doc = "Bits 24:27 - DATLAT" ]
        # [ inline ( always ) ]
        pub fn datlat(&self) -> DATLATR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATLATR { bits }
        }
        # [ doc = "Bits 20:23 - CLKDIV" ]
        # [ inline ( always ) ]
        pub fn clkdiv(&self) -> CLKDIVR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CLKDIVR { bits }
        }
        # [ doc = "Bits 16:19 - BUSTURN" ]
        # [ inline ( always ) ]
        pub fn busturn(&self) -> BUSTURNR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            BUSTURNR { bits }
        }
        # [ doc = "Bits 8:15 - DATAST" ]
        # [ inline ( always ) ]
        pub fn datast(&self) -> DATASTR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATASTR { bits }
        }
        # [ doc = "Bits 4:7 - ADDHLD" ]
        # [ inline ( always ) ]
        pub fn addhld(&self) -> ADDHLDR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ADDHLDR { bits }
        }
        # [ doc = "Bits 0:3 - ADDSET" ]
        # [ inline ( always ) ]
        pub fn addset(&self) -> ADDSETR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ADDSETR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 4294967295 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 28:29 - ACCMOD" ]
        # [ inline ( always ) ]
        pub fn accmod(&mut self) -> _ACCMODW {
            _ACCMODW { w: self }
        }
        # [ doc = "Bits 24:27 - DATLAT" ]
        # [ inline ( always ) ]
        pub fn datlat(&mut self) -> _DATLATW {
            _DATLATW { w: self }
        }
        # [ doc = "Bits 20:23 - CLKDIV" ]
        # [ inline ( always ) ]
        pub fn clkdiv(&mut self) -> _CLKDIVW {
            _CLKDIVW { w: self }
        }
        # [ doc = "Bits 16:19 - BUSTURN" ]
        # [ inline ( always ) ]
        pub fn busturn(&mut self) -> _BUSTURNW {
            _BUSTURNW { w: self }
        }
        # [ doc = "Bits 8:15 - DATAST" ]
        # [ inline ( always ) ]
        pub fn datast(&mut self) -> _DATASTW {
            _DATASTW { w: self }
        }
        # [ doc = "Bits 4:7 - ADDHLD" ]
        # [ inline ( always ) ]
        pub fn addhld(&mut self) -> _ADDHLDW {
            _ADDHLDW { w: self }
        }
        # [ doc = "Bits 0:3 - ADDSET" ]
        # [ inline ( always ) ]
        pub fn addset(&mut self) -> _ADDSETW {
            _ADDSETW { w: self }
        }
    }
}
# [ doc = "SRAM/NOR-Flash chip-select control register 2" ]
pub struct BCR2 {
    register: VolatileCell<u32>,
}
# [ doc = "SRAM/NOR-Flash chip-select control register 2" ]
pub mod bcr2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::BCR2 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CBURSTRWR {
        bits: bool,
    }
    impl CBURSTRWR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ASYNCWAITR {
        bits: bool,
    }
    impl ASYNCWAITR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EXTMODR {
        bits: bool,
    }
    impl EXTMODR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WAITENR {
        bits: bool,
    }
    impl WAITENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WRENR {
        bits: bool,
    }
    impl WRENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WAITCFGR {
        bits: bool,
    }
    impl WAITCFGR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WRAPMODR {
        bits: bool,
    }
    impl WRAPMODR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WAITPOLR {
        bits: bool,
    }
    impl WAITPOLR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct BURSTENR {
        bits: bool,
    }
    impl BURSTENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct FACCENR {
        bits: bool,
    }
    impl FACCENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MWIDR {
        bits: u8,
    }
    impl MWIDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MTYPR {
        bits: u8,
    }
    impl MTYPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MUXENR {
        bits: bool,
    }
    impl MUXENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MBKENR {
        bits: bool,
    }
    impl MBKENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CBURSTRWW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CBURSTRWW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ASYNCWAITW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ASYNCWAITW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EXTMODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTMODW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WAITENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAITENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WRENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WRENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WAITCFGW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAITCFGW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WRAPMODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WRAPMODW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WAITPOLW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAITPOLW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _BURSTENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BURSTENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _FACCENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FACCENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MWIDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MWIDW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MTYPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MTYPW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MUXENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MUXENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MBKENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MBKENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 19 - CBURSTRW" ]
        # [ inline ( always ) ]
        pub fn cburstrw(&self) -> CBURSTRWR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CBURSTRWR { bits }
        }
        # [ doc = "Bit 15 - ASYNCWAIT" ]
        # [ inline ( always ) ]
        pub fn asyncwait(&self) -> ASYNCWAITR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ASYNCWAITR { bits }
        }
        # [ doc = "Bit 14 - EXTMOD" ]
        # [ inline ( always ) ]
        pub fn extmod(&self) -> EXTMODR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTMODR { bits }
        }
        # [ doc = "Bit 13 - WAITEN" ]
        # [ inline ( always ) ]
        pub fn waiten(&self) -> WAITENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAITENR { bits }
        }
        # [ doc = "Bit 12 - WREN" ]
        # [ inline ( always ) ]
        pub fn wren(&self) -> WRENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WRENR { bits }
        }
        # [ doc = "Bit 11 - WAITCFG" ]
        # [ inline ( always ) ]
        pub fn waitcfg(&self) -> WAITCFGR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAITCFGR { bits }
        }
        # [ doc = "Bit 10 - WRAPMOD" ]
        # [ inline ( always ) ]
        pub fn wrapmod(&self) -> WRAPMODR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WRAPMODR { bits }
        }
        # [ doc = "Bit 9 - WAITPOL" ]
        # [ inline ( always ) ]
        pub fn waitpol(&self) -> WAITPOLR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAITPOLR { bits }
        }
        # [ doc = "Bit 8 - BURSTEN" ]
        # [ inline ( always ) ]
        pub fn bursten(&self) -> BURSTENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BURSTENR { bits }
        }
        # [ doc = "Bit 6 - FACCEN" ]
        # [ inline ( always ) ]
        pub fn faccen(&self) -> FACCENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FACCENR { bits }
        }
        # [ doc = "Bits 4:5 - MWID" ]
        # [ inline ( always ) ]
        pub fn mwid(&self) -> MWIDR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MWIDR { bits }
        }
        # [ doc = "Bits 2:3 - MTYP" ]
        # [ inline ( always ) ]
        pub fn mtyp(&self) -> MTYPR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MTYPR { bits }
        }
        # [ doc = "Bit 1 - MUXEN" ]
        # [ inline ( always ) ]
        pub fn muxen(&self) -> MUXENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MUXENR { bits }
        }
        # [ doc = "Bit 0 - MBKEN" ]
        # [ inline ( always ) ]
        pub fn mbken(&self) -> MBKENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MBKENR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 12496 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 19 - CBURSTRW" ]
        # [ inline ( always ) ]
        pub fn cburstrw(&mut self) -> _CBURSTRWW {
            _CBURSTRWW { w: self }
        }
        # [ doc = "Bit 15 - ASYNCWAIT" ]
        # [ inline ( always ) ]
        pub fn asyncwait(&mut self) -> _ASYNCWAITW {
            _ASYNCWAITW { w: self }
        }
        # [ doc = "Bit 14 - EXTMOD" ]
        # [ inline ( always ) ]
        pub fn extmod(&mut self) -> _EXTMODW {
            _EXTMODW { w: self }
        }
        # [ doc = "Bit 13 - WAITEN" ]
        # [ inline ( always ) ]
        pub fn waiten(&mut self) -> _WAITENW {
            _WAITENW { w: self }
        }
        # [ doc = "Bit 12 - WREN" ]
        # [ inline ( always ) ]
        pub fn wren(&mut self) -> _WRENW {
            _WRENW { w: self }
        }
        # [ doc = "Bit 11 - WAITCFG" ]
        # [ inline ( always ) ]
        pub fn waitcfg(&mut self) -> _WAITCFGW {
            _WAITCFGW { w: self }
        }
        # [ doc = "Bit 10 - WRAPMOD" ]
        # [ inline ( always ) ]
        pub fn wrapmod(&mut self) -> _WRAPMODW {
            _WRAPMODW { w: self }
        }
        # [ doc = "Bit 9 - WAITPOL" ]
        # [ inline ( always ) ]
        pub fn waitpol(&mut self) -> _WAITPOLW {
            _WAITPOLW { w: self }
        }
        # [ doc = "Bit 8 - BURSTEN" ]
        # [ inline ( always ) ]
        pub fn bursten(&mut self) -> _BURSTENW {
            _BURSTENW { w: self }
        }
        # [ doc = "Bit 6 - FACCEN" ]
        # [ inline ( always ) ]
        pub fn faccen(&mut self) -> _FACCENW {
            _FACCENW { w: self }
        }
        # [ doc = "Bits 4:5 - MWID" ]
        # [ inline ( always ) ]
        pub fn mwid(&mut self) -> _MWIDW {
            _MWIDW { w: self }
        }
        # [ doc = "Bits 2:3 - MTYP" ]
        # [ inline ( always ) ]
        pub fn mtyp(&mut self) -> _MTYPW {
            _MTYPW { w: self }
        }
        # [ doc = "Bit 1 - MUXEN" ]
        # [ inline ( always ) ]
        pub fn muxen(&mut self) -> _MUXENW {
            _MUXENW { w: self }
        }
        # [ doc = "Bit 0 - MBKEN" ]
        # [ inline ( always ) ]
        pub fn mbken(&mut self) -> _MBKENW {
            _MBKENW { w: self }
        }
    }
}
# [ doc = "SRAM/NOR-Flash chip-select timing register 2" ]
pub struct BTR2 {
    register: VolatileCell<u32>,
}
# [ doc = "SRAM/NOR-Flash chip-select timing register 2" ]
pub mod btr2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::BTR2 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ACCMODR {
        bits: u8,
    }
    impl ACCMODR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATLATR {
        bits: u8,
    }
    impl DATLATR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CLKDIVR {
        bits: u8,
    }
    impl CLKDIVR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct BUSTURNR {
        bits: u8,
    }
    impl BUSTURNR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATASTR {
        bits: u8,
    }
    impl DATASTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ADDHLDR {
        bits: u8,
    }
    impl ADDHLDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ADDSETR {
        bits: u8,
    }
    impl ADDSETR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ACCMODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ACCMODW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DATLATW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATLATW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CLKDIVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CLKDIVW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _BUSTURNW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BUSTURNW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DATASTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATASTW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADDHLDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADDHLDW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADDSETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADDSETW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 28:29 - ACCMOD" ]
        # [ inline ( always ) ]
        pub fn accmod(&self) -> ACCMODR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ACCMODR { bits }
        }
        # [ doc = "Bits 24:27 - DATLAT" ]
        # [ inline ( always ) ]
        pub fn datlat(&self) -> DATLATR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATLATR { bits }
        }
        # [ doc = "Bits 20:23 - CLKDIV" ]
        # [ inline ( always ) ]
        pub fn clkdiv(&self) -> CLKDIVR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CLKDIVR { bits }
        }
        # [ doc = "Bits 16:19 - BUSTURN" ]
        # [ inline ( always ) ]
        pub fn busturn(&self) -> BUSTURNR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            BUSTURNR { bits }
        }
        # [ doc = "Bits 8:15 - DATAST" ]
        # [ inline ( always ) ]
        pub fn datast(&self) -> DATASTR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATASTR { bits }
        }
        # [ doc = "Bits 4:7 - ADDHLD" ]
        # [ inline ( always ) ]
        pub fn addhld(&self) -> ADDHLDR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ADDHLDR { bits }
        }
        # [ doc = "Bits 0:3 - ADDSET" ]
        # [ inline ( always ) ]
        pub fn addset(&self) -> ADDSETR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ADDSETR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 4294967295 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 28:29 - ACCMOD" ]
        # [ inline ( always ) ]
        pub fn accmod(&mut self) -> _ACCMODW {
            _ACCMODW { w: self }
        }
        # [ doc = "Bits 24:27 - DATLAT" ]
        # [ inline ( always ) ]
        pub fn datlat(&mut self) -> _DATLATW {
            _DATLATW { w: self }
        }
        # [ doc = "Bits 20:23 - CLKDIV" ]
        # [ inline ( always ) ]
        pub fn clkdiv(&mut self) -> _CLKDIVW {
            _CLKDIVW { w: self }
        }
        # [ doc = "Bits 16:19 - BUSTURN" ]
        # [ inline ( always ) ]
        pub fn busturn(&mut self) -> _BUSTURNW {
            _BUSTURNW { w: self }
        }
        # [ doc = "Bits 8:15 - DATAST" ]
        # [ inline ( always ) ]
        pub fn datast(&mut self) -> _DATASTW {
            _DATASTW { w: self }
        }
        # [ doc = "Bits 4:7 - ADDHLD" ]
        # [ inline ( always ) ]
        pub fn addhld(&mut self) -> _ADDHLDW {
            _ADDHLDW { w: self }
        }
        # [ doc = "Bits 0:3 - ADDSET" ]
        # [ inline ( always ) ]
        pub fn addset(&mut self) -> _ADDSETW {
            _ADDSETW { w: self }
        }
    }
}
# [ doc = "SRAM/NOR-Flash chip-select control register 3" ]
pub struct BCR3 {
    register: VolatileCell<u32>,
}
# [ doc = "SRAM/NOR-Flash chip-select control register 3" ]
pub mod bcr3 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::BCR3 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CBURSTRWR {
        bits: bool,
    }
    impl CBURSTRWR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ASYNCWAITR {
        bits: bool,
    }
    impl ASYNCWAITR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EXTMODR {
        bits: bool,
    }
    impl EXTMODR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WAITENR {
        bits: bool,
    }
    impl WAITENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WRENR {
        bits: bool,
    }
    impl WRENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WAITCFGR {
        bits: bool,
    }
    impl WAITCFGR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WRAPMODR {
        bits: bool,
    }
    impl WRAPMODR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WAITPOLR {
        bits: bool,
    }
    impl WAITPOLR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct BURSTENR {
        bits: bool,
    }
    impl BURSTENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct FACCENR {
        bits: bool,
    }
    impl FACCENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MWIDR {
        bits: u8,
    }
    impl MWIDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MTYPR {
        bits: u8,
    }
    impl MTYPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MUXENR {
        bits: bool,
    }
    impl MUXENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MBKENR {
        bits: bool,
    }
    impl MBKENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CBURSTRWW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CBURSTRWW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ASYNCWAITW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ASYNCWAITW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EXTMODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTMODW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WAITENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAITENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WRENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WRENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WAITCFGW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAITCFGW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WRAPMODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WRAPMODW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WAITPOLW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAITPOLW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _BURSTENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BURSTENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _FACCENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FACCENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MWIDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MWIDW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MTYPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MTYPW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MUXENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MUXENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MBKENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MBKENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 19 - CBURSTRW" ]
        # [ inline ( always ) ]
        pub fn cburstrw(&self) -> CBURSTRWR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CBURSTRWR { bits }
        }
        # [ doc = "Bit 15 - ASYNCWAIT" ]
        # [ inline ( always ) ]
        pub fn asyncwait(&self) -> ASYNCWAITR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ASYNCWAITR { bits }
        }
        # [ doc = "Bit 14 - EXTMOD" ]
        # [ inline ( always ) ]
        pub fn extmod(&self) -> EXTMODR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTMODR { bits }
        }
        # [ doc = "Bit 13 - WAITEN" ]
        # [ inline ( always ) ]
        pub fn waiten(&self) -> WAITENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAITENR { bits }
        }
        # [ doc = "Bit 12 - WREN" ]
        # [ inline ( always ) ]
        pub fn wren(&self) -> WRENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WRENR { bits }
        }
        # [ doc = "Bit 11 - WAITCFG" ]
        # [ inline ( always ) ]
        pub fn waitcfg(&self) -> WAITCFGR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAITCFGR { bits }
        }
        # [ doc = "Bit 10 - WRAPMOD" ]
        # [ inline ( always ) ]
        pub fn wrapmod(&self) -> WRAPMODR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WRAPMODR { bits }
        }
        # [ doc = "Bit 9 - WAITPOL" ]
        # [ inline ( always ) ]
        pub fn waitpol(&self) -> WAITPOLR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAITPOLR { bits }
        }
        # [ doc = "Bit 8 - BURSTEN" ]
        # [ inline ( always ) ]
        pub fn bursten(&self) -> BURSTENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BURSTENR { bits }
        }
        # [ doc = "Bit 6 - FACCEN" ]
        # [ inline ( always ) ]
        pub fn faccen(&self) -> FACCENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FACCENR { bits }
        }
        # [ doc = "Bits 4:5 - MWID" ]
        # [ inline ( always ) ]
        pub fn mwid(&self) -> MWIDR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MWIDR { bits }
        }
        # [ doc = "Bits 2:3 - MTYP" ]
        # [ inline ( always ) ]
        pub fn mtyp(&self) -> MTYPR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MTYPR { bits }
        }
        # [ doc = "Bit 1 - MUXEN" ]
        # [ inline ( always ) ]
        pub fn muxen(&self) -> MUXENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MUXENR { bits }
        }
        # [ doc = "Bit 0 - MBKEN" ]
        # [ inline ( always ) ]
        pub fn mbken(&self) -> MBKENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MBKENR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 12496 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 19 - CBURSTRW" ]
        # [ inline ( always ) ]
        pub fn cburstrw(&mut self) -> _CBURSTRWW {
            _CBURSTRWW { w: self }
        }
        # [ doc = "Bit 15 - ASYNCWAIT" ]
        # [ inline ( always ) ]
        pub fn asyncwait(&mut self) -> _ASYNCWAITW {
            _ASYNCWAITW { w: self }
        }
        # [ doc = "Bit 14 - EXTMOD" ]
        # [ inline ( always ) ]
        pub fn extmod(&mut self) -> _EXTMODW {
            _EXTMODW { w: self }
        }
        # [ doc = "Bit 13 - WAITEN" ]
        # [ inline ( always ) ]
        pub fn waiten(&mut self) -> _WAITENW {
            _WAITENW { w: self }
        }
        # [ doc = "Bit 12 - WREN" ]
        # [ inline ( always ) ]
        pub fn wren(&mut self) -> _WRENW {
            _WRENW { w: self }
        }
        # [ doc = "Bit 11 - WAITCFG" ]
        # [ inline ( always ) ]
        pub fn waitcfg(&mut self) -> _WAITCFGW {
            _WAITCFGW { w: self }
        }
        # [ doc = "Bit 10 - WRAPMOD" ]
        # [ inline ( always ) ]
        pub fn wrapmod(&mut self) -> _WRAPMODW {
            _WRAPMODW { w: self }
        }
        # [ doc = "Bit 9 - WAITPOL" ]
        # [ inline ( always ) ]
        pub fn waitpol(&mut self) -> _WAITPOLW {
            _WAITPOLW { w: self }
        }
        # [ doc = "Bit 8 - BURSTEN" ]
        # [ inline ( always ) ]
        pub fn bursten(&mut self) -> _BURSTENW {
            _BURSTENW { w: self }
        }
        # [ doc = "Bit 6 - FACCEN" ]
        # [ inline ( always ) ]
        pub fn faccen(&mut self) -> _FACCENW {
            _FACCENW { w: self }
        }
        # [ doc = "Bits 4:5 - MWID" ]
        # [ inline ( always ) ]
        pub fn mwid(&mut self) -> _MWIDW {
            _MWIDW { w: self }
        }
        # [ doc = "Bits 2:3 - MTYP" ]
        # [ inline ( always ) ]
        pub fn mtyp(&mut self) -> _MTYPW {
            _MTYPW { w: self }
        }
        # [ doc = "Bit 1 - MUXEN" ]
        # [ inline ( always ) ]
        pub fn muxen(&mut self) -> _MUXENW {
            _MUXENW { w: self }
        }
        # [ doc = "Bit 0 - MBKEN" ]
        # [ inline ( always ) ]
        pub fn mbken(&mut self) -> _MBKENW {
            _MBKENW { w: self }
        }
    }
}
# [ doc = "SRAM/NOR-Flash chip-select timing register 3" ]
pub struct BTR3 {
    register: VolatileCell<u32>,
}
# [ doc = "SRAM/NOR-Flash chip-select timing register 3" ]
pub mod btr3 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::BTR3 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ACCMODR {
        bits: u8,
    }
    impl ACCMODR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATLATR {
        bits: u8,
    }
    impl DATLATR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CLKDIVR {
        bits: u8,
    }
    impl CLKDIVR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct BUSTURNR {
        bits: u8,
    }
    impl BUSTURNR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATASTR {
        bits: u8,
    }
    impl DATASTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ADDHLDR {
        bits: u8,
    }
    impl ADDHLDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ADDSETR {
        bits: u8,
    }
    impl ADDSETR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ACCMODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ACCMODW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DATLATW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATLATW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CLKDIVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CLKDIVW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _BUSTURNW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BUSTURNW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DATASTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATASTW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADDHLDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADDHLDW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADDSETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADDSETW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 28:29 - ACCMOD" ]
        # [ inline ( always ) ]
        pub fn accmod(&self) -> ACCMODR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ACCMODR { bits }
        }
        # [ doc = "Bits 24:27 - DATLAT" ]
        # [ inline ( always ) ]
        pub fn datlat(&self) -> DATLATR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATLATR { bits }
        }
        # [ doc = "Bits 20:23 - CLKDIV" ]
        # [ inline ( always ) ]
        pub fn clkdiv(&self) -> CLKDIVR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CLKDIVR { bits }
        }
        # [ doc = "Bits 16:19 - BUSTURN" ]
        # [ inline ( always ) ]
        pub fn busturn(&self) -> BUSTURNR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            BUSTURNR { bits }
        }
        # [ doc = "Bits 8:15 - DATAST" ]
        # [ inline ( always ) ]
        pub fn datast(&self) -> DATASTR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATASTR { bits }
        }
        # [ doc = "Bits 4:7 - ADDHLD" ]
        # [ inline ( always ) ]
        pub fn addhld(&self) -> ADDHLDR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ADDHLDR { bits }
        }
        # [ doc = "Bits 0:3 - ADDSET" ]
        # [ inline ( always ) ]
        pub fn addset(&self) -> ADDSETR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ADDSETR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 4294967295 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 28:29 - ACCMOD" ]
        # [ inline ( always ) ]
        pub fn accmod(&mut self) -> _ACCMODW {
            _ACCMODW { w: self }
        }
        # [ doc = "Bits 24:27 - DATLAT" ]
        # [ inline ( always ) ]
        pub fn datlat(&mut self) -> _DATLATW {
            _DATLATW { w: self }
        }
        # [ doc = "Bits 20:23 - CLKDIV" ]
        # [ inline ( always ) ]
        pub fn clkdiv(&mut self) -> _CLKDIVW {
            _CLKDIVW { w: self }
        }
        # [ doc = "Bits 16:19 - BUSTURN" ]
        # [ inline ( always ) ]
        pub fn busturn(&mut self) -> _BUSTURNW {
            _BUSTURNW { w: self }
        }
        # [ doc = "Bits 8:15 - DATAST" ]
        # [ inline ( always ) ]
        pub fn datast(&mut self) -> _DATASTW {
            _DATASTW { w: self }
        }
        # [ doc = "Bits 4:7 - ADDHLD" ]
        # [ inline ( always ) ]
        pub fn addhld(&mut self) -> _ADDHLDW {
            _ADDHLDW { w: self }
        }
        # [ doc = "Bits 0:3 - ADDSET" ]
        # [ inline ( always ) ]
        pub fn addset(&mut self) -> _ADDSETW {
            _ADDSETW { w: self }
        }
    }
}
# [ doc = "SRAM/NOR-Flash chip-select control register 4" ]
pub struct BCR4 {
    register: VolatileCell<u32>,
}
# [ doc = "SRAM/NOR-Flash chip-select control register 4" ]
pub mod bcr4 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::BCR4 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CBURSTRWR {
        bits: bool,
    }
    impl CBURSTRWR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ASYNCWAITR {
        bits: bool,
    }
    impl ASYNCWAITR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EXTMODR {
        bits: bool,
    }
    impl EXTMODR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WAITENR {
        bits: bool,
    }
    impl WAITENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WRENR {
        bits: bool,
    }
    impl WRENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WAITCFGR {
        bits: bool,
    }
    impl WAITCFGR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WRAPMODR {
        bits: bool,
    }
    impl WRAPMODR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WAITPOLR {
        bits: bool,
    }
    impl WAITPOLR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct BURSTENR {
        bits: bool,
    }
    impl BURSTENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct FACCENR {
        bits: bool,
    }
    impl FACCENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MWIDR {
        bits: u8,
    }
    impl MWIDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MTYPR {
        bits: u8,
    }
    impl MTYPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MUXENR {
        bits: bool,
    }
    impl MUXENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MBKENR {
        bits: bool,
    }
    impl MBKENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CBURSTRWW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CBURSTRWW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ASYNCWAITW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ASYNCWAITW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EXTMODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTMODW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WAITENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAITENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WRENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WRENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WAITCFGW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAITCFGW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WRAPMODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WRAPMODW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WAITPOLW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAITPOLW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _BURSTENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BURSTENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _FACCENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FACCENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MWIDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MWIDW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MTYPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MTYPW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MUXENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MUXENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MBKENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MBKENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 19 - CBURSTRW" ]
        # [ inline ( always ) ]
        pub fn cburstrw(&self) -> CBURSTRWR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CBURSTRWR { bits }
        }
        # [ doc = "Bit 15 - ASYNCWAIT" ]
        # [ inline ( always ) ]
        pub fn asyncwait(&self) -> ASYNCWAITR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ASYNCWAITR { bits }
        }
        # [ doc = "Bit 14 - EXTMOD" ]
        # [ inline ( always ) ]
        pub fn extmod(&self) -> EXTMODR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTMODR { bits }
        }
        # [ doc = "Bit 13 - WAITEN" ]
        # [ inline ( always ) ]
        pub fn waiten(&self) -> WAITENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAITENR { bits }
        }
        # [ doc = "Bit 12 - WREN" ]
        # [ inline ( always ) ]
        pub fn wren(&self) -> WRENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WRENR { bits }
        }
        # [ doc = "Bit 11 - WAITCFG" ]
        # [ inline ( always ) ]
        pub fn waitcfg(&self) -> WAITCFGR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAITCFGR { bits }
        }
        # [ doc = "Bit 10 - WRAPMOD" ]
        # [ inline ( always ) ]
        pub fn wrapmod(&self) -> WRAPMODR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WRAPMODR { bits }
        }
        # [ doc = "Bit 9 - WAITPOL" ]
        # [ inline ( always ) ]
        pub fn waitpol(&self) -> WAITPOLR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAITPOLR { bits }
        }
        # [ doc = "Bit 8 - BURSTEN" ]
        # [ inline ( always ) ]
        pub fn bursten(&self) -> BURSTENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BURSTENR { bits }
        }
        # [ doc = "Bit 6 - FACCEN" ]
        # [ inline ( always ) ]
        pub fn faccen(&self) -> FACCENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FACCENR { bits }
        }
        # [ doc = "Bits 4:5 - MWID" ]
        # [ inline ( always ) ]
        pub fn mwid(&self) -> MWIDR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MWIDR { bits }
        }
        # [ doc = "Bits 2:3 - MTYP" ]
        # [ inline ( always ) ]
        pub fn mtyp(&self) -> MTYPR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MTYPR { bits }
        }
        # [ doc = "Bit 1 - MUXEN" ]
        # [ inline ( always ) ]
        pub fn muxen(&self) -> MUXENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MUXENR { bits }
        }
        # [ doc = "Bit 0 - MBKEN" ]
        # [ inline ( always ) ]
        pub fn mbken(&self) -> MBKENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MBKENR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 12496 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 19 - CBURSTRW" ]
        # [ inline ( always ) ]
        pub fn cburstrw(&mut self) -> _CBURSTRWW {
            _CBURSTRWW { w: self }
        }
        # [ doc = "Bit 15 - ASYNCWAIT" ]
        # [ inline ( always ) ]
        pub fn asyncwait(&mut self) -> _ASYNCWAITW {
            _ASYNCWAITW { w: self }
        }
        # [ doc = "Bit 14 - EXTMOD" ]
        # [ inline ( always ) ]
        pub fn extmod(&mut self) -> _EXTMODW {
            _EXTMODW { w: self }
        }
        # [ doc = "Bit 13 - WAITEN" ]
        # [ inline ( always ) ]
        pub fn waiten(&mut self) -> _WAITENW {
            _WAITENW { w: self }
        }
        # [ doc = "Bit 12 - WREN" ]
        # [ inline ( always ) ]
        pub fn wren(&mut self) -> _WRENW {
            _WRENW { w: self }
        }
        # [ doc = "Bit 11 - WAITCFG" ]
        # [ inline ( always ) ]
        pub fn waitcfg(&mut self) -> _WAITCFGW {
            _WAITCFGW { w: self }
        }
        # [ doc = "Bit 10 - WRAPMOD" ]
        # [ inline ( always ) ]
        pub fn wrapmod(&mut self) -> _WRAPMODW {
            _WRAPMODW { w: self }
        }
        # [ doc = "Bit 9 - WAITPOL" ]
        # [ inline ( always ) ]
        pub fn waitpol(&mut self) -> _WAITPOLW {
            _WAITPOLW { w: self }
        }
        # [ doc = "Bit 8 - BURSTEN" ]
        # [ inline ( always ) ]
        pub fn bursten(&mut self) -> _BURSTENW {
            _BURSTENW { w: self }
        }
        # [ doc = "Bit 6 - FACCEN" ]
        # [ inline ( always ) ]
        pub fn faccen(&mut self) -> _FACCENW {
            _FACCENW { w: self }
        }
        # [ doc = "Bits 4:5 - MWID" ]
        # [ inline ( always ) ]
        pub fn mwid(&mut self) -> _MWIDW {
            _MWIDW { w: self }
        }
        # [ doc = "Bits 2:3 - MTYP" ]
        # [ inline ( always ) ]
        pub fn mtyp(&mut self) -> _MTYPW {
            _MTYPW { w: self }
        }
        # [ doc = "Bit 1 - MUXEN" ]
        # [ inline ( always ) ]
        pub fn muxen(&mut self) -> _MUXENW {
            _MUXENW { w: self }
        }
        # [ doc = "Bit 0 - MBKEN" ]
        # [ inline ( always ) ]
        pub fn mbken(&mut self) -> _MBKENW {
            _MBKENW { w: self }
        }
    }
}
# [ doc = "SRAM/NOR-Flash chip-select timing register 4" ]
pub struct BTR4 {
    register: VolatileCell<u32>,
}
# [ doc = "SRAM/NOR-Flash chip-select timing register 4" ]
pub mod btr4 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::BTR4 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ACCMODR {
        bits: u8,
    }
    impl ACCMODR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATLATR {
        bits: u8,
    }
    impl DATLATR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CLKDIVR {
        bits: u8,
    }
    impl CLKDIVR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct BUSTURNR {
        bits: u8,
    }
    impl BUSTURNR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATASTR {
        bits: u8,
    }
    impl DATASTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ADDHLDR {
        bits: u8,
    }
    impl ADDHLDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ADDSETR {
        bits: u8,
    }
    impl ADDSETR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ACCMODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ACCMODW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DATLATW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATLATW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CLKDIVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CLKDIVW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _BUSTURNW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BUSTURNW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DATASTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATASTW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADDHLDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADDHLDW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADDSETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADDSETW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 28:29 - ACCMOD" ]
        # [ inline ( always ) ]
        pub fn accmod(&self) -> ACCMODR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ACCMODR { bits }
        }
        # [ doc = "Bits 24:27 - DATLAT" ]
        # [ inline ( always ) ]
        pub fn datlat(&self) -> DATLATR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATLATR { bits }
        }
        # [ doc = "Bits 20:23 - CLKDIV" ]
        # [ inline ( always ) ]
        pub fn clkdiv(&self) -> CLKDIVR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CLKDIVR { bits }
        }
        # [ doc = "Bits 16:19 - BUSTURN" ]
        # [ inline ( always ) ]
        pub fn busturn(&self) -> BUSTURNR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            BUSTURNR { bits }
        }
        # [ doc = "Bits 8:15 - DATAST" ]
        # [ inline ( always ) ]
        pub fn datast(&self) -> DATASTR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATASTR { bits }
        }
        # [ doc = "Bits 4:7 - ADDHLD" ]
        # [ inline ( always ) ]
        pub fn addhld(&self) -> ADDHLDR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ADDHLDR { bits }
        }
        # [ doc = "Bits 0:3 - ADDSET" ]
        # [ inline ( always ) ]
        pub fn addset(&self) -> ADDSETR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ADDSETR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 4294967295 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 28:29 - ACCMOD" ]
        # [ inline ( always ) ]
        pub fn accmod(&mut self) -> _ACCMODW {
            _ACCMODW { w: self }
        }
        # [ doc = "Bits 24:27 - DATLAT" ]
        # [ inline ( always ) ]
        pub fn datlat(&mut self) -> _DATLATW {
            _DATLATW { w: self }
        }
        # [ doc = "Bits 20:23 - CLKDIV" ]
        # [ inline ( always ) ]
        pub fn clkdiv(&mut self) -> _CLKDIVW {
            _CLKDIVW { w: self }
        }
        # [ doc = "Bits 16:19 - BUSTURN" ]
        # [ inline ( always ) ]
        pub fn busturn(&mut self) -> _BUSTURNW {
            _BUSTURNW { w: self }
        }
        # [ doc = "Bits 8:15 - DATAST" ]
        # [ inline ( always ) ]
        pub fn datast(&mut self) -> _DATASTW {
            _DATASTW { w: self }
        }
        # [ doc = "Bits 4:7 - ADDHLD" ]
        # [ inline ( always ) ]
        pub fn addhld(&mut self) -> _ADDHLDW {
            _ADDHLDW { w: self }
        }
        # [ doc = "Bits 0:3 - ADDSET" ]
        # [ inline ( always ) ]
        pub fn addset(&mut self) -> _ADDSETW {
            _ADDSETW { w: self }
        }
    }
}
# [ doc = "PC Card/NAND Flash control register 2" ]
pub struct PCR2 {
    register: VolatileCell<u32>,
}
# [ doc = "PC Card/NAND Flash control register 2" ]
pub mod pcr2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PCR2 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ECCPSR {
        bits: u8,
    }
    impl ECCPSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TARR {
        bits: u8,
    }
    impl TARR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TCLRR {
        bits: u8,
    }
    impl TCLRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ECCENR {
        bits: bool,
    }
    impl ECCENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct PWIDR {
        bits: u8,
    }
    impl PWIDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct PTYPR {
        bits: bool,
    }
    impl PTYPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct PBKENR {
        bits: bool,
    }
    impl PBKENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct PWAITENR {
        bits: bool,
    }
    impl PWAITENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ECCPSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ECCPSW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 17;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TARW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TARW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 13;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TCLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TCLRW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ECCENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ECCENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWIDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWIDW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PTYPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PTYPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PBKENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PBKENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWAITENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWAITENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 17:19 - ECCPS" ]
        # [ inline ( always ) ]
        pub fn eccps(&self) -> ECCPSR {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ECCPSR { bits }
        }
        # [ doc = "Bits 13:16 - TAR" ]
        # [ inline ( always ) ]
        pub fn tar(&self) -> TARR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            TARR { bits }
        }
        # [ doc = "Bits 9:12 - TCLR" ]
        # [ inline ( always ) ]
        pub fn tclr(&self) -> TCLRR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            TCLRR { bits }
        }
        # [ doc = "Bit 6 - ECCEN" ]
        # [ inline ( always ) ]
        pub fn eccen(&self) -> ECCENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ECCENR { bits }
        }
        # [ doc = "Bits 4:5 - PWID" ]
        # [ inline ( always ) ]
        pub fn pwid(&self) -> PWIDR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            PWIDR { bits }
        }
        # [ doc = "Bit 3 - PTYP" ]
        # [ inline ( always ) ]
        pub fn ptyp(&self) -> PTYPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PTYPR { bits }
        }
        # [ doc = "Bit 2 - PBKEN" ]
        # [ inline ( always ) ]
        pub fn pbken(&self) -> PBKENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PBKENR { bits }
        }
        # [ doc = "Bit 1 - PWAITEN" ]
        # [ inline ( always ) ]
        pub fn pwaiten(&self) -> PWAITENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PWAITENR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 24 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 17:19 - ECCPS" ]
        # [ inline ( always ) ]
        pub fn eccps(&mut self) -> _ECCPSW {
            _ECCPSW { w: self }
        }
        # [ doc = "Bits 13:16 - TAR" ]
        # [ inline ( always ) ]
        pub fn tar(&mut self) -> _TARW {
            _TARW { w: self }
        }
        # [ doc = "Bits 9:12 - TCLR" ]
        # [ inline ( always ) ]
        pub fn tclr(&mut self) -> _TCLRW {
            _TCLRW { w: self }
        }
        # [ doc = "Bit 6 - ECCEN" ]
        # [ inline ( always ) ]
        pub fn eccen(&mut self) -> _ECCENW {
            _ECCENW { w: self }
        }
        # [ doc = "Bits 4:5 - PWID" ]
        # [ inline ( always ) ]
        pub fn pwid(&mut self) -> _PWIDW {
            _PWIDW { w: self }
        }
        # [ doc = "Bit 3 - PTYP" ]
        # [ inline ( always ) ]
        pub fn ptyp(&mut self) -> _PTYPW {
            _PTYPW { w: self }
        }
        # [ doc = "Bit 2 - PBKEN" ]
        # [ inline ( always ) ]
        pub fn pbken(&mut self) -> _PBKENW {
            _PBKENW { w: self }
        }
        # [ doc = "Bit 1 - PWAITEN" ]
        # [ inline ( always ) ]
        pub fn pwaiten(&mut self) -> _PWAITENW {
            _PWAITENW { w: self }
        }
    }
}
# [ doc = "FIFO status and interrupt register 2" ]
pub struct SR2 {
    register: VolatileCell<u32>,
}
# [ doc = "FIFO status and interrupt register 2" ]
pub mod sr2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::SR2 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct FEMPTR {
        bits: bool,
    }
    impl FEMPTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IFENR {
        bits: bool,
    }
    impl IFENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ILENR {
        bits: bool,
    }
    impl ILENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IRENR {
        bits: bool,
    }
    impl IRENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IFSR {
        bits: bool,
    }
    impl IFSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ILSR {
        bits: bool,
    }
    impl ILSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IRSR {
        bits: bool,
    }
    impl IRSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IFENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IFENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ILENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ILENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IRENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IRENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IFSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IFSW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ILSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ILSW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IRSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IRSW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 6 - FEMPT" ]
        # [ inline ( always ) ]
        pub fn fempt(&self) -> FEMPTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FEMPTR { bits }
        }
        # [ doc = "Bit 5 - IFEN" ]
        # [ inline ( always ) ]
        pub fn ifen(&self) -> IFENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IFENR { bits }
        }
        # [ doc = "Bit 4 - ILEN" ]
        # [ inline ( always ) ]
        pub fn ilen(&self) -> ILENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ILENR { bits }
        }
        # [ doc = "Bit 3 - IREN" ]
        # [ inline ( always ) ]
        pub fn iren(&self) -> IRENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IRENR { bits }
        }
        # [ doc = "Bit 2 - IFS" ]
        # [ inline ( always ) ]
        pub fn ifs(&self) -> IFSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IFSR { bits }
        }
        # [ doc = "Bit 1 - ILS" ]
        # [ inline ( always ) ]
        pub fn ils(&self) -> ILSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ILSR { bits }
        }
        # [ doc = "Bit 0 - IRS" ]
        # [ inline ( always ) ]
        pub fn irs(&self) -> IRSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IRSR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 64 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 5 - IFEN" ]
        # [ inline ( always ) ]
        pub fn ifen(&mut self) -> _IFENW {
            _IFENW { w: self }
        }
        # [ doc = "Bit 4 - ILEN" ]
        # [ inline ( always ) ]
        pub fn ilen(&mut self) -> _ILENW {
            _ILENW { w: self }
        }
        # [ doc = "Bit 3 - IREN" ]
        # [ inline ( always ) ]
        pub fn iren(&mut self) -> _IRENW {
            _IRENW { w: self }
        }
        # [ doc = "Bit 2 - IFS" ]
        # [ inline ( always ) ]
        pub fn ifs(&mut self) -> _IFSW {
            _IFSW { w: self }
        }
        # [ doc = "Bit 1 - ILS" ]
        # [ inline ( always ) ]
        pub fn ils(&mut self) -> _ILSW {
            _ILSW { w: self }
        }
        # [ doc = "Bit 0 - IRS" ]
        # [ inline ( always ) ]
        pub fn irs(&mut self) -> _IRSW {
            _IRSW { w: self }
        }
    }
}
# [ doc = "Common memory space timing register 2" ]
pub struct PMEM2 {
    register: VolatileCell<u32>,
}
# [ doc = "Common memory space timing register 2" ]
pub mod pmem2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PMEM2 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MEMHIZXR {
        bits: u8,
    }
    impl MEMHIZXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MEMHOLDXR {
        bits: u8,
    }
    impl MEMHOLDXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MEMWAITXR {
        bits: u8,
    }
    impl MEMWAITXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MEMSETXR {
        bits: u8,
    }
    impl MEMSETXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MEMHIZXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MEMHIZXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MEMHOLDXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MEMHOLDXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MEMWAITXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MEMWAITXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MEMSETXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MEMSETXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 24:31 - MEMHIZx" ]
        # [ inline ( always ) ]
        pub fn memhizx(&self) -> MEMHIZXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MEMHIZXR { bits }
        }
        # [ doc = "Bits 16:23 - MEMHOLDx" ]
        # [ inline ( always ) ]
        pub fn memholdx(&self) -> MEMHOLDXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MEMHOLDXR { bits }
        }
        # [ doc = "Bits 8:15 - MEMWAITx" ]
        # [ inline ( always ) ]
        pub fn memwaitx(&self) -> MEMWAITXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MEMWAITXR { bits }
        }
        # [ doc = "Bits 0:7 - MEMSETx" ]
        # [ inline ( always ) ]
        pub fn memsetx(&self) -> MEMSETXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MEMSETXR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 4244438268 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 24:31 - MEMHIZx" ]
        # [ inline ( always ) ]
        pub fn memhizx(&mut self) -> _MEMHIZXW {
            _MEMHIZXW { w: self }
        }
        # [ doc = "Bits 16:23 - MEMHOLDx" ]
        # [ inline ( always ) ]
        pub fn memholdx(&mut self) -> _MEMHOLDXW {
            _MEMHOLDXW { w: self }
        }
        # [ doc = "Bits 8:15 - MEMWAITx" ]
        # [ inline ( always ) ]
        pub fn memwaitx(&mut self) -> _MEMWAITXW {
            _MEMWAITXW { w: self }
        }
        # [ doc = "Bits 0:7 - MEMSETx" ]
        # [ inline ( always ) ]
        pub fn memsetx(&mut self) -> _MEMSETXW {
            _MEMSETXW { w: self }
        }
    }
}
# [ doc = "Attribute memory space timing register 2" ]
pub struct PATT2 {
    register: VolatileCell<u32>,
}
# [ doc = "Attribute memory space timing register 2" ]
pub mod patt2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PATT2 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ATTHIZXR {
        bits: u8,
    }
    impl ATTHIZXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ATTHOLDXR {
        bits: u8,
    }
    impl ATTHOLDXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ATTWAITXR {
        bits: u8,
    }
    impl ATTWAITXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ATTSETXR {
        bits: u8,
    }
    impl ATTSETXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ATTHIZXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ATTHIZXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ATTHOLDXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ATTHOLDXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ATTWAITXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ATTWAITXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ATTSETXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ATTSETXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 24:31 - Attribute memory x databus HiZ time" ]
        # [ inline ( always ) ]
        pub fn atthizx(&self) -> ATTHIZXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ATTHIZXR { bits }
        }
        # [ doc = "Bits 16:23 - Attribute memory x hold time" ]
        # [ inline ( always ) ]
        pub fn attholdx(&self) -> ATTHOLDXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ATTHOLDXR { bits }
        }
        # [ doc = "Bits 8:15 - Attribute memory x wait time" ]
        # [ inline ( always ) ]
        pub fn attwaitx(&self) -> ATTWAITXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ATTWAITXR { bits }
        }
        # [ doc = "Bits 0:7 - Attribute memory x setup time" ]
        # [ inline ( always ) ]
        pub fn attsetx(&self) -> ATTSETXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ATTSETXR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 4244438268 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 24:31 - Attribute memory x databus HiZ time" ]
        # [ inline ( always ) ]
        pub fn atthizx(&mut self) -> _ATTHIZXW {
            _ATTHIZXW { w: self }
        }
        # [ doc = "Bits 16:23 - Attribute memory x hold time" ]
        # [ inline ( always ) ]
        pub fn attholdx(&mut self) -> _ATTHOLDXW {
            _ATTHOLDXW { w: self }
        }
        # [ doc = "Bits 8:15 - Attribute memory x wait time" ]
        # [ inline ( always ) ]
        pub fn attwaitx(&mut self) -> _ATTWAITXW {
            _ATTWAITXW { w: self }
        }
        # [ doc = "Bits 0:7 - Attribute memory x setup time" ]
        # [ inline ( always ) ]
        pub fn attsetx(&mut self) -> _ATTSETXW {
            _ATTSETXW { w: self }
        }
    }
}
# [ doc = "ECC result register 2" ]
pub struct ECCR2 {
    register: VolatileCell<u32>,
}
# [ doc = "ECC result register 2" ]
pub mod eccr2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::ECCR2 {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ECCXR {
        bits: u32,
    }
    impl ECCXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:31 - ECC result" ]
        # [ inline ( always ) ]
        pub fn eccx(&self) -> ECCXR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            ECCXR { bits }
        }
    }
}
# [ doc = "PC Card/NAND Flash control register 3" ]
pub struct PCR3 {
    register: VolatileCell<u32>,
}
# [ doc = "PC Card/NAND Flash control register 3" ]
pub mod pcr3 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PCR3 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ECCPSR {
        bits: u8,
    }
    impl ECCPSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TARR {
        bits: u8,
    }
    impl TARR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TCLRR {
        bits: u8,
    }
    impl TCLRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ECCENR {
        bits: bool,
    }
    impl ECCENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct PWIDR {
        bits: u8,
    }
    impl PWIDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct PTYPR {
        bits: bool,
    }
    impl PTYPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct PBKENR {
        bits: bool,
    }
    impl PBKENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct PWAITENR {
        bits: bool,
    }
    impl PWAITENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ECCPSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ECCPSW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 17;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TARW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TARW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 13;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TCLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TCLRW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ECCENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ECCENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWIDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWIDW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PTYPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PTYPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PBKENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PBKENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWAITENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWAITENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 17:19 - ECCPS" ]
        # [ inline ( always ) ]
        pub fn eccps(&self) -> ECCPSR {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ECCPSR { bits }
        }
        # [ doc = "Bits 13:16 - TAR" ]
        # [ inline ( always ) ]
        pub fn tar(&self) -> TARR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            TARR { bits }
        }
        # [ doc = "Bits 9:12 - TCLR" ]
        # [ inline ( always ) ]
        pub fn tclr(&self) -> TCLRR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            TCLRR { bits }
        }
        # [ doc = "Bit 6 - ECCEN" ]
        # [ inline ( always ) ]
        pub fn eccen(&self) -> ECCENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ECCENR { bits }
        }
        # [ doc = "Bits 4:5 - PWID" ]
        # [ inline ( always ) ]
        pub fn pwid(&self) -> PWIDR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            PWIDR { bits }
        }
        # [ doc = "Bit 3 - PTYP" ]
        # [ inline ( always ) ]
        pub fn ptyp(&self) -> PTYPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PTYPR { bits }
        }
        # [ doc = "Bit 2 - PBKEN" ]
        # [ inline ( always ) ]
        pub fn pbken(&self) -> PBKENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PBKENR { bits }
        }
        # [ doc = "Bit 1 - PWAITEN" ]
        # [ inline ( always ) ]
        pub fn pwaiten(&self) -> PWAITENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PWAITENR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 24 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 17:19 - ECCPS" ]
        # [ inline ( always ) ]
        pub fn eccps(&mut self) -> _ECCPSW {
            _ECCPSW { w: self }
        }
        # [ doc = "Bits 13:16 - TAR" ]
        # [ inline ( always ) ]
        pub fn tar(&mut self) -> _TARW {
            _TARW { w: self }
        }
        # [ doc = "Bits 9:12 - TCLR" ]
        # [ inline ( always ) ]
        pub fn tclr(&mut self) -> _TCLRW {
            _TCLRW { w: self }
        }
        # [ doc = "Bit 6 - ECCEN" ]
        # [ inline ( always ) ]
        pub fn eccen(&mut self) -> _ECCENW {
            _ECCENW { w: self }
        }
        # [ doc = "Bits 4:5 - PWID" ]
        # [ inline ( always ) ]
        pub fn pwid(&mut self) -> _PWIDW {
            _PWIDW { w: self }
        }
        # [ doc = "Bit 3 - PTYP" ]
        # [ inline ( always ) ]
        pub fn ptyp(&mut self) -> _PTYPW {
            _PTYPW { w: self }
        }
        # [ doc = "Bit 2 - PBKEN" ]
        # [ inline ( always ) ]
        pub fn pbken(&mut self) -> _PBKENW {
            _PBKENW { w: self }
        }
        # [ doc = "Bit 1 - PWAITEN" ]
        # [ inline ( always ) ]
        pub fn pwaiten(&mut self) -> _PWAITENW {
            _PWAITENW { w: self }
        }
    }
}
# [ doc = "FIFO status and interrupt register 3" ]
pub struct SR3 {
    register: VolatileCell<u32>,
}
# [ doc = "FIFO status and interrupt register 3" ]
pub mod sr3 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::SR3 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct FEMPTR {
        bits: bool,
    }
    impl FEMPTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IFENR {
        bits: bool,
    }
    impl IFENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ILENR {
        bits: bool,
    }
    impl ILENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IRENR {
        bits: bool,
    }
    impl IRENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IFSR {
        bits: bool,
    }
    impl IFSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ILSR {
        bits: bool,
    }
    impl ILSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IRSR {
        bits: bool,
    }
    impl IRSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IFENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IFENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ILENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ILENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IRENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IRENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IFSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IFSW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ILSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ILSW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IRSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IRSW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 6 - FEMPT" ]
        # [ inline ( always ) ]
        pub fn fempt(&self) -> FEMPTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FEMPTR { bits }
        }
        # [ doc = "Bit 5 - IFEN" ]
        # [ inline ( always ) ]
        pub fn ifen(&self) -> IFENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IFENR { bits }
        }
        # [ doc = "Bit 4 - ILEN" ]
        # [ inline ( always ) ]
        pub fn ilen(&self) -> ILENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ILENR { bits }
        }
        # [ doc = "Bit 3 - IREN" ]
        # [ inline ( always ) ]
        pub fn iren(&self) -> IRENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IRENR { bits }
        }
        # [ doc = "Bit 2 - IFS" ]
        # [ inline ( always ) ]
        pub fn ifs(&self) -> IFSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IFSR { bits }
        }
        # [ doc = "Bit 1 - ILS" ]
        # [ inline ( always ) ]
        pub fn ils(&self) -> ILSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ILSR { bits }
        }
        # [ doc = "Bit 0 - IRS" ]
        # [ inline ( always ) ]
        pub fn irs(&self) -> IRSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IRSR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 64 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 5 - IFEN" ]
        # [ inline ( always ) ]
        pub fn ifen(&mut self) -> _IFENW {
            _IFENW { w: self }
        }
        # [ doc = "Bit 4 - ILEN" ]
        # [ inline ( always ) ]
        pub fn ilen(&mut self) -> _ILENW {
            _ILENW { w: self }
        }
        # [ doc = "Bit 3 - IREN" ]
        # [ inline ( always ) ]
        pub fn iren(&mut self) -> _IRENW {
            _IRENW { w: self }
        }
        # [ doc = "Bit 2 - IFS" ]
        # [ inline ( always ) ]
        pub fn ifs(&mut self) -> _IFSW {
            _IFSW { w: self }
        }
        # [ doc = "Bit 1 - ILS" ]
        # [ inline ( always ) ]
        pub fn ils(&mut self) -> _ILSW {
            _ILSW { w: self }
        }
        # [ doc = "Bit 0 - IRS" ]
        # [ inline ( always ) ]
        pub fn irs(&mut self) -> _IRSW {
            _IRSW { w: self }
        }
    }
}
# [ doc = "Common memory space timing register 3" ]
pub struct PMEM3 {
    register: VolatileCell<u32>,
}
# [ doc = "Common memory space timing register 3" ]
pub mod pmem3 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PMEM3 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MEMHIZXR {
        bits: u8,
    }
    impl MEMHIZXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MEMHOLDXR {
        bits: u8,
    }
    impl MEMHOLDXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MEMWAITXR {
        bits: u8,
    }
    impl MEMWAITXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MEMSETXR {
        bits: u8,
    }
    impl MEMSETXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MEMHIZXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MEMHIZXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MEMHOLDXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MEMHOLDXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MEMWAITXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MEMWAITXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MEMSETXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MEMSETXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 24:31 - MEMHIZx" ]
        # [ inline ( always ) ]
        pub fn memhizx(&self) -> MEMHIZXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MEMHIZXR { bits }
        }
        # [ doc = "Bits 16:23 - MEMHOLDx" ]
        # [ inline ( always ) ]
        pub fn memholdx(&self) -> MEMHOLDXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MEMHOLDXR { bits }
        }
        # [ doc = "Bits 8:15 - MEMWAITx" ]
        # [ inline ( always ) ]
        pub fn memwaitx(&self) -> MEMWAITXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MEMWAITXR { bits }
        }
        # [ doc = "Bits 0:7 - MEMSETx" ]
        # [ inline ( always ) ]
        pub fn memsetx(&self) -> MEMSETXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MEMSETXR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 4244438268 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 24:31 - MEMHIZx" ]
        # [ inline ( always ) ]
        pub fn memhizx(&mut self) -> _MEMHIZXW {
            _MEMHIZXW { w: self }
        }
        # [ doc = "Bits 16:23 - MEMHOLDx" ]
        # [ inline ( always ) ]
        pub fn memholdx(&mut self) -> _MEMHOLDXW {
            _MEMHOLDXW { w: self }
        }
        # [ doc = "Bits 8:15 - MEMWAITx" ]
        # [ inline ( always ) ]
        pub fn memwaitx(&mut self) -> _MEMWAITXW {
            _MEMWAITXW { w: self }
        }
        # [ doc = "Bits 0:7 - MEMSETx" ]
        # [ inline ( always ) ]
        pub fn memsetx(&mut self) -> _MEMSETXW {
            _MEMSETXW { w: self }
        }
    }
}
# [ doc = "Attribute memory space timing register 3" ]
pub struct PATT3 {
    register: VolatileCell<u32>,
}
# [ doc = "Attribute memory space timing register 3" ]
pub mod patt3 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PATT3 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ATTHIZXR {
        bits: u8,
    }
    impl ATTHIZXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ATTHOLDXR {
        bits: u8,
    }
    impl ATTHOLDXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ATTWAITXR {
        bits: u8,
    }
    impl ATTWAITXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ATTSETXR {
        bits: u8,
    }
    impl ATTSETXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ATTHIZXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ATTHIZXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ATTHOLDXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ATTHOLDXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ATTWAITXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ATTWAITXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ATTSETXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ATTSETXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 24:31 - ATTHIZx" ]
        # [ inline ( always ) ]
        pub fn atthizx(&self) -> ATTHIZXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ATTHIZXR { bits }
        }
        # [ doc = "Bits 16:23 - ATTHOLDx" ]
        # [ inline ( always ) ]
        pub fn attholdx(&self) -> ATTHOLDXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ATTHOLDXR { bits }
        }
        # [ doc = "Bits 8:15 - ATTWAITx" ]
        # [ inline ( always ) ]
        pub fn attwaitx(&self) -> ATTWAITXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ATTWAITXR { bits }
        }
        # [ doc = "Bits 0:7 - ATTSETx" ]
        # [ inline ( always ) ]
        pub fn attsetx(&self) -> ATTSETXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ATTSETXR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 4244438268 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 24:31 - ATTHIZx" ]
        # [ inline ( always ) ]
        pub fn atthizx(&mut self) -> _ATTHIZXW {
            _ATTHIZXW { w: self }
        }
        # [ doc = "Bits 16:23 - ATTHOLDx" ]
        # [ inline ( always ) ]
        pub fn attholdx(&mut self) -> _ATTHOLDXW {
            _ATTHOLDXW { w: self }
        }
        # [ doc = "Bits 8:15 - ATTWAITx" ]
        # [ inline ( always ) ]
        pub fn attwaitx(&mut self) -> _ATTWAITXW {
            _ATTWAITXW { w: self }
        }
        # [ doc = "Bits 0:7 - ATTSETx" ]
        # [ inline ( always ) ]
        pub fn attsetx(&mut self) -> _ATTSETXW {
            _ATTSETXW { w: self }
        }
    }
}
# [ doc = "ECC result register 3" ]
pub struct ECCR3 {
    register: VolatileCell<u32>,
}
# [ doc = "ECC result register 3" ]
pub mod eccr3 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::ECCR3 {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ECCXR {
        bits: u32,
    }
    impl ECCXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:31 - ECCx" ]
        # [ inline ( always ) ]
        pub fn eccx(&self) -> ECCXR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            ECCXR { bits }
        }
    }
}
# [ doc = "PC Card/NAND Flash control register 4" ]
pub struct PCR4 {
    register: VolatileCell<u32>,
}
# [ doc = "PC Card/NAND Flash control register 4" ]
pub mod pcr4 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PCR4 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ECCPSR {
        bits: u8,
    }
    impl ECCPSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TARR {
        bits: u8,
    }
    impl TARR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TCLRR {
        bits: u8,
    }
    impl TCLRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ECCENR {
        bits: bool,
    }
    impl ECCENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct PWIDR {
        bits: u8,
    }
    impl PWIDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct PTYPR {
        bits: bool,
    }
    impl PTYPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct PBKENR {
        bits: bool,
    }
    impl PBKENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct PWAITENR {
        bits: bool,
    }
    impl PWAITENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ECCPSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ECCPSW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 17;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TARW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TARW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 13;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TCLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TCLRW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ECCENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ECCENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWIDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWIDW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PTYPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PTYPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PBKENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PBKENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWAITENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWAITENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 17:19 - ECCPS" ]
        # [ inline ( always ) ]
        pub fn eccps(&self) -> ECCPSR {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ECCPSR { bits }
        }
        # [ doc = "Bits 13:16 - TAR" ]
        # [ inline ( always ) ]
        pub fn tar(&self) -> TARR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            TARR { bits }
        }
        # [ doc = "Bits 9:12 - TCLR" ]
        # [ inline ( always ) ]
        pub fn tclr(&self) -> TCLRR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            TCLRR { bits }
        }
        # [ doc = "Bit 6 - ECCEN" ]
        # [ inline ( always ) ]
        pub fn eccen(&self) -> ECCENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ECCENR { bits }
        }
        # [ doc = "Bits 4:5 - PWID" ]
        # [ inline ( always ) ]
        pub fn pwid(&self) -> PWIDR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            PWIDR { bits }
        }
        # [ doc = "Bit 3 - PTYP" ]
        # [ inline ( always ) ]
        pub fn ptyp(&self) -> PTYPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PTYPR { bits }
        }
        # [ doc = "Bit 2 - PBKEN" ]
        # [ inline ( always ) ]
        pub fn pbken(&self) -> PBKENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PBKENR { bits }
        }
        # [ doc = "Bit 1 - PWAITEN" ]
        # [ inline ( always ) ]
        pub fn pwaiten(&self) -> PWAITENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PWAITENR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 24 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 17:19 - ECCPS" ]
        # [ inline ( always ) ]
        pub fn eccps(&mut self) -> _ECCPSW {
            _ECCPSW { w: self }
        }
        # [ doc = "Bits 13:16 - TAR" ]
        # [ inline ( always ) ]
        pub fn tar(&mut self) -> _TARW {
            _TARW { w: self }
        }
        # [ doc = "Bits 9:12 - TCLR" ]
        # [ inline ( always ) ]
        pub fn tclr(&mut self) -> _TCLRW {
            _TCLRW { w: self }
        }
        # [ doc = "Bit 6 - ECCEN" ]
        # [ inline ( always ) ]
        pub fn eccen(&mut self) -> _ECCENW {
            _ECCENW { w: self }
        }
        # [ doc = "Bits 4:5 - PWID" ]
        # [ inline ( always ) ]
        pub fn pwid(&mut self) -> _PWIDW {
            _PWIDW { w: self }
        }
        # [ doc = "Bit 3 - PTYP" ]
        # [ inline ( always ) ]
        pub fn ptyp(&mut self) -> _PTYPW {
            _PTYPW { w: self }
        }
        # [ doc = "Bit 2 - PBKEN" ]
        # [ inline ( always ) ]
        pub fn pbken(&mut self) -> _PBKENW {
            _PBKENW { w: self }
        }
        # [ doc = "Bit 1 - PWAITEN" ]
        # [ inline ( always ) ]
        pub fn pwaiten(&mut self) -> _PWAITENW {
            _PWAITENW { w: self }
        }
    }
}
# [ doc = "FIFO status and interrupt register 4" ]
pub struct SR4 {
    register: VolatileCell<u32>,
}
# [ doc = "FIFO status and interrupt register 4" ]
pub mod sr4 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::SR4 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct FEMPTR {
        bits: bool,
    }
    impl FEMPTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IFENR {
        bits: bool,
    }
    impl IFENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ILENR {
        bits: bool,
    }
    impl ILENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IRENR {
        bits: bool,
    }
    impl IRENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IFSR {
        bits: bool,
    }
    impl IFSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ILSR {
        bits: bool,
    }
    impl ILSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IRSR {
        bits: bool,
    }
    impl IRSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IFENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IFENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ILENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ILENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IRENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IRENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IFSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IFSW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ILSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ILSW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IRSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IRSW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 6 - FEMPT" ]
        # [ inline ( always ) ]
        pub fn fempt(&self) -> FEMPTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FEMPTR { bits }
        }
        # [ doc = "Bit 5 - IFEN" ]
        # [ inline ( always ) ]
        pub fn ifen(&self) -> IFENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IFENR { bits }
        }
        # [ doc = "Bit 4 - ILEN" ]
        # [ inline ( always ) ]
        pub fn ilen(&self) -> ILENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ILENR { bits }
        }
        # [ doc = "Bit 3 - IREN" ]
        # [ inline ( always ) ]
        pub fn iren(&self) -> IRENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IRENR { bits }
        }
        # [ doc = "Bit 2 - IFS" ]
        # [ inline ( always ) ]
        pub fn ifs(&self) -> IFSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IFSR { bits }
        }
        # [ doc = "Bit 1 - ILS" ]
        # [ inline ( always ) ]
        pub fn ils(&self) -> ILSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ILSR { bits }
        }
        # [ doc = "Bit 0 - IRS" ]
        # [ inline ( always ) ]
        pub fn irs(&self) -> IRSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IRSR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 64 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 5 - IFEN" ]
        # [ inline ( always ) ]
        pub fn ifen(&mut self) -> _IFENW {
            _IFENW { w: self }
        }
        # [ doc = "Bit 4 - ILEN" ]
        # [ inline ( always ) ]
        pub fn ilen(&mut self) -> _ILENW {
            _ILENW { w: self }
        }
        # [ doc = "Bit 3 - IREN" ]
        # [ inline ( always ) ]
        pub fn iren(&mut self) -> _IRENW {
            _IRENW { w: self }
        }
        # [ doc = "Bit 2 - IFS" ]
        # [ inline ( always ) ]
        pub fn ifs(&mut self) -> _IFSW {
            _IFSW { w: self }
        }
        # [ doc = "Bit 1 - ILS" ]
        # [ inline ( always ) ]
        pub fn ils(&mut self) -> _ILSW {
            _ILSW { w: self }
        }
        # [ doc = "Bit 0 - IRS" ]
        # [ inline ( always ) ]
        pub fn irs(&mut self) -> _IRSW {
            _IRSW { w: self }
        }
    }
}
# [ doc = "Common memory space timing register 4" ]
pub struct PMEM4 {
    register: VolatileCell<u32>,
}
# [ doc = "Common memory space timing register 4" ]
pub mod pmem4 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PMEM4 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MEMHIZXR {
        bits: u8,
    }
    impl MEMHIZXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MEMHOLDXR {
        bits: u8,
    }
    impl MEMHOLDXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MEMWAITXR {
        bits: u8,
    }
    impl MEMWAITXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MEMSETXR {
        bits: u8,
    }
    impl MEMSETXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MEMHIZXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MEMHIZXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MEMHOLDXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MEMHOLDXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MEMWAITXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MEMWAITXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MEMSETXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MEMSETXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 24:31 - MEMHIZx" ]
        # [ inline ( always ) ]
        pub fn memhizx(&self) -> MEMHIZXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MEMHIZXR { bits }
        }
        # [ doc = "Bits 16:23 - MEMHOLDx" ]
        # [ inline ( always ) ]
        pub fn memholdx(&self) -> MEMHOLDXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MEMHOLDXR { bits }
        }
        # [ doc = "Bits 8:15 - MEMWAITx" ]
        # [ inline ( always ) ]
        pub fn memwaitx(&self) -> MEMWAITXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MEMWAITXR { bits }
        }
        # [ doc = "Bits 0:7 - MEMSETx" ]
        # [ inline ( always ) ]
        pub fn memsetx(&self) -> MEMSETXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MEMSETXR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 4244438268 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 24:31 - MEMHIZx" ]
        # [ inline ( always ) ]
        pub fn memhizx(&mut self) -> _MEMHIZXW {
            _MEMHIZXW { w: self }
        }
        # [ doc = "Bits 16:23 - MEMHOLDx" ]
        # [ inline ( always ) ]
        pub fn memholdx(&mut self) -> _MEMHOLDXW {
            _MEMHOLDXW { w: self }
        }
        # [ doc = "Bits 8:15 - MEMWAITx" ]
        # [ inline ( always ) ]
        pub fn memwaitx(&mut self) -> _MEMWAITXW {
            _MEMWAITXW { w: self }
        }
        # [ doc = "Bits 0:7 - MEMSETx" ]
        # [ inline ( always ) ]
        pub fn memsetx(&mut self) -> _MEMSETXW {
            _MEMSETXW { w: self }
        }
    }
}
# [ doc = "Attribute memory space timing register 4" ]
pub struct PATT4 {
    register: VolatileCell<u32>,
}
# [ doc = "Attribute memory space timing register 4" ]
pub mod patt4 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PATT4 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ATTHIZXR {
        bits: u8,
    }
    impl ATTHIZXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ATTHOLDXR {
        bits: u8,
    }
    impl ATTHOLDXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ATTWAITXR {
        bits: u8,
    }
    impl ATTWAITXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ATTSETXR {
        bits: u8,
    }
    impl ATTSETXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ATTHIZXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ATTHIZXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ATTHOLDXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ATTHOLDXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ATTWAITXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ATTWAITXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ATTSETXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ATTSETXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 24:31 - ATTHIZx" ]
        # [ inline ( always ) ]
        pub fn atthizx(&self) -> ATTHIZXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ATTHIZXR { bits }
        }
        # [ doc = "Bits 16:23 - ATTHOLDx" ]
        # [ inline ( always ) ]
        pub fn attholdx(&self) -> ATTHOLDXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ATTHOLDXR { bits }
        }
        # [ doc = "Bits 8:15 - ATTWAITx" ]
        # [ inline ( always ) ]
        pub fn attwaitx(&self) -> ATTWAITXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ATTWAITXR { bits }
        }
        # [ doc = "Bits 0:7 - ATTSETx" ]
        # [ inline ( always ) ]
        pub fn attsetx(&self) -> ATTSETXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ATTSETXR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 4244438268 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 24:31 - ATTHIZx" ]
        # [ inline ( always ) ]
        pub fn atthizx(&mut self) -> _ATTHIZXW {
            _ATTHIZXW { w: self }
        }
        # [ doc = "Bits 16:23 - ATTHOLDx" ]
        # [ inline ( always ) ]
        pub fn attholdx(&mut self) -> _ATTHOLDXW {
            _ATTHOLDXW { w: self }
        }
        # [ doc = "Bits 8:15 - ATTWAITx" ]
        # [ inline ( always ) ]
        pub fn attwaitx(&mut self) -> _ATTWAITXW {
            _ATTWAITXW { w: self }
        }
        # [ doc = "Bits 0:7 - ATTSETx" ]
        # [ inline ( always ) ]
        pub fn attsetx(&mut self) -> _ATTSETXW {
            _ATTSETXW { w: self }
        }
    }
}
# [ doc = "I/O space timing register 4" ]
pub struct PIO4 {
    register: VolatileCell<u32>,
}
# [ doc = "I/O space timing register 4" ]
pub mod pio4 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PIO4 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IOHIZXR {
        bits: u8,
    }
    impl IOHIZXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IOHOLDXR {
        bits: u8,
    }
    impl IOHOLDXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IOWAITXR {
        bits: u8,
    }
    impl IOWAITXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IOSETXR {
        bits: u8,
    }
    impl IOSETXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IOHIZXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IOHIZXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IOHOLDXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IOHOLDXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IOWAITXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IOWAITXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IOSETXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IOSETXW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 24:31 - IOHIZx" ]
        # [ inline ( always ) ]
        pub fn iohizx(&self) -> IOHIZXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            IOHIZXR { bits }
        }
        # [ doc = "Bits 16:23 - IOHOLDx" ]
        # [ inline ( always ) ]
        pub fn ioholdx(&self) -> IOHOLDXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            IOHOLDXR { bits }
        }
        # [ doc = "Bits 8:15 - IOWAITx" ]
        # [ inline ( always ) ]
        pub fn iowaitx(&self) -> IOWAITXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            IOWAITXR { bits }
        }
        # [ doc = "Bits 0:7 - IOSETx" ]
        # [ inline ( always ) ]
        pub fn iosetx(&self) -> IOSETXR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            IOSETXR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 4244438268 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 24:31 - IOHIZx" ]
        # [ inline ( always ) ]
        pub fn iohizx(&mut self) -> _IOHIZXW {
            _IOHIZXW { w: self }
        }
        # [ doc = "Bits 16:23 - IOHOLDx" ]
        # [ inline ( always ) ]
        pub fn ioholdx(&mut self) -> _IOHOLDXW {
            _IOHOLDXW { w: self }
        }
        # [ doc = "Bits 8:15 - IOWAITx" ]
        # [ inline ( always ) ]
        pub fn iowaitx(&mut self) -> _IOWAITXW {
            _IOWAITXW { w: self }
        }
        # [ doc = "Bits 0:7 - IOSETx" ]
        # [ inline ( always ) ]
        pub fn iosetx(&mut self) -> _IOSETXW {
            _IOSETXW { w: self }
        }
    }
}
# [ doc = "SRAM/NOR-Flash write timing registers 1" ]
pub struct BWTR1 {
    register: VolatileCell<u32>,
}
# [ doc = "SRAM/NOR-Flash write timing registers 1" ]
pub mod bwtr1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::BWTR1 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ACCMODR {
        bits: u8,
    }
    impl ACCMODR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATLATR {
        bits: u8,
    }
    impl DATLATR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CLKDIVR {
        bits: u8,
    }
    impl CLKDIVR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATASTR {
        bits: u8,
    }
    impl DATASTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ADDHLDR {
        bits: u8,
    }
    impl ADDHLDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ADDSETR {
        bits: u8,
    }
    impl ADDSETR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ACCMODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ACCMODW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DATLATW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATLATW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CLKDIVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CLKDIVW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DATASTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATASTW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADDHLDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADDHLDW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADDSETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADDSETW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 28:29 - ACCMOD" ]
        # [ inline ( always ) ]
        pub fn accmod(&self) -> ACCMODR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ACCMODR { bits }
        }
        # [ doc = "Bits 24:27 - DATLAT" ]
        # [ inline ( always ) ]
        pub fn datlat(&self) -> DATLATR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATLATR { bits }
        }
        # [ doc = "Bits 20:23 - CLKDIV" ]
        # [ inline ( always ) ]
        pub fn clkdiv(&self) -> CLKDIVR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CLKDIVR { bits }
        }
        # [ doc = "Bits 8:15 - DATAST" ]
        # [ inline ( always ) ]
        pub fn datast(&self) -> DATASTR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATASTR { bits }
        }
        # [ doc = "Bits 4:7 - ADDHLD" ]
        # [ inline ( always ) ]
        pub fn addhld(&self) -> ADDHLDR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ADDHLDR { bits }
        }
        # [ doc = "Bits 0:3 - ADDSET" ]
        # [ inline ( always ) ]
        pub fn addset(&self) -> ADDSETR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ADDSETR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 268435455 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 28:29 - ACCMOD" ]
        # [ inline ( always ) ]
        pub fn accmod(&mut self) -> _ACCMODW {
            _ACCMODW { w: self }
        }
        # [ doc = "Bits 24:27 - DATLAT" ]
        # [ inline ( always ) ]
        pub fn datlat(&mut self) -> _DATLATW {
            _DATLATW { w: self }
        }
        # [ doc = "Bits 20:23 - CLKDIV" ]
        # [ inline ( always ) ]
        pub fn clkdiv(&mut self) -> _CLKDIVW {
            _CLKDIVW { w: self }
        }
        # [ doc = "Bits 8:15 - DATAST" ]
        # [ inline ( always ) ]
        pub fn datast(&mut self) -> _DATASTW {
            _DATASTW { w: self }
        }
        # [ doc = "Bits 4:7 - ADDHLD" ]
        # [ inline ( always ) ]
        pub fn addhld(&mut self) -> _ADDHLDW {
            _ADDHLDW { w: self }
        }
        # [ doc = "Bits 0:3 - ADDSET" ]
        # [ inline ( always ) ]
        pub fn addset(&mut self) -> _ADDSETW {
            _ADDSETW { w: self }
        }
    }
}
# [ doc = "SRAM/NOR-Flash write timing registers 2" ]
pub struct BWTR2 {
    register: VolatileCell<u32>,
}
# [ doc = "SRAM/NOR-Flash write timing registers 2" ]
pub mod bwtr2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::BWTR2 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ACCMODR {
        bits: u8,
    }
    impl ACCMODR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATLATR {
        bits: u8,
    }
    impl DATLATR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CLKDIVR {
        bits: u8,
    }
    impl CLKDIVR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATASTR {
        bits: u8,
    }
    impl DATASTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ADDHLDR {
        bits: u8,
    }
    impl ADDHLDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ADDSETR {
        bits: u8,
    }
    impl ADDSETR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ACCMODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ACCMODW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DATLATW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATLATW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CLKDIVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CLKDIVW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DATASTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATASTW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADDHLDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADDHLDW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADDSETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADDSETW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 28:29 - ACCMOD" ]
        # [ inline ( always ) ]
        pub fn accmod(&self) -> ACCMODR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ACCMODR { bits }
        }
        # [ doc = "Bits 24:27 - DATLAT" ]
        # [ inline ( always ) ]
        pub fn datlat(&self) -> DATLATR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATLATR { bits }
        }
        # [ doc = "Bits 20:23 - CLKDIV" ]
        # [ inline ( always ) ]
        pub fn clkdiv(&self) -> CLKDIVR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CLKDIVR { bits }
        }
        # [ doc = "Bits 8:15 - DATAST" ]
        # [ inline ( always ) ]
        pub fn datast(&self) -> DATASTR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATASTR { bits }
        }
        # [ doc = "Bits 4:7 - ADDHLD" ]
        # [ inline ( always ) ]
        pub fn addhld(&self) -> ADDHLDR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ADDHLDR { bits }
        }
        # [ doc = "Bits 0:3 - ADDSET" ]
        # [ inline ( always ) ]
        pub fn addset(&self) -> ADDSETR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ADDSETR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 268435455 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 28:29 - ACCMOD" ]
        # [ inline ( always ) ]
        pub fn accmod(&mut self) -> _ACCMODW {
            _ACCMODW { w: self }
        }
        # [ doc = "Bits 24:27 - DATLAT" ]
        # [ inline ( always ) ]
        pub fn datlat(&mut self) -> _DATLATW {
            _DATLATW { w: self }
        }
        # [ doc = "Bits 20:23 - CLKDIV" ]
        # [ inline ( always ) ]
        pub fn clkdiv(&mut self) -> _CLKDIVW {
            _CLKDIVW { w: self }
        }
        # [ doc = "Bits 8:15 - DATAST" ]
        # [ inline ( always ) ]
        pub fn datast(&mut self) -> _DATASTW {
            _DATASTW { w: self }
        }
        # [ doc = "Bits 4:7 - ADDHLD" ]
        # [ inline ( always ) ]
        pub fn addhld(&mut self) -> _ADDHLDW {
            _ADDHLDW { w: self }
        }
        # [ doc = "Bits 0:3 - ADDSET" ]
        # [ inline ( always ) ]
        pub fn addset(&mut self) -> _ADDSETW {
            _ADDSETW { w: self }
        }
    }
}
# [ doc = "SRAM/NOR-Flash write timing registers 3" ]
pub struct BWTR3 {
    register: VolatileCell<u32>,
}
# [ doc = "SRAM/NOR-Flash write timing registers 3" ]
pub mod bwtr3 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::BWTR3 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ACCMODR {
        bits: u8,
    }
    impl ACCMODR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATLATR {
        bits: u8,
    }
    impl DATLATR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CLKDIVR {
        bits: u8,
    }
    impl CLKDIVR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATASTR {
        bits: u8,
    }
    impl DATASTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ADDHLDR {
        bits: u8,
    }
    impl ADDHLDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ADDSETR {
        bits: u8,
    }
    impl ADDSETR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ACCMODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ACCMODW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DATLATW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATLATW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CLKDIVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CLKDIVW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DATASTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATASTW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADDHLDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADDHLDW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADDSETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADDSETW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 28:29 - ACCMOD" ]
        # [ inline ( always ) ]
        pub fn accmod(&self) -> ACCMODR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ACCMODR { bits }
        }
        # [ doc = "Bits 24:27 - DATLAT" ]
        # [ inline ( always ) ]
        pub fn datlat(&self) -> DATLATR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATLATR { bits }
        }
        # [ doc = "Bits 20:23 - CLKDIV" ]
        # [ inline ( always ) ]
        pub fn clkdiv(&self) -> CLKDIVR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CLKDIVR { bits }
        }
        # [ doc = "Bits 8:15 - DATAST" ]
        # [ inline ( always ) ]
        pub fn datast(&self) -> DATASTR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATASTR { bits }
        }
        # [ doc = "Bits 4:7 - ADDHLD" ]
        # [ inline ( always ) ]
        pub fn addhld(&self) -> ADDHLDR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ADDHLDR { bits }
        }
        # [ doc = "Bits 0:3 - ADDSET" ]
        # [ inline ( always ) ]
        pub fn addset(&self) -> ADDSETR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ADDSETR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 268435455 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 28:29 - ACCMOD" ]
        # [ inline ( always ) ]
        pub fn accmod(&mut self) -> _ACCMODW {
            _ACCMODW { w: self }
        }
        # [ doc = "Bits 24:27 - DATLAT" ]
        # [ inline ( always ) ]
        pub fn datlat(&mut self) -> _DATLATW {
            _DATLATW { w: self }
        }
        # [ doc = "Bits 20:23 - CLKDIV" ]
        # [ inline ( always ) ]
        pub fn clkdiv(&mut self) -> _CLKDIVW {
            _CLKDIVW { w: self }
        }
        # [ doc = "Bits 8:15 - DATAST" ]
        # [ inline ( always ) ]
        pub fn datast(&mut self) -> _DATASTW {
            _DATASTW { w: self }
        }
        # [ doc = "Bits 4:7 - ADDHLD" ]
        # [ inline ( always ) ]
        pub fn addhld(&mut self) -> _ADDHLDW {
            _ADDHLDW { w: self }
        }
        # [ doc = "Bits 0:3 - ADDSET" ]
        # [ inline ( always ) ]
        pub fn addset(&mut self) -> _ADDSETW {
            _ADDSETW { w: self }
        }
    }
}
# [ doc = "SRAM/NOR-Flash write timing registers 4" ]
pub struct BWTR4 {
    register: VolatileCell<u32>,
}
# [ doc = "SRAM/NOR-Flash write timing registers 4" ]
pub mod bwtr4 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::BWTR4 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ACCMODR {
        bits: u8,
    }
    impl ACCMODR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATLATR {
        bits: u8,
    }
    impl DATLATR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CLKDIVR {
        bits: u8,
    }
    impl CLKDIVR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATASTR {
        bits: u8,
    }
    impl DATASTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ADDHLDR {
        bits: u8,
    }
    impl ADDHLDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ADDSETR {
        bits: u8,
    }
    impl ADDSETR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ACCMODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ACCMODW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DATLATW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATLATW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CLKDIVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CLKDIVW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DATASTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATASTW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADDHLDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADDHLDW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADDSETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADDSETW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 28:29 - ACCMOD" ]
        # [ inline ( always ) ]
        pub fn accmod(&self) -> ACCMODR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ACCMODR { bits }
        }
        # [ doc = "Bits 24:27 - DATLAT" ]
        # [ inline ( always ) ]
        pub fn datlat(&self) -> DATLATR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATLATR { bits }
        }
        # [ doc = "Bits 20:23 - CLKDIV" ]
        # [ inline ( always ) ]
        pub fn clkdiv(&self) -> CLKDIVR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CLKDIVR { bits }
        }
        # [ doc = "Bits 8:15 - DATAST" ]
        # [ inline ( always ) ]
        pub fn datast(&self) -> DATASTR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATASTR { bits }
        }
        # [ doc = "Bits 4:7 - ADDHLD" ]
        # [ inline ( always ) ]
        pub fn addhld(&self) -> ADDHLDR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ADDHLDR { bits }
        }
        # [ doc = "Bits 0:3 - ADDSET" ]
        # [ inline ( always ) ]
        pub fn addset(&self) -> ADDSETR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ADDSETR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 268435455 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 28:29 - ACCMOD" ]
        # [ inline ( always ) ]
        pub fn accmod(&mut self) -> _ACCMODW {
            _ACCMODW { w: self }
        }
        # [ doc = "Bits 24:27 - DATLAT" ]
        # [ inline ( always ) ]
        pub fn datlat(&mut self) -> _DATLATW {
            _DATLATW { w: self }
        }
        # [ doc = "Bits 20:23 - CLKDIV" ]
        # [ inline ( always ) ]
        pub fn clkdiv(&mut self) -> _CLKDIVW {
            _CLKDIVW { w: self }
        }
        # [ doc = "Bits 8:15 - DATAST" ]
        # [ inline ( always ) ]
        pub fn datast(&mut self) -> _DATASTW {
            _DATASTW { w: self }
        }
        # [ doc = "Bits 4:7 - ADDHLD" ]
        # [ inline ( always ) ]
        pub fn addhld(&mut self) -> _ADDHLDW {
            _ADDHLDW { w: self }
        }
        # [ doc = "Bits 0:3 - ADDSET" ]
        # [ inline ( always ) ]
        pub fn addset(&mut self) -> _ADDSETW {
            _ADDSETW { w: self }
        }
    }
}
# [ doc = "Flexible static memory controller" ]
pub struct FSMC {
    register_block: RegisterBlock,
}
impl Deref for FSMC {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
