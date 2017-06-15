# ! [ doc = "Backup registers" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;
use vcell::VolatileCell;

# [ doc = "Backup registers" ]
pub const BKP: Peripheral<BKP> = unsafe { Peripheral::new(1073769476) };

# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - Backup data register (BKP_DR)" ]
    pub dr1: DR1,
    # [ doc = "0x04 - Backup data register (BKP_DR)" ]
    pub dr2: DR2,
    # [ doc = "0x08 - Backup data register (BKP_DR)" ]
    pub dr3: DR3,
    # [ doc = "0x0c - Backup data register (BKP_DR)" ]
    pub dr4: DR4,
    # [ doc = "0x10 - Backup data register (BKP_DR)" ]
    pub dr5: DR5,
    # [ doc = "0x14 - Backup data register (BKP_DR)" ]
    pub dr6: DR6,
    # [ doc = "0x18 - Backup data register (BKP_DR)" ]
    pub dr7: DR7,
    # [ doc = "0x1c - Backup data register (BKP_DR)" ]
    pub dr8: DR8,
    # [ doc = "0x20 - Backup data register (BKP_DR)" ]
    pub dr9: DR9,
    # [ doc = "0x24 - Backup data register (BKP_DR)" ]
    pub dr10: DR10,
    # [ doc = "0x28 - RTC clock calibration register (BKP_RTCCR)" ]
    pub rtccr: RTCCR,
    # [ doc = "0x2c - Backup control register (BKP_CR)" ]
    pub cr: CR,
    # [ doc = "0x30 - BKP_CSR control/status register (BKP_CSR)" ]
    pub csr: CSR,
    _reserved0: [u8; 8usize],
    # [ doc = "0x3c - Backup data register (BKP_DR)" ]
    pub dr11: DR11,
    # [ doc = "0x40 - Backup data register (BKP_DR)" ]
    pub dr12: DR12,
    # [ doc = "0x44 - Backup data register (BKP_DR)" ]
    pub dr13: DR13,
    # [ doc = "0x48 - Backup data register (BKP_DR)" ]
    pub dr14: DR14,
    # [ doc = "0x4c - Backup data register (BKP_DR)" ]
    pub dr15: DR15,
    # [ doc = "0x50 - Backup data register (BKP_DR)" ]
    pub dr16: DR16,
    # [ doc = "0x54 - Backup data register (BKP_DR)" ]
    pub dr17: DR17,
    # [ doc = "0x58 - Backup data register (BKP_DR)" ]
    pub dr18: DR18,
    # [ doc = "0x5c - Backup data register (BKP_DR)" ]
    pub dr19: DR19,
    # [ doc = "0x60 - Backup data register (BKP_DR)" ]
    pub dr20: DR20,
    # [ doc = "0x64 - Backup data register (BKP_DR)" ]
    pub dr21: DR21,
    # [ doc = "0x68 - Backup data register (BKP_DR)" ]
    pub dr22: DR22,
    # [ doc = "0x6c - Backup data register (BKP_DR)" ]
    pub dr23: DR23,
    # [ doc = "0x70 - Backup data register (BKP_DR)" ]
    pub dr24: DR24,
    # [ doc = "0x74 - Backup data register (BKP_DR)" ]
    pub dr25: DR25,
    # [ doc = "0x78 - Backup data register (BKP_DR)" ]
    pub dr26: DR26,
    # [ doc = "0x7c - Backup data register (BKP_DR)" ]
    pub dr27: DR27,
    # [ doc = "0x80 - Backup data register (BKP_DR)" ]
    pub dr28: DR28,
    # [ doc = "0x84 - Backup data register (BKP_DR)" ]
    pub dr29: DR29,
    # [ doc = "0x88 - Backup data register (BKP_DR)" ]
    pub dr30: DR30,
    # [ doc = "0x8c - Backup data register (BKP_DR)" ]
    pub dr31: DR31,
    # [ doc = "0x90 - Backup data register (BKP_DR)" ]
    pub dr32: DR32,
    # [ doc = "0x94 - Backup data register (BKP_DR)" ]
    pub dr33: DR33,
    # [ doc = "0x98 - Backup data register (BKP_DR)" ]
    pub dr34: DR34,
    # [ doc = "0x9c - Backup data register (BKP_DR)" ]
    pub dr35: DR35,
    # [ doc = "0xa0 - Backup data register (BKP_DR)" ]
    pub dr36: DR36,
    # [ doc = "0xa4 - Backup data register (BKP_DR)" ]
    pub dr37: DR37,
    # [ doc = "0xa8 - Backup data register (BKP_DR)" ]
    pub dr38: DR38,
    # [ doc = "0xac - Backup data register (BKP_DR)" ]
    pub dr39: DR39,
    # [ doc = "0xb0 - Backup data register (BKP_DR)" ]
    pub dr40: DR40,
    # [ doc = "0xb4 - Backup data register (BKP_DR)" ]
    pub dr41: DR41,
    # [ doc = "0xb8 - Backup data register (BKP_DR)" ]
    pub dr42: DR42,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR1 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR1 {
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
    pub struct D1R {
        bits: u16,
    }
    impl D1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D1W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d1(&self) -> D1R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D1R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d1(&mut self) -> _D1W {
            _D1W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR2 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR2 {
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
    pub struct D2R {
        bits: u16,
    }
    impl D2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D2W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d2(&self) -> D2R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D2R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d2(&mut self) -> _D2W {
            _D2W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR3 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr3 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR3 {
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
    pub struct D3R {
        bits: u16,
    }
    impl D3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D3W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d3(&self) -> D3R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D3R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d3(&mut self) -> _D3W {
            _D3W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR4 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr4 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR4 {
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
    pub struct D4R {
        bits: u16,
    }
    impl D4R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D4W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d4(&self) -> D4R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D4R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d4(&mut self) -> _D4W {
            _D4W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR5 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr5 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR5 {
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
    pub struct D5R {
        bits: u16,
    }
    impl D5R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D5W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d5(&self) -> D5R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D5R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d5(&mut self) -> _D5W {
            _D5W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR6 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr6 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR6 {
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
    pub struct D6R {
        bits: u16,
    }
    impl D6R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D6W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d6(&self) -> D6R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D6R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d6(&mut self) -> _D6W {
            _D6W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR7 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr7 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR7 {
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
    pub struct D7R {
        bits: u16,
    }
    impl D7R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D7W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d7(&self) -> D7R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D7R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d7(&mut self) -> _D7W {
            _D7W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR8 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr8 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR8 {
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
    pub struct D8R {
        bits: u16,
    }
    impl D8R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D8W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d8(&self) -> D8R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D8R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d8(&mut self) -> _D8W {
            _D8W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR9 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr9 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR9 {
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
    pub struct D9R {
        bits: u16,
    }
    impl D9R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D9W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d9(&self) -> D9R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D9R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d9(&mut self) -> _D9W {
            _D9W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR10 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr10 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR10 {
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
    pub struct D10R {
        bits: u16,
    }
    impl D10R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D10W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d10(&self) -> D10R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D10R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d10(&mut self) -> _D10W {
            _D10W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR11 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr11 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR11 {
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
    pub struct DR11R {
        bits: u16,
    }
    impl DR11R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DR11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DR11W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn dr11(&self) -> DR11R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            DR11R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn dr11(&mut self) -> _DR11W {
            _DR11W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR12 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr12 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR12 {
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
    pub struct DR12R {
        bits: u16,
    }
    impl DR12R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DR12W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DR12W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn dr12(&self) -> DR12R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            DR12R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn dr12(&mut self) -> _DR12W {
            _DR12W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR13 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr13 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR13 {
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
    pub struct DR13R {
        bits: u16,
    }
    impl DR13R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DR13W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DR13W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn dr13(&self) -> DR13R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            DR13R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn dr13(&mut self) -> _DR13W {
            _DR13W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR14 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr14 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR14 {
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
    pub struct D14R {
        bits: u16,
    }
    impl D14R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D14W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D14W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d14(&self) -> D14R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D14R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d14(&mut self) -> _D14W {
            _D14W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR15 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr15 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR15 {
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
    pub struct D15R {
        bits: u16,
    }
    impl D15R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D15W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D15W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d15(&self) -> D15R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D15R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d15(&mut self) -> _D15W {
            _D15W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR16 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr16 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR16 {
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
    pub struct D16R {
        bits: u16,
    }
    impl D16R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D16W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D16W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d16(&self) -> D16R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D16R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d16(&mut self) -> _D16W {
            _D16W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR17 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr17 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR17 {
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
    pub struct D17R {
        bits: u16,
    }
    impl D17R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D17W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D17W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d17(&self) -> D17R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D17R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d17(&mut self) -> _D17W {
            _D17W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR18 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr18 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR18 {
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
    pub struct D18R {
        bits: u16,
    }
    impl D18R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D18W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D18W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d18(&self) -> D18R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D18R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d18(&mut self) -> _D18W {
            _D18W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR19 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr19 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR19 {
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
    pub struct D19R {
        bits: u16,
    }
    impl D19R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D19W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D19W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d19(&self) -> D19R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D19R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d19(&mut self) -> _D19W {
            _D19W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR20 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr20 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR20 {
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
    pub struct D20R {
        bits: u16,
    }
    impl D20R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D20W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D20W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d20(&self) -> D20R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D20R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d20(&mut self) -> _D20W {
            _D20W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR21 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr21 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR21 {
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
    pub struct D21R {
        bits: u16,
    }
    impl D21R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D21W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D21W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d21(&self) -> D21R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D21R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d21(&mut self) -> _D21W {
            _D21W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR22 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr22 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR22 {
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
    pub struct D22R {
        bits: u16,
    }
    impl D22R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D22W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D22W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d22(&self) -> D22R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D22R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d22(&mut self) -> _D22W {
            _D22W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR23 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr23 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR23 {
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
    pub struct D23R {
        bits: u16,
    }
    impl D23R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D23W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D23W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d23(&self) -> D23R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D23R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d23(&mut self) -> _D23W {
            _D23W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR24 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr24 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR24 {
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
    pub struct D24R {
        bits: u16,
    }
    impl D24R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D24W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D24W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d24(&self) -> D24R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D24R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d24(&mut self) -> _D24W {
            _D24W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR25 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr25 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR25 {
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
    pub struct D25R {
        bits: u16,
    }
    impl D25R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D25W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D25W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d25(&self) -> D25R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D25R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d25(&mut self) -> _D25W {
            _D25W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR26 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr26 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR26 {
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
    pub struct D26R {
        bits: u16,
    }
    impl D26R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D26W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D26W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d26(&self) -> D26R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D26R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d26(&mut self) -> _D26W {
            _D26W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR27 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr27 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR27 {
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
    pub struct D27R {
        bits: u16,
    }
    impl D27R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D27W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D27W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d27(&self) -> D27R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D27R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d27(&mut self) -> _D27W {
            _D27W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR28 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr28 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR28 {
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
    pub struct D28R {
        bits: u16,
    }
    impl D28R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D28W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D28W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d28(&self) -> D28R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D28R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d28(&mut self) -> _D28W {
            _D28W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR29 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr29 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR29 {
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
    pub struct D29R {
        bits: u16,
    }
    impl D29R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D29W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D29W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d29(&self) -> D29R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D29R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d29(&mut self) -> _D29W {
            _D29W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR30 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr30 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR30 {
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
    pub struct D30R {
        bits: u16,
    }
    impl D30R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D30W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D30W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d30(&self) -> D30R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D30R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d30(&mut self) -> _D30W {
            _D30W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR31 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr31 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR31 {
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
    pub struct D31R {
        bits: u16,
    }
    impl D31R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D31W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D31W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d31(&self) -> D31R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D31R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d31(&mut self) -> _D31W {
            _D31W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR32 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr32 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR32 {
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
    pub struct D32R {
        bits: u16,
    }
    impl D32R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D32W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D32W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d32(&self) -> D32R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D32R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d32(&mut self) -> _D32W {
            _D32W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR33 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr33 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR33 {
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
    pub struct D33R {
        bits: u16,
    }
    impl D33R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D33W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D33W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d33(&self) -> D33R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D33R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d33(&mut self) -> _D33W {
            _D33W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR34 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr34 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR34 {
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
    pub struct D34R {
        bits: u16,
    }
    impl D34R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D34W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D34W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d34(&self) -> D34R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D34R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d34(&mut self) -> _D34W {
            _D34W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR35 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr35 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR35 {
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
    pub struct D35R {
        bits: u16,
    }
    impl D35R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D35W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D35W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d35(&self) -> D35R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D35R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d35(&mut self) -> _D35W {
            _D35W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR36 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr36 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR36 {
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
    pub struct D36R {
        bits: u16,
    }
    impl D36R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D36W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D36W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d36(&self) -> D36R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D36R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d36(&mut self) -> _D36W {
            _D36W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR37 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr37 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR37 {
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
    pub struct D37R {
        bits: u16,
    }
    impl D37R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D37W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D37W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d37(&self) -> D37R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D37R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d37(&mut self) -> _D37W {
            _D37W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR38 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr38 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR38 {
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
    pub struct D38R {
        bits: u16,
    }
    impl D38R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D38W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D38W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d38(&self) -> D38R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D38R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d38(&mut self) -> _D38W {
            _D38W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR39 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr39 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR39 {
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
    pub struct D39R {
        bits: u16,
    }
    impl D39R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D39W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D39W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d39(&self) -> D39R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D39R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d39(&mut self) -> _D39W {
            _D39W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR40 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr40 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR40 {
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
    pub struct D40R {
        bits: u16,
    }
    impl D40R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D40W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D40W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d40(&self) -> D40R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D40R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d40(&mut self) -> _D40W {
            _D40W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR41 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr41 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR41 {
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
    pub struct D41R {
        bits: u16,
    }
    impl D41R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D41W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D41W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d41(&self) -> D41R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D41R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d41(&mut self) -> _D41W {
            _D41W { w: self }
        }
    }
}
# [ doc = "Backup data register (BKP_DR)" ]
pub struct DR42 {
    register: VolatileCell<u32>,
}
# [ doc = "Backup data register (BKP_DR)" ]
pub mod dr42 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR42 {
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
    pub struct D42R {
        bits: u16,
    }
    impl D42R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _D42W<'a> {
        w: &'a mut W,
    }
    impl<'a> _D42W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
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
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d42(&self) -> D42R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            D42R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Backup data" ]
        # [ inline ( always ) ]
        pub fn d42(&mut self) -> _D42W {
            _D42W { w: self }
        }
    }
}
# [ doc = "RTC clock calibration register (BKP_RTCCR)" ]
pub struct RTCCR {
    register: VolatileCell<u32>,
}
# [ doc = "RTC clock calibration register (BKP_RTCCR)" ]
pub mod rtccr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::RTCCR {
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
    pub struct CALR {
        bits: u8,
    }
    impl CALR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CCOR {
        bits: bool,
    }
    impl CCOR {
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
    pub struct ASOER {
        bits: bool,
    }
    impl ASOER {
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
    pub struct ASOSR {
        bits: bool,
    }
    impl ASOSR {
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
    pub struct _CALW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CALW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CCOW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CCOW<'a> {
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
            const OFFSET: u8 = 7;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ASOEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ASOEW<'a> {
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
    pub struct _ASOSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ASOSW<'a> {
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:6 - Calibration value" ]
        # [ inline ( always ) ]
        pub fn cal(&self) -> CALR {
            let bits = {
                const MASK: u8 = 127;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CALR { bits }
        }
        # [ doc = "Bit 7 - Calibration Clock Output" ]
        # [ inline ( always ) ]
        pub fn cco(&self) -> CCOR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CCOR { bits }
        }
        # [ doc = "Bit 8 - Alarm or second output enable" ]
        # [ inline ( always ) ]
        pub fn asoe(&self) -> ASOER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ASOER { bits }
        }
        # [ doc = "Bit 9 - Alarm or second output selection" ]
        # [ inline ( always ) ]
        pub fn asos(&self) -> ASOSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ASOSR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:6 - Calibration value" ]
        # [ inline ( always ) ]
        pub fn cal(&mut self) -> _CALW {
            _CALW { w: self }
        }
        # [ doc = "Bit 7 - Calibration Clock Output" ]
        # [ inline ( always ) ]
        pub fn cco(&mut self) -> _CCOW {
            _CCOW { w: self }
        }
        # [ doc = "Bit 8 - Alarm or second output enable" ]
        # [ inline ( always ) ]
        pub fn asoe(&mut self) -> _ASOEW {
            _ASOEW { w: self }
        }
        # [ doc = "Bit 9 - Alarm or second output selection" ]
        # [ inline ( always ) ]
        pub fn asos(&mut self) -> _ASOSW {
            _ASOSW { w: self }
        }
    }
}
# [ doc = "Backup control register (BKP_CR)" ]
pub struct CR {
    register: VolatileCell<u32>,
}
# [ doc = "Backup control register (BKP_CR)" ]
pub mod cr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CR {
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
    pub struct TPER {
        bits: bool,
    }
    impl TPER {
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
    pub struct TPALR {
        bits: bool,
    }
    impl TPALR {
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
    pub struct _TPEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TPEW<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _TPALW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TPALW<'a> {
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
        # [ doc = "Bit 0 - Tamper pin enable" ]
        # [ inline ( always ) ]
        pub fn tpe(&self) -> TPER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TPER { bits }
        }
        # [ doc = "Bit 1 - Tamper pin active level" ]
        # [ inline ( always ) ]
        pub fn tpal(&self) -> TPALR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TPALR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 0 - Tamper pin enable" ]
        # [ inline ( always ) ]
        pub fn tpe(&mut self) -> _TPEW {
            _TPEW { w: self }
        }
        # [ doc = "Bit 1 - Tamper pin active level" ]
        # [ inline ( always ) ]
        pub fn tpal(&mut self) -> _TPALW {
            _TPALW { w: self }
        }
    }
}
# [ doc = "BKP_CSR control/status register (BKP_CSR)" ]
pub struct CSR {
    register: VolatileCell<u32>,
}
# [ doc = "BKP_CSR control/status register (BKP_CSR)" ]
pub mod csr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CSR {
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
    pub struct TPIER {
        bits: bool,
    }
    impl TPIER {
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
    pub struct TEFR {
        bits: bool,
    }
    impl TEFR {
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
    pub struct TIFR {
        bits: bool,
    }
    impl TIFR {
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
    pub struct _CTEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CTEW<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _CTIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CTIW<'a> {
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
    pub struct _TPIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TPIEW<'a> {
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 2 - Tamper Pin interrupt enable" ]
        # [ inline ( always ) ]
        pub fn tpie(&self) -> TPIER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TPIER { bits }
        }
        # [ doc = "Bit 8 - Tamper Event Flag" ]
        # [ inline ( always ) ]
        pub fn tef(&self) -> TEFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TEFR { bits }
        }
        # [ doc = "Bit 9 - Tamper Interrupt Flag" ]
        # [ inline ( always ) ]
        pub fn tif(&self) -> TIFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIFR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 0 - Clear Tamper event" ]
        # [ inline ( always ) ]
        pub fn cte(&mut self) -> _CTEW {
            _CTEW { w: self }
        }
        # [ doc = "Bit 1 - Clear Tamper Interrupt" ]
        # [ inline ( always ) ]
        pub fn cti(&mut self) -> _CTIW {
            _CTIW { w: self }
        }
        # [ doc = "Bit 2 - Tamper Pin interrupt enable" ]
        # [ inline ( always ) ]
        pub fn tpie(&mut self) -> _TPIEW {
            _TPIEW { w: self }
        }
    }
}
# [ doc = "Backup registers" ]
pub struct BKP {
    register_block: RegisterBlock,
}
impl Deref for BKP {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
