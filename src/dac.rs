# ! [ doc = "Digital to analog converter" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;
use vcell::VolatileCell;

# [ doc = "Digital to analog converter" ]
pub const DAC: Peripheral<DAC> = unsafe { Peripheral::new(1073771520) };

# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - Control register (DAC_CR)" ]
    pub cr: CR,
    # [ doc = "0x04 - DAC software trigger register (DAC_SWTRIGR)" ]
    pub swtrigr: SWTRIGR,
    # [ doc = "0x08 - DAC channel1 12-bit right-aligned data holding register(DAC_DHR12R1)" ]
    pub dhr12r1: DHR12R1,
    # [ doc = "0x0c - DAC channel1 12-bit left aligned data holding register (DAC_DHR12L1)" ]
    pub dhr12l1: DHR12L1,
    # [ doc = "0x10 - DAC channel1 8-bit right aligned data holding register (DAC_DHR8R1)" ]
    pub dhr8r1: DHR8R1,
    # [ doc = "0x14 - DAC channel2 12-bit right aligned data holding register (DAC_DHR12R2)" ]
    pub dhr12r2: DHR12R2,
    # [ doc = "0x18 - DAC channel2 12-bit left aligned data holding register (DAC_DHR12L2)" ]
    pub dhr12l2: DHR12L2,
    # [ doc = "0x1c - DAC channel2 8-bit right-aligned data holding register (DAC_DHR8R2)" ]
    pub dhr8r2: DHR8R2,
    # [ doc = "0x20 - Dual DAC 12-bit right-aligned data holding register (DAC_DHR12RD), Bits 31:28 Reserved, Bits 15:12 Reserved" ]
    pub dhr12rd: DHR12RD,
    # [ doc = "0x24 - DUAL DAC 12-bit left aligned data holding register (DAC_DHR12LD), Bits 19:16 Reserved, Bits 3:0 Reserved" ]
    pub dhr12ld: DHR12LD,
    # [ doc = "0x28 - DUAL DAC 8-bit right aligned data holding register (DAC_DHR8RD), Bits 31:16 Reserved" ]
    pub dhr8rd: DHR8RD,
    # [ doc = "0x2c - DAC channel1 data output register (DAC_DOR1)" ]
    pub dor1: DOR1,
    # [ doc = "0x30 - DAC channel2 data output register (DAC_DOR2)" ]
    pub dor2: DOR2,
}
# [ doc = "Control register (DAC_CR)" ]
pub struct CR {
    register: VolatileCell<u32>,
}
# [ doc = "Control register (DAC_CR)" ]
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
    pub struct EN1R {
        bits: bool,
    }
    impl EN1R {
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
    pub struct BOFF1R {
        bits: bool,
    }
    impl BOFF1R {
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
    pub struct TEN1R {
        bits: bool,
    }
    impl TEN1R {
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
    pub struct TSEL1R {
        bits: u8,
    }
    impl TSEL1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WAVE1R {
        bits: u8,
    }
    impl WAVE1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MAMP1R {
        bits: u8,
    }
    impl MAMP1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DMAEN1R {
        bits: bool,
    }
    impl DMAEN1R {
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
    pub struct EN2R {
        bits: bool,
    }
    impl EN2R {
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
    pub struct BOFF2R {
        bits: bool,
    }
    impl BOFF2R {
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
    pub struct TEN2R {
        bits: bool,
    }
    impl TEN2R {
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
    pub struct TSEL2R {
        bits: u8,
    }
    impl TSEL2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WAVE2R {
        bits: u8,
    }
    impl WAVE2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MAMP2R {
        bits: u8,
    }
    impl MAMP2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DMAEN2R {
        bits: bool,
    }
    impl DMAEN2R {
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
    pub struct _EN1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EN1W<'a> {
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
    pub struct _BOFF1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BOFF1W<'a> {
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
    pub struct _TEN1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TEN1W<'a> {
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
    pub struct _TSEL1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TSEL1W<'a> {
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
    pub struct _WAVE1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAVE1W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MAMP1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MAMP1W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DMAEN1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DMAEN1W<'a> {
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
    pub struct _EN2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EN2W<'a> {
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
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _BOFF2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BOFF2W<'a> {
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
            const OFFSET: u8 = 17;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TEN2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TEN2W<'a> {
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
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TSEL2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TSEL2W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WAVE2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAVE2W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MAMP2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MAMP2W<'a> {
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
    pub struct _DMAEN2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DMAEN2W<'a> {
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
            const OFFSET: u8 = 28;
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
        # [ doc = "Bit 0 - DAC channel1 enable" ]
        # [ inline ( always ) ]
        pub fn en1(&self) -> EN1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EN1R { bits }
        }
        # [ doc = "Bit 1 - DAC channel1 output buffer disable" ]
        # [ inline ( always ) ]
        pub fn boff1(&self) -> BOFF1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BOFF1R { bits }
        }
        # [ doc = "Bit 2 - DAC channel1 trigger enable" ]
        # [ inline ( always ) ]
        pub fn ten1(&self) -> TEN1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TEN1R { bits }
        }
        # [ doc = "Bits 3:5 - DAC channel1 trigger selection" ]
        # [ inline ( always ) ]
        pub fn tsel1(&self) -> TSEL1R {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            TSEL1R { bits }
        }
        # [ doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable" ]
        # [ inline ( always ) ]
        pub fn wave1(&self) -> WAVE1R {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            WAVE1R { bits }
        }
        # [ doc = "Bits 8:11 - DAC channel1 mask/amplitude selector" ]
        # [ inline ( always ) ]
        pub fn mamp1(&self) -> MAMP1R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MAMP1R { bits }
        }
        # [ doc = "Bit 12 - DAC channel1 DMA enable" ]
        # [ inline ( always ) ]
        pub fn dmaen1(&self) -> DMAEN1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DMAEN1R { bits }
        }
        # [ doc = "Bit 16 - DAC channel2 enable" ]
        # [ inline ( always ) ]
        pub fn en2(&self) -> EN2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EN2R { bits }
        }
        # [ doc = "Bit 17 - DAC channel2 output buffer disable" ]
        # [ inline ( always ) ]
        pub fn boff2(&self) -> BOFF2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BOFF2R { bits }
        }
        # [ doc = "Bit 18 - DAC channel2 trigger enable" ]
        # [ inline ( always ) ]
        pub fn ten2(&self) -> TEN2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TEN2R { bits }
        }
        # [ doc = "Bits 19:21 - DAC channel2 trigger selection" ]
        # [ inline ( always ) ]
        pub fn tsel2(&self) -> TSEL2R {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            TSEL2R { bits }
        }
        # [ doc = "Bits 22:23 - DAC channel2 noise/triangle wave generation enable" ]
        # [ inline ( always ) ]
        pub fn wave2(&self) -> WAVE2R {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 22;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            WAVE2R { bits }
        }
        # [ doc = "Bits 24:27 - DAC channel2 mask/amplitude selector" ]
        # [ inline ( always ) ]
        pub fn mamp2(&self) -> MAMP2R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MAMP2R { bits }
        }
        # [ doc = "Bit 28 - DAC channel2 DMA enable" ]
        # [ inline ( always ) ]
        pub fn dmaen2(&self) -> DMAEN2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DMAEN2R { bits }
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
        # [ doc = "Bit 0 - DAC channel1 enable" ]
        # [ inline ( always ) ]
        pub fn en1(&mut self) -> _EN1W {
            _EN1W { w: self }
        }
        # [ doc = "Bit 1 - DAC channel1 output buffer disable" ]
        # [ inline ( always ) ]
        pub fn boff1(&mut self) -> _BOFF1W {
            _BOFF1W { w: self }
        }
        # [ doc = "Bit 2 - DAC channel1 trigger enable" ]
        # [ inline ( always ) ]
        pub fn ten1(&mut self) -> _TEN1W {
            _TEN1W { w: self }
        }
        # [ doc = "Bits 3:5 - DAC channel1 trigger selection" ]
        # [ inline ( always ) ]
        pub fn tsel1(&mut self) -> _TSEL1W {
            _TSEL1W { w: self }
        }
        # [ doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable" ]
        # [ inline ( always ) ]
        pub fn wave1(&mut self) -> _WAVE1W {
            _WAVE1W { w: self }
        }
        # [ doc = "Bits 8:11 - DAC channel1 mask/amplitude selector" ]
        # [ inline ( always ) ]
        pub fn mamp1(&mut self) -> _MAMP1W {
            _MAMP1W { w: self }
        }
        # [ doc = "Bit 12 - DAC channel1 DMA enable" ]
        # [ inline ( always ) ]
        pub fn dmaen1(&mut self) -> _DMAEN1W {
            _DMAEN1W { w: self }
        }
        # [ doc = "Bit 16 - DAC channel2 enable" ]
        # [ inline ( always ) ]
        pub fn en2(&mut self) -> _EN2W {
            _EN2W { w: self }
        }
        # [ doc = "Bit 17 - DAC channel2 output buffer disable" ]
        # [ inline ( always ) ]
        pub fn boff2(&mut self) -> _BOFF2W {
            _BOFF2W { w: self }
        }
        # [ doc = "Bit 18 - DAC channel2 trigger enable" ]
        # [ inline ( always ) ]
        pub fn ten2(&mut self) -> _TEN2W {
            _TEN2W { w: self }
        }
        # [ doc = "Bits 19:21 - DAC channel2 trigger selection" ]
        # [ inline ( always ) ]
        pub fn tsel2(&mut self) -> _TSEL2W {
            _TSEL2W { w: self }
        }
        # [ doc = "Bits 22:23 - DAC channel2 noise/triangle wave generation enable" ]
        # [ inline ( always ) ]
        pub fn wave2(&mut self) -> _WAVE2W {
            _WAVE2W { w: self }
        }
        # [ doc = "Bits 24:27 - DAC channel2 mask/amplitude selector" ]
        # [ inline ( always ) ]
        pub fn mamp2(&mut self) -> _MAMP2W {
            _MAMP2W { w: self }
        }
        # [ doc = "Bit 28 - DAC channel2 DMA enable" ]
        # [ inline ( always ) ]
        pub fn dmaen2(&mut self) -> _DMAEN2W {
            _DMAEN2W { w: self }
        }
    }
}
# [ doc = "DAC software trigger register (DAC_SWTRIGR)" ]
pub struct SWTRIGR {
    register: VolatileCell<u32>,
}
# [ doc = "DAC software trigger register (DAC_SWTRIGR)" ]
pub mod swtrigr {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::SWTRIGR {
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
    }
    # [ doc = r" Proxy" ]
    pub struct _SWTRIG1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWTRIG1W<'a> {
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
    pub struct _SWTRIG2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWTRIG2W<'a> {
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
        # [ doc = "Bit 0 - DAC channel1 software trigger" ]
        # [ inline ( always ) ]
        pub fn swtrig1(&mut self) -> _SWTRIG1W {
            _SWTRIG1W { w: self }
        }
        # [ doc = "Bit 1 - DAC channel2 software trigger" ]
        # [ inline ( always ) ]
        pub fn swtrig2(&mut self) -> _SWTRIG2W {
            _SWTRIG2W { w: self }
        }
    }
}
# [ doc = "DAC channel1 12-bit right-aligned data holding register(DAC_DHR12R1)" ]
pub struct DHR12R1 {
    register: VolatileCell<u32>,
}
# [ doc = "DAC channel1 12-bit right-aligned data holding register(DAC_DHR12R1)" ]
pub mod dhr12r1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DHR12R1 {
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
    pub struct DACC1DHRR {
        bits: u16,
    }
    impl DACC1DHRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DACC1DHRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DACC1DHRW<'a> {
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
        # [ doc = "Bits 0:11 - DAC channel1 12-bit right-aligned data" ]
        # [ inline ( always ) ]
        pub fn dacc1dhr(&self) -> DACC1DHRR {
            let bits = {
                const MASK: u16 = 4095;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            DACC1DHRR { bits }
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
        # [ doc = "Bits 0:11 - DAC channel1 12-bit right-aligned data" ]
        # [ inline ( always ) ]
        pub fn dacc1dhr(&mut self) -> _DACC1DHRW {
            _DACC1DHRW { w: self }
        }
    }
}
# [ doc = "DAC channel1 12-bit left aligned data holding register (DAC_DHR12L1)" ]
pub struct DHR12L1 {
    register: VolatileCell<u32>,
}
# [ doc = "DAC channel1 12-bit left aligned data holding register (DAC_DHR12L1)" ]
pub mod dhr12l1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DHR12L1 {
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
    pub struct DACC1DHRR {
        bits: u16,
    }
    impl DACC1DHRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DACC1DHRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DACC1DHRW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 4;
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
        # [ doc = "Bits 4:15 - DAC channel1 12-bit left-aligned data" ]
        # [ inline ( always ) ]
        pub fn dacc1dhr(&self) -> DACC1DHRR {
            let bits = {
                const MASK: u16 = 4095;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            DACC1DHRR { bits }
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
        # [ doc = "Bits 4:15 - DAC channel1 12-bit left-aligned data" ]
        # [ inline ( always ) ]
        pub fn dacc1dhr(&mut self) -> _DACC1DHRW {
            _DACC1DHRW { w: self }
        }
    }
}
# [ doc = "DAC channel1 8-bit right aligned data holding register (DAC_DHR8R1)" ]
pub struct DHR8R1 {
    register: VolatileCell<u32>,
}
# [ doc = "DAC channel1 8-bit right aligned data holding register (DAC_DHR8R1)" ]
pub mod dhr8r1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DHR8R1 {
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
    pub struct DACC1DHRR {
        bits: u8,
    }
    impl DACC1DHRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DACC1DHRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DACC1DHRW<'a> {
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
        # [ doc = "Bits 0:7 - DAC channel1 8-bit right-aligned data" ]
        # [ inline ( always ) ]
        pub fn dacc1dhr(&self) -> DACC1DHRR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DACC1DHRR { bits }
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
        # [ doc = "Bits 0:7 - DAC channel1 8-bit right-aligned data" ]
        # [ inline ( always ) ]
        pub fn dacc1dhr(&mut self) -> _DACC1DHRW {
            _DACC1DHRW { w: self }
        }
    }
}
# [ doc = "DAC channel2 12-bit right aligned data holding register (DAC_DHR12R2)" ]
pub struct DHR12R2 {
    register: VolatileCell<u32>,
}
# [ doc = "DAC channel2 12-bit right aligned data holding register (DAC_DHR12R2)" ]
pub mod dhr12r2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DHR12R2 {
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
    pub struct DACC2DHRR {
        bits: u16,
    }
    impl DACC2DHRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DACC2DHRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DACC2DHRW<'a> {
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
        # [ doc = "Bits 0:11 - DAC channel2 12-bit right-aligned data" ]
        # [ inline ( always ) ]
        pub fn dacc2dhr(&self) -> DACC2DHRR {
            let bits = {
                const MASK: u16 = 4095;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            DACC2DHRR { bits }
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
        # [ doc = "Bits 0:11 - DAC channel2 12-bit right-aligned data" ]
        # [ inline ( always ) ]
        pub fn dacc2dhr(&mut self) -> _DACC2DHRW {
            _DACC2DHRW { w: self }
        }
    }
}
# [ doc = "DAC channel2 12-bit left aligned data holding register (DAC_DHR12L2)" ]
pub struct DHR12L2 {
    register: VolatileCell<u32>,
}
# [ doc = "DAC channel2 12-bit left aligned data holding register (DAC_DHR12L2)" ]
pub mod dhr12l2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DHR12L2 {
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
    pub struct DACC2DHRR {
        bits: u16,
    }
    impl DACC2DHRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DACC2DHRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DACC2DHRW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 4;
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
        # [ doc = "Bits 4:15 - DAC channel2 12-bit left-aligned data" ]
        # [ inline ( always ) ]
        pub fn dacc2dhr(&self) -> DACC2DHRR {
            let bits = {
                const MASK: u16 = 4095;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            DACC2DHRR { bits }
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
        # [ doc = "Bits 4:15 - DAC channel2 12-bit left-aligned data" ]
        # [ inline ( always ) ]
        pub fn dacc2dhr(&mut self) -> _DACC2DHRW {
            _DACC2DHRW { w: self }
        }
    }
}
# [ doc = "DAC channel2 8-bit right-aligned data holding register (DAC_DHR8R2)" ]
pub struct DHR8R2 {
    register: VolatileCell<u32>,
}
# [ doc = "DAC channel2 8-bit right-aligned data holding register (DAC_DHR8R2)" ]
pub mod dhr8r2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DHR8R2 {
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
    pub struct DACC2DHRR {
        bits: u8,
    }
    impl DACC2DHRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DACC2DHRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DACC2DHRW<'a> {
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
        # [ doc = "Bits 0:7 - DAC channel2 8-bit right-aligned data" ]
        # [ inline ( always ) ]
        pub fn dacc2dhr(&self) -> DACC2DHRR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DACC2DHRR { bits }
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
        # [ doc = "Bits 0:7 - DAC channel2 8-bit right-aligned data" ]
        # [ inline ( always ) ]
        pub fn dacc2dhr(&mut self) -> _DACC2DHRW {
            _DACC2DHRW { w: self }
        }
    }
}
# [ doc = "Dual DAC 12-bit right-aligned data holding register (DAC_DHR12RD), Bits 31:28 Reserved, Bits 15:12 Reserved" ]
pub struct DHR12RD {
    register: VolatileCell<u32>,
}
# [ doc = "Dual DAC 12-bit right-aligned data holding register (DAC_DHR12RD), Bits 31:28 Reserved, Bits 15:12 Reserved" ]
pub mod dhr12rd {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DHR12RD {
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
    pub struct DACC1DHRR {
        bits: u16,
    }
    impl DACC1DHRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DACC2DHRR {
        bits: u16,
    }
    impl DACC2DHRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DACC1DHRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DACC1DHRW<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _DACC2DHRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DACC2DHRW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 16;
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
        # [ doc = "Bits 0:11 - DAC channel1 12-bit right-aligned data" ]
        # [ inline ( always ) ]
        pub fn dacc1dhr(&self) -> DACC1DHRR {
            let bits = {
                const MASK: u16 = 4095;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            DACC1DHRR { bits }
        }
        # [ doc = "Bits 16:27 - DAC channel2 12-bit right-aligned data" ]
        # [ inline ( always ) ]
        pub fn dacc2dhr(&self) -> DACC2DHRR {
            let bits = {
                const MASK: u16 = 4095;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            DACC2DHRR { bits }
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
        # [ doc = "Bits 0:11 - DAC channel1 12-bit right-aligned data" ]
        # [ inline ( always ) ]
        pub fn dacc1dhr(&mut self) -> _DACC1DHRW {
            _DACC1DHRW { w: self }
        }
        # [ doc = "Bits 16:27 - DAC channel2 12-bit right-aligned data" ]
        # [ inline ( always ) ]
        pub fn dacc2dhr(&mut self) -> _DACC2DHRW {
            _DACC2DHRW { w: self }
        }
    }
}
# [ doc = "DUAL DAC 12-bit left aligned data holding register (DAC_DHR12LD), Bits 19:16 Reserved, Bits 3:0 Reserved" ]
pub struct DHR12LD {
    register: VolatileCell<u32>,
}
# [ doc = "DUAL DAC 12-bit left aligned data holding register (DAC_DHR12LD), Bits 19:16 Reserved, Bits 3:0 Reserved" ]
pub mod dhr12ld {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DHR12LD {
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
    pub struct DACC1DHRR {
        bits: u16,
    }
    impl DACC1DHRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DACC2DHRR {
        bits: u16,
    }
    impl DACC2DHRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DACC1DHRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DACC1DHRW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DACC2DHRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DACC2DHRW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 20;
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
        # [ doc = "Bits 4:15 - DAC channel1 12-bit left-aligned data" ]
        # [ inline ( always ) ]
        pub fn dacc1dhr(&self) -> DACC1DHRR {
            let bits = {
                const MASK: u16 = 4095;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            DACC1DHRR { bits }
        }
        # [ doc = "Bits 20:31 - DAC channel2 12-bit right-aligned data" ]
        # [ inline ( always ) ]
        pub fn dacc2dhr(&self) -> DACC2DHRR {
            let bits = {
                const MASK: u16 = 4095;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            DACC2DHRR { bits }
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
        # [ doc = "Bits 4:15 - DAC channel1 12-bit left-aligned data" ]
        # [ inline ( always ) ]
        pub fn dacc1dhr(&mut self) -> _DACC1DHRW {
            _DACC1DHRW { w: self }
        }
        # [ doc = "Bits 20:31 - DAC channel2 12-bit right-aligned data" ]
        # [ inline ( always ) ]
        pub fn dacc2dhr(&mut self) -> _DACC2DHRW {
            _DACC2DHRW { w: self }
        }
    }
}
# [ doc = "DUAL DAC 8-bit right aligned data holding register (DAC_DHR8RD), Bits 31:16 Reserved" ]
pub struct DHR8RD {
    register: VolatileCell<u32>,
}
# [ doc = "DUAL DAC 8-bit right aligned data holding register (DAC_DHR8RD), Bits 31:16 Reserved" ]
pub mod dhr8rd {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DHR8RD {
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
    pub struct DACC1DHRR {
        bits: u8,
    }
    impl DACC1DHRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DACC2DHRR {
        bits: u8,
    }
    impl DACC2DHRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DACC1DHRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DACC1DHRW<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _DACC2DHRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DACC2DHRW<'a> {
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:7 - DAC channel1 8-bit right-aligned data" ]
        # [ inline ( always ) ]
        pub fn dacc1dhr(&self) -> DACC1DHRR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DACC1DHRR { bits }
        }
        # [ doc = "Bits 8:15 - DAC channel2 8-bit right-aligned data" ]
        # [ inline ( always ) ]
        pub fn dacc2dhr(&self) -> DACC2DHRR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DACC2DHRR { bits }
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
        # [ doc = "Bits 0:7 - DAC channel1 8-bit right-aligned data" ]
        # [ inline ( always ) ]
        pub fn dacc1dhr(&mut self) -> _DACC1DHRW {
            _DACC1DHRW { w: self }
        }
        # [ doc = "Bits 8:15 - DAC channel2 8-bit right-aligned data" ]
        # [ inline ( always ) ]
        pub fn dacc2dhr(&mut self) -> _DACC2DHRW {
            _DACC2DHRW { w: self }
        }
    }
}
# [ doc = "DAC channel1 data output register (DAC_DOR1)" ]
pub struct DOR1 {
    register: VolatileCell<u32>,
}
# [ doc = "DAC channel1 data output register (DAC_DOR1)" ]
pub mod dor1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::DOR1 {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DACC1DORR {
        bits: u16,
    }
    impl DACC1DORR {
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
        # [ doc = "Bits 0:11 - DAC channel1 data output" ]
        # [ inline ( always ) ]
        pub fn dacc1dor(&self) -> DACC1DORR {
            let bits = {
                const MASK: u16 = 4095;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            DACC1DORR { bits }
        }
    }
}
# [ doc = "DAC channel2 data output register (DAC_DOR2)" ]
pub struct DOR2 {
    register: VolatileCell<u32>,
}
# [ doc = "DAC channel2 data output register (DAC_DOR2)" ]
pub mod dor2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::DOR2 {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DACC2DORR {
        bits: u16,
    }
    impl DACC2DORR {
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
        # [ doc = "Bits 0:11 - DAC channel2 data output" ]
        # [ inline ( always ) ]
        pub fn dacc2dor(&self) -> DACC2DORR {
            let bits = {
                const MASK: u16 = 4095;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            DACC2DORR { bits }
        }
    }
}
# [ doc = "Digital to analog converter" ]
pub struct DAC {
    register_block: RegisterBlock,
}
impl Deref for DAC {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
