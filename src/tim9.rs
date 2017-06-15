# ! [ doc = "General purpose timer" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;
use vcell::VolatileCell;

# [ doc = "General purpose timer" ]
pub const TIM9: Peripheral<TIM9> = unsafe { Peripheral::new(1073826816) };

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
    _reserved0: [u8; 4usize],
    # [ doc = "0x20 - capture/compare enable register" ]
    pub ccer: CCER,
    # [ doc = "0x24 - counter" ]
    pub cnt: CNT,
    # [ doc = "0x28 - prescaler" ]
    pub psc: PSC,
    # [ doc = "0x2c - auto-reload register" ]
    pub arr: ARR,
    _reserved1: [u8; 4usize],
    # [ doc = "0x34 - capture/compare register 1" ]
    pub ccr1: CCR1,
    # [ doc = "0x38 - capture/compare register 2" ]
    pub ccr2: CCR2,
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
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
        # [ doc = "Bits 4:6 - Master mode selection" ]
        # [ inline ( always ) ]
        pub fn mms(&mut self) -> _MMSW {
            _MMSW { w: self }
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
        # [ doc = r" Reserved" ]
        _Reserved(u8),
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
                TSR::_Reserved(bits) => bits,
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
                i => TSR::_Reserved(i),
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
    }
    # [ doc = "Possible values of the field `SMS`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum SMSR {
        # [ doc = "Counter disabled" ]
        DISABLED,
        # [ doc = "Rising edge of the selected trigger input (TRGI) reinitializes the counter" ]
        RESET,
        # [ doc = " The counter clock is enabled when the trigger input (TRGI) is high" ]
        GATED,
        # [ doc = "The counter starts at a rising edge of the trigger TRGI " ]
        TRIGGER,
        # [ doc = " Rising edges of the selected trigger (TRGI) clock the counter" ]
        EXTERNAL,
        # [ doc = r" Reserved" ]
        _Reserved(u8),
    }
    impl SMSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                SMSR::DISABLED => 0,
                SMSR::RESET => 4,
                SMSR::GATED => 5,
                SMSR::TRIGGER => 6,
                SMSR::EXTERNAL => 7,
                SMSR::_Reserved(bits) => bits,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> SMSR {
            match value {
                0 => SMSR::DISABLED,
                4 => SMSR::RESET,
                5 => SMSR::GATED,
                6 => SMSR::TRIGGER,
                7 => SMSR::EXTERNAL,
                i => SMSR::_Reserved(i),
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == SMSR::DISABLED
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
            unsafe { self.bits(variant._bits()) }
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
    # [ doc = "Values that can be written to the field `SMS`" ]
    pub enum SMSW {
        # [ doc = "Counter disabled" ]
        DISABLED,
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
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "Counter disabled" ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(SMSW::DISABLED)
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
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
        # [ doc = "Bit 6 - Trigger interrupt enable" ]
        # [ inline ( always ) ]
        pub fn tie(&mut self) -> _TIEW {
            _TIEW { w: self }
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
        # [ doc = "Bits 12:14 - Output Compare 2 mode" ]
        # [ inline ( always ) ]
        pub fn oc2m(&self) -> OC2MR {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            OC2MR { bits }
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
        # [ doc = "Bits 4:6 - Output Compare 1 mode" ]
        # [ inline ( always ) ]
        pub fn oc1m(&self) -> OC1MR {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            OC1MR { bits }
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
        # [ doc = "Bit 7 - Capture/Compare 2 output Polarity" ]
        # [ inline ( always ) ]
        pub fn cc2np(&mut self) -> _CC2NPW {
            _CC2NPW { w: self }
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
# [ doc = "General purpose timer" ]
pub struct TIM9 {
    register_block: RegisterBlock,
}
impl Deref for TIM9 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
# [ doc = "TIM12" ]
pub const TIM12: Peripheral<TIM12> = unsafe { Peripheral::new(1073747968) };
# [ doc = r" Register block" ]
pub struct TIM12 {
    register_block: RegisterBlock,
}
impl Deref for TIM12 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
