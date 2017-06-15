# ! [ doc = "General purpose timer" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;
use vcell::VolatileCell;

# [ doc = "General purpose timer" ]
pub const TIM2: Peripheral<TIM2> = unsafe { Peripheral::new(1073741824) };

# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - control register 1" ]
    pub cr1: CR1,
    # [ doc = "0x04 - control register 2" ]
    pub cr2: CR2,
    # [ doc = "0x08 - slave mode control register" ]
    pub smcr: SMCR,
    # [ doc = "0x0c - DMA/Interrupt enable register" ]
    pub dier: DIER,
    # [ doc = "0x10 - status register" ]
    pub sr: SR,
    # [ doc = "0x14 - event generation register" ]
    pub egr: EGR,
    # [ doc = "0x18 - capture/compare mode register 1 (output mode)" ]
    pub ccmr1_output: CCMR1_OUTPUT,
    # [ doc = "0x1c - capture/compare mode register 2 (output mode)" ]
    pub ccmr2_output: CCMR2_OUTPUT,
    # [ doc = "0x20 - capture/compare enable register" ]
    pub ccer: CCER,
    # [ doc = "0x24 - counter" ]
    pub cnt: CNT,
    # [ doc = "0x28 - prescaler" ]
    pub psc: PSC,
    # [ doc = "0x2c - auto-reload register" ]
    pub arr: ARR,
    _reserved0: [u8; 4usize],
    # [ doc = "0x34 - capture/compare register 1" ]
    pub ccr1: CCR1,
    # [ doc = "0x38 - capture/compare register 2" ]
    pub ccr2: CCR2,
    # [ doc = "0x3c - capture/compare register 3" ]
    pub ccr3: CCR3,
    # [ doc = "0x40 - capture/compare register 4" ]
    pub ccr4: CCR4,
    _reserved1: [u8; 4usize],
    # [ doc = "0x48 - DMA control register" ]
    pub dcr: DCR,
    # [ doc = "0x4c - DMA address for full transfer" ]
    pub dmar: DMAR,
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
    # [ doc = "Possible values of the field `CKD`" ]
    pub type CKDR = ::tim1::cr1::CKDR;
    # [ doc = r" Value of the field" ]
    pub struct ARPER {
        bits: bool,
    }
    impl ARPER {
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
    pub struct CMSR {
        bits: u8,
    }
    impl CMSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Possible values of the field `DIR`" ]
    pub type DIRR = ::tim1::cr1::DIRR;
    # [ doc = "Possible values of the field `OPM`" ]
    pub type OPMR = ::tim1::cr1::OPMR;
    # [ doc = r" Value of the field" ]
    pub struct URSR {
        bits: bool,
    }
    impl URSR {
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
    pub struct UDISR {
        bits: bool,
    }
    impl UDISR {
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
    # [ doc = "Possible values of the field `CEN`" ]
    pub type CENR = ::tim1::cr1::CENR;
    # [ doc = "Values that can be written to the field `CKD`" ]
    pub type CKDW = ::tim1::cr1::CKDW;
    # [ doc = r" Proxy" ]
    pub struct _CKDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CKDW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CKDW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "Clock is not divided" ]
        # [ inline ( always ) ]
        pub fn div1(self) -> &'a mut W {
            self.variant(::tim1::cr1::CKDW::DIV1)
        }
        # [ doc = "Clock is divided by 2" ]
        # [ inline ( always ) ]
        pub fn div2(self) -> &'a mut W {
            self.variant(::tim1::cr1::CKDW::DIV2)
        }
        # [ doc = "Clock is divided by 4" ]
        # [ inline ( always ) ]
        pub fn div4(self) -> &'a mut W {
            self.variant(::tim1::cr1::CKDW::DIV4)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ARPEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ARPEW<'a> {
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
    pub struct _CMSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CMSW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `DIR`" ]
    pub type DIRW = ::tim1::cr1::DIRW;
    # [ doc = r" Proxy" ]
    pub struct _DIRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DIRW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: DIRW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Up" ]
        # [ inline ( always ) ]
        pub fn up(self) -> &'a mut W {
            self.variant(::tim1::cr1::DIRW::UP)
        }
        # [ doc = "Down" ]
        # [ inline ( always ) ]
        pub fn down(self) -> &'a mut W {
            self.variant(::tim1::cr1::DIRW::DOWN)
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
    # [ doc = "Values that can be written to the field `OPM`" ]
    pub type OPMW = ::tim1::cr1::OPMW;
    # [ doc = r" Proxy" ]
    pub struct _OPMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OPMW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: OPMW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Counter is not stopped at update event" ]
        # [ inline ( always ) ]
        pub fn continuous(self) -> &'a mut W {
            self.variant(::tim1::cr1::OPMW::CONTINUOUS)
        }
        # [ doc = "Counter stops counting at the next update event (clearing the CEN bit)" ]
        # [ inline ( always ) ]
        pub fn one_pulse(self) -> &'a mut W {
            self.variant(::tim1::cr1::OPMW::ONEPULSE)
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
    pub struct _URSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _URSW<'a> {
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
    pub struct _UDISW<'a> {
        w: &'a mut W,
    }
    impl<'a> _UDISW<'a> {
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
    # [ doc = "Values that can be written to the field `CEN`" ]
    pub type CENW = ::tim1::cr1::CENW;
    # [ doc = r" Proxy" ]
    pub struct _CENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Counter disabled" ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(::tim1::cr1::CENW::DISABLED)
        }
        # [ doc = "Counter enabled" ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(::tim1::cr1::CENW::ENABLED)
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
        # [ doc = "Bits 8:9 - Clock division" ]
        # [ inline ( always ) ]
        pub fn ckd(&self) -> CKDR {
            CKDR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bit 7 - Auto-reload preload enable" ]
        # [ inline ( always ) ]
        pub fn arpe(&self) -> ARPER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ARPER { bits }
        }
        # [ doc = "Bits 5:6 - Center-aligned mode selection" ]
        # [ inline ( always ) ]
        pub fn cms(&self) -> CMSR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CMSR { bits }
        }
        # [ doc = "Bit 4 - Direction" ]
        # [ inline ( always ) ]
        pub fn dir(&self) -> DIRR {
            DIRR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 3 - One-pulse mode" ]
        # [ inline ( always ) ]
        pub fn opm(&self) -> OPMR {
            OPMR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 2 - Update request source" ]
        # [ inline ( always ) ]
        pub fn urs(&self) -> URSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            URSR { bits }
        }
        # [ doc = "Bit 1 - Update disable" ]
        # [ inline ( always ) ]
        pub fn udis(&self) -> UDISR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            UDISR { bits }
        }
        # [ doc = "Bit 0 - Counter enable" ]
        # [ inline ( always ) ]
        pub fn cen(&self) -> CENR {
            CENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
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
        # [ doc = "Bits 8:9 - Clock division" ]
        # [ inline ( always ) ]
        pub fn ckd(&mut self) -> _CKDW {
            _CKDW { w: self }
        }
        # [ doc = "Bit 7 - Auto-reload preload enable" ]
        # [ inline ( always ) ]
        pub fn arpe(&mut self) -> _ARPEW {
            _ARPEW { w: self }
        }
        # [ doc = "Bits 5:6 - Center-aligned mode selection" ]
        # [ inline ( always ) ]
        pub fn cms(&mut self) -> _CMSW {
            _CMSW { w: self }
        }
        # [ doc = "Bit 4 - Direction" ]
        # [ inline ( always ) ]
        pub fn dir(&mut self) -> _DIRW {
            _DIRW { w: self }
        }
        # [ doc = "Bit 3 - One-pulse mode" ]
        # [ inline ( always ) ]
        pub fn opm(&mut self) -> _OPMW {
            _OPMW { w: self }
        }
        # [ doc = "Bit 2 - Update request source" ]
        # [ inline ( always ) ]
        pub fn urs(&mut self) -> _URSW {
            _URSW { w: self }
        }
        # [ doc = "Bit 1 - Update disable" ]
        # [ inline ( always ) ]
        pub fn udis(&mut self) -> _UDISW {
            _UDISW { w: self }
        }
        # [ doc = "Bit 0 - Counter enable" ]
        # [ inline ( always ) ]
        pub fn cen(&mut self) -> _CENW {
            _CENW { w: self }
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
    pub struct TI1SR {
        bits: bool,
    }
    impl TI1SR {
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
    pub struct MMSR {
        bits: u8,
    }
    impl MMSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CCDSR {
        bits: bool,
    }
    impl CCDSR {
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
    pub struct _TI1SW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TI1SW<'a> {
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
    pub struct _MMSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MMSW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CCDSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CCDSW<'a> {
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 7 - TI1 selection" ]
        # [ inline ( always ) ]
        pub fn ti1s(&self) -> TI1SR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TI1SR { bits }
        }
        # [ doc = "Bits 4:6 - Master mode selection" ]
        # [ inline ( always ) ]
        pub fn mms(&self) -> MMSR {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MMSR { bits }
        }
        # [ doc = "Bit 3 - Capture/compare DMA selection" ]
        # [ inline ( always ) ]
        pub fn ccds(&self) -> CCDSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CCDSR { bits }
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
        # [ doc = "Bit 7 - TI1 selection" ]
        # [ inline ( always ) ]
        pub fn ti1s(&mut self) -> _TI1SW {
            _TI1SW { w: self }
        }
        # [ doc = "Bits 4:6 - Master mode selection" ]
        # [ inline ( always ) ]
        pub fn mms(&mut self) -> _MMSW {
            _MMSW { w: self }
        }
        # [ doc = "Bit 3 - Capture/compare DMA selection" ]
        # [ inline ( always ) ]
        pub fn ccds(&mut self) -> _CCDSW {
            _CCDSW { w: self }
        }
    }
}
# [ doc = "slave mode control register" ]
pub struct SMCR {
    register: VolatileCell<u32>,
}
# [ doc = "slave mode control register" ]
pub mod smcr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::SMCR {
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
    pub struct ETPR {
        bits: bool,
    }
    impl ETPR {
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
    pub struct ECER {
        bits: bool,
    }
    impl ECER {
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
    pub struct ETPSR {
        bits: u8,
    }
    impl ETPSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ETFR {
        bits: u8,
    }
    impl ETFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MSMR {
        bits: bool,
    }
    impl MSMR {
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
    # [ doc = "Possible values of the field `TS`" ]
    pub type TSR = ::tim1::smcr::TSR;
    # [ doc = "Possible values of the field `SMS`" ]
    pub type SMSR = ::tim1::smcr::SMSR;
    # [ doc = r" Proxy" ]
    pub struct _ETPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ETPW<'a> {
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
    pub struct _ECEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ECEW<'a> {
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
    pub struct _ETPSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ETPSW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ETFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ETFW<'a> {
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
    pub struct _MSMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MSMW<'a> {
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
    # [ doc = "Values that can be written to the field `TS`" ]
    pub type TSW = ::tim1::smcr::TSW;
    # [ doc = r" Proxy" ]
    pub struct _TSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TSW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: TSW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Internal Trigger 0 (ITR0)" ]
        # [ inline ( always ) ]
        pub fn itr0(self) -> &'a mut W {
            self.variant(::tim1::smcr::TSW::ITR0)
        }
        # [ doc = "Internal Trigger 1 (ITR1)" ]
        # [ inline ( always ) ]
        pub fn itr1(self) -> &'a mut W {
            self.variant(::tim1::smcr::TSW::ITR1)
        }
        # [ doc = "Internal Trigger 2 (ITR2)" ]
        # [ inline ( always ) ]
        pub fn itr2(self) -> &'a mut W {
            self.variant(::tim1::smcr::TSW::ITR2)
        }
        # [ doc = "Internal Trigger 3 (ITR3)" ]
        # [ inline ( always ) ]
        pub fn itr3(self) -> &'a mut W {
            self.variant(::tim1::smcr::TSW::ITR3)
        }
        # [ doc = "TI1 Edge Detector" ]
        # [ inline ( always ) ]
        pub fn ti1f_ed(self) -> &'a mut W {
            self.variant(::tim1::smcr::TSW::TI1F_ED)
        }
        # [ doc = "Filtered Timer Input 1" ]
        # [ inline ( always ) ]
        pub fn ti1fp1(self) -> &'a mut W {
            self.variant(::tim1::smcr::TSW::TI1FP1)
        }
        # [ doc = "Filtered Timer Input 2" ]
        # [ inline ( always ) ]
        pub fn ti2fp2(self) -> &'a mut W {
            self.variant(::tim1::smcr::TSW::TI2FP2)
        }
        # [ doc = "External Trigger input" ]
        # [ inline ( always ) ]
        pub fn etrf(self) -> &'a mut W {
            self.variant(::tim1::smcr::TSW::ETRF)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `SMS`" ]
    pub type SMSW = ::tim1::smcr::SMSW;
    # [ doc = r" Proxy" ]
    pub struct _SMSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SMSW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: SMSW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Counter disabled" ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(::tim1::smcr::SMSW::DISABLED)
        }
        # [ doc = "Encoder mode, count up/down on TI2FP1" ]
        # [ inline ( always ) ]
        pub fn encoder_ti2(self) -> &'a mut W {
            self.variant(::tim1::smcr::SMSW::ENCODERTI2)
        }
        # [ doc = "Encoder mode, count up/down on TI1FP2" ]
        # [ inline ( always ) ]
        pub fn encoder_ti1(self) -> &'a mut W {
            self.variant(::tim1::smcr::SMSW::ENCODERTI1)
        }
        # [ doc = "Encoder mode, count up/down on both TI1FP1 and TI2FP2" ]
        # [ inline ( always ) ]
        pub fn encoder_ti1ti2(self) -> &'a mut W {
            self.variant(::tim1::smcr::SMSW::ENCODERTI1TI2)
        }
        # [ doc = "Rising edge of the selected trigger input (TRGI) reinitializes the counter" ]
        # [ inline ( always ) ]
        pub fn reset(self) -> &'a mut W {
            self.variant(::tim1::smcr::SMSW::RESET)
        }
        # [ doc = "The counter clock is enabled when the trigger input (TRGI) is high" ]
        # [ inline ( always ) ]
        pub fn gated(self) -> &'a mut W {
            self.variant(::tim1::smcr::SMSW::GATED)
        }
        # [ doc = "The counter starts at a rising edge of the trigger TRGI" ]
        # [ inline ( always ) ]
        pub fn trigger(self) -> &'a mut W {
            self.variant(::tim1::smcr::SMSW::TRIGGER)
        }
        # [ doc = "Rising edges of the selected trigger (TRGI) clock the counter" ]
        # [ inline ( always ) ]
        pub fn external(self) -> &'a mut W {
            self.variant(::tim1::smcr::SMSW::EXTERNAL)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
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
        # [ doc = "Bit 15 - External trigger polarity" ]
        # [ inline ( always ) ]
        pub fn etp(&self) -> ETPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ETPR { bits }
        }
        # [ doc = "Bit 14 - External clock enable" ]
        # [ inline ( always ) ]
        pub fn ece(&self) -> ECER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ECER { bits }
        }
        # [ doc = "Bits 12:13 - External trigger prescaler" ]
        # [ inline ( always ) ]
        pub fn etps(&self) -> ETPSR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ETPSR { bits }
        }
        # [ doc = "Bits 8:11 - External trigger filter" ]
        # [ inline ( always ) ]
        pub fn etf(&self) -> ETFR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ETFR { bits }
        }
        # [ doc = "Bit 7 - Master/Slave mode" ]
        # [ inline ( always ) ]
        pub fn msm(&self) -> MSMR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MSMR { bits }
        }
        # [ doc = "Bits 4:6 - Trigger selection" ]
        # [ inline ( always ) ]
        pub fn ts(&self) -> TSR {
            TSR::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 0:2 - Slave mode selection" ]
        # [ inline ( always ) ]
        pub fn sms(&self) -> SMSR {
            SMSR::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
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
        # [ doc = "Bit 15 - External trigger polarity" ]
        # [ inline ( always ) ]
        pub fn etp(&mut self) -> _ETPW {
            _ETPW { w: self }
        }
        # [ doc = "Bit 14 - External clock enable" ]
        # [ inline ( always ) ]
        pub fn ece(&mut self) -> _ECEW {
            _ECEW { w: self }
        }
        # [ doc = "Bits 12:13 - External trigger prescaler" ]
        # [ inline ( always ) ]
        pub fn etps(&mut self) -> _ETPSW {
            _ETPSW { w: self }
        }
        # [ doc = "Bits 8:11 - External trigger filter" ]
        # [ inline ( always ) ]
        pub fn etf(&mut self) -> _ETFW {
            _ETFW { w: self }
        }
        # [ doc = "Bit 7 - Master/Slave mode" ]
        # [ inline ( always ) ]
        pub fn msm(&mut self) -> _MSMW {
            _MSMW { w: self }
        }
        # [ doc = "Bits 4:6 - Trigger selection" ]
        # [ inline ( always ) ]
        pub fn ts(&mut self) -> _TSW {
            _TSW { w: self }
        }
        # [ doc = "Bits 0:2 - Slave mode selection" ]
        # [ inline ( always ) ]
        pub fn sms(&mut self) -> _SMSW {
            _SMSW { w: self }
        }
    }
}
# [ doc = "DMA/Interrupt enable register" ]
pub struct DIER {
    register: VolatileCell<u32>,
}
# [ doc = "DMA/Interrupt enable register" ]
pub mod dier {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DIER {
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
    pub struct TDER {
        bits: bool,
    }
    impl TDER {
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
    pub struct CC4DER {
        bits: bool,
    }
    impl CC4DER {
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
    pub struct CC3DER {
        bits: bool,
    }
    impl CC3DER {
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
    pub struct CC2DER {
        bits: bool,
    }
    impl CC2DER {
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
    pub struct CC1DER {
        bits: bool,
    }
    impl CC1DER {
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
    pub struct UDER {
        bits: bool,
    }
    impl UDER {
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
    pub struct TIER {
        bits: bool,
    }
    impl TIER {
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
    pub struct CC4IER {
        bits: bool,
    }
    impl CC4IER {
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
    pub struct CC3IER {
        bits: bool,
    }
    impl CC3IER {
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
    pub struct CC2IER {
        bits: bool,
    }
    impl CC2IER {
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
    pub struct CC1IER {
        bits: bool,
    }
    impl CC1IER {
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
    pub struct UIER {
        bits: bool,
    }
    impl UIER {
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
    pub struct _TDEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TDEW<'a> {
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
    pub struct _CC4DEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC4DEW<'a> {
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
    pub struct _CC3DEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC3DEW<'a> {
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
    pub struct _CC2DEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC2DEW<'a> {
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
    pub struct _CC1DEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC1DEW<'a> {
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
    pub struct _UDEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _UDEW<'a> {
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
    pub struct _TIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIEW<'a> {
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
    pub struct _CC4IEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC4IEW<'a> {
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
    pub struct _CC3IEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC3IEW<'a> {
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
    pub struct _CC2IEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC2IEW<'a> {
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
    pub struct _CC1IEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC1IEW<'a> {
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
    pub struct _UIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _UIEW<'a> {
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
        # [ doc = "Bit 14 - Trigger DMA request enable" ]
        # [ inline ( always ) ]
        pub fn tde(&self) -> TDER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TDER { bits }
        }
        # [ doc = "Bit 12 - Capture/Compare 4 DMA request enable" ]
        # [ inline ( always ) ]
        pub fn cc4de(&self) -> CC4DER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC4DER { bits }
        }
        # [ doc = "Bit 11 - Capture/Compare 3 DMA request enable" ]
        # [ inline ( always ) ]
        pub fn cc3de(&self) -> CC3DER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC3DER { bits }
        }
        # [ doc = "Bit 10 - Capture/Compare 2 DMA request enable" ]
        # [ inline ( always ) ]
        pub fn cc2de(&self) -> CC2DER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC2DER { bits }
        }
        # [ doc = "Bit 9 - Capture/Compare 1 DMA request enable" ]
        # [ inline ( always ) ]
        pub fn cc1de(&self) -> CC1DER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC1DER { bits }
        }
        # [ doc = "Bit 8 - Update DMA request enable" ]
        # [ inline ( always ) ]
        pub fn ude(&self) -> UDER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            UDER { bits }
        }
        # [ doc = "Bit 6 - Trigger interrupt enable" ]
        # [ inline ( always ) ]
        pub fn tie(&self) -> TIER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIER { bits }
        }
        # [ doc = "Bit 4 - Capture/Compare 4 interrupt enable" ]
        # [ inline ( always ) ]
        pub fn cc4ie(&self) -> CC4IER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC4IER { bits }
        }
        # [ doc = "Bit 3 - Capture/Compare 3 interrupt enable" ]
        # [ inline ( always ) ]
        pub fn cc3ie(&self) -> CC3IER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC3IER { bits }
        }
        # [ doc = "Bit 2 - Capture/Compare 2 interrupt enable" ]
        # [ inline ( always ) ]
        pub fn cc2ie(&self) -> CC2IER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC2IER { bits }
        }
        # [ doc = "Bit 1 - Capture/Compare 1 interrupt enable" ]
        # [ inline ( always ) ]
        pub fn cc1ie(&self) -> CC1IER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC1IER { bits }
        }
        # [ doc = "Bit 0 - Update interrupt enable" ]
        # [ inline ( always ) ]
        pub fn uie(&self) -> UIER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            UIER { bits }
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
        # [ doc = "Bit 14 - Trigger DMA request enable" ]
        # [ inline ( always ) ]
        pub fn tde(&mut self) -> _TDEW {
            _TDEW { w: self }
        }
        # [ doc = "Bit 12 - Capture/Compare 4 DMA request enable" ]
        # [ inline ( always ) ]
        pub fn cc4de(&mut self) -> _CC4DEW {
            _CC4DEW { w: self }
        }
        # [ doc = "Bit 11 - Capture/Compare 3 DMA request enable" ]
        # [ inline ( always ) ]
        pub fn cc3de(&mut self) -> _CC3DEW {
            _CC3DEW { w: self }
        }
        # [ doc = "Bit 10 - Capture/Compare 2 DMA request enable" ]
        # [ inline ( always ) ]
        pub fn cc2de(&mut self) -> _CC2DEW {
            _CC2DEW { w: self }
        }
        # [ doc = "Bit 9 - Capture/Compare 1 DMA request enable" ]
        # [ inline ( always ) ]
        pub fn cc1de(&mut self) -> _CC1DEW {
            _CC1DEW { w: self }
        }
        # [ doc = "Bit 8 - Update DMA request enable" ]
        # [ inline ( always ) ]
        pub fn ude(&mut self) -> _UDEW {
            _UDEW { w: self }
        }
        # [ doc = "Bit 6 - Trigger interrupt enable" ]
        # [ inline ( always ) ]
        pub fn tie(&mut self) -> _TIEW {
            _TIEW { w: self }
        }
        # [ doc = "Bit 4 - Capture/Compare 4 interrupt enable" ]
        # [ inline ( always ) ]
        pub fn cc4ie(&mut self) -> _CC4IEW {
            _CC4IEW { w: self }
        }
        # [ doc = "Bit 3 - Capture/Compare 3 interrupt enable" ]
        # [ inline ( always ) ]
        pub fn cc3ie(&mut self) -> _CC3IEW {
            _CC3IEW { w: self }
        }
        # [ doc = "Bit 2 - Capture/Compare 2 interrupt enable" ]
        # [ inline ( always ) ]
        pub fn cc2ie(&mut self) -> _CC2IEW {
            _CC2IEW { w: self }
        }
        # [ doc = "Bit 1 - Capture/Compare 1 interrupt enable" ]
        # [ inline ( always ) ]
        pub fn cc1ie(&mut self) -> _CC1IEW {
            _CC1IEW { w: self }
        }
        # [ doc = "Bit 0 - Update interrupt enable" ]
        # [ inline ( always ) ]
        pub fn uie(&mut self) -> _UIEW {
            _UIEW { w: self }
        }
    }
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
    pub struct CC4OFR {
        bits: bool,
    }
    impl CC4OFR {
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
    pub struct CC3OFR {
        bits: bool,
    }
    impl CC3OFR {
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
    pub struct CC2OFR {
        bits: bool,
    }
    impl CC2OFR {
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
    pub struct CC1OFR {
        bits: bool,
    }
    impl CC1OFR {
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
    # [ doc = r" Value of the field" ]
    pub struct CC4IFR {
        bits: bool,
    }
    impl CC4IFR {
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
    pub struct CC3IFR {
        bits: bool,
    }
    impl CC3IFR {
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
    pub struct CC2IFR {
        bits: bool,
    }
    impl CC2IFR {
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
    pub struct CC1IFR {
        bits: bool,
    }
    impl CC1IFR {
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
    # [ doc = "Possible values of the field `UIF`" ]
    pub type UIFR = ::tim1::sr::UIFR;
    # [ doc = r" Proxy" ]
    pub struct _CC4OFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC4OFW<'a> {
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
    pub struct _CC3OFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC3OFW<'a> {
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
    pub struct _CC2OFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC2OFW<'a> {
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
    pub struct _CC1OFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC1OFW<'a> {
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
    pub struct _TIFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIFW<'a> {
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
    pub struct _CC4IFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC4IFW<'a> {
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
    pub struct _CC3IFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC3IFW<'a> {
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
    pub struct _CC2IFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC2IFW<'a> {
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
    pub struct _CC1IFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC1IFW<'a> {
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
    # [ doc = "Values that can be written to the field `UIF`" ]
    pub type UIFW = ::tim1::sr::UIFW;
    # [ doc = r" Proxy" ]
    pub struct _UIFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _UIFW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: UIFW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Clears the update interrupt flag" ]
        # [ inline ( always ) ]
        pub fn clear(self) -> &'a mut W {
            self.variant(::tim1::sr::UIFW::CLEAR)
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
        # [ doc = "Bit 12 - Capture/Compare 4 overcapture flag" ]
        # [ inline ( always ) ]
        pub fn cc4of(&self) -> CC4OFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC4OFR { bits }
        }
        # [ doc = "Bit 11 - Capture/Compare 3 overcapture flag" ]
        # [ inline ( always ) ]
        pub fn cc3of(&self) -> CC3OFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC3OFR { bits }
        }
        # [ doc = "Bit 10 - Capture/compare 2 overcapture flag" ]
        # [ inline ( always ) ]
        pub fn cc2of(&self) -> CC2OFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC2OFR { bits }
        }
        # [ doc = "Bit 9 - Capture/Compare 1 overcapture flag" ]
        # [ inline ( always ) ]
        pub fn cc1of(&self) -> CC1OFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC1OFR { bits }
        }
        # [ doc = "Bit 6 - Trigger interrupt flag" ]
        # [ inline ( always ) ]
        pub fn tif(&self) -> TIFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIFR { bits }
        }
        # [ doc = "Bit 4 - Capture/Compare 4 interrupt flag" ]
        # [ inline ( always ) ]
        pub fn cc4if(&self) -> CC4IFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC4IFR { bits }
        }
        # [ doc = "Bit 3 - Capture/Compare 3 interrupt flag" ]
        # [ inline ( always ) ]
        pub fn cc3if(&self) -> CC3IFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC3IFR { bits }
        }
        # [ doc = "Bit 2 - Capture/Compare 2 interrupt flag" ]
        # [ inline ( always ) ]
        pub fn cc2if(&self) -> CC2IFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC2IFR { bits }
        }
        # [ doc = "Bit 1 - Capture/compare 1 interrupt flag" ]
        # [ inline ( always ) ]
        pub fn cc1if(&self) -> CC1IFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC1IFR { bits }
        }
        # [ doc = "Bit 0 - Update interrupt flag" ]
        # [ inline ( always ) ]
        pub fn uif(&self) -> UIFR {
            UIFR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
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
        # [ doc = "Bit 12 - Capture/Compare 4 overcapture flag" ]
        # [ inline ( always ) ]
        pub fn cc4of(&mut self) -> _CC4OFW {
            _CC4OFW { w: self }
        }
        # [ doc = "Bit 11 - Capture/Compare 3 overcapture flag" ]
        # [ inline ( always ) ]
        pub fn cc3of(&mut self) -> _CC3OFW {
            _CC3OFW { w: self }
        }
        # [ doc = "Bit 10 - Capture/compare 2 overcapture flag" ]
        # [ inline ( always ) ]
        pub fn cc2of(&mut self) -> _CC2OFW {
            _CC2OFW { w: self }
        }
        # [ doc = "Bit 9 - Capture/Compare 1 overcapture flag" ]
        # [ inline ( always ) ]
        pub fn cc1of(&mut self) -> _CC1OFW {
            _CC1OFW { w: self }
        }
        # [ doc = "Bit 6 - Trigger interrupt flag" ]
        # [ inline ( always ) ]
        pub fn tif(&mut self) -> _TIFW {
            _TIFW { w: self }
        }
        # [ doc = "Bit 4 - Capture/Compare 4 interrupt flag" ]
        # [ inline ( always ) ]
        pub fn cc4if(&mut self) -> _CC4IFW {
            _CC4IFW { w: self }
        }
        # [ doc = "Bit 3 - Capture/Compare 3 interrupt flag" ]
        # [ inline ( always ) ]
        pub fn cc3if(&mut self) -> _CC3IFW {
            _CC3IFW { w: self }
        }
        # [ doc = "Bit 2 - Capture/Compare 2 interrupt flag" ]
        # [ inline ( always ) ]
        pub fn cc2if(&mut self) -> _CC2IFW {
            _CC2IFW { w: self }
        }
        # [ doc = "Bit 1 - Capture/compare 1 interrupt flag" ]
        # [ inline ( always ) ]
        pub fn cc1if(&mut self) -> _CC1IFW {
            _CC1IFW { w: self }
        }
        # [ doc = "Bit 0 - Update interrupt flag" ]
        # [ inline ( always ) ]
        pub fn uif(&mut self) -> _UIFW {
            _UIFW { w: self }
        }
    }
}
# [ doc = "event generation register" ]
pub struct EGR {
    register: VolatileCell<u32>,
}
# [ doc = "event generation register" ]
pub mod egr {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::EGR {
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
    pub struct _TGW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TGW<'a> {
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
    pub struct _CC4GW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC4GW<'a> {
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
    pub struct _CC3GW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC3GW<'a> {
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
    pub struct _CC2GW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC2GW<'a> {
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
    pub struct _CC1GW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC1GW<'a> {
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
    pub struct _UGW<'a> {
        w: &'a mut W,
    }
    impl<'a> _UGW<'a> {
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
        # [ doc = "Bit 6 - Trigger generation" ]
        # [ inline ( always ) ]
        pub fn tg(&mut self) -> _TGW {
            _TGW { w: self }
        }
        # [ doc = "Bit 4 - Capture/compare 4 generation" ]
        # [ inline ( always ) ]
        pub fn cc4g(&mut self) -> _CC4GW {
            _CC4GW { w: self }
        }
        # [ doc = "Bit 3 - Capture/compare 3 generation" ]
        # [ inline ( always ) ]
        pub fn cc3g(&mut self) -> _CC3GW {
            _CC3GW { w: self }
        }
        # [ doc = "Bit 2 - Capture/compare 2 generation" ]
        # [ inline ( always ) ]
        pub fn cc2g(&mut self) -> _CC2GW {
            _CC2GW { w: self }
        }
        # [ doc = "Bit 1 - Capture/compare 1 generation" ]
        # [ inline ( always ) ]
        pub fn cc1g(&mut self) -> _CC1GW {
            _CC1GW { w: self }
        }
        # [ doc = "Bit 0 - Update generation" ]
        # [ inline ( always ) ]
        pub fn ug(&mut self) -> _UGW {
            _UGW { w: self }
        }
    }
}
# [ doc = "capture/compare mode register 1 (output mode)" ]
pub struct CCMR1_OUTPUT {
    register: VolatileCell<u32>,
}
# [ doc = "capture/compare mode register 1 (output mode)" ]
pub mod ccmr1_output {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CCMR1_OUTPUT {
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
    pub struct OC2CER {
        bits: bool,
    }
    impl OC2CER {
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
    pub struct OC2MR {
        bits: u8,
    }
    impl OC2MR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct OC2PER {
        bits: bool,
    }
    impl OC2PER {
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
    pub struct OC2FER {
        bits: bool,
    }
    impl OC2FER {
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
    pub struct CC2SR {
        bits: u8,
    }
    impl CC2SR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct OC1CER {
        bits: bool,
    }
    impl OC1CER {
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
    pub struct OC1MR {
        bits: u8,
    }
    impl OC1MR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct OC1PER {
        bits: bool,
    }
    impl OC1PER {
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
    pub struct OC1FER {
        bits: bool,
    }
    impl OC1FER {
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
    pub struct CC1SR {
        bits: u8,
    }
    impl CC1SR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _OC2CEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OC2CEW<'a> {
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
    pub struct _OC2MW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OC2MW<'a> {
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
    pub struct _OC2PEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OC2PEW<'a> {
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
    pub struct _OC2FEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OC2FEW<'a> {
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
    pub struct _CC2SW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC2SW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _OC1CEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OC1CEW<'a> {
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
    pub struct _OC1MW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OC1MW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _OC1PEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OC1PEW<'a> {
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
    pub struct _OC1FEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OC1FEW<'a> {
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
    pub struct _CC1SW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC1SW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
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
        # [ doc = "Bit 15 - Output compare 2 clear enable" ]
        # [ inline ( always ) ]
        pub fn oc2ce(&self) -> OC2CER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OC2CER { bits }
        }
        # [ doc = "Bits 12:14 - Output compare 2 mode" ]
        # [ inline ( always ) ]
        pub fn oc2m(&self) -> OC2MR {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            OC2MR { bits }
        }
        # [ doc = "Bit 11 - Output compare 2 preload enable" ]
        # [ inline ( always ) ]
        pub fn oc2pe(&self) -> OC2PER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OC2PER { bits }
        }
        # [ doc = "Bit 10 - Output compare 2 fast enable" ]
        # [ inline ( always ) ]
        pub fn oc2fe(&self) -> OC2FER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OC2FER { bits }
        }
        # [ doc = "Bits 8:9 - Capture/Compare 2 selection" ]
        # [ inline ( always ) ]
        pub fn cc2s(&self) -> CC2SR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CC2SR { bits }
        }
        # [ doc = "Bit 7 - Output compare 1 clear enable" ]
        # [ inline ( always ) ]
        pub fn oc1ce(&self) -> OC1CER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OC1CER { bits }
        }
        # [ doc = "Bits 4:6 - Output compare 1 mode" ]
        # [ inline ( always ) ]
        pub fn oc1m(&self) -> OC1MR {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            OC1MR { bits }
        }
        # [ doc = "Bit 3 - Output compare 1 preload enable" ]
        # [ inline ( always ) ]
        pub fn oc1pe(&self) -> OC1PER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OC1PER { bits }
        }
        # [ doc = "Bit 2 - Output compare 1 fast enable" ]
        # [ inline ( always ) ]
        pub fn oc1fe(&self) -> OC1FER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OC1FER { bits }
        }
        # [ doc = "Bits 0:1 - Capture/Compare 1 selection" ]
        # [ inline ( always ) ]
        pub fn cc1s(&self) -> CC1SR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CC1SR { bits }
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
        # [ doc = "Bit 15 - Output compare 2 clear enable" ]
        # [ inline ( always ) ]
        pub fn oc2ce(&mut self) -> _OC2CEW {
            _OC2CEW { w: self }
        }
        # [ doc = "Bits 12:14 - Output compare 2 mode" ]
        # [ inline ( always ) ]
        pub fn oc2m(&mut self) -> _OC2MW {
            _OC2MW { w: self }
        }
        # [ doc = "Bit 11 - Output compare 2 preload enable" ]
        # [ inline ( always ) ]
        pub fn oc2pe(&mut self) -> _OC2PEW {
            _OC2PEW { w: self }
        }
        # [ doc = "Bit 10 - Output compare 2 fast enable" ]
        # [ inline ( always ) ]
        pub fn oc2fe(&mut self) -> _OC2FEW {
            _OC2FEW { w: self }
        }
        # [ doc = "Bits 8:9 - Capture/Compare 2 selection" ]
        # [ inline ( always ) ]
        pub fn cc2s(&mut self) -> _CC2SW {
            _CC2SW { w: self }
        }
        # [ doc = "Bit 7 - Output compare 1 clear enable" ]
        # [ inline ( always ) ]
        pub fn oc1ce(&mut self) -> _OC1CEW {
            _OC1CEW { w: self }
        }
        # [ doc = "Bits 4:6 - Output compare 1 mode" ]
        # [ inline ( always ) ]
        pub fn oc1m(&mut self) -> _OC1MW {
            _OC1MW { w: self }
        }
        # [ doc = "Bit 3 - Output compare 1 preload enable" ]
        # [ inline ( always ) ]
        pub fn oc1pe(&mut self) -> _OC1PEW {
            _OC1PEW { w: self }
        }
        # [ doc = "Bit 2 - Output compare 1 fast enable" ]
        # [ inline ( always ) ]
        pub fn oc1fe(&mut self) -> _OC1FEW {
            _OC1FEW { w: self }
        }
        # [ doc = "Bits 0:1 - Capture/Compare 1 selection" ]
        # [ inline ( always ) ]
        pub fn cc1s(&mut self) -> _CC1SW {
            _CC1SW { w: self }
        }
    }
}
# [ doc = "capture/compare mode register 1 (input mode)" ]
pub struct CCMR1_INPUT {
    register: VolatileCell<u32>,
}
# [ doc = "capture/compare mode register 1 (input mode)" ]
pub mod ccmr1_input {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CCMR1_INPUT {
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
    pub struct IC2FR {
        bits: u8,
    }
    impl IC2FR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IC2PSCR {
        bits: u8,
    }
    impl IC2PSCR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CC2SR {
        bits: u8,
    }
    impl CC2SR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IC1FR {
        bits: u8,
    }
    impl IC1FR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IC1PSCR {
        bits: u8,
    }
    impl IC1PSCR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CC1SR {
        bits: u8,
    }
    impl CC1SR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IC2FW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IC2FW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IC2PSCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IC2PSCW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CC2SW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC2SW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IC1FW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IC1FW<'a> {
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
    pub struct _IC1PSCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IC1PSCW<'a> {
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
    pub struct _CC1SW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC1SW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
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
        # [ doc = "Bits 12:15 - Input capture 2 filter" ]
        # [ inline ( always ) ]
        pub fn ic2f(&self) -> IC2FR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            IC2FR { bits }
        }
        # [ doc = "Bits 10:11 - Input capture 2 prescaler" ]
        # [ inline ( always ) ]
        pub fn ic2psc(&self) -> IC2PSCR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            IC2PSCR { bits }
        }
        # [ doc = "Bits 8:9 - Capture/compare 2 selection" ]
        # [ inline ( always ) ]
        pub fn cc2s(&self) -> CC2SR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CC2SR { bits }
        }
        # [ doc = "Bits 4:7 - Input capture 1 filter" ]
        # [ inline ( always ) ]
        pub fn ic1f(&self) -> IC1FR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            IC1FR { bits }
        }
        # [ doc = "Bits 2:3 - Input capture 1 prescaler" ]
        # [ inline ( always ) ]
        pub fn ic1psc(&self) -> IC1PSCR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            IC1PSCR { bits }
        }
        # [ doc = "Bits 0:1 - Capture/Compare 1 selection" ]
        # [ inline ( always ) ]
        pub fn cc1s(&self) -> CC1SR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CC1SR { bits }
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
        # [ doc = "Bits 12:15 - Input capture 2 filter" ]
        # [ inline ( always ) ]
        pub fn ic2f(&mut self) -> _IC2FW {
            _IC2FW { w: self }
        }
        # [ doc = "Bits 10:11 - Input capture 2 prescaler" ]
        # [ inline ( always ) ]
        pub fn ic2psc(&mut self) -> _IC2PSCW {
            _IC2PSCW { w: self }
        }
        # [ doc = "Bits 8:9 - Capture/compare 2 selection" ]
        # [ inline ( always ) ]
        pub fn cc2s(&mut self) -> _CC2SW {
            _CC2SW { w: self }
        }
        # [ doc = "Bits 4:7 - Input capture 1 filter" ]
        # [ inline ( always ) ]
        pub fn ic1f(&mut self) -> _IC1FW {
            _IC1FW { w: self }
        }
        # [ doc = "Bits 2:3 - Input capture 1 prescaler" ]
        # [ inline ( always ) ]
        pub fn ic1psc(&mut self) -> _IC1PSCW {
            _IC1PSCW { w: self }
        }
        # [ doc = "Bits 0:1 - Capture/Compare 1 selection" ]
        # [ inline ( always ) ]
        pub fn cc1s(&mut self) -> _CC1SW {
            _CC1SW { w: self }
        }
    }
}
# [ doc = "capture/compare mode register 2 (output mode)" ]
pub struct CCMR2_OUTPUT {
    register: VolatileCell<u32>,
}
# [ doc = "capture/compare mode register 2 (output mode)" ]
pub mod ccmr2_output {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CCMR2_OUTPUT {
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
    pub struct O24CER {
        bits: bool,
    }
    impl O24CER {
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
    pub struct OC4MR {
        bits: u8,
    }
    impl OC4MR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct OC4PER {
        bits: bool,
    }
    impl OC4PER {
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
    pub struct OC4FER {
        bits: bool,
    }
    impl OC4FER {
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
    pub struct CC4SR {
        bits: u8,
    }
    impl CC4SR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct OC3CER {
        bits: bool,
    }
    impl OC3CER {
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
    pub struct OC3MR {
        bits: u8,
    }
    impl OC3MR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct OC3PER {
        bits: bool,
    }
    impl OC3PER {
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
    pub struct OC3FER {
        bits: bool,
    }
    impl OC3FER {
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
    pub struct CC3SR {
        bits: u8,
    }
    impl CC3SR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _O24CEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _O24CEW<'a> {
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
    pub struct _OC4MW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OC4MW<'a> {
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
    pub struct _OC4PEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OC4PEW<'a> {
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
    pub struct _OC4FEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OC4FEW<'a> {
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
    pub struct _CC4SW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC4SW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _OC3CEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OC3CEW<'a> {
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
    pub struct _OC3MW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OC3MW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _OC3PEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OC3PEW<'a> {
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
    pub struct _OC3FEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OC3FEW<'a> {
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
    pub struct _CC3SW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC3SW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
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
        # [ doc = "Bit 15 - Output compare 4 clear enable" ]
        # [ inline ( always ) ]
        pub fn o24ce(&self) -> O24CER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            O24CER { bits }
        }
        # [ doc = "Bits 12:14 - Output compare 4 mode" ]
        # [ inline ( always ) ]
        pub fn oc4m(&self) -> OC4MR {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            OC4MR { bits }
        }
        # [ doc = "Bit 11 - Output compare 4 preload enable" ]
        # [ inline ( always ) ]
        pub fn oc4pe(&self) -> OC4PER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OC4PER { bits }
        }
        # [ doc = "Bit 10 - Output compare 4 fast enable" ]
        # [ inline ( always ) ]
        pub fn oc4fe(&self) -> OC4FER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OC4FER { bits }
        }
        # [ doc = "Bits 8:9 - Capture/Compare 4 selection" ]
        # [ inline ( always ) ]
        pub fn cc4s(&self) -> CC4SR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CC4SR { bits }
        }
        # [ doc = "Bit 7 - Output compare 3 clear enable" ]
        # [ inline ( always ) ]
        pub fn oc3ce(&self) -> OC3CER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OC3CER { bits }
        }
        # [ doc = "Bits 4:6 - Output compare 3 mode" ]
        # [ inline ( always ) ]
        pub fn oc3m(&self) -> OC3MR {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            OC3MR { bits }
        }
        # [ doc = "Bit 3 - Output compare 3 preload enable" ]
        # [ inline ( always ) ]
        pub fn oc3pe(&self) -> OC3PER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OC3PER { bits }
        }
        # [ doc = "Bit 2 - Output compare 3 fast enable" ]
        # [ inline ( always ) ]
        pub fn oc3fe(&self) -> OC3FER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OC3FER { bits }
        }
        # [ doc = "Bits 0:1 - Capture/Compare 3 selection" ]
        # [ inline ( always ) ]
        pub fn cc3s(&self) -> CC3SR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CC3SR { bits }
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
        # [ doc = "Bit 15 - Output compare 4 clear enable" ]
        # [ inline ( always ) ]
        pub fn o24ce(&mut self) -> _O24CEW {
            _O24CEW { w: self }
        }
        # [ doc = "Bits 12:14 - Output compare 4 mode" ]
        # [ inline ( always ) ]
        pub fn oc4m(&mut self) -> _OC4MW {
            _OC4MW { w: self }
        }
        # [ doc = "Bit 11 - Output compare 4 preload enable" ]
        # [ inline ( always ) ]
        pub fn oc4pe(&mut self) -> _OC4PEW {
            _OC4PEW { w: self }
        }
        # [ doc = "Bit 10 - Output compare 4 fast enable" ]
        # [ inline ( always ) ]
        pub fn oc4fe(&mut self) -> _OC4FEW {
            _OC4FEW { w: self }
        }
        # [ doc = "Bits 8:9 - Capture/Compare 4 selection" ]
        # [ inline ( always ) ]
        pub fn cc4s(&mut self) -> _CC4SW {
            _CC4SW { w: self }
        }
        # [ doc = "Bit 7 - Output compare 3 clear enable" ]
        # [ inline ( always ) ]
        pub fn oc3ce(&mut self) -> _OC3CEW {
            _OC3CEW { w: self }
        }
        # [ doc = "Bits 4:6 - Output compare 3 mode" ]
        # [ inline ( always ) ]
        pub fn oc3m(&mut self) -> _OC3MW {
            _OC3MW { w: self }
        }
        # [ doc = "Bit 3 - Output compare 3 preload enable" ]
        # [ inline ( always ) ]
        pub fn oc3pe(&mut self) -> _OC3PEW {
            _OC3PEW { w: self }
        }
        # [ doc = "Bit 2 - Output compare 3 fast enable" ]
        # [ inline ( always ) ]
        pub fn oc3fe(&mut self) -> _OC3FEW {
            _OC3FEW { w: self }
        }
        # [ doc = "Bits 0:1 - Capture/Compare 3 selection" ]
        # [ inline ( always ) ]
        pub fn cc3s(&mut self) -> _CC3SW {
            _CC3SW { w: self }
        }
    }
}
# [ doc = "capture/compare mode register 2 (input mode)" ]
pub struct CCMR2_INPUT {
    register: VolatileCell<u32>,
}
# [ doc = "capture/compare mode register 2 (input mode)" ]
pub mod ccmr2_input {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CCMR2_INPUT {
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
    pub struct IC4FR {
        bits: u8,
    }
    impl IC4FR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IC4PSCR {
        bits: u8,
    }
    impl IC4PSCR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CC4SR {
        bits: u8,
    }
    impl CC4SR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IC3FR {
        bits: u8,
    }
    impl IC3FR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IC3PSCR {
        bits: u8,
    }
    impl IC3PSCR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CC3SR {
        bits: u8,
    }
    impl CC3SR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IC4FW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IC4FW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IC4PSCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IC4PSCW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CC4SW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC4SW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IC3FW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IC3FW<'a> {
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
    pub struct _IC3PSCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IC3PSCW<'a> {
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
    pub struct _CC3SW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC3SW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
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
        # [ doc = "Bits 12:15 - Input capture 4 filter" ]
        # [ inline ( always ) ]
        pub fn ic4f(&self) -> IC4FR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            IC4FR { bits }
        }
        # [ doc = "Bits 10:11 - Input capture 4 prescaler" ]
        # [ inline ( always ) ]
        pub fn ic4psc(&self) -> IC4PSCR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            IC4PSCR { bits }
        }
        # [ doc = "Bits 8:9 - Capture/Compare 4 selection" ]
        # [ inline ( always ) ]
        pub fn cc4s(&self) -> CC4SR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CC4SR { bits }
        }
        # [ doc = "Bits 4:7 - Input capture 3 filter" ]
        # [ inline ( always ) ]
        pub fn ic3f(&self) -> IC3FR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            IC3FR { bits }
        }
        # [ doc = "Bits 2:3 - Input capture 3 prescaler" ]
        # [ inline ( always ) ]
        pub fn ic3psc(&self) -> IC3PSCR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            IC3PSCR { bits }
        }
        # [ doc = "Bits 0:1 - Capture/Compare 3 selection" ]
        # [ inline ( always ) ]
        pub fn cc3s(&self) -> CC3SR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CC3SR { bits }
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
        # [ doc = "Bits 12:15 - Input capture 4 filter" ]
        # [ inline ( always ) ]
        pub fn ic4f(&mut self) -> _IC4FW {
            _IC4FW { w: self }
        }
        # [ doc = "Bits 10:11 - Input capture 4 prescaler" ]
        # [ inline ( always ) ]
        pub fn ic4psc(&mut self) -> _IC4PSCW {
            _IC4PSCW { w: self }
        }
        # [ doc = "Bits 8:9 - Capture/Compare 4 selection" ]
        # [ inline ( always ) ]
        pub fn cc4s(&mut self) -> _CC4SW {
            _CC4SW { w: self }
        }
        # [ doc = "Bits 4:7 - Input capture 3 filter" ]
        # [ inline ( always ) ]
        pub fn ic3f(&mut self) -> _IC3FW {
            _IC3FW { w: self }
        }
        # [ doc = "Bits 2:3 - Input capture 3 prescaler" ]
        # [ inline ( always ) ]
        pub fn ic3psc(&mut self) -> _IC3PSCW {
            _IC3PSCW { w: self }
        }
        # [ doc = "Bits 0:1 - Capture/Compare 3 selection" ]
        # [ inline ( always ) ]
        pub fn cc3s(&mut self) -> _CC3SW {
            _CC3SW { w: self }
        }
    }
}
# [ doc = "capture/compare enable register" ]
pub struct CCER {
    register: VolatileCell<u32>,
}
# [ doc = "capture/compare enable register" ]
pub mod ccer {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CCER {
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
    pub struct CC4PR {
        bits: bool,
    }
    impl CC4PR {
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
    pub struct CC4ER {
        bits: bool,
    }
    impl CC4ER {
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
    pub struct CC3PR {
        bits: bool,
    }
    impl CC3PR {
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
    pub struct CC3ER {
        bits: bool,
    }
    impl CC3ER {
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
    pub struct CC2PR {
        bits: bool,
    }
    impl CC2PR {
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
    pub struct CC2ER {
        bits: bool,
    }
    impl CC2ER {
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
    pub struct CC1PR {
        bits: bool,
    }
    impl CC1PR {
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
    pub struct CC1ER {
        bits: bool,
    }
    impl CC1ER {
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
    pub struct _CC4PW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC4PW<'a> {
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
    pub struct _CC4EW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC4EW<'a> {
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
    pub struct _CC3PW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC3PW<'a> {
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
    pub struct _CC3EW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC3EW<'a> {
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
    pub struct _CC2PW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC2PW<'a> {
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
    pub struct _CC2EW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC2EW<'a> {
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
    pub struct _CC1PW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC1PW<'a> {
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
    pub struct _CC1EW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC1EW<'a> {
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
        # [ doc = "Bit 13 - Capture/Compare 3 output Polarity" ]
        # [ inline ( always ) ]
        pub fn cc4p(&self) -> CC4PR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC4PR { bits }
        }
        # [ doc = "Bit 12 - Capture/Compare 4 output enable" ]
        # [ inline ( always ) ]
        pub fn cc4e(&self) -> CC4ER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC4ER { bits }
        }
        # [ doc = "Bit 9 - Capture/Compare 3 output Polarity" ]
        # [ inline ( always ) ]
        pub fn cc3p(&self) -> CC3PR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC3PR { bits }
        }
        # [ doc = "Bit 8 - Capture/Compare 3 output enable" ]
        # [ inline ( always ) ]
        pub fn cc3e(&self) -> CC3ER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC3ER { bits }
        }
        # [ doc = "Bit 5 - Capture/Compare 2 output Polarity" ]
        # [ inline ( always ) ]
        pub fn cc2p(&self) -> CC2PR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC2PR { bits }
        }
        # [ doc = "Bit 4 - Capture/Compare 2 output enable" ]
        # [ inline ( always ) ]
        pub fn cc2e(&self) -> CC2ER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC2ER { bits }
        }
        # [ doc = "Bit 1 - Capture/Compare 1 output Polarity" ]
        # [ inline ( always ) ]
        pub fn cc1p(&self) -> CC1PR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC1PR { bits }
        }
        # [ doc = "Bit 0 - Capture/Compare 1 output enable" ]
        # [ inline ( always ) ]
        pub fn cc1e(&self) -> CC1ER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC1ER { bits }
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
        # [ doc = "Bit 13 - Capture/Compare 3 output Polarity" ]
        # [ inline ( always ) ]
        pub fn cc4p(&mut self) -> _CC4PW {
            _CC4PW { w: self }
        }
        # [ doc = "Bit 12 - Capture/Compare 4 output enable" ]
        # [ inline ( always ) ]
        pub fn cc4e(&mut self) -> _CC4EW {
            _CC4EW { w: self }
        }
        # [ doc = "Bit 9 - Capture/Compare 3 output Polarity" ]
        # [ inline ( always ) ]
        pub fn cc3p(&mut self) -> _CC3PW {
            _CC3PW { w: self }
        }
        # [ doc = "Bit 8 - Capture/Compare 3 output enable" ]
        # [ inline ( always ) ]
        pub fn cc3e(&mut self) -> _CC3EW {
            _CC3EW { w: self }
        }
        # [ doc = "Bit 5 - Capture/Compare 2 output Polarity" ]
        # [ inline ( always ) ]
        pub fn cc2p(&mut self) -> _CC2PW {
            _CC2PW { w: self }
        }
        # [ doc = "Bit 4 - Capture/Compare 2 output enable" ]
        # [ inline ( always ) ]
        pub fn cc2e(&mut self) -> _CC2EW {
            _CC2EW { w: self }
        }
        # [ doc = "Bit 1 - Capture/Compare 1 output Polarity" ]
        # [ inline ( always ) ]
        pub fn cc1p(&mut self) -> _CC1PW {
            _CC1PW { w: self }
        }
        # [ doc = "Bit 0 - Capture/Compare 1 output enable" ]
        # [ inline ( always ) ]
        pub fn cc1e(&mut self) -> _CC1EW {
            _CC1EW { w: self }
        }
    }
}
# [ doc = "counter" ]
pub struct CNT {
    register: VolatileCell<u32>,
}
# [ doc = "counter" ]
pub mod cnt {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CNT {
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
    pub struct CNTR {
        bits: u16,
    }
    impl CNTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CNTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNTW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u16) -> &'a mut W {
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
        # [ doc = "Bits 0:15 - counter value" ]
        # [ inline ( always ) ]
        pub fn cnt(&self) -> CNTR {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            CNTR { bits }
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
        # [ doc = "Bits 0:15 - counter value" ]
        # [ inline ( always ) ]
        pub fn cnt(&mut self) -> _CNTW {
            _CNTW { w: self }
        }
    }
}
# [ doc = "prescaler" ]
pub struct PSC {
    register: VolatileCell<u32>,
}
# [ doc = "prescaler" ]
pub mod psc {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PSC {
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
    pub struct PSCR {
        bits: u16,
    }
    impl PSCR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PSCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PSCW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u16) -> &'a mut W {
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
        # [ doc = "Bits 0:15 - Prescaler value" ]
        # [ inline ( always ) ]
        pub fn psc(&self) -> PSCR {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            PSCR { bits }
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
        # [ doc = "Bits 0:15 - Prescaler value" ]
        # [ inline ( always ) ]
        pub fn psc(&mut self) -> _PSCW {
            _PSCW { w: self }
        }
    }
}
# [ doc = "auto-reload register" ]
pub struct ARR {
    register: VolatileCell<u32>,
}
# [ doc = "auto-reload register" ]
pub mod arr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::ARR {
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
    pub struct ARRR {
        bits: u16,
    }
    impl ARRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ARRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ARRW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u16) -> &'a mut W {
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
        # [ doc = "Bits 0:15 - Auto-reload value" ]
        # [ inline ( always ) ]
        pub fn arr(&self) -> ARRR {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            ARRR { bits }
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
        # [ doc = "Bits 0:15 - Auto-reload value" ]
        # [ inline ( always ) ]
        pub fn arr(&mut self) -> _ARRW {
            _ARRW { w: self }
        }
    }
}
# [ doc = "capture/compare register 1" ]
pub struct CCR1 {
    register: VolatileCell<u32>,
}
# [ doc = "capture/compare register 1" ]
pub mod ccr1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CCR1 {
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
    pub struct CCR1R {
        bits: u16,
    }
    impl CCR1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CCR1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CCR1W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u16) -> &'a mut W {
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
        # [ doc = "Bits 0:15 - Capture/Compare 1 value" ]
        # [ inline ( always ) ]
        pub fn ccr1(&self) -> CCR1R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            CCR1R { bits }
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
        # [ doc = "Bits 0:15 - Capture/Compare 1 value" ]
        # [ inline ( always ) ]
        pub fn ccr1(&mut self) -> _CCR1W {
            _CCR1W { w: self }
        }
    }
}
# [ doc = "capture/compare register 2" ]
pub struct CCR2 {
    register: VolatileCell<u32>,
}
# [ doc = "capture/compare register 2" ]
pub mod ccr2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CCR2 {
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
    pub struct CCR2R {
        bits: u16,
    }
    impl CCR2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CCR2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CCR2W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u16) -> &'a mut W {
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
        # [ doc = "Bits 0:15 - Capture/Compare 2 value" ]
        # [ inline ( always ) ]
        pub fn ccr2(&self) -> CCR2R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            CCR2R { bits }
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
        # [ doc = "Bits 0:15 - Capture/Compare 2 value" ]
        # [ inline ( always ) ]
        pub fn ccr2(&mut self) -> _CCR2W {
            _CCR2W { w: self }
        }
    }
}
# [ doc = "capture/compare register 3" ]
pub struct CCR3 {
    register: VolatileCell<u32>,
}
# [ doc = "capture/compare register 3" ]
pub mod ccr3 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CCR3 {
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
    pub struct CCR3R {
        bits: u16,
    }
    impl CCR3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CCR3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CCR3W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u16) -> &'a mut W {
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
        # [ doc = "Bits 0:15 - Capture/Compare value" ]
        # [ inline ( always ) ]
        pub fn ccr3(&self) -> CCR3R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            CCR3R { bits }
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
        # [ doc = "Bits 0:15 - Capture/Compare value" ]
        # [ inline ( always ) ]
        pub fn ccr3(&mut self) -> _CCR3W {
            _CCR3W { w: self }
        }
    }
}
# [ doc = "capture/compare register 4" ]
pub struct CCR4 {
    register: VolatileCell<u32>,
}
# [ doc = "capture/compare register 4" ]
pub mod ccr4 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CCR4 {
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
    pub struct CCR4R {
        bits: u16,
    }
    impl CCR4R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CCR4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CCR4W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u16) -> &'a mut W {
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
        # [ doc = "Bits 0:15 - Capture/Compare value" ]
        # [ inline ( always ) ]
        pub fn ccr4(&self) -> CCR4R {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            CCR4R { bits }
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
        # [ doc = "Bits 0:15 - Capture/Compare value" ]
        # [ inline ( always ) ]
        pub fn ccr4(&mut self) -> _CCR4W {
            _CCR4W { w: self }
        }
    }
}
# [ doc = "DMA control register" ]
pub struct DCR {
    register: VolatileCell<u32>,
}
# [ doc = "DMA control register" ]
pub mod dcr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DCR {
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
    pub struct DBLR {
        bits: u8,
    }
    impl DBLR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DBAR {
        bits: u8,
    }
    impl DBAR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DBLW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DBLW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DBAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DBAW<'a> {
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
        # [ doc = "Bits 8:12 - DMA burst length" ]
        # [ inline ( always ) ]
        pub fn dbl(&self) -> DBLR {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DBLR { bits }
        }
        # [ doc = "Bits 0:4 - DMA base address" ]
        # [ inline ( always ) ]
        pub fn dba(&self) -> DBAR {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DBAR { bits }
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
        # [ doc = "Bits 8:12 - DMA burst length" ]
        # [ inline ( always ) ]
        pub fn dbl(&mut self) -> _DBLW {
            _DBLW { w: self }
        }
        # [ doc = "Bits 0:4 - DMA base address" ]
        # [ inline ( always ) ]
        pub fn dba(&mut self) -> _DBAW {
            _DBAW { w: self }
        }
    }
}
# [ doc = "DMA address for full transfer" ]
pub struct DMAR {
    register: VolatileCell<u32>,
}
# [ doc = "DMA address for full transfer" ]
pub mod dmar {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DMAR {
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
    pub struct DMABR {
        bits: u16,
    }
    impl DMABR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DMABW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DMABW<'a> {
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
        # [ doc = "Bits 0:15 - DMA register for burst accesses" ]
        # [ inline ( always ) ]
        pub fn dmab(&self) -> DMABR {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            DMABR { bits }
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
        # [ doc = "Bits 0:15 - DMA register for burst accesses" ]
        # [ inline ( always ) ]
        pub fn dmab(&mut self) -> _DMABW {
            _DMABW { w: self }
        }
    }
}
# [ doc = "General purpose timer" ]
pub struct TIM2 {
    register_block: RegisterBlock,
}
impl Deref for TIM2 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
# [ doc = "TIM3" ]
pub const TIM3: Peripheral<TIM3> = unsafe { Peripheral::new(1073742848) };
# [ doc = r" Register block" ]
pub struct TIM3 {
    register_block: RegisterBlock,
}
impl Deref for TIM3 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
# [ doc = "TIM4" ]
pub const TIM4: Peripheral<TIM4> = unsafe { Peripheral::new(1073743872) };
# [ doc = r" Register block" ]
pub struct TIM4 {
    register_block: RegisterBlock,
}
impl Deref for TIM4 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
# [ doc = "TIM5" ]
pub const TIM5: Peripheral<TIM5> = unsafe { Peripheral::new(1073744896) };
# [ doc = r" Register block" ]
pub struct TIM5 {
    register_block: RegisterBlock,
}
impl Deref for TIM5 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
