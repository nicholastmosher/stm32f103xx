# ! [ doc = "Analog to digital converter" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;
use vcell::VolatileCell;

# [ doc = "Analog to digital converter" ]
pub const ADC1: Peripheral<ADC1> = unsafe { Peripheral::new(1073816576) };

# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - status register" ]
    pub sr: SR,
    # [ doc = "0x04 - control register 1" ]
    pub cr1: CR1,
    # [ doc = "0x08 - control register 2" ]
    pub cr2: CR2,
    # [ doc = "0x0c - sample time register 1" ]
    pub smpr1: SMPR1,
    # [ doc = "0x10 - sample time register 2" ]
    pub smpr2: SMPR2,
    # [ doc = "0x14 - injected channel data offset register x" ]
    pub jofr1: JOFR1,
    # [ doc = "0x18 - injected channel data offset register x" ]
    pub jofr2: JOFR2,
    # [ doc = "0x1c - injected channel data offset register x" ]
    pub jofr3: JOFR3,
    # [ doc = "0x20 - injected channel data offset register x" ]
    pub jofr4: JOFR4,
    # [ doc = "0x24 - watchdog higher threshold register" ]
    pub htr: HTR,
    # [ doc = "0x28 - watchdog lower threshold register" ]
    pub ltr: LTR,
    # [ doc = "0x2c - regular sequence register 1" ]
    pub sqr1: SQR1,
    # [ doc = "0x30 - regular sequence register 2" ]
    pub sqr2: SQR2,
    # [ doc = "0x34 - regular sequence register 3" ]
    pub sqr3: SQR3,
    # [ doc = "0x38 - injected sequence register" ]
    pub jsqr: JSQR,
    # [ doc = "0x3c - injected data register x" ]
    pub jdr1: JDR1,
    # [ doc = "0x40 - injected data register x" ]
    pub jdr2: JDR2,
    # [ doc = "0x44 - injected data register x" ]
    pub jdr3: JDR3,
    # [ doc = "0x48 - injected data register x" ]
    pub jdr4: JDR4,
    # [ doc = "0x4c - regular data register" ]
    pub dr: DR,
}
# [ doc = "status register" ]
pub struct SR {
    register: VolatileCell<u32>,
}
# [ doc = "status register" ]
pub mod sr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::SR {
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
    pub struct STRTR {
        bits: bool,
    }
    impl STRTR {
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
    pub struct JSTRTR {
        bits: bool,
    }
    impl JSTRTR {
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
    pub struct JEOCR {
        bits: bool,
    }
    impl JEOCR {
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
    pub struct EOCR {
        bits: bool,
    }
    impl EOCR {
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
    pub struct AWDR {
        bits: bool,
    }
    impl AWDR {
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
    pub struct _STRTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STRTW<'a> {
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
    pub struct _JSTRTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _JSTRTW<'a> {
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
    pub struct _JEOCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _JEOCW<'a> {
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
    pub struct _EOCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EOCW<'a> {
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
    pub struct _AWDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _AWDW<'a> {
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
        # [ doc = "Bit 4 - Regular channel start flag" ]
        # [ inline ( always ) ]
        pub fn strt(&self) -> STRTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            STRTR { bits }
        }
        # [ doc = "Bit 3 - Injected channel start flag" ]
        # [ inline ( always ) ]
        pub fn jstrt(&self) -> JSTRTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            JSTRTR { bits }
        }
        # [ doc = "Bit 2 - Injected channel end of conversion" ]
        # [ inline ( always ) ]
        pub fn jeoc(&self) -> JEOCR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            JEOCR { bits }
        }
        # [ doc = "Bit 1 - Regular channel end of conversion" ]
        # [ inline ( always ) ]
        pub fn eoc(&self) -> EOCR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EOCR { bits }
        }
        # [ doc = "Bit 0 - Analog watchdog flag" ]
        # [ inline ( always ) ]
        pub fn awd(&self) -> AWDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            AWDR { bits }
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
        # [ doc = "Bit 4 - Regular channel start flag" ]
        # [ inline ( always ) ]
        pub fn strt(&mut self) -> _STRTW {
            _STRTW { w: self }
        }
        # [ doc = "Bit 3 - Injected channel start flag" ]
        # [ inline ( always ) ]
        pub fn jstrt(&mut self) -> _JSTRTW {
            _JSTRTW { w: self }
        }
        # [ doc = "Bit 2 - Injected channel end of conversion" ]
        # [ inline ( always ) ]
        pub fn jeoc(&mut self) -> _JEOCW {
            _JEOCW { w: self }
        }
        # [ doc = "Bit 1 - Regular channel end of conversion" ]
        # [ inline ( always ) ]
        pub fn eoc(&mut self) -> _EOCW {
            _EOCW { w: self }
        }
        # [ doc = "Bit 0 - Analog watchdog flag" ]
        # [ inline ( always ) ]
        pub fn awd(&mut self) -> _AWDW {
            _AWDW { w: self }
        }
    }
}
# [ doc = "control register 1" ]
pub struct CR1 {
    register: VolatileCell<u32>,
}
# [ doc = "control register 1" ]
pub mod cr1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CR1 {
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
    pub struct AWDENR {
        bits: bool,
    }
    impl AWDENR {
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
    pub struct JAWDENR {
        bits: bool,
    }
    impl JAWDENR {
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
    pub struct DUALMODR {
        bits: u8,
    }
    impl DUALMODR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DISCNUMR {
        bits: u8,
    }
    impl DISCNUMR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct JDISCENR {
        bits: bool,
    }
    impl JDISCENR {
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
    pub struct DISCENR {
        bits: bool,
    }
    impl DISCENR {
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
    pub struct JAUTOR {
        bits: bool,
    }
    impl JAUTOR {
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
    pub struct AWDSGLR {
        bits: bool,
    }
    impl AWDSGLR {
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
    pub struct SCANR {
        bits: bool,
    }
    impl SCANR {
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
    pub struct JEOCIER {
        bits: bool,
    }
    impl JEOCIER {
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
    pub struct AWDIER {
        bits: bool,
    }
    impl AWDIER {
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
    pub struct EOCIER {
        bits: bool,
    }
    impl EOCIER {
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
    pub struct AWDCHR {
        bits: u8,
    }
    impl AWDCHR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _AWDENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _AWDENW<'a> {
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
            const OFFSET: u8 = 23;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _JAWDENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _JAWDENW<'a> {
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
            const OFFSET: u8 = 22;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DUALMODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DUALMODW<'a> {
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
    pub struct _DISCNUMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DISCNUMW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _JDISCENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _JDISCENW<'a> {
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
    pub struct _DISCENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DISCENW<'a> {
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
    pub struct _JAUTOW<'a> {
        w: &'a mut W,
    }
    impl<'a> _JAUTOW<'a> {
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
    pub struct _AWDSGLW<'a> {
        w: &'a mut W,
    }
    impl<'a> _AWDSGLW<'a> {
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
    pub struct _SCANW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SCANW<'a> {
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
    pub struct _JEOCIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _JEOCIEW<'a> {
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
    pub struct _AWDIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _AWDIEW<'a> {
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
    pub struct _EOCIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EOCIEW<'a> {
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
    pub struct _AWDCHW<'a> {
        w: &'a mut W,
    }
    impl<'a> _AWDCHW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
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
        # [ doc = "Bit 23 - Analog watchdog enable on regular channels" ]
        # [ inline ( always ) ]
        pub fn awden(&self) -> AWDENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 23;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            AWDENR { bits }
        }
        # [ doc = "Bit 22 - Analog watchdog enable on injected channels" ]
        # [ inline ( always ) ]
        pub fn jawden(&self) -> JAWDENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 22;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            JAWDENR { bits }
        }
        # [ doc = "Bits 16:19 - Dual mode selection" ]
        # [ inline ( always ) ]
        pub fn dualmod(&self) -> DUALMODR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DUALMODR { bits }
        }
        # [ doc = "Bits 13:15 - Discontinuous mode channel count" ]
        # [ inline ( always ) ]
        pub fn discnum(&self) -> DISCNUMR {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DISCNUMR { bits }
        }
        # [ doc = "Bit 12 - Discontinuous mode on injected channels" ]
        # [ inline ( always ) ]
        pub fn jdiscen(&self) -> JDISCENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            JDISCENR { bits }
        }
        # [ doc = "Bit 11 - Discontinuous mode on regular channels" ]
        # [ inline ( always ) ]
        pub fn discen(&self) -> DISCENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DISCENR { bits }
        }
        # [ doc = "Bit 10 - Automatic injected group conversion" ]
        # [ inline ( always ) ]
        pub fn jauto(&self) -> JAUTOR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            JAUTOR { bits }
        }
        # [ doc = "Bit 9 - Enable the watchdog on a single channel in scan mode" ]
        # [ inline ( always ) ]
        pub fn awdsgl(&self) -> AWDSGLR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            AWDSGLR { bits }
        }
        # [ doc = "Bit 8 - Scan mode" ]
        # [ inline ( always ) ]
        pub fn scan(&self) -> SCANR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SCANR { bits }
        }
        # [ doc = "Bit 7 - Interrupt enable for injected channels" ]
        # [ inline ( always ) ]
        pub fn jeocie(&self) -> JEOCIER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            JEOCIER { bits }
        }
        # [ doc = "Bit 6 - Analog watchdog interrupt enable" ]
        # [ inline ( always ) ]
        pub fn awdie(&self) -> AWDIER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            AWDIER { bits }
        }
        # [ doc = "Bit 5 - Interrupt enable for EOC" ]
        # [ inline ( always ) ]
        pub fn eocie(&self) -> EOCIER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EOCIER { bits }
        }
        # [ doc = "Bits 0:4 - Analog watchdog channel select bits" ]
        # [ inline ( always ) ]
        pub fn awdch(&self) -> AWDCHR {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            AWDCHR { bits }
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
        # [ doc = "Bit 23 - Analog watchdog enable on regular channels" ]
        # [ inline ( always ) ]
        pub fn awden(&mut self) -> _AWDENW {
            _AWDENW { w: self }
        }
        # [ doc = "Bit 22 - Analog watchdog enable on injected channels" ]
        # [ inline ( always ) ]
        pub fn jawden(&mut self) -> _JAWDENW {
            _JAWDENW { w: self }
        }
        # [ doc = "Bits 16:19 - Dual mode selection" ]
        # [ inline ( always ) ]
        pub fn dualmod(&mut self) -> _DUALMODW {
            _DUALMODW { w: self }
        }
        # [ doc = "Bits 13:15 - Discontinuous mode channel count" ]
        # [ inline ( always ) ]
        pub fn discnum(&mut self) -> _DISCNUMW {
            _DISCNUMW { w: self }
        }
        # [ doc = "Bit 12 - Discontinuous mode on injected channels" ]
        # [ inline ( always ) ]
        pub fn jdiscen(&mut self) -> _JDISCENW {
            _JDISCENW { w: self }
        }
        # [ doc = "Bit 11 - Discontinuous mode on regular channels" ]
        # [ inline ( always ) ]
        pub fn discen(&mut self) -> _DISCENW {
            _DISCENW { w: self }
        }
        # [ doc = "Bit 10 - Automatic injected group conversion" ]
        # [ inline ( always ) ]
        pub fn jauto(&mut self) -> _JAUTOW {
            _JAUTOW { w: self }
        }
        # [ doc = "Bit 9 - Enable the watchdog on a single channel in scan mode" ]
        # [ inline ( always ) ]
        pub fn awdsgl(&mut self) -> _AWDSGLW {
            _AWDSGLW { w: self }
        }
        # [ doc = "Bit 8 - Scan mode" ]
        # [ inline ( always ) ]
        pub fn scan(&mut self) -> _SCANW {
            _SCANW { w: self }
        }
        # [ doc = "Bit 7 - Interrupt enable for injected channels" ]
        # [ inline ( always ) ]
        pub fn jeocie(&mut self) -> _JEOCIEW {
            _JEOCIEW { w: self }
        }
        # [ doc = "Bit 6 - Analog watchdog interrupt enable" ]
        # [ inline ( always ) ]
        pub fn awdie(&mut self) -> _AWDIEW {
            _AWDIEW { w: self }
        }
        # [ doc = "Bit 5 - Interrupt enable for EOC" ]
        # [ inline ( always ) ]
        pub fn eocie(&mut self) -> _EOCIEW {
            _EOCIEW { w: self }
        }
        # [ doc = "Bits 0:4 - Analog watchdog channel select bits" ]
        # [ inline ( always ) ]
        pub fn awdch(&mut self) -> _AWDCHW {
            _AWDCHW { w: self }
        }
    }
}
# [ doc = "control register 2" ]
pub struct CR2 {
    register: VolatileCell<u32>,
}
# [ doc = "control register 2" ]
pub mod cr2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CR2 {
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
    pub struct TSVREFER {
        bits: bool,
    }
    impl TSVREFER {
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
    pub struct SWSTARTR {
        bits: bool,
    }
    impl SWSTARTR {
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
    pub struct JSWSTARTR {
        bits: bool,
    }
    impl JSWSTARTR {
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
    pub struct EXTTRIGR {
        bits: bool,
    }
    impl EXTTRIGR {
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
    pub struct EXTSELR {
        bits: u8,
    }
    impl EXTSELR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct JEXTTRIGR {
        bits: bool,
    }
    impl JEXTTRIGR {
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
    pub struct JEXTSELR {
        bits: u8,
    }
    impl JEXTSELR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ALIGNR {
        bits: bool,
    }
    impl ALIGNR {
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
    pub struct DMAR {
        bits: bool,
    }
    impl DMAR {
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
    pub struct RSTCALR {
        bits: bool,
    }
    impl RSTCALR {
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
    pub struct CALR {
        bits: bool,
    }
    impl CALR {
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
    pub struct CONTR {
        bits: bool,
    }
    impl CONTR {
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
    pub struct ADONR {
        bits: bool,
    }
    impl ADONR {
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
    pub struct _TSVREFEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TSVREFEW<'a> {
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
            const OFFSET: u8 = 23;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SWSTARTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWSTARTW<'a> {
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
            const OFFSET: u8 = 22;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _JSWSTARTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _JSWSTARTW<'a> {
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
            const OFFSET: u8 = 21;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EXTTRIGW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTTRIGW<'a> {
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
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EXTSELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTSELW<'a> {
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
    pub struct _JEXTTRIGW<'a> {
        w: &'a mut W,
    }
    impl<'a> _JEXTTRIGW<'a> {
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
    pub struct _JEXTSELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _JEXTSELW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ALIGNW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ALIGNW<'a> {
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
    pub struct _DMAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DMAW<'a> {
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
    pub struct _RSTCALW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RSTCALW<'a> {
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
    pub struct _CALW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CALW<'a> {
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
    pub struct _CONTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CONTW<'a> {
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
    pub struct _ADONW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADONW<'a> {
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
        # [ doc = "Bit 23 - Temperature sensor and VREFINT enable" ]
        # [ inline ( always ) ]
        pub fn tsvrefe(&self) -> TSVREFER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 23;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TSVREFER { bits }
        }
        # [ doc = "Bit 22 - Start conversion of regular channels" ]
        # [ inline ( always ) ]
        pub fn swstart(&self) -> SWSTARTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 22;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SWSTARTR { bits }
        }
        # [ doc = "Bit 21 - Start conversion of injected channels" ]
        # [ inline ( always ) ]
        pub fn jswstart(&self) -> JSWSTARTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 21;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            JSWSTARTR { bits }
        }
        # [ doc = "Bit 20 - External trigger conversion mode for regular channels" ]
        # [ inline ( always ) ]
        pub fn exttrig(&self) -> EXTTRIGR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTTRIGR { bits }
        }
        # [ doc = "Bits 17:19 - External event select for regular group" ]
        # [ inline ( always ) ]
        pub fn extsel(&self) -> EXTSELR {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EXTSELR { bits }
        }
        # [ doc = "Bit 15 - External trigger conversion mode for injected channels" ]
        # [ inline ( always ) ]
        pub fn jexttrig(&self) -> JEXTTRIGR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            JEXTTRIGR { bits }
        }
        # [ doc = "Bits 12:14 - External event select for injected group" ]
        # [ inline ( always ) ]
        pub fn jextsel(&self) -> JEXTSELR {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            JEXTSELR { bits }
        }
        # [ doc = "Bit 11 - Data alignment" ]
        # [ inline ( always ) ]
        pub fn align(&self) -> ALIGNR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ALIGNR { bits }
        }
        # [ doc = "Bit 8 - Direct memory access mode" ]
        # [ inline ( always ) ]
        pub fn dma(&self) -> DMAR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DMAR { bits }
        }
        # [ doc = "Bit 3 - Reset calibration" ]
        # [ inline ( always ) ]
        pub fn rstcal(&self) -> RSTCALR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RSTCALR { bits }
        }
        # [ doc = "Bit 2 - A/D calibration" ]
        # [ inline ( always ) ]
        pub fn cal(&self) -> CALR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CALR { bits }
        }
        # [ doc = "Bit 1 - Continuous conversion" ]
        # [ inline ( always ) ]
        pub fn cont(&self) -> CONTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CONTR { bits }
        }
        # [ doc = "Bit 0 - A/D converter ON / OFF" ]
        # [ inline ( always ) ]
        pub fn adon(&self) -> ADONR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ADONR { bits }
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
        # [ doc = "Bit 23 - Temperature sensor and VREFINT enable" ]
        # [ inline ( always ) ]
        pub fn tsvrefe(&mut self) -> _TSVREFEW {
            _TSVREFEW { w: self }
        }
        # [ doc = "Bit 22 - Start conversion of regular channels" ]
        # [ inline ( always ) ]
        pub fn swstart(&mut self) -> _SWSTARTW {
            _SWSTARTW { w: self }
        }
        # [ doc = "Bit 21 - Start conversion of injected channels" ]
        # [ inline ( always ) ]
        pub fn jswstart(&mut self) -> _JSWSTARTW {
            _JSWSTARTW { w: self }
        }
        # [ doc = "Bit 20 - External trigger conversion mode for regular channels" ]
        # [ inline ( always ) ]
        pub fn exttrig(&mut self) -> _EXTTRIGW {
            _EXTTRIGW { w: self }
        }
        # [ doc = "Bits 17:19 - External event select for regular group" ]
        # [ inline ( always ) ]
        pub fn extsel(&mut self) -> _EXTSELW {
            _EXTSELW { w: self }
        }
        # [ doc = "Bit 15 - External trigger conversion mode for injected channels" ]
        # [ inline ( always ) ]
        pub fn jexttrig(&mut self) -> _JEXTTRIGW {
            _JEXTTRIGW { w: self }
        }
        # [ doc = "Bits 12:14 - External event select for injected group" ]
        # [ inline ( always ) ]
        pub fn jextsel(&mut self) -> _JEXTSELW {
            _JEXTSELW { w: self }
        }
        # [ doc = "Bit 11 - Data alignment" ]
        # [ inline ( always ) ]
        pub fn align(&mut self) -> _ALIGNW {
            _ALIGNW { w: self }
        }
        # [ doc = "Bit 8 - Direct memory access mode" ]
        # [ inline ( always ) ]
        pub fn dma(&mut self) -> _DMAW {
            _DMAW { w: self }
        }
        # [ doc = "Bit 3 - Reset calibration" ]
        # [ inline ( always ) ]
        pub fn rstcal(&mut self) -> _RSTCALW {
            _RSTCALW { w: self }
        }
        # [ doc = "Bit 2 - A/D calibration" ]
        # [ inline ( always ) ]
        pub fn cal(&mut self) -> _CALW {
            _CALW { w: self }
        }
        # [ doc = "Bit 1 - Continuous conversion" ]
        # [ inline ( always ) ]
        pub fn cont(&mut self) -> _CONTW {
            _CONTW { w: self }
        }
        # [ doc = "Bit 0 - A/D converter ON / OFF" ]
        # [ inline ( always ) ]
        pub fn adon(&mut self) -> _ADONW {
            _ADONW { w: self }
        }
    }
}
# [ doc = "sample time register 1" ]
pub struct SMPR1 {
    register: VolatileCell<u32>,
}
# [ doc = "sample time register 1" ]
pub mod smpr1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::SMPR1 {
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
    pub struct SMP10R {
        bits: u8,
    }
    impl SMP10R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SMP11R {
        bits: u8,
    }
    impl SMP11R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SMP12R {
        bits: u8,
    }
    impl SMP12R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SMP13R {
        bits: u8,
    }
    impl SMP13R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SMP14R {
        bits: u8,
    }
    impl SMP14R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SMP15R {
        bits: u8,
    }
    impl SMP15R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SMP16R {
        bits: u8,
    }
    impl SMP16R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SMP17R {
        bits: u8,
    }
    impl SMP17R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SMP10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SMP10W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SMP11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SMP11W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SMP12W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SMP12W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SMP13W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SMP13W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SMP14W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SMP14W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SMP15W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SMP15W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 15;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SMP16W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SMP16W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SMP17W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SMP17W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 21;
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
        # [ doc = "Bits 0:2 - Channel 10 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp10(&self) -> SMP10R {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SMP10R { bits }
        }
        # [ doc = "Bits 3:5 - Channel 11 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp11(&self) -> SMP11R {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SMP11R { bits }
        }
        # [ doc = "Bits 6:8 - Channel 12 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp12(&self) -> SMP12R {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SMP12R { bits }
        }
        # [ doc = "Bits 9:11 - Channel 13 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp13(&self) -> SMP13R {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SMP13R { bits }
        }
        # [ doc = "Bits 12:14 - Channel 14 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp14(&self) -> SMP14R {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SMP14R { bits }
        }
        # [ doc = "Bits 15:17 - Channel 15 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp15(&self) -> SMP15R {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SMP15R { bits }
        }
        # [ doc = "Bits 18:20 - Channel 16 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp16(&self) -> SMP16R {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SMP16R { bits }
        }
        # [ doc = "Bits 21:23 - Channel 17 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp17(&self) -> SMP17R {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 21;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SMP17R { bits }
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
        # [ doc = "Bits 0:2 - Channel 10 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp10(&mut self) -> _SMP10W {
            _SMP10W { w: self }
        }
        # [ doc = "Bits 3:5 - Channel 11 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp11(&mut self) -> _SMP11W {
            _SMP11W { w: self }
        }
        # [ doc = "Bits 6:8 - Channel 12 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp12(&mut self) -> _SMP12W {
            _SMP12W { w: self }
        }
        # [ doc = "Bits 9:11 - Channel 13 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp13(&mut self) -> _SMP13W {
            _SMP13W { w: self }
        }
        # [ doc = "Bits 12:14 - Channel 14 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp14(&mut self) -> _SMP14W {
            _SMP14W { w: self }
        }
        # [ doc = "Bits 15:17 - Channel 15 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp15(&mut self) -> _SMP15W {
            _SMP15W { w: self }
        }
        # [ doc = "Bits 18:20 - Channel 16 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp16(&mut self) -> _SMP16W {
            _SMP16W { w: self }
        }
        # [ doc = "Bits 21:23 - Channel 17 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp17(&mut self) -> _SMP17W {
            _SMP17W { w: self }
        }
    }
}
# [ doc = "sample time register 2" ]
pub struct SMPR2 {
    register: VolatileCell<u32>,
}
# [ doc = "sample time register 2" ]
pub mod smpr2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::SMPR2 {
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
    pub struct SMP0R {
        bits: u8,
    }
    impl SMP0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SMP1R {
        bits: u8,
    }
    impl SMP1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SMP2R {
        bits: u8,
    }
    impl SMP2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SMP3R {
        bits: u8,
    }
    impl SMP3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SMP4R {
        bits: u8,
    }
    impl SMP4R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SMP5R {
        bits: u8,
    }
    impl SMP5R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SMP6R {
        bits: u8,
    }
    impl SMP6R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SMP7R {
        bits: u8,
    }
    impl SMP7R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SMP8R {
        bits: u8,
    }
    impl SMP8R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SMP9R {
        bits: u8,
    }
    impl SMP9R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SMP0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SMP0W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SMP1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SMP1W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SMP2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SMP2W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SMP3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SMP3W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SMP4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SMP4W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SMP5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SMP5W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 15;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SMP6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SMP6W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SMP7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SMP7W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 21;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SMP8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SMP8W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SMP9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SMP9W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 27;
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
        # [ doc = "Bits 0:2 - Channel 0 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp0(&self) -> SMP0R {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SMP0R { bits }
        }
        # [ doc = "Bits 3:5 - Channel 1 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp1(&self) -> SMP1R {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SMP1R { bits }
        }
        # [ doc = "Bits 6:8 - Channel 2 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp2(&self) -> SMP2R {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SMP2R { bits }
        }
        # [ doc = "Bits 9:11 - Channel 3 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp3(&self) -> SMP3R {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SMP3R { bits }
        }
        # [ doc = "Bits 12:14 - Channel 4 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp4(&self) -> SMP4R {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SMP4R { bits }
        }
        # [ doc = "Bits 15:17 - Channel 5 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp5(&self) -> SMP5R {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SMP5R { bits }
        }
        # [ doc = "Bits 18:20 - Channel 6 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp6(&self) -> SMP6R {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SMP6R { bits }
        }
        # [ doc = "Bits 21:23 - Channel 7 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp7(&self) -> SMP7R {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 21;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SMP7R { bits }
        }
        # [ doc = "Bits 24:26 - Channel 8 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp8(&self) -> SMP8R {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SMP8R { bits }
        }
        # [ doc = "Bits 27:29 - Channel 9 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp9(&self) -> SMP9R {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 27;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SMP9R { bits }
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
        # [ doc = "Bits 0:2 - Channel 0 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp0(&mut self) -> _SMP0W {
            _SMP0W { w: self }
        }
        # [ doc = "Bits 3:5 - Channel 1 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp1(&mut self) -> _SMP1W {
            _SMP1W { w: self }
        }
        # [ doc = "Bits 6:8 - Channel 2 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp2(&mut self) -> _SMP2W {
            _SMP2W { w: self }
        }
        # [ doc = "Bits 9:11 - Channel 3 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp3(&mut self) -> _SMP3W {
            _SMP3W { w: self }
        }
        # [ doc = "Bits 12:14 - Channel 4 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp4(&mut self) -> _SMP4W {
            _SMP4W { w: self }
        }
        # [ doc = "Bits 15:17 - Channel 5 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp5(&mut self) -> _SMP5W {
            _SMP5W { w: self }
        }
        # [ doc = "Bits 18:20 - Channel 6 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp6(&mut self) -> _SMP6W {
            _SMP6W { w: self }
        }
        # [ doc = "Bits 21:23 - Channel 7 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp7(&mut self) -> _SMP7W {
            _SMP7W { w: self }
        }
        # [ doc = "Bits 24:26 - Channel 8 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp8(&mut self) -> _SMP8W {
            _SMP8W { w: self }
        }
        # [ doc = "Bits 27:29 - Channel 9 sample time selection" ]
        # [ inline ( always ) ]
        pub fn smp9(&mut self) -> _SMP9W {
            _SMP9W { w: self }
        }
    }
}
# [ doc = "injected channel data offset register x" ]
pub struct JOFR1 {
    register: VolatileCell<u32>,
}
# [ doc = "injected channel data offset register x" ]
pub mod jofr1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::JOFR1 {
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
    pub struct JOFFSET1R {
        bits: u16,
    }
    impl JOFFSET1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _JOFFSET1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _JOFFSET1W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 4095;
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
        # [ doc = "Bits 0:11 - Data offset for injected channel x" ]
        # [ inline ( always ) ]
        pub fn joffset1(&self) -> JOFFSET1R {
            let bits = {
                const MASK: u16 = 4095;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            JOFFSET1R { bits }
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
        # [ doc = "Bits 0:11 - Data offset for injected channel x" ]
        # [ inline ( always ) ]
        pub fn joffset1(&mut self) -> _JOFFSET1W {
            _JOFFSET1W { w: self }
        }
    }
}
# [ doc = "injected channel data offset register x" ]
pub struct JOFR2 {
    register: VolatileCell<u32>,
}
# [ doc = "injected channel data offset register x" ]
pub mod jofr2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::JOFR2 {
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
    pub struct JOFFSET2R {
        bits: u16,
    }
    impl JOFFSET2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _JOFFSET2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _JOFFSET2W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 4095;
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
        # [ doc = "Bits 0:11 - Data offset for injected channel x" ]
        # [ inline ( always ) ]
        pub fn joffset2(&self) -> JOFFSET2R {
            let bits = {
                const MASK: u16 = 4095;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            JOFFSET2R { bits }
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
        # [ doc = "Bits 0:11 - Data offset for injected channel x" ]
        # [ inline ( always ) ]
        pub fn joffset2(&mut self) -> _JOFFSET2W {
            _JOFFSET2W { w: self }
        }
    }
}
# [ doc = "injected channel data offset register x" ]
pub struct JOFR3 {
    register: VolatileCell<u32>,
}
# [ doc = "injected channel data offset register x" ]
pub mod jofr3 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::JOFR3 {
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
    pub struct JOFFSET3R {
        bits: u16,
    }
    impl JOFFSET3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _JOFFSET3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _JOFFSET3W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 4095;
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
        # [ doc = "Bits 0:11 - Data offset for injected channel x" ]
        # [ inline ( always ) ]
        pub fn joffset3(&self) -> JOFFSET3R {
            let bits = {
                const MASK: u16 = 4095;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            JOFFSET3R { bits }
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
        # [ doc = "Bits 0:11 - Data offset for injected channel x" ]
        # [ inline ( always ) ]
        pub fn joffset3(&mut self) -> _JOFFSET3W {
            _JOFFSET3W { w: self }
        }
    }
}
# [ doc = "injected channel data offset register x" ]
pub struct JOFR4 {
    register: VolatileCell<u32>,
}
# [ doc = "injected channel data offset register x" ]
pub mod jofr4 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::JOFR4 {
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
    pub struct JOFFSET4R {
        bits: u16,
    }
    impl JOFFSET4R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _JOFFSET4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _JOFFSET4W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 4095;
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
        # [ doc = "Bits 0:11 - Data offset for injected channel x" ]
        # [ inline ( always ) ]
        pub fn joffset4(&self) -> JOFFSET4R {
            let bits = {
                const MASK: u16 = 4095;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            JOFFSET4R { bits }
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
        # [ doc = "Bits 0:11 - Data offset for injected channel x" ]
        # [ inline ( always ) ]
        pub fn joffset4(&mut self) -> _JOFFSET4W {
            _JOFFSET4W { w: self }
        }
    }
}
# [ doc = "watchdog higher threshold register" ]
pub struct HTR {
    register: VolatileCell<u32>,
}
# [ doc = "watchdog higher threshold register" ]
pub mod htr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::HTR {
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
    pub struct HTR {
        bits: u16,
    }
    impl HTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _HTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _HTW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 4095;
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
        # [ doc = "Bits 0:11 - Analog watchdog higher threshold" ]
        # [ inline ( always ) ]
        pub fn ht(&self) -> HTR {
            let bits = {
                const MASK: u16 = 4095;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            HTR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 4095 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:11 - Analog watchdog higher threshold" ]
        # [ inline ( always ) ]
        pub fn ht(&mut self) -> _HTW {
            _HTW { w: self }
        }
    }
}
# [ doc = "watchdog lower threshold register" ]
pub struct LTR {
    register: VolatileCell<u32>,
}
# [ doc = "watchdog lower threshold register" ]
pub mod ltr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::LTR {
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
    pub struct LTR {
        bits: u16,
    }
    impl LTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _LTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LTW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 4095;
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
        # [ doc = "Bits 0:11 - Analog watchdog lower threshold" ]
        # [ inline ( always ) ]
        pub fn lt(&self) -> LTR {
            let bits = {
                const MASK: u16 = 4095;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            LTR { bits }
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
        # [ doc = "Bits 0:11 - Analog watchdog lower threshold" ]
        # [ inline ( always ) ]
        pub fn lt(&mut self) -> _LTW {
            _LTW { w: self }
        }
    }
}
# [ doc = "regular sequence register 1" ]
pub struct SQR1 {
    register: VolatileCell<u32>,
}
# [ doc = "regular sequence register 1" ]
pub mod sqr1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::SQR1 {
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
    pub struct LR {
        bits: u8,
    }
    impl LR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SQ16R {
        bits: u8,
    }
    impl SQ16R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SQ15R {
        bits: u8,
    }
    impl SQ15R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SQ14R {
        bits: u8,
    }
    impl SQ14R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SQ13R {
        bits: u8,
    }
    impl SQ13R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _LW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LW<'a> {
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
    pub struct _SQ16W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SQ16W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 15;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SQ15W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SQ15W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SQ14W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SQ14W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 5;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SQ13W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SQ13W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
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
        # [ doc = "Bits 20:23 - Regular channel sequence length" ]
        # [ inline ( always ) ]
        pub fn l(&self) -> LR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            LR { bits }
        }
        # [ doc = "Bits 15:19 - 16th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq16(&self) -> SQ16R {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SQ16R { bits }
        }
        # [ doc = "Bits 10:14 - 15th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq15(&self) -> SQ15R {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SQ15R { bits }
        }
        # [ doc = "Bits 5:9 - 14th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq14(&self) -> SQ14R {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SQ14R { bits }
        }
        # [ doc = "Bits 0:4 - 13th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq13(&self) -> SQ13R {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SQ13R { bits }
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
        # [ doc = "Bits 20:23 - Regular channel sequence length" ]
        # [ inline ( always ) ]
        pub fn l(&mut self) -> _LW {
            _LW { w: self }
        }
        # [ doc = "Bits 15:19 - 16th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq16(&mut self) -> _SQ16W {
            _SQ16W { w: self }
        }
        # [ doc = "Bits 10:14 - 15th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq15(&mut self) -> _SQ15W {
            _SQ15W { w: self }
        }
        # [ doc = "Bits 5:9 - 14th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq14(&mut self) -> _SQ14W {
            _SQ14W { w: self }
        }
        # [ doc = "Bits 0:4 - 13th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq13(&mut self) -> _SQ13W {
            _SQ13W { w: self }
        }
    }
}
# [ doc = "regular sequence register 2" ]
pub struct SQR2 {
    register: VolatileCell<u32>,
}
# [ doc = "regular sequence register 2" ]
pub mod sqr2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::SQR2 {
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
    pub struct SQ12R {
        bits: u8,
    }
    impl SQ12R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SQ11R {
        bits: u8,
    }
    impl SQ11R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SQ10R {
        bits: u8,
    }
    impl SQ10R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SQ9R {
        bits: u8,
    }
    impl SQ9R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SQ8R {
        bits: u8,
    }
    impl SQ8R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SQ7R {
        bits: u8,
    }
    impl SQ7R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SQ12W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SQ12W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 25;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SQ11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SQ11W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SQ10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SQ10W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 15;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SQ9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SQ9W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SQ8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SQ8W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 5;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SQ7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SQ7W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
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
        # [ doc = "Bits 25:29 - 12th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq12(&self) -> SQ12R {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SQ12R { bits }
        }
        # [ doc = "Bits 20:24 - 11th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq11(&self) -> SQ11R {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SQ11R { bits }
        }
        # [ doc = "Bits 15:19 - 10th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq10(&self) -> SQ10R {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SQ10R { bits }
        }
        # [ doc = "Bits 10:14 - 9th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq9(&self) -> SQ9R {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SQ9R { bits }
        }
        # [ doc = "Bits 5:9 - 8th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq8(&self) -> SQ8R {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SQ8R { bits }
        }
        # [ doc = "Bits 0:4 - 7th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq7(&self) -> SQ7R {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SQ7R { bits }
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
        # [ doc = "Bits 25:29 - 12th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq12(&mut self) -> _SQ12W {
            _SQ12W { w: self }
        }
        # [ doc = "Bits 20:24 - 11th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq11(&mut self) -> _SQ11W {
            _SQ11W { w: self }
        }
        # [ doc = "Bits 15:19 - 10th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq10(&mut self) -> _SQ10W {
            _SQ10W { w: self }
        }
        # [ doc = "Bits 10:14 - 9th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq9(&mut self) -> _SQ9W {
            _SQ9W { w: self }
        }
        # [ doc = "Bits 5:9 - 8th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq8(&mut self) -> _SQ8W {
            _SQ8W { w: self }
        }
        # [ doc = "Bits 0:4 - 7th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq7(&mut self) -> _SQ7W {
            _SQ7W { w: self }
        }
    }
}
# [ doc = "regular sequence register 3" ]
pub struct SQR3 {
    register: VolatileCell<u32>,
}
# [ doc = "regular sequence register 3" ]
pub mod sqr3 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::SQR3 {
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
    pub struct SQ6R {
        bits: u8,
    }
    impl SQ6R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SQ5R {
        bits: u8,
    }
    impl SQ5R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SQ4R {
        bits: u8,
    }
    impl SQ4R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SQ3R {
        bits: u8,
    }
    impl SQ3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SQ2R {
        bits: u8,
    }
    impl SQ2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SQ1R {
        bits: u8,
    }
    impl SQ1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SQ6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SQ6W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 25;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SQ5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SQ5W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SQ4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SQ4W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 15;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SQ3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SQ3W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SQ2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SQ2W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 5;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SQ1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SQ1W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
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
        # [ doc = "Bits 25:29 - 6th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq6(&self) -> SQ6R {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SQ6R { bits }
        }
        # [ doc = "Bits 20:24 - 5th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq5(&self) -> SQ5R {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SQ5R { bits }
        }
        # [ doc = "Bits 15:19 - 4th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq4(&self) -> SQ4R {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SQ4R { bits }
        }
        # [ doc = "Bits 10:14 - 3rd conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq3(&self) -> SQ3R {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SQ3R { bits }
        }
        # [ doc = "Bits 5:9 - 2nd conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq2(&self) -> SQ2R {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SQ2R { bits }
        }
        # [ doc = "Bits 0:4 - 1st conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq1(&self) -> SQ1R {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SQ1R { bits }
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
        # [ doc = "Bits 25:29 - 6th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq6(&mut self) -> _SQ6W {
            _SQ6W { w: self }
        }
        # [ doc = "Bits 20:24 - 5th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq5(&mut self) -> _SQ5W {
            _SQ5W { w: self }
        }
        # [ doc = "Bits 15:19 - 4th conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq4(&mut self) -> _SQ4W {
            _SQ4W { w: self }
        }
        # [ doc = "Bits 10:14 - 3rd conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq3(&mut self) -> _SQ3W {
            _SQ3W { w: self }
        }
        # [ doc = "Bits 5:9 - 2nd conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq2(&mut self) -> _SQ2W {
            _SQ2W { w: self }
        }
        # [ doc = "Bits 0:4 - 1st conversion in regular sequence" ]
        # [ inline ( always ) ]
        pub fn sq1(&mut self) -> _SQ1W {
            _SQ1W { w: self }
        }
    }
}
# [ doc = "injected sequence register" ]
pub struct JSQR {
    register: VolatileCell<u32>,
}
# [ doc = "injected sequence register" ]
pub mod jsqr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::JSQR {
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
    pub struct JLR {
        bits: u8,
    }
    impl JLR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct JSQ4R {
        bits: u8,
    }
    impl JSQ4R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct JSQ3R {
        bits: u8,
    }
    impl JSQ3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct JSQ2R {
        bits: u8,
    }
    impl JSQ2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct JSQ1R {
        bits: u8,
    }
    impl JSQ1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _JLW<'a> {
        w: &'a mut W,
    }
    impl<'a> _JLW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _JSQ4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _JSQ4W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 15;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _JSQ3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _JSQ3W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _JSQ2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _JSQ2W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 5;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _JSQ1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _JSQ1W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
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
        # [ doc = "Bits 20:21 - Injected sequence length" ]
        # [ inline ( always ) ]
        pub fn jl(&self) -> JLR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            JLR { bits }
        }
        # [ doc = "Bits 15:19 - 4th conversion in injected sequence" ]
        # [ inline ( always ) ]
        pub fn jsq4(&self) -> JSQ4R {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            JSQ4R { bits }
        }
        # [ doc = "Bits 10:14 - 3rd conversion in injected sequence" ]
        # [ inline ( always ) ]
        pub fn jsq3(&self) -> JSQ3R {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            JSQ3R { bits }
        }
        # [ doc = "Bits 5:9 - 2nd conversion in injected sequence" ]
        # [ inline ( always ) ]
        pub fn jsq2(&self) -> JSQ2R {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            JSQ2R { bits }
        }
        # [ doc = "Bits 0:4 - 1st conversion in injected sequence" ]
        # [ inline ( always ) ]
        pub fn jsq1(&self) -> JSQ1R {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            JSQ1R { bits }
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
        # [ doc = "Bits 20:21 - Injected sequence length" ]
        # [ inline ( always ) ]
        pub fn jl(&mut self) -> _JLW {
            _JLW { w: self }
        }
        # [ doc = "Bits 15:19 - 4th conversion in injected sequence" ]
        # [ inline ( always ) ]
        pub fn jsq4(&mut self) -> _JSQ4W {
            _JSQ4W { w: self }
        }
        # [ doc = "Bits 10:14 - 3rd conversion in injected sequence" ]
        # [ inline ( always ) ]
        pub fn jsq3(&mut self) -> _JSQ3W {
            _JSQ3W { w: self }
        }
        # [ doc = "Bits 5:9 - 2nd conversion in injected sequence" ]
        # [ inline ( always ) ]
        pub fn jsq2(&mut self) -> _JSQ2W {
            _JSQ2W { w: self }
        }
        # [ doc = "Bits 0:4 - 1st conversion in injected sequence" ]
        # [ inline ( always ) ]
        pub fn jsq1(&mut self) -> _JSQ1W {
            _JSQ1W { w: self }
        }
    }
}
# [ doc = "injected data register x" ]
pub struct JDR1 {
    register: VolatileCell<u32>,
}
# [ doc = "injected data register x" ]
pub mod jdr1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::JDR1 {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct JDATAR {
        bits: u16,
    }
    impl JDATAR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:15 - Injected data" ]
        # [ inline ( always ) ]
        pub fn jdata(&self) -> JDATAR {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            JDATAR { bits }
        }
    }
}
# [ doc = "injected data register x" ]
pub struct JDR2 {
    register: VolatileCell<u32>,
}
# [ doc = "injected data register x" ]
pub mod jdr2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::JDR2 {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct JDATAR {
        bits: u16,
    }
    impl JDATAR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:15 - Injected data" ]
        # [ inline ( always ) ]
        pub fn jdata(&self) -> JDATAR {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            JDATAR { bits }
        }
    }
}
# [ doc = "injected data register x" ]
pub struct JDR3 {
    register: VolatileCell<u32>,
}
# [ doc = "injected data register x" ]
pub mod jdr3 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::JDR3 {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct JDATAR {
        bits: u16,
    }
    impl JDATAR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:15 - Injected data" ]
        # [ inline ( always ) ]
        pub fn jdata(&self) -> JDATAR {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            JDATAR { bits }
        }
    }
}
# [ doc = "injected data register x" ]
pub struct JDR4 {
    register: VolatileCell<u32>,
}
# [ doc = "injected data register x" ]
pub mod jdr4 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::JDR4 {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct JDATAR {
        bits: u16,
    }
    impl JDATAR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:15 - Injected data" ]
        # [ inline ( always ) ]
        pub fn jdata(&self) -> JDATAR {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            JDATAR { bits }
        }
    }
}
# [ doc = "regular data register" ]
pub struct DR {
    register: VolatileCell<u32>,
}
# [ doc = "regular data register" ]
pub mod dr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::DR {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATAR {
        bits: u16,
    }
    impl DATAR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ADC2DATAR {
        bits: u16,
    }
    impl ADC2DATAR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:15 - Regular data" ]
        # [ inline ( always ) ]
        pub fn data(&self) -> DATAR {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            DATAR { bits }
        }
        # [ doc = "Bits 16:31 - ADC2 data" ]
        # [ inline ( always ) ]
        pub fn adc2data(&self) -> ADC2DATAR {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            ADC2DATAR { bits }
        }
    }
}
# [ doc = "Analog to digital converter" ]
pub struct ADC1 {
    register_block: RegisterBlock,
}
impl Deref for ADC1 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
