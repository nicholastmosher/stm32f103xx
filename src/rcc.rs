# ! [ doc = "Reset and clock control" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;
use vcell::VolatileCell;

# [ doc = "Reset and clock control" ]
pub const RCC: Peripheral<RCC> = unsafe { Peripheral::new(1073876992) };

# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - Clock control register" ]
    pub cr: CR,
    # [ doc = "0x04 - Clock configuration register (RCC_CFGR)" ]
    pub cfgr: CFGR,
    # [ doc = "0x08 - Clock interrupt register (RCC_CIR)" ]
    pub cir: CIR,
    # [ doc = "0x0c - APB2 peripheral reset register (RCC_APB2RSTR)" ]
    pub apb2rstr: APB2RSTR,
    # [ doc = "0x10 - APB1 peripheral reset register (RCC_APB1RSTR)" ]
    pub apb1rstr: APB1RSTR,
    # [ doc = "0x14 - AHB Peripheral Clock enable register (RCC_AHBENR)" ]
    pub ahbenr: AHBENR,
    # [ doc = "0x18 - APB2 peripheral clock enable register (RCC_APB2ENR)" ]
    pub apb2enr: APB2ENR,
    # [ doc = "0x1c - APB1 peripheral clock enable register (RCC_APB1ENR)" ]
    pub apb1enr: APB1ENR,
    # [ doc = "0x20 - Backup domain control register (RCC_BDCR)" ]
    pub bdcr: BDCR,
    # [ doc = "0x24 - Control/status register (RCC_CSR)" ]
    pub csr: CSR,
}
# [ doc = "Clock control register" ]
pub struct CR {
    register: VolatileCell<u32>,
}
# [ doc = "Clock control register" ]
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
    pub struct HSIONR {
        bits: bool,
    }
    impl HSIONR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct HSIRDYR {
        bits: bool,
    }
    impl HSIRDYR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct HSITRIMR {
        bits: u8,
    }
    impl HSITRIMR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct HSICALR {
        bits: u8,
    }
    impl HSICALR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Possible values of the field `HSEON`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum HSEONR {
        # [ doc = "HSE disabled" ]
        DISABLED,
        # [ doc = "HSE enabled" ]
        ENABLED,
    }
    impl HSEONR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                HSEONR::DISABLED => false,
                HSEONR::ENABLED => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> HSEONR {
            match value {
                false => HSEONR::DISABLED,
                true => HSEONR::ENABLED,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == HSEONR::DISABLED
        }
        # [ doc = "Checks if the value of the field is `ENABLED`" ]
        # [ inline ( always ) ]
        pub fn is_enabled(&self) -> bool {
            *self == HSEONR::ENABLED
        }
    }
    # [ doc = "Possible values of the field `HSERDY`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum HSERDYR {
        # [ doc = "HSE Not Ready" ]
        NOTREADY,
        # [ doc = "HSE Ready" ]
        READY,
    }
    impl HSERDYR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                HSERDYR::NOTREADY => false,
                HSERDYR::READY => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> HSERDYR {
            match value {
                false => HSERDYR::NOTREADY,
                true => HSERDYR::READY,
            }
        }
        # [ doc = "Checks if the value of the field is `NOTREADY`" ]
        # [ inline ( always ) ]
        pub fn is_notready(&self) -> bool {
            *self == HSERDYR::NOTREADY
        }
        # [ doc = "Checks if the value of the field is `READY`" ]
        # [ inline ( always ) ]
        pub fn is_ready(&self) -> bool {
            *self == HSERDYR::READY
        }
    }
    # [ doc = "Possible values of the field `HSEBYP`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum HSEBYPR {
        # [ doc = "external 4-16 MHz oscillator not bypassed" ]
        DISABLED,
        # [ doc = "external 4-16 MHz oscillator bypassed with external clock" ]
        ENABLED,
    }
    impl HSEBYPR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                HSEBYPR::DISABLED => false,
                HSEBYPR::ENABLED => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> HSEBYPR {
            match value {
                false => HSEBYPR::DISABLED,
                true => HSEBYPR::ENABLED,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == HSEBYPR::DISABLED
        }
        # [ doc = "Checks if the value of the field is `ENABLED`" ]
        # [ inline ( always ) ]
        pub fn is_enabled(&self) -> bool {
            *self == HSEBYPR::ENABLED
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CSSONR {
        bits: bool,
    }
    impl CSSONR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = "Possible values of the field `PLLON`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PLLONR {
        # [ doc = "PLL disabled" ]
        DISABLED,
        # [ doc = "Pll enabled" ]
        ENABLED,
    }
    impl PLLONR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                PLLONR::DISABLED => false,
                PLLONR::ENABLED => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PLLONR {
            match value {
                false => PLLONR::DISABLED,
                true => PLLONR::ENABLED,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == PLLONR::DISABLED
        }
        # [ doc = "Checks if the value of the field is `ENABLED`" ]
        # [ inline ( always ) ]
        pub fn is_enabled(&self) -> bool {
            *self == PLLONR::ENABLED
        }
    }
    # [ doc = "Possible values of the field `PLLRDY`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PLLRDYR {
        # [ doc = "PLL Unlocked" ]
        UNLOCKED,
        # [ doc = "PLL Locked" ]
        LOCKED,
    }
    impl PLLRDYR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                PLLRDYR::UNLOCKED => false,
                PLLRDYR::LOCKED => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PLLRDYR {
            match value {
                false => PLLRDYR::UNLOCKED,
                true => PLLRDYR::LOCKED,
            }
        }
        # [ doc = "Checks if the value of the field is `UNLOCKED`" ]
        # [ inline ( always ) ]
        pub fn is_unlocked(&self) -> bool {
            *self == PLLRDYR::UNLOCKED
        }
        # [ doc = "Checks if the value of the field is `LOCKED`" ]
        # [ inline ( always ) ]
        pub fn is_locked(&self) -> bool {
            *self == PLLRDYR::LOCKED
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _HSIONW<'a> {
        w: &'a mut W,
    }
    impl<'a> _HSIONW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _HSITRIMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _HSITRIMW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `HSEON`" ]
    pub enum HSEONW {
        # [ doc = "HSE disabled" ]
        DISABLED,
        # [ doc = "HSE enabled" ]
        ENABLED,
    }
    impl HSEONW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                HSEONW::DISABLED => false,
                HSEONW::ENABLED => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _HSEONW<'a> {
        w: &'a mut W,
    }
    impl<'a> _HSEONW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: HSEONW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "HSE disabled" ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(HSEONW::DISABLED)
        }
        # [ doc = "HSE enabled" ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(HSEONW::ENABLED)
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
    # [ doc = "Values that can be written to the field `HSEBYP`" ]
    pub enum HSEBYPW {
        # [ doc = "external 4-16 MHz oscillator not bypassed" ]
        DISABLED,
        # [ doc = "external 4-16 MHz oscillator bypassed with external clock" ]
        ENABLED,
    }
    impl HSEBYPW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                HSEBYPW::DISABLED => false,
                HSEBYPW::ENABLED => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _HSEBYPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _HSEBYPW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: HSEBYPW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "external 4-16 MHz oscillator not bypassed" ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(HSEBYPW::DISABLED)
        }
        # [ doc = "external 4-16 MHz oscillator bypassed with external clock" ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(HSEBYPW::ENABLED)
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
    pub struct _CSSONW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CSSONW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    # [ doc = "Values that can be written to the field `PLLON`" ]
    pub enum PLLONW {
        # [ doc = "PLL disabled" ]
        DISABLED,
        # [ doc = "Pll enabled" ]
        ENABLED,
    }
    impl PLLONW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PLLONW::DISABLED => false,
                PLLONW::ENABLED => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PLLONW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PLLONW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PLLONW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "PLL disabled" ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(PLLONW::DISABLED)
        }
        # [ doc = "Pll enabled" ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(PLLONW::ENABLED)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
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
        # [ doc = "Bit 0 - Internal High Speed clock enable" ]
        # [ inline ( always ) ]
        pub fn hsion(&self) -> HSIONR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            HSIONR { bits }
        }
        # [ doc = "Bit 1 - Internal High Speed clock ready flag" ]
        # [ inline ( always ) ]
        pub fn hsirdy(&self) -> HSIRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            HSIRDYR { bits }
        }
        # [ doc = "Bits 3:7 - Internal High Speed clock trimming" ]
        # [ inline ( always ) ]
        pub fn hsitrim(&self) -> HSITRIMR {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            HSITRIMR { bits }
        }
        # [ doc = "Bits 8:15 - Internal High Speed clock Calibration" ]
        # [ inline ( always ) ]
        pub fn hsical(&self) -> HSICALR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            HSICALR { bits }
        }
        # [ doc = "Bit 16 - External High Speed clock enable" ]
        # [ inline ( always ) ]
        pub fn hseon(&self) -> HSEONR {
            HSEONR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 17 - External High Speed clock ready flag" ]
        # [ inline ( always ) ]
        pub fn hserdy(&self) -> HSERDYR {
            HSERDYR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 18 - External High Speed clock Bypass" ]
        # [ inline ( always ) ]
        pub fn hsebyp(&self) -> HSEBYPR {
            HSEBYPR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 19 - Clock Security System enable" ]
        # [ inline ( always ) ]
        pub fn csson(&self) -> CSSONR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CSSONR { bits }
        }
        # [ doc = "Bit 24 - PLL enable" ]
        # [ inline ( always ) ]
        pub fn pllon(&self) -> PLLONR {
            PLLONR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 25 - PLL clock ready flag" ]
        # [ inline ( always ) ]
        pub fn pllrdy(&self) -> PLLRDYR {
            PLLRDYR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 131 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 0 - Internal High Speed clock enable" ]
        # [ inline ( always ) ]
        pub fn hsion(&mut self) -> _HSIONW {
            _HSIONW { w: self }
        }
        # [ doc = "Bits 3:7 - Internal High Speed clock trimming" ]
        # [ inline ( always ) ]
        pub fn hsitrim(&mut self) -> _HSITRIMW {
            _HSITRIMW { w: self }
        }
        # [ doc = "Bit 16 - External High Speed clock enable" ]
        # [ inline ( always ) ]
        pub fn hseon(&mut self) -> _HSEONW {
            _HSEONW { w: self }
        }
        # [ doc = "Bit 18 - External High Speed clock Bypass" ]
        # [ inline ( always ) ]
        pub fn hsebyp(&mut self) -> _HSEBYPW {
            _HSEBYPW { w: self }
        }
        # [ doc = "Bit 19 - Clock Security System enable" ]
        # [ inline ( always ) ]
        pub fn csson(&mut self) -> _CSSONW {
            _CSSONW { w: self }
        }
        # [ doc = "Bit 24 - PLL enable" ]
        # [ inline ( always ) ]
        pub fn pllon(&mut self) -> _PLLONW {
            _PLLONW { w: self }
        }
    }
}
# [ doc = "Clock configuration register (RCC_CFGR)" ]
pub struct CFGR {
    register: VolatileCell<u32>,
}
# [ doc = "Clock configuration register (RCC_CFGR)" ]
pub mod cfgr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CFGR {
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
    # [ doc = "Possible values of the field `SW`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum SWR {
        # [ doc = " HSI selected as system clock" ]
        HSI,
        # [ doc = " HSE selected as system clock" ]
        HSE,
        # [ doc = "PLL selected as system clock" ]
        PLL,
        # [ doc = r" Reserved" ]
        _Reserved(u8),
    }
    impl SWR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                SWR::HSI => 0,
                SWR::HSE => 1,
                SWR::PLL => 2,
                SWR::_Reserved(bits) => bits,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> SWR {
            match value {
                0 => SWR::HSI,
                1 => SWR::HSE,
                2 => SWR::PLL,
                i => SWR::_Reserved(i),
            }
        }
        # [ doc = "Checks if the value of the field is `HSI`" ]
        # [ inline ( always ) ]
        pub fn is_hsi(&self) -> bool {
            *self == SWR::HSI
        }
        # [ doc = "Checks if the value of the field is `HSE`" ]
        # [ inline ( always ) ]
        pub fn is_hse(&self) -> bool {
            *self == SWR::HSE
        }
        # [ doc = "Checks if the value of the field is `PLL`" ]
        # [ inline ( always ) ]
        pub fn is_pll(&self) -> bool {
            *self == SWR::PLL
        }
    }
    # [ doc = "Possible values of the field `SWS`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum SWSR {
        # [ doc = " HSI selected as system clock" ]
        HSI,
        # [ doc = " HSE selected as system clock" ]
        HSE,
        # [ doc = "PLL selected as system clock" ]
        PLL,
        # [ doc = r" Reserved" ]
        _Reserved(u8),
    }
    impl SWSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                SWSR::HSI => 0,
                SWSR::HSE => 1,
                SWSR::PLL => 2,
                SWSR::_Reserved(bits) => bits,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> SWSR {
            match value {
                0 => SWSR::HSI,
                1 => SWSR::HSE,
                2 => SWSR::PLL,
                i => SWSR::_Reserved(i),
            }
        }
        # [ doc = "Checks if the value of the field is `HSI`" ]
        # [ inline ( always ) ]
        pub fn is_hsi(&self) -> bool {
            *self == SWSR::HSI
        }
        # [ doc = "Checks if the value of the field is `HSE`" ]
        # [ inline ( always ) ]
        pub fn is_hse(&self) -> bool {
            *self == SWSR::HSE
        }
        # [ doc = "Checks if the value of the field is `PLL`" ]
        # [ inline ( always ) ]
        pub fn is_pll(&self) -> bool {
            *self == SWSR::PLL
        }
    }
    # [ doc = "Possible values of the field `HPRE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum HPRER {
        # [ doc = "SYSCLK not divided" ]
        DIV1,
        # [ doc = "SYSCLK divided by 2" ]
        DIV2,
        # [ doc = "SYSCLK divided by 4" ]
        DIV4,
        # [ doc = "SYSCLK divided by 8" ]
        DIV8,
        # [ doc = "SYSCLK divided by 16" ]
        DIV16,
        # [ doc = "SYSCLK divided by 64" ]
        DIV64,
        # [ doc = "SYSCLK divided by 128" ]
        DIV128,
        # [ doc = "SYSCLK divided by 256" ]
        DIV256,
        # [ doc = "SYSCLK divided by 512" ]
        DIV512,
        # [ doc = r" Reserved" ]
        _Reserved(u8),
    }
    impl HPRER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                HPRER::DIV1 => 0,
                HPRER::DIV2 => 8,
                HPRER::DIV4 => 9,
                HPRER::DIV8 => 10,
                HPRER::DIV16 => 11,
                HPRER::DIV64 => 12,
                HPRER::DIV128 => 13,
                HPRER::DIV256 => 14,
                HPRER::DIV512 => 15,
                HPRER::_Reserved(bits) => bits,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> HPRER {
            match value {
                0 => HPRER::DIV1,
                8 => HPRER::DIV2,
                9 => HPRER::DIV4,
                10 => HPRER::DIV8,
                11 => HPRER::DIV16,
                12 => HPRER::DIV64,
                13 => HPRER::DIV128,
                14 => HPRER::DIV256,
                15 => HPRER::DIV512,
                i => HPRER::_Reserved(i),
            }
        }
        # [ doc = "Checks if the value of the field is `DIV1`" ]
        # [ inline ( always ) ]
        pub fn is_div1(&self) -> bool {
            *self == HPRER::DIV1
        }
        # [ doc = "Checks if the value of the field is `DIV2`" ]
        # [ inline ( always ) ]
        pub fn is_div2(&self) -> bool {
            *self == HPRER::DIV2
        }
        # [ doc = "Checks if the value of the field is `DIV4`" ]
        # [ inline ( always ) ]
        pub fn is_div4(&self) -> bool {
            *self == HPRER::DIV4
        }
        # [ doc = "Checks if the value of the field is `DIV8`" ]
        # [ inline ( always ) ]
        pub fn is_div8(&self) -> bool {
            *self == HPRER::DIV8
        }
        # [ doc = "Checks if the value of the field is `DIV16`" ]
        # [ inline ( always ) ]
        pub fn is_div16(&self) -> bool {
            *self == HPRER::DIV16
        }
        # [ doc = "Checks if the value of the field is `DIV64`" ]
        # [ inline ( always ) ]
        pub fn is_div64(&self) -> bool {
            *self == HPRER::DIV64
        }
        # [ doc = "Checks if the value of the field is `DIV128`" ]
        # [ inline ( always ) ]
        pub fn is_div128(&self) -> bool {
            *self == HPRER::DIV128
        }
        # [ doc = "Checks if the value of the field is `DIV256`" ]
        # [ inline ( always ) ]
        pub fn is_div256(&self) -> bool {
            *self == HPRER::DIV256
        }
        # [ doc = "Checks if the value of the field is `DIV512`" ]
        # [ inline ( always ) ]
        pub fn is_div512(&self) -> bool {
            *self == HPRER::DIV512
        }
    }
    # [ doc = "Possible values of the field `PPRE1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PPRE1R {
        # [ doc = "HCLK not divided" ]
        DIV1,
        # [ doc = "HCLK divided by 2" ]
        DIV2,
        # [ doc = "HCLK divided by 4" ]
        DIV4,
        # [ doc = "HCLK divided by 8" ]
        DIV8,
        # [ doc = "HCLK divided by 16" ]
        DIV16,
        # [ doc = r" Reserved" ]
        _Reserved(u8),
    }
    impl PPRE1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PPRE1R::DIV1 => 0,
                PPRE1R::DIV2 => 4,
                PPRE1R::DIV4 => 5,
                PPRE1R::DIV8 => 6,
                PPRE1R::DIV16 => 7,
                PPRE1R::_Reserved(bits) => bits,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PPRE1R {
            match value {
                0 => PPRE1R::DIV1,
                4 => PPRE1R::DIV2,
                5 => PPRE1R::DIV4,
                6 => PPRE1R::DIV8,
                7 => PPRE1R::DIV16,
                i => PPRE1R::_Reserved(i),
            }
        }
        # [ doc = "Checks if the value of the field is `DIV1`" ]
        # [ inline ( always ) ]
        pub fn is_div1(&self) -> bool {
            *self == PPRE1R::DIV1
        }
        # [ doc = "Checks if the value of the field is `DIV2`" ]
        # [ inline ( always ) ]
        pub fn is_div2(&self) -> bool {
            *self == PPRE1R::DIV2
        }
        # [ doc = "Checks if the value of the field is `DIV4`" ]
        # [ inline ( always ) ]
        pub fn is_div4(&self) -> bool {
            *self == PPRE1R::DIV4
        }
        # [ doc = "Checks if the value of the field is `DIV8`" ]
        # [ inline ( always ) ]
        pub fn is_div8(&self) -> bool {
            *self == PPRE1R::DIV8
        }
        # [ doc = "Checks if the value of the field is `DIV16`" ]
        # [ inline ( always ) ]
        pub fn is_div16(&self) -> bool {
            *self == PPRE1R::DIV16
        }
    }
    # [ doc = "Possible values of the field `PPRE2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PPRE2R {
        # [ doc = "HCLK not divided" ]
        DIV1,
        # [ doc = "HCLK divided by 2" ]
        DIV2,
        # [ doc = "HCLK divided by 4" ]
        DIV4,
        # [ doc = "HCLK divided by 8" ]
        DIV8,
        # [ doc = "HCLK divided by 16" ]
        DIV16,
        # [ doc = r" Reserved" ]
        _Reserved(u8),
    }
    impl PPRE2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PPRE2R::DIV1 => 0,
                PPRE2R::DIV2 => 4,
                PPRE2R::DIV4 => 5,
                PPRE2R::DIV8 => 6,
                PPRE2R::DIV16 => 7,
                PPRE2R::_Reserved(bits) => bits,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PPRE2R {
            match value {
                0 => PPRE2R::DIV1,
                4 => PPRE2R::DIV2,
                5 => PPRE2R::DIV4,
                6 => PPRE2R::DIV8,
                7 => PPRE2R::DIV16,
                i => PPRE2R::_Reserved(i),
            }
        }
        # [ doc = "Checks if the value of the field is `DIV1`" ]
        # [ inline ( always ) ]
        pub fn is_div1(&self) -> bool {
            *self == PPRE2R::DIV1
        }
        # [ doc = "Checks if the value of the field is `DIV2`" ]
        # [ inline ( always ) ]
        pub fn is_div2(&self) -> bool {
            *self == PPRE2R::DIV2
        }
        # [ doc = "Checks if the value of the field is `DIV4`" ]
        # [ inline ( always ) ]
        pub fn is_div4(&self) -> bool {
            *self == PPRE2R::DIV4
        }
        # [ doc = "Checks if the value of the field is `DIV8`" ]
        # [ inline ( always ) ]
        pub fn is_div8(&self) -> bool {
            *self == PPRE2R::DIV8
        }
        # [ doc = "Checks if the value of the field is `DIV16`" ]
        # [ inline ( always ) ]
        pub fn is_div16(&self) -> bool {
            *self == PPRE2R::DIV16
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ADCPRER {
        bits: u8,
    }
    impl ADCPRER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Possible values of the field `PLLSRC`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PLLSRCR {
        # [ doc = " HSI oscillator clock / 2 " ]
        INTERNAL,
        # [ doc = "HSE oscillator clock " ]
        EXTERNAL,
    }
    impl PLLSRCR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                PLLSRCR::INTERNAL => false,
                PLLSRCR::EXTERNAL => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PLLSRCR {
            match value {
                false => PLLSRCR::INTERNAL,
                true => PLLSRCR::EXTERNAL,
            }
        }
        # [ doc = "Checks if the value of the field is `INTERNAL`" ]
        # [ inline ( always ) ]
        pub fn is_internal(&self) -> bool {
            *self == PLLSRCR::INTERNAL
        }
        # [ doc = "Checks if the value of the field is `EXTERNAL`" ]
        # [ inline ( always ) ]
        pub fn is_external(&self) -> bool {
            *self == PLLSRCR::EXTERNAL
        }
    }
    # [ doc = "Possible values of the field `PLLXTPRE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PLLXTPRER {
        # [ doc = "HSE not divided" ]
        DIV1,
        # [ doc = "HSE divided by 2" ]
        DIV2,
    }
    impl PLLXTPRER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                PLLXTPRER::DIV1 => false,
                PLLXTPRER::DIV2 => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PLLXTPRER {
            match value {
                false => PLLXTPRER::DIV1,
                true => PLLXTPRER::DIV2,
            }
        }
        # [ doc = "Checks if the value of the field is `DIV1`" ]
        # [ inline ( always ) ]
        pub fn is_div1(&self) -> bool {
            *self == PLLXTPRER::DIV1
        }
        # [ doc = "Checks if the value of the field is `DIV2`" ]
        # [ inline ( always ) ]
        pub fn is_div2(&self) -> bool {
            *self == PLLXTPRER::DIV2
        }
    }
    # [ doc = "Possible values of the field `PLLMUL`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PLLMULR {
        # [ doc = "PLL input clock x 2" ]
        MUL2,
        # [ doc = "PLL input clock x 3" ]
        MUL3,
        # [ doc = "PLL input clock x 4" ]
        MUL4,
        # [ doc = "PLL input clock x 5" ]
        MUL5,
        # [ doc = "PLL input clock x 6" ]
        MUL6,
        # [ doc = "PLL input clock x 7" ]
        MUL7,
        # [ doc = "PLL input clock x 8" ]
        MUL8,
        # [ doc = "PLL input clock x 9" ]
        MUL9,
        # [ doc = "PLL input clock x 10" ]
        MUL10,
        # [ doc = "PLL input clock x 11" ]
        MUL11,
        # [ doc = "PLL input clock x 12" ]
        MUL12,
        # [ doc = "PLL input clock x 13" ]
        MUL13,
        # [ doc = "PLL input clock x 14" ]
        MUL14,
        # [ doc = "PLL input clock x 15" ]
        MUL15,
        # [ doc = "PLL input clock x 16" ]
        MUL16,
        # [ doc = r" Reserved" ]
        _Reserved(u8),
    }
    impl PLLMULR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PLLMULR::MUL2 => 0,
                PLLMULR::MUL3 => 1,
                PLLMULR::MUL4 => 2,
                PLLMULR::MUL5 => 3,
                PLLMULR::MUL6 => 4,
                PLLMULR::MUL7 => 5,
                PLLMULR::MUL8 => 6,
                PLLMULR::MUL9 => 7,
                PLLMULR::MUL10 => 8,
                PLLMULR::MUL11 => 9,
                PLLMULR::MUL12 => 10,
                PLLMULR::MUL13 => 11,
                PLLMULR::MUL14 => 12,
                PLLMULR::MUL15 => 13,
                PLLMULR::MUL16 => 14,
                PLLMULR::_Reserved(bits) => bits,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PLLMULR {
            match value {
                0 => PLLMULR::MUL2,
                1 => PLLMULR::MUL3,
                2 => PLLMULR::MUL4,
                3 => PLLMULR::MUL5,
                4 => PLLMULR::MUL6,
                5 => PLLMULR::MUL7,
                6 => PLLMULR::MUL8,
                7 => PLLMULR::MUL9,
                8 => PLLMULR::MUL10,
                9 => PLLMULR::MUL11,
                10 => PLLMULR::MUL12,
                11 => PLLMULR::MUL13,
                12 => PLLMULR::MUL14,
                13 => PLLMULR::MUL15,
                14 => PLLMULR::MUL16,
                i => PLLMULR::_Reserved(i),
            }
        }
        # [ doc = "Checks if the value of the field is `MUL2`" ]
        # [ inline ( always ) ]
        pub fn is_mul2(&self) -> bool {
            *self == PLLMULR::MUL2
        }
        # [ doc = "Checks if the value of the field is `MUL3`" ]
        # [ inline ( always ) ]
        pub fn is_mul3(&self) -> bool {
            *self == PLLMULR::MUL3
        }
        # [ doc = "Checks if the value of the field is `MUL4`" ]
        # [ inline ( always ) ]
        pub fn is_mul4(&self) -> bool {
            *self == PLLMULR::MUL4
        }
        # [ doc = "Checks if the value of the field is `MUL5`" ]
        # [ inline ( always ) ]
        pub fn is_mul5(&self) -> bool {
            *self == PLLMULR::MUL5
        }
        # [ doc = "Checks if the value of the field is `MUL6`" ]
        # [ inline ( always ) ]
        pub fn is_mul6(&self) -> bool {
            *self == PLLMULR::MUL6
        }
        # [ doc = "Checks if the value of the field is `MUL7`" ]
        # [ inline ( always ) ]
        pub fn is_mul7(&self) -> bool {
            *self == PLLMULR::MUL7
        }
        # [ doc = "Checks if the value of the field is `MUL8`" ]
        # [ inline ( always ) ]
        pub fn is_mul8(&self) -> bool {
            *self == PLLMULR::MUL8
        }
        # [ doc = "Checks if the value of the field is `MUL9`" ]
        # [ inline ( always ) ]
        pub fn is_mul9(&self) -> bool {
            *self == PLLMULR::MUL9
        }
        # [ doc = "Checks if the value of the field is `MUL10`" ]
        # [ inline ( always ) ]
        pub fn is_mul10(&self) -> bool {
            *self == PLLMULR::MUL10
        }
        # [ doc = "Checks if the value of the field is `MUL11`" ]
        # [ inline ( always ) ]
        pub fn is_mul11(&self) -> bool {
            *self == PLLMULR::MUL11
        }
        # [ doc = "Checks if the value of the field is `MUL12`" ]
        # [ inline ( always ) ]
        pub fn is_mul12(&self) -> bool {
            *self == PLLMULR::MUL12
        }
        # [ doc = "Checks if the value of the field is `MUL13`" ]
        # [ inline ( always ) ]
        pub fn is_mul13(&self) -> bool {
            *self == PLLMULR::MUL13
        }
        # [ doc = "Checks if the value of the field is `MUL14`" ]
        # [ inline ( always ) ]
        pub fn is_mul14(&self) -> bool {
            *self == PLLMULR::MUL14
        }
        # [ doc = "Checks if the value of the field is `MUL15`" ]
        # [ inline ( always ) ]
        pub fn is_mul15(&self) -> bool {
            *self == PLLMULR::MUL15
        }
        # [ doc = "Checks if the value of the field is `MUL16`" ]
        # [ inline ( always ) ]
        pub fn is_mul16(&self) -> bool {
            *self == PLLMULR::MUL16
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct OTGFSPRER {
        bits: bool,
    }
    impl OTGFSPRER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct MCOR {
        bits: u8,
    }
    impl MCOR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Values that can be written to the field `SW`" ]
    pub enum SWW {
        # [ doc = " HSI selected as system clock" ]
        HSI,
        # [ doc = " HSE selected as system clock" ]
        HSE,
        # [ doc = "PLL selected as system clock" ]
        PLL,
    }
    impl SWW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                SWW::HSI => 0,
                SWW::HSE => 1,
                SWW::PLL => 2,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SWW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: SWW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "HSI selected as system clock" ]
        # [ inline ( always ) ]
        pub fn hsi(self) -> &'a mut W {
            self.variant(SWW::HSI)
        }
        # [ doc = "HSE selected as system clock" ]
        # [ inline ( always ) ]
        pub fn hse(self) -> &'a mut W {
            self.variant(SWW::HSE)
        }
        # [ doc = "PLL selected as system clock" ]
        # [ inline ( always ) ]
        pub fn pll(self) -> &'a mut W {
            self.variant(SWW::PLL)
        }
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
    # [ doc = "Values that can be written to the field `HPRE`" ]
    pub enum HPREW {
        # [ doc = "SYSCLK not divided" ]
        DIV1,
        # [ doc = "SYSCLK divided by 2" ]
        DIV2,
        # [ doc = "SYSCLK divided by 4" ]
        DIV4,
        # [ doc = "SYSCLK divided by 8" ]
        DIV8,
        # [ doc = "SYSCLK divided by 16" ]
        DIV16,
        # [ doc = "SYSCLK divided by 64" ]
        DIV64,
        # [ doc = "SYSCLK divided by 128" ]
        DIV128,
        # [ doc = "SYSCLK divided by 256" ]
        DIV256,
        # [ doc = "SYSCLK divided by 512" ]
        DIV512,
    }
    impl HPREW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                HPREW::DIV1 => 0,
                HPREW::DIV2 => 8,
                HPREW::DIV4 => 9,
                HPREW::DIV8 => 10,
                HPREW::DIV16 => 11,
                HPREW::DIV64 => 12,
                HPREW::DIV128 => 13,
                HPREW::DIV256 => 14,
                HPREW::DIV512 => 15,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _HPREW<'a> {
        w: &'a mut W,
    }
    impl<'a> _HPREW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: HPREW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "SYSCLK not divided" ]
        # [ inline ( always ) ]
        pub fn div1(self) -> &'a mut W {
            self.variant(HPREW::DIV1)
        }
        # [ doc = "SYSCLK divided by 2" ]
        # [ inline ( always ) ]
        pub fn div2(self) -> &'a mut W {
            self.variant(HPREW::DIV2)
        }
        # [ doc = "SYSCLK divided by 4" ]
        # [ inline ( always ) ]
        pub fn div4(self) -> &'a mut W {
            self.variant(HPREW::DIV4)
        }
        # [ doc = "SYSCLK divided by 8" ]
        # [ inline ( always ) ]
        pub fn div8(self) -> &'a mut W {
            self.variant(HPREW::DIV8)
        }
        # [ doc = "SYSCLK divided by 16" ]
        # [ inline ( always ) ]
        pub fn div16(self) -> &'a mut W {
            self.variant(HPREW::DIV16)
        }
        # [ doc = "SYSCLK divided by 64" ]
        # [ inline ( always ) ]
        pub fn div64(self) -> &'a mut W {
            self.variant(HPREW::DIV64)
        }
        # [ doc = "SYSCLK divided by 128" ]
        # [ inline ( always ) ]
        pub fn div128(self) -> &'a mut W {
            self.variant(HPREW::DIV128)
        }
        # [ doc = "SYSCLK divided by 256" ]
        # [ inline ( always ) ]
        pub fn div256(self) -> &'a mut W {
            self.variant(HPREW::DIV256)
        }
        # [ doc = "SYSCLK divided by 512" ]
        # [ inline ( always ) ]
        pub fn div512(self) -> &'a mut W {
            self.variant(HPREW::DIV512)
        }
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
    # [ doc = "Values that can be written to the field `PPRE1`" ]
    pub enum PPRE1W {
        # [ doc = "HCLK not divided" ]
        DIV1,
        # [ doc = "HCLK divided by 2" ]
        DIV2,
        # [ doc = "HCLK divided by 4" ]
        DIV4,
        # [ doc = "HCLK divided by 8" ]
        DIV8,
        # [ doc = "HCLK divided by 16" ]
        DIV16,
    }
    impl PPRE1W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PPRE1W::DIV1 => 0,
                PPRE1W::DIV2 => 4,
                PPRE1W::DIV4 => 5,
                PPRE1W::DIV8 => 6,
                PPRE1W::DIV16 => 7,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PPRE1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PPRE1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PPRE1W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "HCLK not divided" ]
        # [ inline ( always ) ]
        pub fn div1(self) -> &'a mut W {
            self.variant(PPRE1W::DIV1)
        }
        # [ doc = "HCLK divided by 2" ]
        # [ inline ( always ) ]
        pub fn div2(self) -> &'a mut W {
            self.variant(PPRE1W::DIV2)
        }
        # [ doc = "HCLK divided by 4" ]
        # [ inline ( always ) ]
        pub fn div4(self) -> &'a mut W {
            self.variant(PPRE1W::DIV4)
        }
        # [ doc = "HCLK divided by 8" ]
        # [ inline ( always ) ]
        pub fn div8(self) -> &'a mut W {
            self.variant(PPRE1W::DIV8)
        }
        # [ doc = "HCLK divided by 16" ]
        # [ inline ( always ) ]
        pub fn div16(self) -> &'a mut W {
            self.variant(PPRE1W::DIV16)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `PPRE2`" ]
    pub enum PPRE2W {
        # [ doc = "HCLK not divided" ]
        DIV1,
        # [ doc = "HCLK divided by 2" ]
        DIV2,
        # [ doc = "HCLK divided by 4" ]
        DIV4,
        # [ doc = "HCLK divided by 8" ]
        DIV8,
        # [ doc = "HCLK divided by 16" ]
        DIV16,
    }
    impl PPRE2W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PPRE2W::DIV1 => 0,
                PPRE2W::DIV2 => 4,
                PPRE2W::DIV4 => 5,
                PPRE2W::DIV8 => 6,
                PPRE2W::DIV16 => 7,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PPRE2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PPRE2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PPRE2W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "HCLK not divided" ]
        # [ inline ( always ) ]
        pub fn div1(self) -> &'a mut W {
            self.variant(PPRE2W::DIV1)
        }
        # [ doc = "HCLK divided by 2" ]
        # [ inline ( always ) ]
        pub fn div2(self) -> &'a mut W {
            self.variant(PPRE2W::DIV2)
        }
        # [ doc = "HCLK divided by 4" ]
        # [ inline ( always ) ]
        pub fn div4(self) -> &'a mut W {
            self.variant(PPRE2W::DIV4)
        }
        # [ doc = "HCLK divided by 8" ]
        # [ inline ( always ) ]
        pub fn div8(self) -> &'a mut W {
            self.variant(PPRE2W::DIV8)
        }
        # [ doc = "HCLK divided by 16" ]
        # [ inline ( always ) ]
        pub fn div16(self) -> &'a mut W {
            self.variant(PPRE2W::DIV16)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADCPREW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADCPREW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `PLLSRC`" ]
    pub enum PLLSRCW {
        # [ doc = " HSI oscillator clock / 2 " ]
        INTERNAL,
        # [ doc = "HSE oscillator clock " ]
        EXTERNAL,
    }
    impl PLLSRCW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PLLSRCW::INTERNAL => false,
                PLLSRCW::EXTERNAL => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PLLSRCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PLLSRCW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PLLSRCW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "HSI oscillator clock / 2" ]
        # [ inline ( always ) ]
        pub fn internal(self) -> &'a mut W {
            self.variant(PLLSRCW::INTERNAL)
        }
        # [ doc = "HSE oscillator clock" ]
        # [ inline ( always ) ]
        pub fn external(self) -> &'a mut W {
            self.variant(PLLSRCW::EXTERNAL)
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
    # [ doc = "Values that can be written to the field `PLLXTPRE`" ]
    pub enum PLLXTPREW {
        # [ doc = "HSE not divided" ]
        DIV1,
        # [ doc = "HSE divided by 2" ]
        DIV2,
    }
    impl PLLXTPREW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PLLXTPREW::DIV1 => false,
                PLLXTPREW::DIV2 => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PLLXTPREW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PLLXTPREW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PLLXTPREW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "HSE not divided" ]
        # [ inline ( always ) ]
        pub fn div1(self) -> &'a mut W {
            self.variant(PLLXTPREW::DIV1)
        }
        # [ doc = "HSE divided by 2" ]
        # [ inline ( always ) ]
        pub fn div2(self) -> &'a mut W {
            self.variant(PLLXTPREW::DIV2)
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
    # [ doc = "Values that can be written to the field `PLLMUL`" ]
    pub enum PLLMULW {
        # [ doc = "PLL input clock x 2" ]
        MUL2,
        # [ doc = "PLL input clock x 3" ]
        MUL3,
        # [ doc = "PLL input clock x 4" ]
        MUL4,
        # [ doc = "PLL input clock x 5" ]
        MUL5,
        # [ doc = "PLL input clock x 6" ]
        MUL6,
        # [ doc = "PLL input clock x 7" ]
        MUL7,
        # [ doc = "PLL input clock x 8" ]
        MUL8,
        # [ doc = "PLL input clock x 9" ]
        MUL9,
        # [ doc = "PLL input clock x 10" ]
        MUL10,
        # [ doc = "PLL input clock x 11" ]
        MUL11,
        # [ doc = "PLL input clock x 12" ]
        MUL12,
        # [ doc = "PLL input clock x 13" ]
        MUL13,
        # [ doc = "PLL input clock x 14" ]
        MUL14,
        # [ doc = "PLL input clock x 15" ]
        MUL15,
        # [ doc = "PLL input clock x 16" ]
        MUL16,
    }
    impl PLLMULW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PLLMULW::MUL2 => 0,
                PLLMULW::MUL3 => 1,
                PLLMULW::MUL4 => 2,
                PLLMULW::MUL5 => 3,
                PLLMULW::MUL6 => 4,
                PLLMULW::MUL7 => 5,
                PLLMULW::MUL8 => 6,
                PLLMULW::MUL9 => 7,
                PLLMULW::MUL10 => 8,
                PLLMULW::MUL11 => 9,
                PLLMULW::MUL12 => 10,
                PLLMULW::MUL13 => 11,
                PLLMULW::MUL14 => 12,
                PLLMULW::MUL15 => 13,
                PLLMULW::MUL16 => 14,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PLLMULW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PLLMULW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PLLMULW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "PLL input clock x 2" ]
        # [ inline ( always ) ]
        pub fn mul2(self) -> &'a mut W {
            self.variant(PLLMULW::MUL2)
        }
        # [ doc = "PLL input clock x 3" ]
        # [ inline ( always ) ]
        pub fn mul3(self) -> &'a mut W {
            self.variant(PLLMULW::MUL3)
        }
        # [ doc = "PLL input clock x 4" ]
        # [ inline ( always ) ]
        pub fn mul4(self) -> &'a mut W {
            self.variant(PLLMULW::MUL4)
        }
        # [ doc = "PLL input clock x 5" ]
        # [ inline ( always ) ]
        pub fn mul5(self) -> &'a mut W {
            self.variant(PLLMULW::MUL5)
        }
        # [ doc = "PLL input clock x 6" ]
        # [ inline ( always ) ]
        pub fn mul6(self) -> &'a mut W {
            self.variant(PLLMULW::MUL6)
        }
        # [ doc = "PLL input clock x 7" ]
        # [ inline ( always ) ]
        pub fn mul7(self) -> &'a mut W {
            self.variant(PLLMULW::MUL7)
        }
        # [ doc = "PLL input clock x 8" ]
        # [ inline ( always ) ]
        pub fn mul8(self) -> &'a mut W {
            self.variant(PLLMULW::MUL8)
        }
        # [ doc = "PLL input clock x 9" ]
        # [ inline ( always ) ]
        pub fn mul9(self) -> &'a mut W {
            self.variant(PLLMULW::MUL9)
        }
        # [ doc = "PLL input clock x 10" ]
        # [ inline ( always ) ]
        pub fn mul10(self) -> &'a mut W {
            self.variant(PLLMULW::MUL10)
        }
        # [ doc = "PLL input clock x 11" ]
        # [ inline ( always ) ]
        pub fn mul11(self) -> &'a mut W {
            self.variant(PLLMULW::MUL11)
        }
        # [ doc = "PLL input clock x 12" ]
        # [ inline ( always ) ]
        pub fn mul12(self) -> &'a mut W {
            self.variant(PLLMULW::MUL12)
        }
        # [ doc = "PLL input clock x 13" ]
        # [ inline ( always ) ]
        pub fn mul13(self) -> &'a mut W {
            self.variant(PLLMULW::MUL13)
        }
        # [ doc = "PLL input clock x 14" ]
        # [ inline ( always ) ]
        pub fn mul14(self) -> &'a mut W {
            self.variant(PLLMULW::MUL14)
        }
        # [ doc = "PLL input clock x 15" ]
        # [ inline ( always ) ]
        pub fn mul15(self) -> &'a mut W {
            self.variant(PLLMULW::MUL15)
        }
        # [ doc = "PLL input clock x 16" ]
        # [ inline ( always ) ]
        pub fn mul16(self) -> &'a mut W {
            self.variant(PLLMULW::MUL16)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _OTGFSPREW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OTGFSPREW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _MCOW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MCOW<'a> {
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:1 - System clock Switch" ]
        # [ inline ( always ) ]
        pub fn sw(&self) -> SWR {
            SWR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 2:3 - System Clock Switch Status" ]
        # [ inline ( always ) ]
        pub fn sws(&self) -> SWSR {
            SWSR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 4:7 - AHB prescaler" ]
        # [ inline ( always ) ]
        pub fn hpre(&self) -> HPRER {
            HPRER::_from({
                const MASK: u8 = 15;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 8:10 - APB Low speed prescaler (APB1)" ]
        # [ inline ( always ) ]
        pub fn ppre1(&self) -> PPRE1R {
            PPRE1R::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 11:13 - APB High speed prescaler (APB2)" ]
        # [ inline ( always ) ]
        pub fn ppre2(&self) -> PPRE2R {
            PPRE2R::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 14:15 - ADC prescaler" ]
        # [ inline ( always ) ]
        pub fn adcpre(&self) -> ADCPRER {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ADCPRER { bits }
        }
        # [ doc = "Bit 16 - PLL entry clock source" ]
        # [ inline ( always ) ]
        pub fn pllsrc(&self) -> PLLSRCR {
            PLLSRCR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 17 - HSE divider for PLL entry" ]
        # [ inline ( always ) ]
        pub fn pllxtpre(&self) -> PLLXTPRER {
            PLLXTPRER::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bits 18:21 - PLL Multiplication Factor" ]
        # [ inline ( always ) ]
        pub fn pllmul(&self) -> PLLMULR {
            PLLMULR::_from({
                const MASK: u8 = 15;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bit 22 - USB OTG FS prescaler" ]
        # [ inline ( always ) ]
        pub fn otgfspre(&self) -> OTGFSPRER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 22;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OTGFSPRER { bits }
        }
        # [ doc = "Bits 24:26 - Microcontroller clock output" ]
        # [ inline ( always ) ]
        pub fn mco(&self) -> MCOR {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MCOR { bits }
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
        # [ doc = "Bits 0:1 - System clock Switch" ]
        # [ inline ( always ) ]
        pub fn sw(&mut self) -> _SWW {
            _SWW { w: self }
        }
        # [ doc = "Bits 4:7 - AHB prescaler" ]
        # [ inline ( always ) ]
        pub fn hpre(&mut self) -> _HPREW {
            _HPREW { w: self }
        }
        # [ doc = "Bits 8:10 - APB Low speed prescaler (APB1)" ]
        # [ inline ( always ) ]
        pub fn ppre1(&mut self) -> _PPRE1W {
            _PPRE1W { w: self }
        }
        # [ doc = "Bits 11:13 - APB High speed prescaler (APB2)" ]
        # [ inline ( always ) ]
        pub fn ppre2(&mut self) -> _PPRE2W {
            _PPRE2W { w: self }
        }
        # [ doc = "Bits 14:15 - ADC prescaler" ]
        # [ inline ( always ) ]
        pub fn adcpre(&mut self) -> _ADCPREW {
            _ADCPREW { w: self }
        }
        # [ doc = "Bit 16 - PLL entry clock source" ]
        # [ inline ( always ) ]
        pub fn pllsrc(&mut self) -> _PLLSRCW {
            _PLLSRCW { w: self }
        }
        # [ doc = "Bit 17 - HSE divider for PLL entry" ]
        # [ inline ( always ) ]
        pub fn pllxtpre(&mut self) -> _PLLXTPREW {
            _PLLXTPREW { w: self }
        }
        # [ doc = "Bits 18:21 - PLL Multiplication Factor" ]
        # [ inline ( always ) ]
        pub fn pllmul(&mut self) -> _PLLMULW {
            _PLLMULW { w: self }
        }
        # [ doc = "Bit 22 - USB OTG FS prescaler" ]
        # [ inline ( always ) ]
        pub fn otgfspre(&mut self) -> _OTGFSPREW {
            _OTGFSPREW { w: self }
        }
        # [ doc = "Bits 24:26 - Microcontroller clock output" ]
        # [ inline ( always ) ]
        pub fn mco(&mut self) -> _MCOW {
            _MCOW { w: self }
        }
    }
}
# [ doc = "Clock interrupt register (RCC_CIR)" ]
pub struct CIR {
    register: VolatileCell<u32>,
}
# [ doc = "Clock interrupt register (RCC_CIR)" ]
pub mod cir {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CIR {
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
    pub struct LSIRDYFR {
        bits: bool,
    }
    impl LSIRDYFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct LSERDYFR {
        bits: bool,
    }
    impl LSERDYFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct HSIRDYFR {
        bits: bool,
    }
    impl HSIRDYFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct HSERDYFR {
        bits: bool,
    }
    impl HSERDYFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct PLLRDYFR {
        bits: bool,
    }
    impl PLLRDYFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct CSSFR {
        bits: bool,
    }
    impl CSSFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct LSIRDYIER {
        bits: bool,
    }
    impl LSIRDYIER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct LSERDYIER {
        bits: bool,
    }
    impl LSERDYIER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct HSIRDYIER {
        bits: bool,
    }
    impl HSIRDYIER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct HSERDYIER {
        bits: bool,
    }
    impl HSERDYIER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct PLLRDYIER {
        bits: bool,
    }
    impl PLLRDYIER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _LSIRDYIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LSIRDYIEW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LSERDYIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LSERDYIEW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _HSIRDYIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _HSIRDYIEW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _HSERDYIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _HSERDYIEW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _PLLRDYIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PLLRDYIEW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LSIRDYCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LSIRDYCW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LSERDYCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LSERDYCW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _HSIRDYCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _HSIRDYCW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _HSERDYCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _HSERDYCW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _PLLRDYCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PLLRDYCW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CSSCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CSSCW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - LSI Ready Interrupt flag" ]
        # [ inline ( always ) ]
        pub fn lsirdyf(&self) -> LSIRDYFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LSIRDYFR { bits }
        }
        # [ doc = "Bit 1 - LSE Ready Interrupt flag" ]
        # [ inline ( always ) ]
        pub fn lserdyf(&self) -> LSERDYFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LSERDYFR { bits }
        }
        # [ doc = "Bit 2 - HSI Ready Interrupt flag" ]
        # [ inline ( always ) ]
        pub fn hsirdyf(&self) -> HSIRDYFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            HSIRDYFR { bits }
        }
        # [ doc = "Bit 3 - HSE Ready Interrupt flag" ]
        # [ inline ( always ) ]
        pub fn hserdyf(&self) -> HSERDYFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            HSERDYFR { bits }
        }
        # [ doc = "Bit 4 - PLL Ready Interrupt flag" ]
        # [ inline ( always ) ]
        pub fn pllrdyf(&self) -> PLLRDYFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PLLRDYFR { bits }
        }
        # [ doc = "Bit 7 - Clock Security System Interrupt flag" ]
        # [ inline ( always ) ]
        pub fn cssf(&self) -> CSSFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CSSFR { bits }
        }
        # [ doc = "Bit 8 - LSI Ready Interrupt Enable" ]
        # [ inline ( always ) ]
        pub fn lsirdyie(&self) -> LSIRDYIER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LSIRDYIER { bits }
        }
        # [ doc = "Bit 9 - LSE Ready Interrupt Enable" ]
        # [ inline ( always ) ]
        pub fn lserdyie(&self) -> LSERDYIER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LSERDYIER { bits }
        }
        # [ doc = "Bit 10 - HSI Ready Interrupt Enable" ]
        # [ inline ( always ) ]
        pub fn hsirdyie(&self) -> HSIRDYIER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            HSIRDYIER { bits }
        }
        # [ doc = "Bit 11 - HSE Ready Interrupt Enable" ]
        # [ inline ( always ) ]
        pub fn hserdyie(&self) -> HSERDYIER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            HSERDYIER { bits }
        }
        # [ doc = "Bit 12 - PLL Ready Interrupt Enable" ]
        # [ inline ( always ) ]
        pub fn pllrdyie(&self) -> PLLRDYIER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PLLRDYIER { bits }
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
        # [ doc = "Bit 8 - LSI Ready Interrupt Enable" ]
        # [ inline ( always ) ]
        pub fn lsirdyie(&mut self) -> _LSIRDYIEW {
            _LSIRDYIEW { w: self }
        }
        # [ doc = "Bit 9 - LSE Ready Interrupt Enable" ]
        # [ inline ( always ) ]
        pub fn lserdyie(&mut self) -> _LSERDYIEW {
            _LSERDYIEW { w: self }
        }
        # [ doc = "Bit 10 - HSI Ready Interrupt Enable" ]
        # [ inline ( always ) ]
        pub fn hsirdyie(&mut self) -> _HSIRDYIEW {
            _HSIRDYIEW { w: self }
        }
        # [ doc = "Bit 11 - HSE Ready Interrupt Enable" ]
        # [ inline ( always ) ]
        pub fn hserdyie(&mut self) -> _HSERDYIEW {
            _HSERDYIEW { w: self }
        }
        # [ doc = "Bit 12 - PLL Ready Interrupt Enable" ]
        # [ inline ( always ) ]
        pub fn pllrdyie(&mut self) -> _PLLRDYIEW {
            _PLLRDYIEW { w: self }
        }
        # [ doc = "Bit 16 - LSI Ready Interrupt Clear" ]
        # [ inline ( always ) ]
        pub fn lsirdyc(&mut self) -> _LSIRDYCW {
            _LSIRDYCW { w: self }
        }
        # [ doc = "Bit 17 - LSE Ready Interrupt Clear" ]
        # [ inline ( always ) ]
        pub fn lserdyc(&mut self) -> _LSERDYCW {
            _LSERDYCW { w: self }
        }
        # [ doc = "Bit 18 - HSI Ready Interrupt Clear" ]
        # [ inline ( always ) ]
        pub fn hsirdyc(&mut self) -> _HSIRDYCW {
            _HSIRDYCW { w: self }
        }
        # [ doc = "Bit 19 - HSE Ready Interrupt Clear" ]
        # [ inline ( always ) ]
        pub fn hserdyc(&mut self) -> _HSERDYCW {
            _HSERDYCW { w: self }
        }
        # [ doc = "Bit 20 - PLL Ready Interrupt Clear" ]
        # [ inline ( always ) ]
        pub fn pllrdyc(&mut self) -> _PLLRDYCW {
            _PLLRDYCW { w: self }
        }
        # [ doc = "Bit 23 - Clock security system interrupt clear" ]
        # [ inline ( always ) ]
        pub fn cssc(&mut self) -> _CSSCW {
            _CSSCW { w: self }
        }
    }
}
# [ doc = "APB2 peripheral reset register (RCC_APB2RSTR)" ]
pub struct APB2RSTR {
    register: VolatileCell<u32>,
}
# [ doc = "APB2 peripheral reset register (RCC_APB2RSTR)" ]
pub mod apb2rstr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::APB2RSTR {
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
    pub struct AFIORSTR {
        bits: bool,
    }
    impl AFIORSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct IOPARSTR {
        bits: bool,
    }
    impl IOPARSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct IOPBRSTR {
        bits: bool,
    }
    impl IOPBRSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct IOPCRSTR {
        bits: bool,
    }
    impl IOPCRSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct IOPDRSTR {
        bits: bool,
    }
    impl IOPDRSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct IOPERSTR {
        bits: bool,
    }
    impl IOPERSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct IOPFRSTR {
        bits: bool,
    }
    impl IOPFRSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct IOPGRSTR {
        bits: bool,
    }
    impl IOPGRSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct ADC1RSTR {
        bits: bool,
    }
    impl ADC1RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct ADC2RSTR {
        bits: bool,
    }
    impl ADC2RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct TIM1RSTR {
        bits: bool,
    }
    impl TIM1RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct SPI1RSTR {
        bits: bool,
    }
    impl SPI1RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct TIM8RSTR {
        bits: bool,
    }
    impl TIM8RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct USART1RSTR {
        bits: bool,
    }
    impl USART1RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct ADC3RSTR {
        bits: bool,
    }
    impl ADC3RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct TIM9RSTR {
        bits: bool,
    }
    impl TIM9RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct TIM10RSTR {
        bits: bool,
    }
    impl TIM10RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct TIM11RSTR {
        bits: bool,
    }
    impl TIM11RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _AFIORSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _AFIORSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _IOPARSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IOPARSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _IOPBRSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IOPBRSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _IOPCRSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IOPCRSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _IOPDRSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IOPDRSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _IOPERSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IOPERSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _IOPFRSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IOPFRSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _IOPGRSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IOPGRSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ADC1RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADC1RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ADC2RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADC2RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TIM1RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM1RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SPI1RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SPI1RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TIM8RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM8RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _USART1RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _USART1RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ADC3RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADC3RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TIM9RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM9RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TIM10RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM10RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TIM11RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM11RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Alternate function I/O reset" ]
        # [ inline ( always ) ]
        pub fn afiorst(&self) -> AFIORSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            AFIORSTR { bits }
        }
        # [ doc = "Bit 2 - IO port A reset" ]
        # [ inline ( always ) ]
        pub fn ioparst(&self) -> IOPARSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IOPARSTR { bits }
        }
        # [ doc = "Bit 3 - IO port B reset" ]
        # [ inline ( always ) ]
        pub fn iopbrst(&self) -> IOPBRSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IOPBRSTR { bits }
        }
        # [ doc = "Bit 4 - IO port C reset" ]
        # [ inline ( always ) ]
        pub fn iopcrst(&self) -> IOPCRSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IOPCRSTR { bits }
        }
        # [ doc = "Bit 5 - IO port D reset" ]
        # [ inline ( always ) ]
        pub fn iopdrst(&self) -> IOPDRSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IOPDRSTR { bits }
        }
        # [ doc = "Bit 6 - IO port E reset" ]
        # [ inline ( always ) ]
        pub fn ioperst(&self) -> IOPERSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IOPERSTR { bits }
        }
        # [ doc = "Bit 7 - IO port F reset" ]
        # [ inline ( always ) ]
        pub fn iopfrst(&self) -> IOPFRSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IOPFRSTR { bits }
        }
        # [ doc = "Bit 8 - IO port G reset" ]
        # [ inline ( always ) ]
        pub fn iopgrst(&self) -> IOPGRSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IOPGRSTR { bits }
        }
        # [ doc = "Bit 9 - ADC 1 interface reset" ]
        # [ inline ( always ) ]
        pub fn adc1rst(&self) -> ADC1RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ADC1RSTR { bits }
        }
        # [ doc = "Bit 10 - ADC 2 interface reset" ]
        # [ inline ( always ) ]
        pub fn adc2rst(&self) -> ADC2RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ADC2RSTR { bits }
        }
        # [ doc = "Bit 11 - TIM1 timer reset" ]
        # [ inline ( always ) ]
        pub fn tim1rst(&self) -> TIM1RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIM1RSTR { bits }
        }
        # [ doc = "Bit 12 - SPI 1 reset" ]
        # [ inline ( always ) ]
        pub fn spi1rst(&self) -> SPI1RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SPI1RSTR { bits }
        }
        # [ doc = "Bit 13 - TIM8 timer reset" ]
        # [ inline ( always ) ]
        pub fn tim8rst(&self) -> TIM8RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIM8RSTR { bits }
        }
        # [ doc = "Bit 14 - USART1 reset" ]
        # [ inline ( always ) ]
        pub fn usart1rst(&self) -> USART1RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USART1RSTR { bits }
        }
        # [ doc = "Bit 15 - ADC 3 interface reset" ]
        # [ inline ( always ) ]
        pub fn adc3rst(&self) -> ADC3RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ADC3RSTR { bits }
        }
        # [ doc = "Bit 19 - TIM9 timer reset" ]
        # [ inline ( always ) ]
        pub fn tim9rst(&self) -> TIM9RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIM9RSTR { bits }
        }
        # [ doc = "Bit 20 - TIM10 timer reset" ]
        # [ inline ( always ) ]
        pub fn tim10rst(&self) -> TIM10RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIM10RSTR { bits }
        }
        # [ doc = "Bit 21 - TIM11 timer reset" ]
        # [ inline ( always ) ]
        pub fn tim11rst(&self) -> TIM11RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 21;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIM11RSTR { bits }
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
        # [ doc = "Bit 0 - Alternate function I/O reset" ]
        # [ inline ( always ) ]
        pub fn afiorst(&mut self) -> _AFIORSTW {
            _AFIORSTW { w: self }
        }
        # [ doc = "Bit 2 - IO port A reset" ]
        # [ inline ( always ) ]
        pub fn ioparst(&mut self) -> _IOPARSTW {
            _IOPARSTW { w: self }
        }
        # [ doc = "Bit 3 - IO port B reset" ]
        # [ inline ( always ) ]
        pub fn iopbrst(&mut self) -> _IOPBRSTW {
            _IOPBRSTW { w: self }
        }
        # [ doc = "Bit 4 - IO port C reset" ]
        # [ inline ( always ) ]
        pub fn iopcrst(&mut self) -> _IOPCRSTW {
            _IOPCRSTW { w: self }
        }
        # [ doc = "Bit 5 - IO port D reset" ]
        # [ inline ( always ) ]
        pub fn iopdrst(&mut self) -> _IOPDRSTW {
            _IOPDRSTW { w: self }
        }
        # [ doc = "Bit 6 - IO port E reset" ]
        # [ inline ( always ) ]
        pub fn ioperst(&mut self) -> _IOPERSTW {
            _IOPERSTW { w: self }
        }
        # [ doc = "Bit 7 - IO port F reset" ]
        # [ inline ( always ) ]
        pub fn iopfrst(&mut self) -> _IOPFRSTW {
            _IOPFRSTW { w: self }
        }
        # [ doc = "Bit 8 - IO port G reset" ]
        # [ inline ( always ) ]
        pub fn iopgrst(&mut self) -> _IOPGRSTW {
            _IOPGRSTW { w: self }
        }
        # [ doc = "Bit 9 - ADC 1 interface reset" ]
        # [ inline ( always ) ]
        pub fn adc1rst(&mut self) -> _ADC1RSTW {
            _ADC1RSTW { w: self }
        }
        # [ doc = "Bit 10 - ADC 2 interface reset" ]
        # [ inline ( always ) ]
        pub fn adc2rst(&mut self) -> _ADC2RSTW {
            _ADC2RSTW { w: self }
        }
        # [ doc = "Bit 11 - TIM1 timer reset" ]
        # [ inline ( always ) ]
        pub fn tim1rst(&mut self) -> _TIM1RSTW {
            _TIM1RSTW { w: self }
        }
        # [ doc = "Bit 12 - SPI 1 reset" ]
        # [ inline ( always ) ]
        pub fn spi1rst(&mut self) -> _SPI1RSTW {
            _SPI1RSTW { w: self }
        }
        # [ doc = "Bit 13 - TIM8 timer reset" ]
        # [ inline ( always ) ]
        pub fn tim8rst(&mut self) -> _TIM8RSTW {
            _TIM8RSTW { w: self }
        }
        # [ doc = "Bit 14 - USART1 reset" ]
        # [ inline ( always ) ]
        pub fn usart1rst(&mut self) -> _USART1RSTW {
            _USART1RSTW { w: self }
        }
        # [ doc = "Bit 15 - ADC 3 interface reset" ]
        # [ inline ( always ) ]
        pub fn adc3rst(&mut self) -> _ADC3RSTW {
            _ADC3RSTW { w: self }
        }
        # [ doc = "Bit 19 - TIM9 timer reset" ]
        # [ inline ( always ) ]
        pub fn tim9rst(&mut self) -> _TIM9RSTW {
            _TIM9RSTW { w: self }
        }
        # [ doc = "Bit 20 - TIM10 timer reset" ]
        # [ inline ( always ) ]
        pub fn tim10rst(&mut self) -> _TIM10RSTW {
            _TIM10RSTW { w: self }
        }
        # [ doc = "Bit 21 - TIM11 timer reset" ]
        # [ inline ( always ) ]
        pub fn tim11rst(&mut self) -> _TIM11RSTW {
            _TIM11RSTW { w: self }
        }
    }
}
# [ doc = "APB1 peripheral reset register (RCC_APB1RSTR)" ]
pub struct APB1RSTR {
    register: VolatileCell<u32>,
}
# [ doc = "APB1 peripheral reset register (RCC_APB1RSTR)" ]
pub mod apb1rstr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::APB1RSTR {
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
    pub struct TIM2RSTR {
        bits: bool,
    }
    impl TIM2RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct TIM3RSTR {
        bits: bool,
    }
    impl TIM3RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct TIM4RSTR {
        bits: bool,
    }
    impl TIM4RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct TIM5RSTR {
        bits: bool,
    }
    impl TIM5RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct TIM6RSTR {
        bits: bool,
    }
    impl TIM6RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct TIM7RSTR {
        bits: bool,
    }
    impl TIM7RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct TIM12RSTR {
        bits: bool,
    }
    impl TIM12RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct TIM13RSTR {
        bits: bool,
    }
    impl TIM13RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct TIM14RSTR {
        bits: bool,
    }
    impl TIM14RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct WWDGRSTR {
        bits: bool,
    }
    impl WWDGRSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct SPI2RSTR {
        bits: bool,
    }
    impl SPI2RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct SPI3RSTR {
        bits: bool,
    }
    impl SPI3RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct USART2RSTR {
        bits: bool,
    }
    impl USART2RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct USART3RSTR {
        bits: bool,
    }
    impl USART3RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct UART4RSTR {
        bits: bool,
    }
    impl UART4RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct UART5RSTR {
        bits: bool,
    }
    impl UART5RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct I2C1RSTR {
        bits: bool,
    }
    impl I2C1RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct I2C2RSTR {
        bits: bool,
    }
    impl I2C2RSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct USBRSTR {
        bits: bool,
    }
    impl USBRSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct CANRSTR {
        bits: bool,
    }
    impl CANRSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct BKPRSTR {
        bits: bool,
    }
    impl BKPRSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct PWRRSTR {
        bits: bool,
    }
    impl PWRRSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct DACRSTR {
        bits: bool,
    }
    impl DACRSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _TIM2RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM2RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TIM3RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM3RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TIM4RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM4RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TIM5RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM5RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TIM6RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM6RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TIM7RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM7RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TIM12RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM12RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TIM13RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM13RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TIM14RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM14RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _WWDGRSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WWDGRSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SPI2RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SPI2RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SPI3RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SPI3RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _USART2RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _USART2RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _USART3RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _USART3RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _UART4RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _UART4RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _UART5RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _UART5RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _I2C1RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _I2C1RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _I2C2RSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _I2C2RSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _USBRSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _USBRSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CANRSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CANRSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _BKPRSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BKPRSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWRRSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWRRSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    # [ doc = r" Proxy" ]
    pub struct _DACRSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DACRSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
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
        # [ doc = "Bit 0 - Timer 2 reset" ]
        # [ inline ( always ) ]
        pub fn tim2rst(&self) -> TIM2RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIM2RSTR { bits }
        }
        # [ doc = "Bit 1 - Timer 3 reset" ]
        # [ inline ( always ) ]
        pub fn tim3rst(&self) -> TIM3RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIM3RSTR { bits }
        }
        # [ doc = "Bit 2 - Timer 4 reset" ]
        # [ inline ( always ) ]
        pub fn tim4rst(&self) -> TIM4RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIM4RSTR { bits }
        }
        # [ doc = "Bit 3 - Timer 5 reset" ]
        # [ inline ( always ) ]
        pub fn tim5rst(&self) -> TIM5RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIM5RSTR { bits }
        }
        # [ doc = "Bit 4 - Timer 6 reset" ]
        # [ inline ( always ) ]
        pub fn tim6rst(&self) -> TIM6RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIM6RSTR { bits }
        }
        # [ doc = "Bit 5 - Timer 7 reset" ]
        # [ inline ( always ) ]
        pub fn tim7rst(&self) -> TIM7RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIM7RSTR { bits }
        }
        # [ doc = "Bit 6 - Timer 12 reset" ]
        # [ inline ( always ) ]
        pub fn tim12rst(&self) -> TIM12RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIM12RSTR { bits }
        }
        # [ doc = "Bit 7 - Timer 13 reset" ]
        # [ inline ( always ) ]
        pub fn tim13rst(&self) -> TIM13RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIM13RSTR { bits }
        }
        # [ doc = "Bit 8 - Timer 14 reset" ]
        # [ inline ( always ) ]
        pub fn tim14rst(&self) -> TIM14RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIM14RSTR { bits }
        }
        # [ doc = "Bit 11 - Window watchdog reset" ]
        # [ inline ( always ) ]
        pub fn wwdgrst(&self) -> WWDGRSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WWDGRSTR { bits }
        }
        # [ doc = "Bit 14 - SPI2 reset" ]
        # [ inline ( always ) ]
        pub fn spi2rst(&self) -> SPI2RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SPI2RSTR { bits }
        }
        # [ doc = "Bit 15 - SPI3 reset" ]
        # [ inline ( always ) ]
        pub fn spi3rst(&self) -> SPI3RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SPI3RSTR { bits }
        }
        # [ doc = "Bit 17 - USART 2 reset" ]
        # [ inline ( always ) ]
        pub fn usart2rst(&self) -> USART2RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USART2RSTR { bits }
        }
        # [ doc = "Bit 18 - USART 3 reset" ]
        # [ inline ( always ) ]
        pub fn usart3rst(&self) -> USART3RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USART3RSTR { bits }
        }
        # [ doc = "Bit 19 - UART 4 reset" ]
        # [ inline ( always ) ]
        pub fn uart4rst(&self) -> UART4RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            UART4RSTR { bits }
        }
        # [ doc = "Bit 20 - UART 5 reset" ]
        # [ inline ( always ) ]
        pub fn uart5rst(&self) -> UART5RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            UART5RSTR { bits }
        }
        # [ doc = "Bit 21 - I2C1 reset" ]
        # [ inline ( always ) ]
        pub fn i2c1rst(&self) -> I2C1RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 21;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            I2C1RSTR { bits }
        }
        # [ doc = "Bit 22 - I2C2 reset" ]
        # [ inline ( always ) ]
        pub fn i2c2rst(&self) -> I2C2RSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 22;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            I2C2RSTR { bits }
        }
        # [ doc = "Bit 23 - USB reset" ]
        # [ inline ( always ) ]
        pub fn usbrst(&self) -> USBRSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 23;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USBRSTR { bits }
        }
        # [ doc = "Bit 25 - CAN reset" ]
        # [ inline ( always ) ]
        pub fn canrst(&self) -> CANRSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CANRSTR { bits }
        }
        # [ doc = "Bit 27 - Backup interface reset" ]
        # [ inline ( always ) ]
        pub fn bkprst(&self) -> BKPRSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 27;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BKPRSTR { bits }
        }
        # [ doc = "Bit 28 - Power interface reset" ]
        # [ inline ( always ) ]
        pub fn pwrrst(&self) -> PWRRSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PWRRSTR { bits }
        }
        # [ doc = "Bit 29 - DAC interface reset" ]
        # [ inline ( always ) ]
        pub fn dacrst(&self) -> DACRSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 29;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DACRSTR { bits }
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
        # [ doc = "Bit 0 - Timer 2 reset" ]
        # [ inline ( always ) ]
        pub fn tim2rst(&mut self) -> _TIM2RSTW {
            _TIM2RSTW { w: self }
        }
        # [ doc = "Bit 1 - Timer 3 reset" ]
        # [ inline ( always ) ]
        pub fn tim3rst(&mut self) -> _TIM3RSTW {
            _TIM3RSTW { w: self }
        }
        # [ doc = "Bit 2 - Timer 4 reset" ]
        # [ inline ( always ) ]
        pub fn tim4rst(&mut self) -> _TIM4RSTW {
            _TIM4RSTW { w: self }
        }
        # [ doc = "Bit 3 - Timer 5 reset" ]
        # [ inline ( always ) ]
        pub fn tim5rst(&mut self) -> _TIM5RSTW {
            _TIM5RSTW { w: self }
        }
        # [ doc = "Bit 4 - Timer 6 reset" ]
        # [ inline ( always ) ]
        pub fn tim6rst(&mut self) -> _TIM6RSTW {
            _TIM6RSTW { w: self }
        }
        # [ doc = "Bit 5 - Timer 7 reset" ]
        # [ inline ( always ) ]
        pub fn tim7rst(&mut self) -> _TIM7RSTW {
            _TIM7RSTW { w: self }
        }
        # [ doc = "Bit 6 - Timer 12 reset" ]
        # [ inline ( always ) ]
        pub fn tim12rst(&mut self) -> _TIM12RSTW {
            _TIM12RSTW { w: self }
        }
        # [ doc = "Bit 7 - Timer 13 reset" ]
        # [ inline ( always ) ]
        pub fn tim13rst(&mut self) -> _TIM13RSTW {
            _TIM13RSTW { w: self }
        }
        # [ doc = "Bit 8 - Timer 14 reset" ]
        # [ inline ( always ) ]
        pub fn tim14rst(&mut self) -> _TIM14RSTW {
            _TIM14RSTW { w: self }
        }
        # [ doc = "Bit 11 - Window watchdog reset" ]
        # [ inline ( always ) ]
        pub fn wwdgrst(&mut self) -> _WWDGRSTW {
            _WWDGRSTW { w: self }
        }
        # [ doc = "Bit 14 - SPI2 reset" ]
        # [ inline ( always ) ]
        pub fn spi2rst(&mut self) -> _SPI2RSTW {
            _SPI2RSTW { w: self }
        }
        # [ doc = "Bit 15 - SPI3 reset" ]
        # [ inline ( always ) ]
        pub fn spi3rst(&mut self) -> _SPI3RSTW {
            _SPI3RSTW { w: self }
        }
        # [ doc = "Bit 17 - USART 2 reset" ]
        # [ inline ( always ) ]
        pub fn usart2rst(&mut self) -> _USART2RSTW {
            _USART2RSTW { w: self }
        }
        # [ doc = "Bit 18 - USART 3 reset" ]
        # [ inline ( always ) ]
        pub fn usart3rst(&mut self) -> _USART3RSTW {
            _USART3RSTW { w: self }
        }
        # [ doc = "Bit 19 - UART 4 reset" ]
        # [ inline ( always ) ]
        pub fn uart4rst(&mut self) -> _UART4RSTW {
            _UART4RSTW { w: self }
        }
        # [ doc = "Bit 20 - UART 5 reset" ]
        # [ inline ( always ) ]
        pub fn uart5rst(&mut self) -> _UART5RSTW {
            _UART5RSTW { w: self }
        }
        # [ doc = "Bit 21 - I2C1 reset" ]
        # [ inline ( always ) ]
        pub fn i2c1rst(&mut self) -> _I2C1RSTW {
            _I2C1RSTW { w: self }
        }
        # [ doc = "Bit 22 - I2C2 reset" ]
        # [ inline ( always ) ]
        pub fn i2c2rst(&mut self) -> _I2C2RSTW {
            _I2C2RSTW { w: self }
        }
        # [ doc = "Bit 23 - USB reset" ]
        # [ inline ( always ) ]
        pub fn usbrst(&mut self) -> _USBRSTW {
            _USBRSTW { w: self }
        }
        # [ doc = "Bit 25 - CAN reset" ]
        # [ inline ( always ) ]
        pub fn canrst(&mut self) -> _CANRSTW {
            _CANRSTW { w: self }
        }
        # [ doc = "Bit 27 - Backup interface reset" ]
        # [ inline ( always ) ]
        pub fn bkprst(&mut self) -> _BKPRSTW {
            _BKPRSTW { w: self }
        }
        # [ doc = "Bit 28 - Power interface reset" ]
        # [ inline ( always ) ]
        pub fn pwrrst(&mut self) -> _PWRRSTW {
            _PWRRSTW { w: self }
        }
        # [ doc = "Bit 29 - DAC interface reset" ]
        # [ inline ( always ) ]
        pub fn dacrst(&mut self) -> _DACRSTW {
            _DACRSTW { w: self }
        }
    }
}
# [ doc = "AHB Peripheral Clock enable register (RCC_AHBENR)" ]
pub struct AHBENR {
    register: VolatileCell<u32>,
}
# [ doc = "AHB Peripheral Clock enable register (RCC_AHBENR)" ]
pub mod ahbenr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::AHBENR {
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
    # [ doc = "Possible values of the field `DMA1EN`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum DMA1ENR {
        # [ doc = "Disabled." ]
        DISABLED,
        # [ doc = "Enabled." ]
        ENABLED,
    }
    impl DMA1ENR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                DMA1ENR::DISABLED => false,
                DMA1ENR::ENABLED => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> DMA1ENR {
            match value {
                false => DMA1ENR::DISABLED,
                true => DMA1ENR::ENABLED,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == DMA1ENR::DISABLED
        }
        # [ doc = "Checks if the value of the field is `ENABLED`" ]
        # [ inline ( always ) ]
        pub fn is_enabled(&self) -> bool {
            *self == DMA1ENR::ENABLED
        }
    }
    # [ doc = "Possible values of the field `DMA2EN`" ]
    pub type DMA2ENR = DMA1ENR;
    # [ doc = "Possible values of the field `SRAMEN`" ]
    pub type SRAMENR = DMA1ENR;
    # [ doc = "Possible values of the field `FLITFEN`" ]
    pub type FLITFENR = DMA1ENR;
    # [ doc = "Possible values of the field `CRCEN`" ]
    pub type CRCENR = DMA1ENR;
    # [ doc = "Possible values of the field `FSMCEN`" ]
    pub type FSMCENR = DMA1ENR;
    # [ doc = "Possible values of the field `SDIOEN`" ]
    pub type SDIOENR = DMA1ENR;
    # [ doc = "Values that can be written to the field `DMA1EN`" ]
    pub enum DMA1ENW {
        # [ doc = "Disabled." ]
        DISABLED,
        # [ doc = "Enabled." ]
        ENABLED,
    }
    impl DMA1ENW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                DMA1ENW::DISABLED => false,
                DMA1ENW::ENABLED => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DMA1ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DMA1ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: DMA1ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `DMA2EN`" ]
    pub type DMA2ENW = DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _DMA2ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DMA2ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: DMA2ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `SRAMEN`" ]
    pub type SRAMENW = DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _SRAMENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SRAMENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: SRAMENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `FLITFEN`" ]
    pub type FLITFENW = DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _FLITFENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FLITFENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: FLITFENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `CRCEN`" ]
    pub type CRCENW = DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _CRCENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CRCENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CRCENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `FSMCEN`" ]
    pub type FSMCENW = DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _FSMCENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FSMCENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: FSMCENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `SDIOEN`" ]
    pub type SDIOENW = DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _SDIOENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SDIOENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: SDIOENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(DMA1ENW::ENABLED)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - DMA1 clock enable" ]
        # [ inline ( always ) ]
        pub fn dma1en(&self) -> DMA1ENR {
            DMA1ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 1 - DMA2 clock enable" ]
        # [ inline ( always ) ]
        pub fn dma2en(&self) -> DMA2ENR {
            DMA2ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 2 - SRAM interface clock enable" ]
        # [ inline ( always ) ]
        pub fn sramen(&self) -> SRAMENR {
            SRAMENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 4 - FLITF clock enable" ]
        # [ inline ( always ) ]
        pub fn flitfen(&self) -> FLITFENR {
            FLITFENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 6 - CRC clock enable" ]
        # [ inline ( always ) ]
        pub fn crcen(&self) -> CRCENR {
            CRCENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 8 - FSMC clock enable" ]
        # [ inline ( always ) ]
        pub fn fsmcen(&self) -> FSMCENR {
            FSMCENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 10 - SDIO clock enable" ]
        # [ inline ( always ) ]
        pub fn sdioen(&self) -> SDIOENR {
            SDIOENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 20 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 0 - DMA1 clock enable" ]
        # [ inline ( always ) ]
        pub fn dma1en(&mut self) -> _DMA1ENW {
            _DMA1ENW { w: self }
        }
        # [ doc = "Bit 1 - DMA2 clock enable" ]
        # [ inline ( always ) ]
        pub fn dma2en(&mut self) -> _DMA2ENW {
            _DMA2ENW { w: self }
        }
        # [ doc = "Bit 2 - SRAM interface clock enable" ]
        # [ inline ( always ) ]
        pub fn sramen(&mut self) -> _SRAMENW {
            _SRAMENW { w: self }
        }
        # [ doc = "Bit 4 - FLITF clock enable" ]
        # [ inline ( always ) ]
        pub fn flitfen(&mut self) -> _FLITFENW {
            _FLITFENW { w: self }
        }
        # [ doc = "Bit 6 - CRC clock enable" ]
        # [ inline ( always ) ]
        pub fn crcen(&mut self) -> _CRCENW {
            _CRCENW { w: self }
        }
        # [ doc = "Bit 8 - FSMC clock enable" ]
        # [ inline ( always ) ]
        pub fn fsmcen(&mut self) -> _FSMCENW {
            _FSMCENW { w: self }
        }
        # [ doc = "Bit 10 - SDIO clock enable" ]
        # [ inline ( always ) ]
        pub fn sdioen(&mut self) -> _SDIOENW {
            _SDIOENW { w: self }
        }
    }
}
# [ doc = "APB2 peripheral clock enable register (RCC_APB2ENR)" ]
pub struct APB2ENR {
    register: VolatileCell<u32>,
}
# [ doc = "APB2 peripheral clock enable register (RCC_APB2ENR)" ]
pub mod apb2enr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::APB2ENR {
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
    # [ doc = "Possible values of the field `AFIOEN`" ]
    pub type AFIOENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `IOPAEN`" ]
    pub type IOPAENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `IOPBEN`" ]
    pub type IOPBENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `IOPCEN`" ]
    pub type IOPCENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `IOPDEN`" ]
    pub type IOPDENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `IOPEEN`" ]
    pub type IOPEENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `IOPFEN`" ]
    pub type IOPFENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `IOPGEN`" ]
    pub type IOPGENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `ADC1EN`" ]
    pub type ADC1ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `ADC2EN`" ]
    pub type ADC2ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `TIM1EN`" ]
    pub type TIM1ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `SPI1EN`" ]
    pub type SPI1ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `TIM8EN`" ]
    pub type TIM8ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `USART1EN`" ]
    pub type USART1ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `ADC3EN`" ]
    pub type ADC3ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `TIM9EN`" ]
    pub type TIM9ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `TIM10EN`" ]
    pub type TIM10ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `TIM11EN`" ]
    pub type TIM11ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Values that can be written to the field `AFIOEN`" ]
    pub type AFIOENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _AFIOENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _AFIOENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: AFIOENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `IOPAEN`" ]
    pub type IOPAENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _IOPAENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IOPAENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: IOPAENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `IOPBEN`" ]
    pub type IOPBENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _IOPBENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IOPBENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: IOPBENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `IOPCEN`" ]
    pub type IOPCENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _IOPCENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IOPCENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: IOPCENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `IOPDEN`" ]
    pub type IOPDENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _IOPDENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IOPDENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: IOPDENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `IOPEEN`" ]
    pub type IOPEENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _IOPEENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IOPEENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: IOPEENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `IOPFEN`" ]
    pub type IOPFENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _IOPFENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IOPFENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: IOPFENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `IOPGEN`" ]
    pub type IOPGENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _IOPGENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IOPGENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: IOPGENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `ADC1EN`" ]
    pub type ADC1ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _ADC1ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADC1ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: ADC1ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `ADC2EN`" ]
    pub type ADC2ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _ADC2ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADC2ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: ADC2ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `TIM1EN`" ]
    pub type TIM1ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _TIM1ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM1ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: TIM1ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `SPI1EN`" ]
    pub type SPI1ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _SPI1ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SPI1ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: SPI1ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `TIM8EN`" ]
    pub type TIM8ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _TIM8ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM8ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: TIM8ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `USART1EN`" ]
    pub type USART1ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _USART1ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _USART1ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: USART1ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `ADC3EN`" ]
    pub type ADC3ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _ADC3ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADC3ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: ADC3ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `TIM9EN`" ]
    pub type TIM9ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _TIM9ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM9ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: TIM9ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `TIM10EN`" ]
    pub type TIM10ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _TIM10ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM10ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: TIM10ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `TIM11EN`" ]
    pub type TIM11ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _TIM11ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM11ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: TIM11ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Alternate function I/O clock enable" ]
        # [ inline ( always ) ]
        pub fn afioen(&self) -> AFIOENR {
            AFIOENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 2 - I/O port A clock enable" ]
        # [ inline ( always ) ]
        pub fn iopaen(&self) -> IOPAENR {
            IOPAENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 3 - I/O port B clock enable" ]
        # [ inline ( always ) ]
        pub fn iopben(&self) -> IOPBENR {
            IOPBENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 4 - I/O port C clock enable" ]
        # [ inline ( always ) ]
        pub fn iopcen(&self) -> IOPCENR {
            IOPCENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 5 - I/O port D clock enable" ]
        # [ inline ( always ) ]
        pub fn iopden(&self) -> IOPDENR {
            IOPDENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 6 - I/O port E clock enable" ]
        # [ inline ( always ) ]
        pub fn iopeen(&self) -> IOPEENR {
            IOPEENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 7 - I/O port F clock enable" ]
        # [ inline ( always ) ]
        pub fn iopfen(&self) -> IOPFENR {
            IOPFENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 8 - I/O port G clock enable" ]
        # [ inline ( always ) ]
        pub fn iopgen(&self) -> IOPGENR {
            IOPGENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 9 - ADC 1 interface clock enable" ]
        # [ inline ( always ) ]
        pub fn adc1en(&self) -> ADC1ENR {
            ADC1ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 10 - ADC 2 interface clock enable" ]
        # [ inline ( always ) ]
        pub fn adc2en(&self) -> ADC2ENR {
            ADC2ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 11 - TIM1 Timer clock enable" ]
        # [ inline ( always ) ]
        pub fn tim1en(&self) -> TIM1ENR {
            TIM1ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 12 - SPI 1 clock enable" ]
        # [ inline ( always ) ]
        pub fn spi1en(&self) -> SPI1ENR {
            SPI1ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 13 - TIM8 Timer clock enable" ]
        # [ inline ( always ) ]
        pub fn tim8en(&self) -> TIM8ENR {
            TIM8ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 14 - USART1 clock enable" ]
        # [ inline ( always ) ]
        pub fn usart1en(&self) -> USART1ENR {
            USART1ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 15 - ADC3 interface clock enable" ]
        # [ inline ( always ) ]
        pub fn adc3en(&self) -> ADC3ENR {
            ADC3ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 19 - TIM9 Timer clock enable" ]
        # [ inline ( always ) ]
        pub fn tim9en(&self) -> TIM9ENR {
            TIM9ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 20 - TIM10 Timer clock enable" ]
        # [ inline ( always ) ]
        pub fn tim10en(&self) -> TIM10ENR {
            TIM10ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 21 - TIM11 Timer clock enable" ]
        # [ inline ( always ) ]
        pub fn tim11en(&self) -> TIM11ENR {
            TIM11ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 21;
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
        # [ doc = "Bit 0 - Alternate function I/O clock enable" ]
        # [ inline ( always ) ]
        pub fn afioen(&mut self) -> _AFIOENW {
            _AFIOENW { w: self }
        }
        # [ doc = "Bit 2 - I/O port A clock enable" ]
        # [ inline ( always ) ]
        pub fn iopaen(&mut self) -> _IOPAENW {
            _IOPAENW { w: self }
        }
        # [ doc = "Bit 3 - I/O port B clock enable" ]
        # [ inline ( always ) ]
        pub fn iopben(&mut self) -> _IOPBENW {
            _IOPBENW { w: self }
        }
        # [ doc = "Bit 4 - I/O port C clock enable" ]
        # [ inline ( always ) ]
        pub fn iopcen(&mut self) -> _IOPCENW {
            _IOPCENW { w: self }
        }
        # [ doc = "Bit 5 - I/O port D clock enable" ]
        # [ inline ( always ) ]
        pub fn iopden(&mut self) -> _IOPDENW {
            _IOPDENW { w: self }
        }
        # [ doc = "Bit 6 - I/O port E clock enable" ]
        # [ inline ( always ) ]
        pub fn iopeen(&mut self) -> _IOPEENW {
            _IOPEENW { w: self }
        }
        # [ doc = "Bit 7 - I/O port F clock enable" ]
        # [ inline ( always ) ]
        pub fn iopfen(&mut self) -> _IOPFENW {
            _IOPFENW { w: self }
        }
        # [ doc = "Bit 8 - I/O port G clock enable" ]
        # [ inline ( always ) ]
        pub fn iopgen(&mut self) -> _IOPGENW {
            _IOPGENW { w: self }
        }
        # [ doc = "Bit 9 - ADC 1 interface clock enable" ]
        # [ inline ( always ) ]
        pub fn adc1en(&mut self) -> _ADC1ENW {
            _ADC1ENW { w: self }
        }
        # [ doc = "Bit 10 - ADC 2 interface clock enable" ]
        # [ inline ( always ) ]
        pub fn adc2en(&mut self) -> _ADC2ENW {
            _ADC2ENW { w: self }
        }
        # [ doc = "Bit 11 - TIM1 Timer clock enable" ]
        # [ inline ( always ) ]
        pub fn tim1en(&mut self) -> _TIM1ENW {
            _TIM1ENW { w: self }
        }
        # [ doc = "Bit 12 - SPI 1 clock enable" ]
        # [ inline ( always ) ]
        pub fn spi1en(&mut self) -> _SPI1ENW {
            _SPI1ENW { w: self }
        }
        # [ doc = "Bit 13 - TIM8 Timer clock enable" ]
        # [ inline ( always ) ]
        pub fn tim8en(&mut self) -> _TIM8ENW {
            _TIM8ENW { w: self }
        }
        # [ doc = "Bit 14 - USART1 clock enable" ]
        # [ inline ( always ) ]
        pub fn usart1en(&mut self) -> _USART1ENW {
            _USART1ENW { w: self }
        }
        # [ doc = "Bit 15 - ADC3 interface clock enable" ]
        # [ inline ( always ) ]
        pub fn adc3en(&mut self) -> _ADC3ENW {
            _ADC3ENW { w: self }
        }
        # [ doc = "Bit 19 - TIM9 Timer clock enable" ]
        # [ inline ( always ) ]
        pub fn tim9en(&mut self) -> _TIM9ENW {
            _TIM9ENW { w: self }
        }
        # [ doc = "Bit 20 - TIM10 Timer clock enable" ]
        # [ inline ( always ) ]
        pub fn tim10en(&mut self) -> _TIM10ENW {
            _TIM10ENW { w: self }
        }
        # [ doc = "Bit 21 - TIM11 Timer clock enable" ]
        # [ inline ( always ) ]
        pub fn tim11en(&mut self) -> _TIM11ENW {
            _TIM11ENW { w: self }
        }
    }
}
# [ doc = "APB1 peripheral clock enable register (RCC_APB1ENR)" ]
pub struct APB1ENR {
    register: VolatileCell<u32>,
}
# [ doc = "APB1 peripheral clock enable register (RCC_APB1ENR)" ]
pub mod apb1enr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::APB1ENR {
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
    # [ doc = "Possible values of the field `TIM2EN`" ]
    pub type TIM2ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `TIM3EN`" ]
    pub type TIM3ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `TIM4EN`" ]
    pub type TIM4ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `TIM5EN`" ]
    pub type TIM5ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `TIM6EN`" ]
    pub type TIM6ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `TIM7EN`" ]
    pub type TIM7ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `TIM12EN`" ]
    pub type TIM12ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `TIM13EN`" ]
    pub type TIM13ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `TIM14EN`" ]
    pub type TIM14ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `WWDGEN`" ]
    pub type WWDGENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `SPI2EN`" ]
    pub type SPI2ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `SPI3EN`" ]
    pub type SPI3ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `USART2EN`" ]
    pub type USART2ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `USART3EN`" ]
    pub type USART3ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `UART4EN`" ]
    pub type UART4ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `UART5EN`" ]
    pub type UART5ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `I2C1EN`" ]
    pub type I2C1ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `I2C2EN`" ]
    pub type I2C2ENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `USBEN`" ]
    pub type USBENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `CANEN`" ]
    pub type CANENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `BKPEN`" ]
    pub type BKPENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `PWREN`" ]
    pub type PWRENR = super::ahbenr::DMA1ENR;
    # [ doc = "Possible values of the field `DACEN`" ]
    pub type DACENR = super::ahbenr::DMA1ENR;
    # [ doc = "Values that can be written to the field `TIM2EN`" ]
    pub type TIM2ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _TIM2ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM2ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: TIM2ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `TIM3EN`" ]
    pub type TIM3ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _TIM3ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM3ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: TIM3ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `TIM4EN`" ]
    pub type TIM4ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _TIM4ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM4ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: TIM4ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `TIM5EN`" ]
    pub type TIM5ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _TIM5ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM5ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: TIM5ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `TIM6EN`" ]
    pub type TIM6ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _TIM6ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM6ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: TIM6ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `TIM7EN`" ]
    pub type TIM7ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _TIM7ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM7ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: TIM7ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `TIM12EN`" ]
    pub type TIM12ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _TIM12ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM12ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: TIM12ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `TIM13EN`" ]
    pub type TIM13ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _TIM13ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM13ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: TIM13ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `TIM14EN`" ]
    pub type TIM14ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _TIM14ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM14ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: TIM14ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `WWDGEN`" ]
    pub type WWDGENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _WWDGENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WWDGENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: WWDGENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `SPI2EN`" ]
    pub type SPI2ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _SPI2ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SPI2ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: SPI2ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `SPI3EN`" ]
    pub type SPI3ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _SPI3ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SPI3ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: SPI3ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `USART2EN`" ]
    pub type USART2ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _USART2ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _USART2ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: USART2ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `USART3EN`" ]
    pub type USART3ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _USART3ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _USART3ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: USART3ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `UART4EN`" ]
    pub type UART4ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _UART4ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _UART4ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: UART4ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `UART5EN`" ]
    pub type UART5ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _UART5ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _UART5ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: UART5ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `I2C1EN`" ]
    pub type I2C1ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _I2C1ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _I2C1ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: I2C1ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `I2C2EN`" ]
    pub type I2C2ENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _I2C2ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _I2C2ENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: I2C2ENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `USBEN`" ]
    pub type USBENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _USBENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _USBENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: USBENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `CANEN`" ]
    pub type CANENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _CANENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CANENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CANENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `BKPEN`" ]
    pub type BKPENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _BKPENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BKPENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BKPENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `PWREN`" ]
    pub type PWRENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _PWRENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWRENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWRENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `DACEN`" ]
    pub type DACENW = super::ahbenr::DMA1ENW;
    # [ doc = r" Proxy" ]
    pub struct _DACENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DACENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: DACENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(super::ahbenr::DMA1ENW::ENABLED)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
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
        # [ doc = "Bit 0 - Timer 2 clock enable" ]
        # [ inline ( always ) ]
        pub fn tim2en(&self) -> TIM2ENR {
            TIM2ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 1 - Timer 3 clock enable" ]
        # [ inline ( always ) ]
        pub fn tim3en(&self) -> TIM3ENR {
            TIM3ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 2 - Timer 4 clock enable" ]
        # [ inline ( always ) ]
        pub fn tim4en(&self) -> TIM4ENR {
            TIM4ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 3 - Timer 5 clock enable" ]
        # [ inline ( always ) ]
        pub fn tim5en(&self) -> TIM5ENR {
            TIM5ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 4 - Timer 6 clock enable" ]
        # [ inline ( always ) ]
        pub fn tim6en(&self) -> TIM6ENR {
            TIM6ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 5 - Timer 7 clock enable" ]
        # [ inline ( always ) ]
        pub fn tim7en(&self) -> TIM7ENR {
            TIM7ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 6 - Timer 12 clock enable" ]
        # [ inline ( always ) ]
        pub fn tim12en(&self) -> TIM12ENR {
            TIM12ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 7 - Timer 13 clock enable" ]
        # [ inline ( always ) ]
        pub fn tim13en(&self) -> TIM13ENR {
            TIM13ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 8 - Timer 14 clock enable" ]
        # [ inline ( always ) ]
        pub fn tim14en(&self) -> TIM14ENR {
            TIM14ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 11 - Window watchdog clock enable" ]
        # [ inline ( always ) ]
        pub fn wwdgen(&self) -> WWDGENR {
            WWDGENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 14 - SPI 2 clock enable" ]
        # [ inline ( always ) ]
        pub fn spi2en(&self) -> SPI2ENR {
            SPI2ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 15 - SPI 3 clock enable" ]
        # [ inline ( always ) ]
        pub fn spi3en(&self) -> SPI3ENR {
            SPI3ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 17 - USART 2 clock enable" ]
        # [ inline ( always ) ]
        pub fn usart2en(&self) -> USART2ENR {
            USART2ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 18 - USART 3 clock enable" ]
        # [ inline ( always ) ]
        pub fn usart3en(&self) -> USART3ENR {
            USART3ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 19 - UART 4 clock enable" ]
        # [ inline ( always ) ]
        pub fn uart4en(&self) -> UART4ENR {
            UART4ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 20 - UART 5 clock enable" ]
        # [ inline ( always ) ]
        pub fn uart5en(&self) -> UART5ENR {
            UART5ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 21 - I2C 1 clock enable" ]
        # [ inline ( always ) ]
        pub fn i2c1en(&self) -> I2C1ENR {
            I2C1ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 21;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 22 - I2C 2 clock enable" ]
        # [ inline ( always ) ]
        pub fn i2c2en(&self) -> I2C2ENR {
            I2C2ENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 22;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 23 - USB clock enable" ]
        # [ inline ( always ) ]
        pub fn usben(&self) -> USBENR {
            USBENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 23;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 25 - CAN clock enable" ]
        # [ inline ( always ) ]
        pub fn canen(&self) -> CANENR {
            CANENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 27 - Backup interface clock enable" ]
        # [ inline ( always ) ]
        pub fn bkpen(&self) -> BKPENR {
            BKPENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 27;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 28 - Power interface clock enable" ]
        # [ inline ( always ) ]
        pub fn pwren(&self) -> PWRENR {
            PWRENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 29 - DAC interface clock enable" ]
        # [ inline ( always ) ]
        pub fn dacen(&self) -> DACENR {
            DACENR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 29;
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
        # [ doc = "Bit 0 - Timer 2 clock enable" ]
        # [ inline ( always ) ]
        pub fn tim2en(&mut self) -> _TIM2ENW {
            _TIM2ENW { w: self }
        }
        # [ doc = "Bit 1 - Timer 3 clock enable" ]
        # [ inline ( always ) ]
        pub fn tim3en(&mut self) -> _TIM3ENW {
            _TIM3ENW { w: self }
        }
        # [ doc = "Bit 2 - Timer 4 clock enable" ]
        # [ inline ( always ) ]
        pub fn tim4en(&mut self) -> _TIM4ENW {
            _TIM4ENW { w: self }
        }
        # [ doc = "Bit 3 - Timer 5 clock enable" ]
        # [ inline ( always ) ]
        pub fn tim5en(&mut self) -> _TIM5ENW {
            _TIM5ENW { w: self }
        }
        # [ doc = "Bit 4 - Timer 6 clock enable" ]
        # [ inline ( always ) ]
        pub fn tim6en(&mut self) -> _TIM6ENW {
            _TIM6ENW { w: self }
        }
        # [ doc = "Bit 5 - Timer 7 clock enable" ]
        # [ inline ( always ) ]
        pub fn tim7en(&mut self) -> _TIM7ENW {
            _TIM7ENW { w: self }
        }
        # [ doc = "Bit 6 - Timer 12 clock enable" ]
        # [ inline ( always ) ]
        pub fn tim12en(&mut self) -> _TIM12ENW {
            _TIM12ENW { w: self }
        }
        # [ doc = "Bit 7 - Timer 13 clock enable" ]
        # [ inline ( always ) ]
        pub fn tim13en(&mut self) -> _TIM13ENW {
            _TIM13ENW { w: self }
        }
        # [ doc = "Bit 8 - Timer 14 clock enable" ]
        # [ inline ( always ) ]
        pub fn tim14en(&mut self) -> _TIM14ENW {
            _TIM14ENW { w: self }
        }
        # [ doc = "Bit 11 - Window watchdog clock enable" ]
        # [ inline ( always ) ]
        pub fn wwdgen(&mut self) -> _WWDGENW {
            _WWDGENW { w: self }
        }
        # [ doc = "Bit 14 - SPI 2 clock enable" ]
        # [ inline ( always ) ]
        pub fn spi2en(&mut self) -> _SPI2ENW {
            _SPI2ENW { w: self }
        }
        # [ doc = "Bit 15 - SPI 3 clock enable" ]
        # [ inline ( always ) ]
        pub fn spi3en(&mut self) -> _SPI3ENW {
            _SPI3ENW { w: self }
        }
        # [ doc = "Bit 17 - USART 2 clock enable" ]
        # [ inline ( always ) ]
        pub fn usart2en(&mut self) -> _USART2ENW {
            _USART2ENW { w: self }
        }
        # [ doc = "Bit 18 - USART 3 clock enable" ]
        # [ inline ( always ) ]
        pub fn usart3en(&mut self) -> _USART3ENW {
            _USART3ENW { w: self }
        }
        # [ doc = "Bit 19 - UART 4 clock enable" ]
        # [ inline ( always ) ]
        pub fn uart4en(&mut self) -> _UART4ENW {
            _UART4ENW { w: self }
        }
        # [ doc = "Bit 20 - UART 5 clock enable" ]
        # [ inline ( always ) ]
        pub fn uart5en(&mut self) -> _UART5ENW {
            _UART5ENW { w: self }
        }
        # [ doc = "Bit 21 - I2C 1 clock enable" ]
        # [ inline ( always ) ]
        pub fn i2c1en(&mut self) -> _I2C1ENW {
            _I2C1ENW { w: self }
        }
        # [ doc = "Bit 22 - I2C 2 clock enable" ]
        # [ inline ( always ) ]
        pub fn i2c2en(&mut self) -> _I2C2ENW {
            _I2C2ENW { w: self }
        }
        # [ doc = "Bit 23 - USB clock enable" ]
        # [ inline ( always ) ]
        pub fn usben(&mut self) -> _USBENW {
            _USBENW { w: self }
        }
        # [ doc = "Bit 25 - CAN clock enable" ]
        # [ inline ( always ) ]
        pub fn canen(&mut self) -> _CANENW {
            _CANENW { w: self }
        }
        # [ doc = "Bit 27 - Backup interface clock enable" ]
        # [ inline ( always ) ]
        pub fn bkpen(&mut self) -> _BKPENW {
            _BKPENW { w: self }
        }
        # [ doc = "Bit 28 - Power interface clock enable" ]
        # [ inline ( always ) ]
        pub fn pwren(&mut self) -> _PWRENW {
            _PWRENW { w: self }
        }
        # [ doc = "Bit 29 - DAC interface clock enable" ]
        # [ inline ( always ) ]
        pub fn dacen(&mut self) -> _DACENW {
            _DACENW { w: self }
        }
    }
}
# [ doc = "Backup domain control register (RCC_BDCR)" ]
pub struct BDCR {
    register: VolatileCell<u32>,
}
# [ doc = "Backup domain control register (RCC_BDCR)" ]
pub mod bdcr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::BDCR {
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
    pub struct LSEONR {
        bits: bool,
    }
    impl LSEONR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct LSERDYR {
        bits: bool,
    }
    impl LSERDYR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct LSEBYPR {
        bits: bool,
    }
    impl LSEBYPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct RTCSELR {
        bits: u8,
    }
    impl RTCSELR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RTCENR {
        bits: bool,
    }
    impl RTCENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct BDRSTR {
        bits: bool,
    }
    impl BDRSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _LSEONW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LSEONW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LSEBYPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LSEBYPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _RTCSELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RTCSELW<'a> {
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
    pub struct _RTCENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RTCENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _BDRSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BDRSTW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - External Low Speed oscillator enable" ]
        # [ inline ( always ) ]
        pub fn lseon(&self) -> LSEONR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LSEONR { bits }
        }
        # [ doc = "Bit 1 - External Low Speed oscillator ready" ]
        # [ inline ( always ) ]
        pub fn lserdy(&self) -> LSERDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LSERDYR { bits }
        }
        # [ doc = "Bit 2 - External Low Speed oscillator bypass" ]
        # [ inline ( always ) ]
        pub fn lsebyp(&self) -> LSEBYPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LSEBYPR { bits }
        }
        # [ doc = "Bits 8:9 - RTC clock source selection" ]
        # [ inline ( always ) ]
        pub fn rtcsel(&self) -> RTCSELR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            RTCSELR { bits }
        }
        # [ doc = "Bit 15 - RTC clock enable" ]
        # [ inline ( always ) ]
        pub fn rtcen(&self) -> RTCENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RTCENR { bits }
        }
        # [ doc = "Bit 16 - Backup domain software reset" ]
        # [ inline ( always ) ]
        pub fn bdrst(&self) -> BDRSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BDRSTR { bits }
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
        # [ doc = "Bit 0 - External Low Speed oscillator enable" ]
        # [ inline ( always ) ]
        pub fn lseon(&mut self) -> _LSEONW {
            _LSEONW { w: self }
        }
        # [ doc = "Bit 2 - External Low Speed oscillator bypass" ]
        # [ inline ( always ) ]
        pub fn lsebyp(&mut self) -> _LSEBYPW {
            _LSEBYPW { w: self }
        }
        # [ doc = "Bits 8:9 - RTC clock source selection" ]
        # [ inline ( always ) ]
        pub fn rtcsel(&mut self) -> _RTCSELW {
            _RTCSELW { w: self }
        }
        # [ doc = "Bit 15 - RTC clock enable" ]
        # [ inline ( always ) ]
        pub fn rtcen(&mut self) -> _RTCENW {
            _RTCENW { w: self }
        }
        # [ doc = "Bit 16 - Backup domain software reset" ]
        # [ inline ( always ) ]
        pub fn bdrst(&mut self) -> _BDRSTW {
            _BDRSTW { w: self }
        }
    }
}
# [ doc = "Control/status register (RCC_CSR)" ]
pub struct CSR {
    register: VolatileCell<u32>,
}
# [ doc = "Control/status register (RCC_CSR)" ]
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
    pub struct LSIONR {
        bits: bool,
    }
    impl LSIONR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct LSIRDYR {
        bits: bool,
    }
    impl LSIRDYR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct RMVFR {
        bits: bool,
    }
    impl RMVFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct PINRSTFR {
        bits: bool,
    }
    impl PINRSTFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct PORRSTFR {
        bits: bool,
    }
    impl PORRSTFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct SFTRSTFR {
        bits: bool,
    }
    impl SFTRSTFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct IWDGRSTFR {
        bits: bool,
    }
    impl IWDGRSTFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct WWDGRSTFR {
        bits: bool,
    }
    impl WWDGRSTFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct LPWRRSTFR {
        bits: bool,
    }
    impl LPWRRSTFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _LSIONW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LSIONW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _RMVFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RMVFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PINRSTFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINRSTFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PORRSTFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PORRSTFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SFTRSTFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SFTRSTFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    # [ doc = r" Proxy" ]
    pub struct _IWDGRSTFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IWDGRSTFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WWDGRSTFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WWDGRSTFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _LPWRRSTFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LPWRRSTFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
        # [ doc = "Bit 0 - Internal low speed oscillator enable" ]
        # [ inline ( always ) ]
        pub fn lsion(&self) -> LSIONR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LSIONR { bits }
        }
        # [ doc = "Bit 1 - Internal low speed oscillator ready" ]
        # [ inline ( always ) ]
        pub fn lsirdy(&self) -> LSIRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LSIRDYR { bits }
        }
        # [ doc = "Bit 24 - Remove reset flag" ]
        # [ inline ( always ) ]
        pub fn rmvf(&self) -> RMVFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RMVFR { bits }
        }
        # [ doc = "Bit 26 - PIN reset flag" ]
        # [ inline ( always ) ]
        pub fn pinrstf(&self) -> PINRSTFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINRSTFR { bits }
        }
        # [ doc = "Bit 27 - POR/PDR reset flag" ]
        # [ inline ( always ) ]
        pub fn porrstf(&self) -> PORRSTFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 27;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PORRSTFR { bits }
        }
        # [ doc = "Bit 28 - Software reset flag" ]
        # [ inline ( always ) ]
        pub fn sftrstf(&self) -> SFTRSTFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SFTRSTFR { bits }
        }
        # [ doc = "Bit 29 - Independent watchdog reset flag" ]
        # [ inline ( always ) ]
        pub fn iwdgrstf(&self) -> IWDGRSTFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 29;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IWDGRSTFR { bits }
        }
        # [ doc = "Bit 30 - Window watchdog reset flag" ]
        # [ inline ( always ) ]
        pub fn wwdgrstf(&self) -> WWDGRSTFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 30;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WWDGRSTFR { bits }
        }
        # [ doc = "Bit 31 - Low-power reset flag" ]
        # [ inline ( always ) ]
        pub fn lpwrrstf(&self) -> LPWRRSTFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 31;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LPWRRSTFR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 201326592 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 0 - Internal low speed oscillator enable" ]
        # [ inline ( always ) ]
        pub fn lsion(&mut self) -> _LSIONW {
            _LSIONW { w: self }
        }
        # [ doc = "Bit 24 - Remove reset flag" ]
        # [ inline ( always ) ]
        pub fn rmvf(&mut self) -> _RMVFW {
            _RMVFW { w: self }
        }
        # [ doc = "Bit 26 - PIN reset flag" ]
        # [ inline ( always ) ]
        pub fn pinrstf(&mut self) -> _PINRSTFW {
            _PINRSTFW { w: self }
        }
        # [ doc = "Bit 27 - POR/PDR reset flag" ]
        # [ inline ( always ) ]
        pub fn porrstf(&mut self) -> _PORRSTFW {
            _PORRSTFW { w: self }
        }
        # [ doc = "Bit 28 - Software reset flag" ]
        # [ inline ( always ) ]
        pub fn sftrstf(&mut self) -> _SFTRSTFW {
            _SFTRSTFW { w: self }
        }
        # [ doc = "Bit 29 - Independent watchdog reset flag" ]
        # [ inline ( always ) ]
        pub fn iwdgrstf(&mut self) -> _IWDGRSTFW {
            _IWDGRSTFW { w: self }
        }
        # [ doc = "Bit 30 - Window watchdog reset flag" ]
        # [ inline ( always ) ]
        pub fn wwdgrstf(&mut self) -> _WWDGRSTFW {
            _WWDGRSTFW { w: self }
        }
        # [ doc = "Bit 31 - Low-power reset flag" ]
        # [ inline ( always ) ]
        pub fn lpwrrstf(&mut self) -> _LPWRRSTFW {
            _LPWRRSTFW { w: self }
        }
    }
}
# [ doc = "Reset and clock control" ]
pub struct RCC {
    register_block: RegisterBlock,
}
impl Deref for RCC {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
