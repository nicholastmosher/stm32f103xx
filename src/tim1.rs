# ! [ doc = "Advanced timer" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;
use vcell::VolatileCell;

# [ doc = "Advanced timer" ]
pub const TIM1: Peripheral<TIM1> = unsafe { Peripheral::new(1073818624) };

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
    # [ doc = "0x18 - capture/compare mode register (output mode)" ]
    pub ccmr1_output: CCMR1_OUTPUT,
    # [ doc = "0x1c - capture/compare mode register (output mode)" ]
    pub ccmr2_output: CCMR2_OUTPUT,
    # [ doc = "0x20 - capture/compare enable register" ]
    pub ccer: CCER,
    # [ doc = "0x24 - counter" ]
    pub cnt: CNT,
    # [ doc = "0x28 - prescaler" ]
    pub psc: PSC,
    # [ doc = "0x2c - auto-reload register" ]
    pub arr: ARR,
    # [ doc = "0x30 - repetition counter register" ]
    pub rcr: RCR,
    # [ doc = "0x34 - capture/compare register 1" ]
    pub ccr1: CCR1,
    # [ doc = "0x38 - capture/compare register 2" ]
    pub ccr2: CCR2,
    # [ doc = "0x3c - capture/compare register 3" ]
    pub ccr3: CCR3,
    # [ doc = "0x40 - capture/compare register 4" ]
    pub ccr4: CCR4,
    # [ doc = "0x44 - break and dead-time register" ]
    pub bdtr: BDTR,
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
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CKDR {
        # [ doc = "Clock is not divided" ]
        DIV1,
        # [ doc = "Clock is divided by 2" ]
        DIV2,
        # [ doc = "Clock is divided by 4" ]
        DIV4,
        # [ doc = r" Reserved" ]
        _Reserved(u8),
    }
    impl CKDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                CKDR::DIV1 => 0,
                CKDR::DIV2 => 1,
                CKDR::DIV4 => 2,
                CKDR::_Reserved(bits) => bits,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> CKDR {
            match value {
                0 => CKDR::DIV1,
                1 => CKDR::DIV2,
                2 => CKDR::DIV4,
                i => CKDR::_Reserved(i),
            }
        }
        # [ doc = "Checks if the value of the field is `DIV1`" ]
        # [ inline ( always ) ]
        pub fn is_div1(&self) -> bool {
            *self == CKDR::DIV1
        }
        # [ doc = "Checks if the value of the field is `DIV2`" ]
        # [ inline ( always ) ]
        pub fn is_div2(&self) -> bool {
            *self == CKDR::DIV2
        }
        # [ doc = "Checks if the value of the field is `DIV4`" ]
        # [ inline ( always ) ]
        pub fn is_div4(&self) -> bool {
            *self == CKDR::DIV4
        }
    }
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
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum DIRR {
        # [ doc = "Up" ]
        UP,
        # [ doc = "Down" ]
        DOWN,
    }
    impl DIRR {
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
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                DIRR::UP => false,
                DIRR::DOWN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> DIRR {
            match value {
                false => DIRR::UP,
                true => DIRR::DOWN,
            }
        }
        # [ doc = "Checks if the value of the field is `UP`" ]
        # [ inline ( always ) ]
        pub fn is_up(&self) -> bool {
            *self == DIRR::UP
        }
        # [ doc = "Checks if the value of the field is `DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_down(&self) -> bool {
            *self == DIRR::DOWN
        }
    }
    # [ doc = "Possible values of the field `OPM`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum OPMR {
        # [ doc = "Counter is not stopped at update event" ]
        CONTINUOUS,
        # [ doc = "Counter stops counting at the next update event (clearing the CEN bit)" ]
        ONEPULSE,
    }
    impl OPMR {
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
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                OPMR::CONTINUOUS => false,
                OPMR::ONEPULSE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> OPMR {
            match value {
                false => OPMR::CONTINUOUS,
                true => OPMR::ONEPULSE,
            }
        }
        # [ doc = "Checks if the value of the field is `CONTINUOUS`" ]
        # [ inline ( always ) ]
        pub fn is_continuous(&self) -> bool {
            *self == OPMR::CONTINUOUS
        }
        # [ doc = "Checks if the value of the field is `ONEPULSE`" ]
        # [ inline ( always ) ]
        pub fn is_one_pulse(&self) -> bool {
            *self == OPMR::ONEPULSE
        }
    }
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
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CENR {
        # [ doc = "Counter disabled" ]
        DISABLED,
        # [ doc = "Counter enabled" ]
        ENABLED,
    }
    impl CENR {
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
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                CENR::DISABLED => false,
                CENR::ENABLED => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CENR {
            match value {
                false => CENR::DISABLED,
                true => CENR::ENABLED,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == CENR::DISABLED
        }
        # [ doc = "Checks if the value of the field is `ENABLED`" ]
        # [ inline ( always ) ]
        pub fn is_enabled(&self) -> bool {
            *self == CENR::ENABLED
        }
    }
    # [ doc = "Values that can be written to the field `CKD`" ]
    pub enum CKDW {
        # [ doc = "Clock is not divided" ]
        DIV1,
        # [ doc = "Clock is divided by 2" ]
        DIV2,
        # [ doc = "Clock is divided by 4" ]
        DIV4,
    }
    impl CKDW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                CKDW::DIV1 => 0,
                CKDW::DIV2 => 1,
                CKDW::DIV4 => 2,
            }
        }
    }
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
            self.variant(CKDW::DIV1)
        }
        # [ doc = "Clock is divided by 2" ]
        # [ inline ( always ) ]
        pub fn div2(self) -> &'a mut W {
            self.variant(CKDW::DIV2)
        }
        # [ doc = "Clock is divided by 4" ]
        # [ inline ( always ) ]
        pub fn div4(self) -> &'a mut W {
            self.variant(CKDW::DIV4)
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
    pub enum DIRW {
        # [ doc = "Up" ]
        UP,
        # [ doc = "Down" ]
        DOWN,
    }
    impl DIRW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                DIRW::UP => false,
                DIRW::DOWN => true,
            }
        }
    }
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
            self.variant(DIRW::UP)
        }
        # [ doc = "Down" ]
        # [ inline ( always ) ]
        pub fn down(self) -> &'a mut W {
            self.variant(DIRW::DOWN)
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
    pub enum OPMW {
        # [ doc = "Counter is not stopped at update event" ]
        CONTINUOUS,
        # [ doc = "Counter stops counting at the next update event (clearing the CEN bit)" ]
        ONEPULSE,
    }
    impl OPMW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                OPMW::CONTINUOUS => false,
                OPMW::ONEPULSE => true,
            }
        }
    }
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
            self.variant(OPMW::CONTINUOUS)
        }
        # [ doc = "Counter stops counting at the next update event (clearing the CEN bit)" ]
        # [ inline ( always ) ]
        pub fn one_pulse(self) -> &'a mut W {
            self.variant(OPMW::ONEPULSE)
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
    pub enum CENW {
        # [ doc = "Counter disabled" ]
        DISABLED,
        # [ doc = "Counter enabled" ]
        ENABLED,
    }
    impl CENW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CENW::DISABLED => false,
                CENW::ENABLED => true,
            }
        }
    }
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
            self.variant(CENW::DISABLED)
        }
        # [ doc = "Counter enabled" ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(CENW::ENABLED)
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
        # [ doc = "Bits 8:9 - Division ratio between the timer clock (CK_INT) frequency and sampling clock" ]
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
        # [ doc = "Bits 8:9 - Division ratio between the timer clock (CK_INT) frequency and sampling clock" ]
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
    pub struct OIS4R {
        bits: bool,
    }
    impl OIS4R {
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
    pub struct OIS3NR {
        bits: bool,
    }
    impl OIS3NR {
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
    pub struct OIS3R {
        bits: bool,
    }
    impl OIS3R {
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
    pub struct OIS2NR {
        bits: bool,
    }
    impl OIS2NR {
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
    pub struct OIS2R {
        bits: bool,
    }
    impl OIS2R {
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
    pub struct OIS1NR {
        bits: bool,
    }
    impl OIS1NR {
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
    pub struct OIS1R {
        bits: bool,
    }
    impl OIS1R {
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
    # [ doc = r" Value of the field" ]
    pub struct CCUSR {
        bits: bool,
    }
    impl CCUSR {
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
    pub struct CCPCR {
        bits: bool,
    }
    impl CCPCR {
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
    pub struct _OIS4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OIS4W<'a> {
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
    pub struct _OIS3NW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OIS3NW<'a> {
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
    pub struct _OIS3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OIS3W<'a> {
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
    pub struct _OIS2NW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OIS2NW<'a> {
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
    pub struct _OIS2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OIS2W<'a> {
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
    pub struct _OIS1NW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OIS1NW<'a> {
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
    pub struct _OIS1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OIS1W<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _CCUSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CCUSW<'a> {
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
    pub struct _CCPCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CCPCW<'a> {
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
        # [ doc = "Bit 14 - Output Idle state 4" ]
        # [ inline ( always ) ]
        pub fn ois4(&self) -> OIS4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OIS4R { bits }
        }
        # [ doc = "Bit 13 - Output Idle state 3" ]
        # [ inline ( always ) ]
        pub fn ois3n(&self) -> OIS3NR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OIS3NR { bits }
        }
        # [ doc = "Bit 12 - Output Idle state 3" ]
        # [ inline ( always ) ]
        pub fn ois3(&self) -> OIS3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OIS3R { bits }
        }
        # [ doc = "Bit 11 - Output Idle state 2" ]
        # [ inline ( always ) ]
        pub fn ois2n(&self) -> OIS2NR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OIS2NR { bits }
        }
        # [ doc = "Bit 10 - Output Idle state 2" ]
        # [ inline ( always ) ]
        pub fn ois2(&self) -> OIS2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OIS2R { bits }
        }
        # [ doc = "Bit 9 - Output Idle state 1" ]
        # [ inline ( always ) ]
        pub fn ois1n(&self) -> OIS1NR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OIS1NR { bits }
        }
        # [ doc = "Bit 8 - Output Idle state 1" ]
        # [ inline ( always ) ]
        pub fn ois1(&self) -> OIS1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OIS1R { bits }
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
        # [ doc = "Bit 2 - Capture/compare control update selection" ]
        # [ inline ( always ) ]
        pub fn ccus(&self) -> CCUSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CCUSR { bits }
        }
        # [ doc = "Bit 0 - Capture/compare preloaded control" ]
        # [ inline ( always ) ]
        pub fn ccpc(&self) -> CCPCR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CCPCR { bits }
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
        # [ doc = "Bit 14 - Output Idle state 4" ]
        # [ inline ( always ) ]
        pub fn ois4(&mut self) -> _OIS4W {
            _OIS4W { w: self }
        }
        # [ doc = "Bit 13 - Output Idle state 3" ]
        # [ inline ( always ) ]
        pub fn ois3n(&mut self) -> _OIS3NW {
            _OIS3NW { w: self }
        }
        # [ doc = "Bit 12 - Output Idle state 3" ]
        # [ inline ( always ) ]
        pub fn ois3(&mut self) -> _OIS3W {
            _OIS3W { w: self }
        }
        # [ doc = "Bit 11 - Output Idle state 2" ]
        # [ inline ( always ) ]
        pub fn ois2n(&mut self) -> _OIS2NW {
            _OIS2NW { w: self }
        }
        # [ doc = "Bit 10 - Output Idle state 2" ]
        # [ inline ( always ) ]
        pub fn ois2(&mut self) -> _OIS2W {
            _OIS2W { w: self }
        }
        # [ doc = "Bit 9 - Output Idle state 1" ]
        # [ inline ( always ) ]
        pub fn ois1n(&mut self) -> _OIS1NW {
            _OIS1NW { w: self }
        }
        # [ doc = "Bit 8 - Output Idle state 1" ]
        # [ inline ( always ) ]
        pub fn ois1(&mut self) -> _OIS1W {
            _OIS1W { w: self }
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
        # [ doc = "Bit 2 - Capture/compare control update selection" ]
        # [ inline ( always ) ]
        pub fn ccus(&mut self) -> _CCUSW {
            _CCUSW { w: self }
        }
        # [ doc = "Bit 0 - Capture/compare preloaded control" ]
        # [ inline ( always ) ]
        pub fn ccpc(&mut self) -> _CCPCW {
            _CCPCW { w: self }
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
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TSR {
        # [ doc = "Internal Trigger 0 (ITR0)" ]
        ITR0,
        # [ doc = "Internal Trigger 1 (ITR1)" ]
        ITR1,
        # [ doc = "Internal Trigger 2 (ITR2)" ]
        ITR2,
        # [ doc = "Internal Trigger 3 (ITR3)" ]
        ITR3,
        # [ doc = "TI1 Edge Detector" ]
        TI1F_ED,
        # [ doc = "Filtered Timer Input 1" ]
        TI1FP1,
        # [ doc = "Filtered Timer Input 2" ]
        TI2FP2,
        # [ doc = "External Trigger input" ]
        ETRF,
    }
    impl TSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                TSR::ITR0 => 0,
                TSR::ITR1 => 1,
                TSR::ITR2 => 2,
                TSR::ITR3 => 3,
                TSR::TI1F_ED => 4,
                TSR::TI1FP1 => 5,
                TSR::TI2FP2 => 6,
                TSR::ETRF => 7,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> TSR {
            match value {
                0 => TSR::ITR0,
                1 => TSR::ITR1,
                2 => TSR::ITR2,
                3 => TSR::ITR3,
                4 => TSR::TI1F_ED,
                5 => TSR::TI1FP1,
                6 => TSR::TI2FP2,
                7 => TSR::ETRF,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `ITR0`" ]
        # [ inline ( always ) ]
        pub fn is_itr0(&self) -> bool {
            *self == TSR::ITR0
        }
        # [ doc = "Checks if the value of the field is `ITR1`" ]
        # [ inline ( always ) ]
        pub fn is_itr1(&self) -> bool {
            *self == TSR::ITR1
        }
        # [ doc = "Checks if the value of the field is `ITR2`" ]
        # [ inline ( always ) ]
        pub fn is_itr2(&self) -> bool {
            *self == TSR::ITR2
        }
        # [ doc = "Checks if the value of the field is `ITR3`" ]
        # [ inline ( always ) ]
        pub fn is_itr3(&self) -> bool {
            *self == TSR::ITR3
        }
        # [ doc = "Checks if the value of the field is `TI1F_ED`" ]
        # [ inline ( always ) ]
        pub fn is_ti1f_ed(&self) -> bool {
            *self == TSR::TI1F_ED
        }
        # [ doc = "Checks if the value of the field is `TI1FP1`" ]
        # [ inline ( always ) ]
        pub fn is_ti1fp1(&self) -> bool {
            *self == TSR::TI1FP1
        }
        # [ doc = "Checks if the value of the field is `TI2FP2`" ]
        # [ inline ( always ) ]
        pub fn is_ti2fp2(&self) -> bool {
            *self == TSR::TI2FP2
        }
        # [ doc = "Checks if the value of the field is `ETRF`" ]
        # [ inline ( always ) ]
        pub fn is_etrf(&self) -> bool {
            *self == TSR::ETRF
        }
    }
    # [ doc = "Possible values of the field `SMS`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum SMSR {
        # [ doc = "Counter disabled" ]
        DISABLED,
        # [ doc = "Encoder mode, count up/down on TI2FP1" ]
        ENCODERTI2,
        # [ doc = "Encoder mode, count up/down on TI1FP2" ]
        ENCODERTI1,
        # [ doc = "Encoder mode, count up/down on both TI1FP1 and TI2FP2" ]
        ENCODERTI1TI2,
        # [ doc = "Rising edge of the selected trigger input (TRGI) reinitializes the counter" ]
        RESET,
        # [ doc = " The counter clock is enabled when the trigger input (TRGI) is high" ]
        GATED,
        # [ doc = "The counter starts at a rising edge of the trigger TRGI " ]
        TRIGGER,
        # [ doc = " Rising edges of the selected trigger (TRGI) clock the counter" ]
        EXTERNAL,
    }
    impl SMSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                SMSR::DISABLED => 0,
                SMSR::ENCODERTI2 => 1,
                SMSR::ENCODERTI1 => 2,
                SMSR::ENCODERTI1TI2 => 3,
                SMSR::RESET => 4,
                SMSR::GATED => 5,
                SMSR::TRIGGER => 6,
                SMSR::EXTERNAL => 7,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> SMSR {
            match value {
                0 => SMSR::DISABLED,
                1 => SMSR::ENCODERTI2,
                2 => SMSR::ENCODERTI1,
                3 => SMSR::ENCODERTI1TI2,
                4 => SMSR::RESET,
                5 => SMSR::GATED,
                6 => SMSR::TRIGGER,
                7 => SMSR::EXTERNAL,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == SMSR::DISABLED
        }
        # [ doc = "Checks if the value of the field is `ENCODERTI2`" ]
        # [ inline ( always ) ]
        pub fn is_encoder_ti2(&self) -> bool {
            *self == SMSR::ENCODERTI2
        }
        # [ doc = "Checks if the value of the field is `ENCODERTI1`" ]
        # [ inline ( always ) ]
        pub fn is_encoder_ti1(&self) -> bool {
            *self == SMSR::ENCODERTI1
        }
        # [ doc = "Checks if the value of the field is `ENCODERTI1TI2`" ]
        # [ inline ( always ) ]
        pub fn is_encoder_ti1ti2(&self) -> bool {
            *self == SMSR::ENCODERTI1TI2
        }
        # [ doc = "Checks if the value of the field is `RESET`" ]
        # [ inline ( always ) ]
        pub fn is_reset(&self) -> bool {
            *self == SMSR::RESET
        }
        # [ doc = "Checks if the value of the field is `GATED`" ]
        # [ inline ( always ) ]
        pub fn is_gated(&self) -> bool {
            *self == SMSR::GATED
        }
        # [ doc = "Checks if the value of the field is `TRIGGER`" ]
        # [ inline ( always ) ]
        pub fn is_trigger(&self) -> bool {
            *self == SMSR::TRIGGER
        }
        # [ doc = "Checks if the value of the field is `EXTERNAL`" ]
        # [ inline ( always ) ]
        pub fn is_external(&self) -> bool {
            *self == SMSR::EXTERNAL
        }
    }
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
    pub enum TSW {
        # [ doc = "Internal Trigger 0 (ITR0)" ]
        ITR0,
        # [ doc = "Internal Trigger 1 (ITR1)" ]
        ITR1,
        # [ doc = "Internal Trigger 2 (ITR2)" ]
        ITR2,
        # [ doc = "Internal Trigger 3 (ITR3)" ]
        ITR3,
        # [ doc = "TI1 Edge Detector" ]
        TI1F_ED,
        # [ doc = "Filtered Timer Input 1" ]
        TI1FP1,
        # [ doc = "Filtered Timer Input 2" ]
        TI2FP2,
        # [ doc = "External Trigger input" ]
        ETRF,
    }
    impl TSW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                TSW::ITR0 => 0,
                TSW::ITR1 => 1,
                TSW::ITR2 => 2,
                TSW::ITR3 => 3,
                TSW::TI1F_ED => 4,
                TSW::TI1FP1 => 5,
                TSW::TI2FP2 => 6,
                TSW::ETRF => 7,
            }
        }
    }
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
            self.variant(TSW::ITR0)
        }
        # [ doc = "Internal Trigger 1 (ITR1)" ]
        # [ inline ( always ) ]
        pub fn itr1(self) -> &'a mut W {
            self.variant(TSW::ITR1)
        }
        # [ doc = "Internal Trigger 2 (ITR2)" ]
        # [ inline ( always ) ]
        pub fn itr2(self) -> &'a mut W {
            self.variant(TSW::ITR2)
        }
        # [ doc = "Internal Trigger 3 (ITR3)" ]
        # [ inline ( always ) ]
        pub fn itr3(self) -> &'a mut W {
            self.variant(TSW::ITR3)
        }
        # [ doc = "TI1 Edge Detector" ]
        # [ inline ( always ) ]
        pub fn ti1f_ed(self) -> &'a mut W {
            self.variant(TSW::TI1F_ED)
        }
        # [ doc = "Filtered Timer Input 1" ]
        # [ inline ( always ) ]
        pub fn ti1fp1(self) -> &'a mut W {
            self.variant(TSW::TI1FP1)
        }
        # [ doc = "Filtered Timer Input 2" ]
        # [ inline ( always ) ]
        pub fn ti2fp2(self) -> &'a mut W {
            self.variant(TSW::TI2FP2)
        }
        # [ doc = "External Trigger input" ]
        # [ inline ( always ) ]
        pub fn etrf(self) -> &'a mut W {
            self.variant(TSW::ETRF)
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
    pub enum SMSW {
        # [ doc = "Counter disabled" ]
        DISABLED,
        # [ doc = "Encoder mode, count up/down on TI2FP1" ]
        ENCODERTI2,
        # [ doc = "Encoder mode, count up/down on TI1FP2" ]
        ENCODERTI1,
        # [ doc = "Encoder mode, count up/down on both TI1FP1 and TI2FP2" ]
        ENCODERTI1TI2,
        # [ doc = "Rising edge of the selected trigger input (TRGI) reinitializes the counter" ]
        RESET,
        # [ doc = " The counter clock is enabled when the trigger input (TRGI) is high" ]
        GATED,
        # [ doc = "The counter starts at a rising edge of the trigger TRGI " ]
        TRIGGER,
        # [ doc = " Rising edges of the selected trigger (TRGI) clock the counter" ]
        EXTERNAL,
    }
    impl SMSW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                SMSW::DISABLED => 0,
                SMSW::ENCODERTI2 => 1,
                SMSW::ENCODERTI1 => 2,
                SMSW::ENCODERTI1TI2 => 3,
                SMSW::RESET => 4,
                SMSW::GATED => 5,
                SMSW::TRIGGER => 6,
                SMSW::EXTERNAL => 7,
            }
        }
    }
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
            self.variant(SMSW::DISABLED)
        }
        # [ doc = "Encoder mode, count up/down on TI2FP1" ]
        # [ inline ( always ) ]
        pub fn encoder_ti2(self) -> &'a mut W {
            self.variant(SMSW::ENCODERTI2)
        }
        # [ doc = "Encoder mode, count up/down on TI1FP2" ]
        # [ inline ( always ) ]
        pub fn encoder_ti1(self) -> &'a mut W {
            self.variant(SMSW::ENCODERTI1)
        }
        # [ doc = "Encoder mode, count up/down on both TI1FP1 and TI2FP2" ]
        # [ inline ( always ) ]
        pub fn encoder_ti1ti2(self) -> &'a mut W {
            self.variant(SMSW::ENCODERTI1TI2)
        }
        # [ doc = "Rising edge of the selected trigger input (TRGI) reinitializes the counter" ]
        # [ inline ( always ) ]
        pub fn reset(self) -> &'a mut W {
            self.variant(SMSW::RESET)
        }
        # [ doc = "The counter clock is enabled when the trigger input (TRGI) is high" ]
        # [ inline ( always ) ]
        pub fn gated(self) -> &'a mut W {
            self.variant(SMSW::GATED)
        }
        # [ doc = "The counter starts at a rising edge of the trigger TRGI" ]
        # [ inline ( always ) ]
        pub fn trigger(self) -> &'a mut W {
            self.variant(SMSW::TRIGGER)
        }
        # [ doc = "Rising edges of the selected trigger (TRGI) clock the counter" ]
        # [ inline ( always ) ]
        pub fn external(self) -> &'a mut W {
            self.variant(SMSW::EXTERNAL)
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
    pub struct COMDER {
        bits: bool,
    }
    impl COMDER {
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
    # [ doc = r" Value of the field" ]
    pub struct BIER {
        bits: bool,
    }
    impl BIER {
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
    pub struct COMIER {
        bits: bool,
    }
    impl COMIER {
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
    pub struct _COMDEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _COMDEW<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _BIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BIEW<'a> {
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
    pub struct _COMIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _COMIEW<'a> {
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
        # [ doc = "Bit 13 - COM DMA request enable" ]
        # [ inline ( always ) ]
        pub fn comde(&self) -> COMDER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            COMDER { bits }
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
        # [ doc = "Bit 7 - Break interrupt enable" ]
        # [ inline ( always ) ]
        pub fn bie(&self) -> BIER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BIER { bits }
        }
        # [ doc = "Bit 5 - COM interrupt enable" ]
        # [ inline ( always ) ]
        pub fn comie(&self) -> COMIER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            COMIER { bits }
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
        # [ doc = "Bit 13 - COM DMA request enable" ]
        # [ inline ( always ) ]
        pub fn comde(&mut self) -> _COMDEW {
            _COMDEW { w: self }
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
        # [ doc = "Bit 7 - Break interrupt enable" ]
        # [ inline ( always ) ]
        pub fn bie(&mut self) -> _BIEW {
            _BIEW { w: self }
        }
        # [ doc = "Bit 5 - COM interrupt enable" ]
        # [ inline ( always ) ]
        pub fn comie(&mut self) -> _COMIEW {
            _COMIEW { w: self }
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
    pub struct BIFR {
        bits: bool,
    }
    impl BIFR {
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
    pub struct COMIFR {
        bits: bool,
    }
    impl COMIFR {
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
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum UIFR {
        # [ doc = "No update occurred" ]
        NOUPDATE,
        # [ doc = "Update interrupt pending" ]
        PENDING,
    }
    impl UIFR {
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
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                UIFR::NOUPDATE => false,
                UIFR::PENDING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> UIFR {
            match value {
                false => UIFR::NOUPDATE,
                true => UIFR::PENDING,
            }
        }
        # [ doc = "Checks if the value of the field is `NOUPDATE`" ]
        # [ inline ( always ) ]
        pub fn is_no_update(&self) -> bool {
            *self == UIFR::NOUPDATE
        }
        # [ doc = "Checks if the value of the field is `PENDING`" ]
        # [ inline ( always ) ]
        pub fn is_pending(&self) -> bool {
            *self == UIFR::PENDING
        }
    }
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
    pub struct _BIFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BIFW<'a> {
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
    pub struct _COMIFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _COMIFW<'a> {
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
    pub enum UIFW {
        # [ doc = "Clears the update interrupt flag" ]
        CLEAR,
    }
    impl UIFW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                UIFW::CLEAR => false,
            }
        }
    }
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
            self.variant(UIFW::CLEAR)
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
        # [ doc = "Bit 7 - Break interrupt flag" ]
        # [ inline ( always ) ]
        pub fn bif(&self) -> BIFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BIFR { bits }
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
        # [ doc = "Bit 5 - COM interrupt flag" ]
        # [ inline ( always ) ]
        pub fn comif(&self) -> COMIFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            COMIFR { bits }
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
        # [ doc = "Bit 7 - Break interrupt flag" ]
        # [ inline ( always ) ]
        pub fn bif(&mut self) -> _BIFW {
            _BIFW { w: self }
        }
        # [ doc = "Bit 6 - Trigger interrupt flag" ]
        # [ inline ( always ) ]
        pub fn tif(&mut self) -> _TIFW {
            _TIFW { w: self }
        }
        # [ doc = "Bit 5 - COM interrupt flag" ]
        # [ inline ( always ) ]
        pub fn comif(&mut self) -> _COMIFW {
            _COMIFW { w: self }
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
    pub struct _BGW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BGW<'a> {
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
    pub struct _COMGW<'a> {
        w: &'a mut W,
    }
    impl<'a> _COMGW<'a> {
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
        # [ doc = "Bit 7 - Break generation" ]
        # [ inline ( always ) ]
        pub fn bg(&mut self) -> _BGW {
            _BGW { w: self }
        }
        # [ doc = "Bit 6 - Trigger generation" ]
        # [ inline ( always ) ]
        pub fn tg(&mut self) -> _TGW {
            _TGW { w: self }
        }
        # [ doc = "Bit 5 - Capture/Compare control update generation" ]
        # [ inline ( always ) ]
        pub fn comg(&mut self) -> _COMGW {
            _COMGW { w: self }
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
# [ doc = "capture/compare mode register (output mode)" ]
pub struct CCMR1_OUTPUT {
    register: VolatileCell<u32>,
}
# [ doc = "capture/compare mode register (output mode)" ]
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
    # [ doc = "Possible values of the field `OC2M`" ]
    pub type OC2MR = super::ccmr1_output::OC1MR;
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
    # [ doc = "Possible values of the field `OC1M`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum OC1MR {
        # [ doc = "The comparison between the output compare register TIMx_CCRy and the\n                    counter TIMx_CNT has no effect on the outputs(" ]
        FROZEN,
        # [ doc = "Set channel y to active level on match. OCyREF signal is forced high when the counter\n                    TIMx_CNT matches the capture/compare register y (TIMx_CCRy)." ]
        SETACTIVE,
        # [ doc = "Set channel y to inactive level on match. OCyREF signal is forced low when the\n                    counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)." ]
        SETINACTIVE,
        # [ doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy." ]
        TOGGLE,
        # [ doc = "OCyREF is forced low." ]
        FORCEINACTIVE,
        # [ doc = "OCyREF is forced high." ]
        FORCEACTIVE,
        # [ doc = "In upcounting, channel 1 is active as long as TIMx_CNT<TIMx_CCRy\n                    else inactive. In downcounting, channel 1 is inactive (OCyREF=\u{2018}0) as long as\n                    TIMx_CNT>TIMx_CCRy else active (OCyREF=1)." ]
        PWM1,
        # [ doc = "In upcounting, channel y is inactive as long as TIMx_CNT<TIMx_CCRy\n                    else active. In downcounting, channel y is active as long as TIMx_CNT>TIMx_CCRy else\n                    inactive." ]
        PWM2,
    }
    impl OC1MR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                OC1MR::FROZEN => 0,
                OC1MR::SETACTIVE => 1,
                OC1MR::SETINACTIVE => 2,
                OC1MR::TOGGLE => 3,
                OC1MR::FORCEINACTIVE => 4,
                OC1MR::FORCEACTIVE => 5,
                OC1MR::PWM1 => 6,
                OC1MR::PWM2 => 7,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> OC1MR {
            match value {
                0 => OC1MR::FROZEN,
                1 => OC1MR::SETACTIVE,
                2 => OC1MR::SETINACTIVE,
                3 => OC1MR::TOGGLE,
                4 => OC1MR::FORCEINACTIVE,
                5 => OC1MR::FORCEACTIVE,
                6 => OC1MR::PWM1,
                7 => OC1MR::PWM2,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `FROZEN`" ]
        # [ inline ( always ) ]
        pub fn is_frozen(&self) -> bool {
            *self == OC1MR::FROZEN
        }
        # [ doc = "Checks if the value of the field is `SETACTIVE`" ]
        # [ inline ( always ) ]
        pub fn is_set_active(&self) -> bool {
            *self == OC1MR::SETACTIVE
        }
        # [ doc = "Checks if the value of the field is `SETINACTIVE`" ]
        # [ inline ( always ) ]
        pub fn is_set_inactive(&self) -> bool {
            *self == OC1MR::SETINACTIVE
        }
        # [ doc = "Checks if the value of the field is `TOGGLE`" ]
        # [ inline ( always ) ]
        pub fn is_toggle(&self) -> bool {
            *self == OC1MR::TOGGLE
        }
        # [ doc = "Checks if the value of the field is `FORCEINACTIVE`" ]
        # [ inline ( always ) ]
        pub fn is_force_inactive(&self) -> bool {
            *self == OC1MR::FORCEINACTIVE
        }
        # [ doc = "Checks if the value of the field is `FORCEACTIVE`" ]
        # [ inline ( always ) ]
        pub fn is_force_active(&self) -> bool {
            *self == OC1MR::FORCEACTIVE
        }
        # [ doc = "Checks if the value of the field is `PWM1`" ]
        # [ inline ( always ) ]
        pub fn is_pwm1(&self) -> bool {
            *self == OC1MR::PWM1
        }
        # [ doc = "Checks if the value of the field is `PWM2`" ]
        # [ inline ( always ) ]
        pub fn is_pwm2(&self) -> bool {
            *self == OC1MR::PWM2
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
    # [ doc = "Values that can be written to the field `OC2M`" ]
    pub type OC2MW = super::ccmr1_output::OC1MW;
    # [ doc = r" Proxy" ]
    pub struct _OC2MW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OC2MW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: OC2MW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs(" ]
        # [ inline ( always ) ]
        pub fn frozen(self) -> &'a mut W {
            self.variant(super::ccmr1_output::OC1MW::FROZEN)
        }
        # [ doc = "Set channel y to active level on match. OCyREF signal is forced high when the counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)." ]
        # [ inline ( always ) ]
        pub fn set_active(self) -> &'a mut W {
            self.variant(super::ccmr1_output::OC1MW::SETACTIVE)
        }
        # [ doc = "Set channel y to inactive level on match. OCyREF signal is forced low when the counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)." ]
        # [ inline ( always ) ]
        pub fn set_inactive(self) -> &'a mut W {
            self.variant(super::ccmr1_output::OC1MW::SETINACTIVE)
        }
        # [ doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy." ]
        # [ inline ( always ) ]
        pub fn toggle(self) -> &'a mut W {
            self.variant(super::ccmr1_output::OC1MW::TOGGLE)
        }
        # [ doc = "OCyREF is forced low." ]
        # [ inline ( always ) ]
        pub fn force_inactive(self) -> &'a mut W {
            self.variant(super::ccmr1_output::OC1MW::FORCEINACTIVE)
        }
        # [ doc = "OCyREF is forced high." ]
        # [ inline ( always ) ]
        pub fn force_active(self) -> &'a mut W {
            self.variant(super::ccmr1_output::OC1MW::FORCEACTIVE)
        }
        # [ doc = "In upcounting, channel 1 is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel 1 is inactive (OCyREF=\u{2018}0) as long as TIMx_CNT>TIMx_CCRy else active (OCyREF=1)." ]
        # [ inline ( always ) ]
        pub fn pwm1(self) -> &'a mut W {
            self.variant(super::ccmr1_output::OC1MW::PWM1)
        }
        # [ doc = "In upcounting, channel y is inactive as long as TIMx_CNT<TIMx_CCRy else active. In downcounting, channel y is active as long as TIMx_CNT>TIMx_CCRy else inactive." ]
        # [ inline ( always ) ]
        pub fn pwm2(self) -> &'a mut W {
            self.variant(super::ccmr1_output::OC1MW::PWM2)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
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
    # [ doc = "Values that can be written to the field `OC1M`" ]
    pub enum OC1MW {
        # [ doc = "The comparison between the output compare register TIMx_CCRy and the\n                    counter TIMx_CNT has no effect on the outputs(" ]
        FROZEN,
        # [ doc = "Set channel y to active level on match. OCyREF signal is forced high when the counter\n                    TIMx_CNT matches the capture/compare register y (TIMx_CCRy)." ]
        SETACTIVE,
        # [ doc = "Set channel y to inactive level on match. OCyREF signal is forced low when the\n                    counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)." ]
        SETINACTIVE,
        # [ doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy." ]
        TOGGLE,
        # [ doc = "OCyREF is forced low." ]
        FORCEINACTIVE,
        # [ doc = "OCyREF is forced high." ]
        FORCEACTIVE,
        # [ doc = "In upcounting, channel 1 is active as long as TIMx_CNT<TIMx_CCRy\n                    else inactive. In downcounting, channel 1 is inactive (OCyREF=\u{2018}0) as long as\n                    TIMx_CNT>TIMx_CCRy else active (OCyREF=1)." ]
        PWM1,
        # [ doc = "In upcounting, channel y is inactive as long as TIMx_CNT<TIMx_CCRy\n                    else active. In downcounting, channel y is active as long as TIMx_CNT>TIMx_CCRy else\n                    inactive." ]
        PWM2,
    }
    impl OC1MW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                OC1MW::FROZEN => 0,
                OC1MW::SETACTIVE => 1,
                OC1MW::SETINACTIVE => 2,
                OC1MW::TOGGLE => 3,
                OC1MW::FORCEINACTIVE => 4,
                OC1MW::FORCEACTIVE => 5,
                OC1MW::PWM1 => 6,
                OC1MW::PWM2 => 7,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _OC1MW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OC1MW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: OC1MW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs(" ]
        # [ inline ( always ) ]
        pub fn frozen(self) -> &'a mut W {
            self.variant(OC1MW::FROZEN)
        }
        # [ doc = "Set channel y to active level on match. OCyREF signal is forced high when the counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)." ]
        # [ inline ( always ) ]
        pub fn set_active(self) -> &'a mut W {
            self.variant(OC1MW::SETACTIVE)
        }
        # [ doc = "Set channel y to inactive level on match. OCyREF signal is forced low when the counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)." ]
        # [ inline ( always ) ]
        pub fn set_inactive(self) -> &'a mut W {
            self.variant(OC1MW::SETINACTIVE)
        }
        # [ doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy." ]
        # [ inline ( always ) ]
        pub fn toggle(self) -> &'a mut W {
            self.variant(OC1MW::TOGGLE)
        }
        # [ doc = "OCyREF is forced low." ]
        # [ inline ( always ) ]
        pub fn force_inactive(self) -> &'a mut W {
            self.variant(OC1MW::FORCEINACTIVE)
        }
        # [ doc = "OCyREF is forced high." ]
        # [ inline ( always ) ]
        pub fn force_active(self) -> &'a mut W {
            self.variant(OC1MW::FORCEACTIVE)
        }
        # [ doc = "In upcounting, channel 1 is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel 1 is inactive (OCyREF=\u{2018}0) as long as TIMx_CNT>TIMx_CCRy else active (OCyREF=1)." ]
        # [ inline ( always ) ]
        pub fn pwm1(self) -> &'a mut W {
            self.variant(OC1MW::PWM1)
        }
        # [ doc = "In upcounting, channel y is inactive as long as TIMx_CNT<TIMx_CCRy else active. In downcounting, channel y is active as long as TIMx_CNT>TIMx_CCRy else inactive." ]
        # [ inline ( always ) ]
        pub fn pwm2(self) -> &'a mut W {
            self.variant(OC1MW::PWM2)
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
        # [ doc = "Bit 15 - Output Compare 2 clear enable" ]
        # [ inline ( always ) ]
        pub fn oc2ce(&self) -> OC2CER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OC2CER { bits }
        }
        # [ doc = "Bits 12:14 - Output Compare 2 mode" ]
        # [ inline ( always ) ]
        pub fn oc2m(&self) -> OC2MR {
            OC2MR::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bit 11 - Output Compare 2 preload enable" ]
        # [ inline ( always ) ]
        pub fn oc2pe(&self) -> OC2PER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OC2PER { bits }
        }
        # [ doc = "Bit 10 - Output Compare 2 fast enable" ]
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
        # [ doc = "Bit 7 - Output Compare 1 clear enable" ]
        # [ inline ( always ) ]
        pub fn oc1ce(&self) -> OC1CER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OC1CER { bits }
        }
        # [ doc = "Bits 4:6 - Output Compare 1 mode" ]
        # [ inline ( always ) ]
        pub fn oc1m(&self) -> OC1MR {
            OC1MR::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bit 3 - Output Compare 1 preload enable" ]
        # [ inline ( always ) ]
        pub fn oc1pe(&self) -> OC1PER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OC1PER { bits }
        }
        # [ doc = "Bit 2 - Output Compare 1 fast enable" ]
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
        # [ doc = "Bit 15 - Output Compare 2 clear enable" ]
        # [ inline ( always ) ]
        pub fn oc2ce(&mut self) -> _OC2CEW {
            _OC2CEW { w: self }
        }
        # [ doc = "Bits 12:14 - Output Compare 2 mode" ]
        # [ inline ( always ) ]
        pub fn oc2m(&mut self) -> _OC2MW {
            _OC2MW { w: self }
        }
        # [ doc = "Bit 11 - Output Compare 2 preload enable" ]
        # [ inline ( always ) ]
        pub fn oc2pe(&mut self) -> _OC2PEW {
            _OC2PEW { w: self }
        }
        # [ doc = "Bit 10 - Output Compare 2 fast enable" ]
        # [ inline ( always ) ]
        pub fn oc2fe(&mut self) -> _OC2FEW {
            _OC2FEW { w: self }
        }
        # [ doc = "Bits 8:9 - Capture/Compare 2 selection" ]
        # [ inline ( always ) ]
        pub fn cc2s(&mut self) -> _CC2SW {
            _CC2SW { w: self }
        }
        # [ doc = "Bit 7 - Output Compare 1 clear enable" ]
        # [ inline ( always ) ]
        pub fn oc1ce(&mut self) -> _OC1CEW {
            _OC1CEW { w: self }
        }
        # [ doc = "Bits 4:6 - Output Compare 1 mode" ]
        # [ inline ( always ) ]
        pub fn oc1m(&mut self) -> _OC1MW {
            _OC1MW { w: self }
        }
        # [ doc = "Bit 3 - Output Compare 1 preload enable" ]
        # [ inline ( always ) ]
        pub fn oc1pe(&mut self) -> _OC1PEW {
            _OC1PEW { w: self }
        }
        # [ doc = "Bit 2 - Output Compare 1 fast enable" ]
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
    pub struct IC2PCSR {
        bits: u8,
    }
    impl IC2PCSR {
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
    pub struct ICPCSR {
        bits: u8,
    }
    impl ICPCSR {
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
    pub struct _IC2PCSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IC2PCSW<'a> {
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
    pub struct _ICPCSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ICPCSW<'a> {
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
        pub fn ic2pcs(&self) -> IC2PCSR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            IC2PCSR { bits }
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
        pub fn icpcs(&self) -> ICPCSR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ICPCSR { bits }
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
        pub fn ic2pcs(&mut self) -> _IC2PCSW {
            _IC2PCSW { w: self }
        }
        # [ doc = "Bits 8:9 - Capture/Compare 2 selection" ]
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
        pub fn icpcs(&mut self) -> _ICPCSW {
            _ICPCSW { w: self }
        }
        # [ doc = "Bits 0:1 - Capture/Compare 1 selection" ]
        # [ inline ( always ) ]
        pub fn cc1s(&mut self) -> _CC1SW {
            _CC1SW { w: self }
        }
    }
}
# [ doc = "capture/compare mode register (output mode)" ]
pub struct CCMR2_OUTPUT {
    register: VolatileCell<u32>,
}
# [ doc = "capture/compare mode register (output mode)" ]
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
    pub struct OC4CER {
        bits: bool,
    }
    impl OC4CER {
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
    # [ doc = "Possible values of the field `OC4M`" ]
    pub type OC4MR = super::ccmr1_output::OC1MR;
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
    # [ doc = "Possible values of the field `OC3M`" ]
    pub type OC3MR = super::ccmr1_output::OC1MR;
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
    pub struct _OC4CEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OC4CEW<'a> {
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
    # [ doc = "Values that can be written to the field `OC4M`" ]
    pub type OC4MW = super::ccmr1_output::OC1MW;
    # [ doc = r" Proxy" ]
    pub struct _OC4MW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OC4MW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: OC4MW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs(" ]
        # [ inline ( always ) ]
        pub fn frozen(self) -> &'a mut W {
            self.variant(super::ccmr1_output::OC1MW::FROZEN)
        }
        # [ doc = "Set channel y to active level on match. OCyREF signal is forced high when the counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)." ]
        # [ inline ( always ) ]
        pub fn set_active(self) -> &'a mut W {
            self.variant(super::ccmr1_output::OC1MW::SETACTIVE)
        }
        # [ doc = "Set channel y to inactive level on match. OCyREF signal is forced low when the counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)." ]
        # [ inline ( always ) ]
        pub fn set_inactive(self) -> &'a mut W {
            self.variant(super::ccmr1_output::OC1MW::SETINACTIVE)
        }
        # [ doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy." ]
        # [ inline ( always ) ]
        pub fn toggle(self) -> &'a mut W {
            self.variant(super::ccmr1_output::OC1MW::TOGGLE)
        }
        # [ doc = "OCyREF is forced low." ]
        # [ inline ( always ) ]
        pub fn force_inactive(self) -> &'a mut W {
            self.variant(super::ccmr1_output::OC1MW::FORCEINACTIVE)
        }
        # [ doc = "OCyREF is forced high." ]
        # [ inline ( always ) ]
        pub fn force_active(self) -> &'a mut W {
            self.variant(super::ccmr1_output::OC1MW::FORCEACTIVE)
        }
        # [ doc = "In upcounting, channel 1 is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel 1 is inactive (OCyREF=\u{2018}0) as long as TIMx_CNT>TIMx_CCRy else active (OCyREF=1)." ]
        # [ inline ( always ) ]
        pub fn pwm1(self) -> &'a mut W {
            self.variant(super::ccmr1_output::OC1MW::PWM1)
        }
        # [ doc = "In upcounting, channel y is inactive as long as TIMx_CNT<TIMx_CCRy else active. In downcounting, channel y is active as long as TIMx_CNT>TIMx_CCRy else inactive." ]
        # [ inline ( always ) ]
        pub fn pwm2(self) -> &'a mut W {
            self.variant(super::ccmr1_output::OC1MW::PWM2)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
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
    # [ doc = "Values that can be written to the field `OC3M`" ]
    pub type OC3MW = super::ccmr1_output::OC1MW;
    # [ doc = r" Proxy" ]
    pub struct _OC3MW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OC3MW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: OC3MW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs(" ]
        # [ inline ( always ) ]
        pub fn frozen(self) -> &'a mut W {
            self.variant(super::ccmr1_output::OC1MW::FROZEN)
        }
        # [ doc = "Set channel y to active level on match. OCyREF signal is forced high when the counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)." ]
        # [ inline ( always ) ]
        pub fn set_active(self) -> &'a mut W {
            self.variant(super::ccmr1_output::OC1MW::SETACTIVE)
        }
        # [ doc = "Set channel y to inactive level on match. OCyREF signal is forced low when the counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)." ]
        # [ inline ( always ) ]
        pub fn set_inactive(self) -> &'a mut W {
            self.variant(super::ccmr1_output::OC1MW::SETINACTIVE)
        }
        # [ doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy." ]
        # [ inline ( always ) ]
        pub fn toggle(self) -> &'a mut W {
            self.variant(super::ccmr1_output::OC1MW::TOGGLE)
        }
        # [ doc = "OCyREF is forced low." ]
        # [ inline ( always ) ]
        pub fn force_inactive(self) -> &'a mut W {
            self.variant(super::ccmr1_output::OC1MW::FORCEINACTIVE)
        }
        # [ doc = "OCyREF is forced high." ]
        # [ inline ( always ) ]
        pub fn force_active(self) -> &'a mut W {
            self.variant(super::ccmr1_output::OC1MW::FORCEACTIVE)
        }
        # [ doc = "In upcounting, channel 1 is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel 1 is inactive (OCyREF=\u{2018}0) as long as TIMx_CNT>TIMx_CCRy else active (OCyREF=1)." ]
        # [ inline ( always ) ]
        pub fn pwm1(self) -> &'a mut W {
            self.variant(super::ccmr1_output::OC1MW::PWM1)
        }
        # [ doc = "In upcounting, channel y is inactive as long as TIMx_CNT<TIMx_CCRy else active. In downcounting, channel y is active as long as TIMx_CNT>TIMx_CCRy else inactive." ]
        # [ inline ( always ) ]
        pub fn pwm2(self) -> &'a mut W {
            self.variant(super::ccmr1_output::OC1MW::PWM2)
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
        pub fn oc4ce(&self) -> OC4CER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OC4CER { bits }
        }
        # [ doc = "Bits 12:14 - Output compare 4 mode" ]
        # [ inline ( always ) ]
        pub fn oc4m(&self) -> OC4MR {
            OC4MR::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
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
            OC3MR::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
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
        pub fn oc4ce(&mut self) -> _OC4CEW {
            _OC4CEW { w: self }
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
        # [ doc = "Bits 0:1 - Capture/compare 3 selection" ]
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
        # [ doc = "Bits 0:1 - Capture/compare 3 selection" ]
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
    pub struct CC3NPR {
        bits: bool,
    }
    impl CC3NPR {
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
    pub struct CC3NER {
        bits: bool,
    }
    impl CC3NER {
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
    pub struct CC2NPR {
        bits: bool,
    }
    impl CC2NPR {
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
    pub struct CC2NER {
        bits: bool,
    }
    impl CC2NER {
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
    pub struct CC1NPR {
        bits: bool,
    }
    impl CC1NPR {
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
    pub struct CC1NER {
        bits: bool,
    }
    impl CC1NER {
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
    pub struct _CC3NPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC3NPW<'a> {
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
    pub struct _CC3NEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC3NEW<'a> {
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
    pub struct _CC2NPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC2NPW<'a> {
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
    pub struct _CC2NEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC2NEW<'a> {
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
    pub struct _CC1NPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC1NPW<'a> {
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
    pub struct _CC1NEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CC1NEW<'a> {
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
        # [ doc = "Bit 11 - Capture/Compare 3 output Polarity" ]
        # [ inline ( always ) ]
        pub fn cc3np(&self) -> CC3NPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC3NPR { bits }
        }
        # [ doc = "Bit 10 - Capture/Compare 3 complementary output enable" ]
        # [ inline ( always ) ]
        pub fn cc3ne(&self) -> CC3NER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC3NER { bits }
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
        # [ doc = "Bit 7 - Capture/Compare 2 output Polarity" ]
        # [ inline ( always ) ]
        pub fn cc2np(&self) -> CC2NPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC2NPR { bits }
        }
        # [ doc = "Bit 6 - Capture/Compare 2 complementary output enable" ]
        # [ inline ( always ) ]
        pub fn cc2ne(&self) -> CC2NER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC2NER { bits }
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
        # [ doc = "Bit 3 - Capture/Compare 1 output Polarity" ]
        # [ inline ( always ) ]
        pub fn cc1np(&self) -> CC1NPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC1NPR { bits }
        }
        # [ doc = "Bit 2 - Capture/Compare 1 complementary output enable" ]
        # [ inline ( always ) ]
        pub fn cc1ne(&self) -> CC1NER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC1NER { bits }
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
        # [ doc = "Bit 11 - Capture/Compare 3 output Polarity" ]
        # [ inline ( always ) ]
        pub fn cc3np(&mut self) -> _CC3NPW {
            _CC3NPW { w: self }
        }
        # [ doc = "Bit 10 - Capture/Compare 3 complementary output enable" ]
        # [ inline ( always ) ]
        pub fn cc3ne(&mut self) -> _CC3NEW {
            _CC3NEW { w: self }
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
        # [ doc = "Bit 7 - Capture/Compare 2 output Polarity" ]
        # [ inline ( always ) ]
        pub fn cc2np(&mut self) -> _CC2NPW {
            _CC2NPW { w: self }
        }
        # [ doc = "Bit 6 - Capture/Compare 2 complementary output enable" ]
        # [ inline ( always ) ]
        pub fn cc2ne(&mut self) -> _CC2NEW {
            _CC2NEW { w: self }
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
        # [ doc = "Bit 3 - Capture/Compare 1 output Polarity" ]
        # [ inline ( always ) ]
        pub fn cc1np(&mut self) -> _CC1NPW {
            _CC1NPW { w: self }
        }
        # [ doc = "Bit 2 - Capture/Compare 1 complementary output enable" ]
        # [ inline ( always ) ]
        pub fn cc1ne(&mut self) -> _CC1NEW {
            _CC1NEW { w: self }
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
# [ doc = "repetition counter register" ]
pub struct RCR {
    register: VolatileCell<u32>,
}
# [ doc = "repetition counter register" ]
pub mod rcr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::RCR {
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
    pub struct REPR {
        bits: u8,
    }
    impl REPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _REPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _REPW<'a> {
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
        # [ doc = "Bits 0:7 - Repetition counter value" ]
        # [ inline ( always ) ]
        pub fn rep(&self) -> REPR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            REPR { bits }
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
        # [ doc = "Bits 0:7 - Repetition counter value" ]
        # [ inline ( always ) ]
        pub fn rep(&mut self) -> _REPW {
            _REPW { w: self }
        }
    }
}
# [ doc = "break and dead-time register" ]
pub struct BDTR {
    register: VolatileCell<u32>,
}
# [ doc = "break and dead-time register" ]
pub mod bdtr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::BDTR {
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
    pub struct MOER {
        bits: bool,
    }
    impl MOER {
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
    pub struct AOER {
        bits: bool,
    }
    impl AOER {
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
    pub struct BKPR {
        bits: bool,
    }
    impl BKPR {
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
    pub struct BKER {
        bits: bool,
    }
    impl BKER {
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
    pub struct OSSRR {
        bits: bool,
    }
    impl OSSRR {
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
    pub struct OSSIR {
        bits: bool,
    }
    impl OSSIR {
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
    pub struct LOCKR {
        bits: u8,
    }
    impl LOCKR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DTGR {
        bits: u8,
    }
    impl DTGR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MOEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MOEW<'a> {
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
    pub struct _AOEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _AOEW<'a> {
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
    pub struct _BKPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BKPW<'a> {
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
    pub struct _BKEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BKEW<'a> {
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
    pub struct _OSSRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OSSRW<'a> {
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
    pub struct _OSSIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OSSIW<'a> {
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
    pub struct _LOCKW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LOCKW<'a> {
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
    pub struct _DTGW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTGW<'a> {
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
        # [ doc = "Bit 15 - Main output enable" ]
        # [ inline ( always ) ]
        pub fn moe(&self) -> MOER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MOER { bits }
        }
        # [ doc = "Bit 14 - Automatic output enable" ]
        # [ inline ( always ) ]
        pub fn aoe(&self) -> AOER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            AOER { bits }
        }
        # [ doc = "Bit 13 - Break polarity" ]
        # [ inline ( always ) ]
        pub fn bkp(&self) -> BKPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BKPR { bits }
        }
        # [ doc = "Bit 12 - Break enable" ]
        # [ inline ( always ) ]
        pub fn bke(&self) -> BKER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BKER { bits }
        }
        # [ doc = "Bit 11 - Off-state selection for Run mode" ]
        # [ inline ( always ) ]
        pub fn ossr(&self) -> OSSRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OSSRR { bits }
        }
        # [ doc = "Bit 10 - Off-state selection for Idle mode" ]
        # [ inline ( always ) ]
        pub fn ossi(&self) -> OSSIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OSSIR { bits }
        }
        # [ doc = "Bits 8:9 - Lock configuration" ]
        # [ inline ( always ) ]
        pub fn lock(&self) -> LOCKR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            LOCKR { bits }
        }
        # [ doc = "Bits 0:7 - Dead-time generator setup" ]
        # [ inline ( always ) ]
        pub fn dtg(&self) -> DTGR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DTGR { bits }
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
        # [ doc = "Bit 15 - Main output enable" ]
        # [ inline ( always ) ]
        pub fn moe(&mut self) -> _MOEW {
            _MOEW { w: self }
        }
        # [ doc = "Bit 14 - Automatic output enable" ]
        # [ inline ( always ) ]
        pub fn aoe(&mut self) -> _AOEW {
            _AOEW { w: self }
        }
        # [ doc = "Bit 13 - Break polarity" ]
        # [ inline ( always ) ]
        pub fn bkp(&mut self) -> _BKPW {
            _BKPW { w: self }
        }
        # [ doc = "Bit 12 - Break enable" ]
        # [ inline ( always ) ]
        pub fn bke(&mut self) -> _BKEW {
            _BKEW { w: self }
        }
        # [ doc = "Bit 11 - Off-state selection for Run mode" ]
        # [ inline ( always ) ]
        pub fn ossr(&mut self) -> _OSSRW {
            _OSSRW { w: self }
        }
        # [ doc = "Bit 10 - Off-state selection for Idle mode" ]
        # [ inline ( always ) ]
        pub fn ossi(&mut self) -> _OSSIW {
            _OSSIW { w: self }
        }
        # [ doc = "Bits 8:9 - Lock configuration" ]
        # [ inline ( always ) ]
        pub fn lock(&mut self) -> _LOCKW {
            _LOCKW { w: self }
        }
        # [ doc = "Bits 0:7 - Dead-time generator setup" ]
        # [ inline ( always ) ]
        pub fn dtg(&mut self) -> _DTGW {
            _DTGW { w: self }
        }
    }
}
# [ doc = "Advanced timer" ]
pub struct TIM1 {
    register_block: RegisterBlock,
}
impl Deref for TIM1 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
# [ doc = "TIM8" ]
pub const TIM8: Peripheral<TIM8> = unsafe { Peripheral::new(1073820672) };
# [ doc = r" Register block" ]
pub struct TIM8 {
    register_block: RegisterBlock,
}
impl Deref for TIM8 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
