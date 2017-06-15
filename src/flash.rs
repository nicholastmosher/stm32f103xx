# ! [ doc = "FLASH" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;
use vcell::VolatileCell;

# [ doc = "FLASH" ]
pub const FLASH: Peripheral<FLASH> = unsafe { Peripheral::new(1073881088) };

# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - Flash access control register" ]
    pub acr: ACR,
    # [ doc = "0x04 - Flash key register" ]
    pub keyr: KEYR,
    # [ doc = "0x08 - Flash option key register" ]
    pub optkeyr: OPTKEYR,
    # [ doc = "0x0c - Status register" ]
    pub sr: SR,
    # [ doc = "0x10 - Control register" ]
    pub cr: CR,
    # [ doc = "0x14 - Flash address register" ]
    pub ar: AR,
    _reserved0: [u8; 4usize],
    # [ doc = "0x1c - Option byte register" ]
    pub obr: OBR,
    # [ doc = "0x20 - Write protection register" ]
    pub wrpr: WRPR,
}
# [ doc = "Flash access control register" ]
pub struct ACR {
    register: VolatileCell<u32>,
}
# [ doc = "Flash access control register" ]
pub mod acr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::ACR {
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
    # [ doc = "Possible values of the field `LATENCY`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum LATENCYR {
        # [ doc = "Zero wait state, if 0hz  SYSCLK to 24 MHz" ]
        ZERO,
        # [ doc = "One wait state, if 24 MHz SYSCLK to 48 MHz" ]
        ONE,
        # [ doc = "Two wait states, if 48 MHz SYSCLK to 72 MHz" ]
        TWO,
        # [ doc = r" Reserved" ]
        _Reserved(u8),
    }
    impl LATENCYR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                LATENCYR::ZERO => 0,
                LATENCYR::ONE => 1,
                LATENCYR::TWO => 2,
                LATENCYR::_Reserved(bits) => bits,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> LATENCYR {
            match value {
                0 => LATENCYR::ZERO,
                1 => LATENCYR::ONE,
                2 => LATENCYR::TWO,
                i => LATENCYR::_Reserved(i),
            }
        }
        # [ doc = "Checks if the value of the field is `ZERO`" ]
        # [ inline ( always ) ]
        pub fn is_zero(&self) -> bool {
            *self == LATENCYR::ZERO
        }
        # [ doc = "Checks if the value of the field is `ONE`" ]
        # [ inline ( always ) ]
        pub fn is_one(&self) -> bool {
            *self == LATENCYR::ONE
        }
        # [ doc = "Checks if the value of the field is `TWO`" ]
        # [ inline ( always ) ]
        pub fn is_two(&self) -> bool {
            *self == LATENCYR::TWO
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct HLFCYAR {
        bits: bool,
    }
    impl HLFCYAR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = "Possible values of the field `PRFTBE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PRFTBER {
        # [ doc = "Disabled." ]
        DISABLED,
        # [ doc = "Enabled." ]
        ENABLED,
    }
    impl PRFTBER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                PRFTBER::DISABLED => false,
                PRFTBER::ENABLED => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PRFTBER {
            match value {
                false => PRFTBER::DISABLED,
                true => PRFTBER::ENABLED,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == PRFTBER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `ENABLED`" ]
        # [ inline ( always ) ]
        pub fn is_enabled(&self) -> bool {
            *self == PRFTBER::ENABLED
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct PRFTBSR {
        bits: bool,
    }
    impl PRFTBSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = "Values that can be written to the field `LATENCY`" ]
    pub enum LATENCYW {
        # [ doc = "Zero wait state, if 0hz  SYSCLK to 24 MHz" ]
        ZERO,
        # [ doc = "One wait state, if 24 MHz SYSCLK to 48 MHz" ]
        ONE,
        # [ doc = "Two wait states, if 48 MHz SYSCLK to 72 MHz" ]
        TWO,
    }
    impl LATENCYW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                LATENCYW::ZERO => 0,
                LATENCYW::ONE => 1,
                LATENCYW::TWO => 2,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _LATENCYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LATENCYW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: LATENCYW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "Zero wait state, if 0hz SYSCLK to 24 MHz" ]
        # [ inline ( always ) ]
        pub fn zero(self) -> &'a mut W {
            self.variant(LATENCYW::ZERO)
        }
        # [ doc = "One wait state, if 24 MHz SYSCLK to 48 MHz" ]
        # [ inline ( always ) ]
        pub fn one(self) -> &'a mut W {
            self.variant(LATENCYW::ONE)
        }
        # [ doc = "Two wait states, if 48 MHz SYSCLK to 72 MHz" ]
        # [ inline ( always ) ]
        pub fn two(self) -> &'a mut W {
            self.variant(LATENCYW::TWO)
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
    # [ doc = r" Proxy" ]
    pub struct _HLFCYAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _HLFCYAW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    # [ doc = "Values that can be written to the field `PRFTBE`" ]
    pub enum PRFTBEW {
        # [ doc = "Disabled." ]
        DISABLED,
        # [ doc = "Enabled." ]
        ENABLED,
    }
    impl PRFTBEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PRFTBEW::DISABLED => false,
                PRFTBEW::ENABLED => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PRFTBEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PRFTBEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PRFTBEW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(PRFTBEW::DISABLED)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(PRFTBEW::ENABLED)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:2 - Latency" ]
        # [ inline ( always ) ]
        pub fn latency(&self) -> LATENCYR {
            LATENCYR::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bit 3 - Flash half cycle access enable" ]
        # [ inline ( always ) ]
        pub fn hlfcya(&self) -> HLFCYAR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            HLFCYAR { bits }
        }
        # [ doc = "Bit 4 - Prefetch buffer enable" ]
        # [ inline ( always ) ]
        pub fn prftbe(&self) -> PRFTBER {
            PRFTBER::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        # [ doc = "Bit 5 - Prefetch buffer status" ]
        # [ inline ( always ) ]
        pub fn prftbs(&self) -> PRFTBSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PRFTBSR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 48 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:2 - Latency" ]
        # [ inline ( always ) ]
        pub fn latency(&mut self) -> _LATENCYW {
            _LATENCYW { w: self }
        }
        # [ doc = "Bit 3 - Flash half cycle access enable" ]
        # [ inline ( always ) ]
        pub fn hlfcya(&mut self) -> _HLFCYAW {
            _HLFCYAW { w: self }
        }
        # [ doc = "Bit 4 - Prefetch buffer enable" ]
        # [ inline ( always ) ]
        pub fn prftbe(&mut self) -> _PRFTBEW {
            _PRFTBEW { w: self }
        }
    }
}
# [ doc = "Flash key register" ]
pub struct KEYR {
    register: VolatileCell<u32>,
}
# [ doc = "Flash key register" ]
pub mod keyr {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::KEYR {
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
    pub struct _KEYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _KEYW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 4294967295;
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
        # [ doc = "Bits 0:31 - FPEC key" ]
        # [ inline ( always ) ]
        pub fn key(&mut self) -> _KEYW {
            _KEYW { w: self }
        }
    }
}
# [ doc = "Flash option key register" ]
pub struct OPTKEYR {
    register: VolatileCell<u32>,
}
# [ doc = "Flash option key register" ]
pub mod optkeyr {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::OPTKEYR {
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
    pub struct _OPTKEYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OPTKEYW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 4294967295;
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
        # [ doc = "Bits 0:31 - Option byte key" ]
        # [ inline ( always ) ]
        pub fn optkey(&mut self) -> _OPTKEYW {
            _OPTKEYW { w: self }
        }
    }
}
# [ doc = "Status register" ]
pub struct SR {
    register: VolatileCell<u32>,
}
# [ doc = "Status register" ]
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
    pub struct EOPR {
        bits: bool,
    }
    impl EOPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WRPRTERRR {
        bits: bool,
    }
    impl WRPRTERRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct PGERRR {
        bits: bool,
    }
    impl PGERRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct BSYR {
        bits: bool,
    }
    impl BSYR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _EOPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EOPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _WRPRTERRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WRPRTERRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _PGERRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGERRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
        # [ doc = "Bit 5 - End of operation" ]
        # [ inline ( always ) ]
        pub fn eop(&self) -> EOPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EOPR { bits }
        }
        # [ doc = "Bit 4 - Write protection error" ]
        # [ inline ( always ) ]
        pub fn wrprterr(&self) -> WRPRTERRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WRPRTERRR { bits }
        }
        # [ doc = "Bit 2 - Programming error" ]
        # [ inline ( always ) ]
        pub fn pgerr(&self) -> PGERRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PGERRR { bits }
        }
        # [ doc = "Bit 0 - Busy" ]
        # [ inline ( always ) ]
        pub fn bsy(&self) -> BSYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BSYR { bits }
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
        # [ doc = "Bit 5 - End of operation" ]
        # [ inline ( always ) ]
        pub fn eop(&mut self) -> _EOPW {
            _EOPW { w: self }
        }
        # [ doc = "Bit 4 - Write protection error" ]
        # [ inline ( always ) ]
        pub fn wrprterr(&mut self) -> _WRPRTERRW {
            _WRPRTERRW { w: self }
        }
        # [ doc = "Bit 2 - Programming error" ]
        # [ inline ( always ) ]
        pub fn pgerr(&mut self) -> _PGERRW {
            _PGERRW { w: self }
        }
    }
}
# [ doc = "Control register" ]
pub struct CR {
    register: VolatileCell<u32>,
}
# [ doc = "Control register" ]
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
    pub struct PGR {
        bits: bool,
    }
    impl PGR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct PERR {
        bits: bool,
    }
    impl PERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MERR {
        bits: bool,
    }
    impl MERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct OPTPGR {
        bits: bool,
    }
    impl OPTPGR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct OPTERR {
        bits: bool,
    }
    impl OPTERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
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
    pub struct LOCKR {
        bits: bool,
    }
    impl LOCKR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct OPTWRER {
        bits: bool,
    }
    impl OPTWRER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ERRIER {
        bits: bool,
    }
    impl ERRIER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EOPIER {
        bits: bool,
    }
    impl EOPIER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _PGW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _PERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _MERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _OPTPGW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OPTPGW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _OPTERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OPTERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
            const OFFSET: u8 = 6;
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
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _OPTWREW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OPTWREW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ERRIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ERRIEW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EOPIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EOPIEW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Programming" ]
        # [ inline ( always ) ]
        pub fn pg(&self) -> PGR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PGR { bits }
        }
        # [ doc = "Bit 1 - Page Erase" ]
        # [ inline ( always ) ]
        pub fn per(&self) -> PERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PERR { bits }
        }
        # [ doc = "Bit 2 - Mass Erase" ]
        # [ inline ( always ) ]
        pub fn mer(&self) -> MERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MERR { bits }
        }
        # [ doc = "Bit 4 - Option byte programming" ]
        # [ inline ( always ) ]
        pub fn optpg(&self) -> OPTPGR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OPTPGR { bits }
        }
        # [ doc = "Bit 5 - Option byte erase" ]
        # [ inline ( always ) ]
        pub fn opter(&self) -> OPTERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OPTERR { bits }
        }
        # [ doc = "Bit 6 - Start" ]
        # [ inline ( always ) ]
        pub fn strt(&self) -> STRTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            STRTR { bits }
        }
        # [ doc = "Bit 7 - Lock" ]
        # [ inline ( always ) ]
        pub fn lock(&self) -> LOCKR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LOCKR { bits }
        }
        # [ doc = "Bit 9 - Option bytes write enable" ]
        # [ inline ( always ) ]
        pub fn optwre(&self) -> OPTWRER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OPTWRER { bits }
        }
        # [ doc = "Bit 10 - Error interrupt enable" ]
        # [ inline ( always ) ]
        pub fn errie(&self) -> ERRIER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ERRIER { bits }
        }
        # [ doc = "Bit 12 - End of operation interrupt enable" ]
        # [ inline ( always ) ]
        pub fn eopie(&self) -> EOPIER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EOPIER { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 128 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 0 - Programming" ]
        # [ inline ( always ) ]
        pub fn pg(&mut self) -> _PGW {
            _PGW { w: self }
        }
        # [ doc = "Bit 1 - Page Erase" ]
        # [ inline ( always ) ]
        pub fn per(&mut self) -> _PERW {
            _PERW { w: self }
        }
        # [ doc = "Bit 2 - Mass Erase" ]
        # [ inline ( always ) ]
        pub fn mer(&mut self) -> _MERW {
            _MERW { w: self }
        }
        # [ doc = "Bit 4 - Option byte programming" ]
        # [ inline ( always ) ]
        pub fn optpg(&mut self) -> _OPTPGW {
            _OPTPGW { w: self }
        }
        # [ doc = "Bit 5 - Option byte erase" ]
        # [ inline ( always ) ]
        pub fn opter(&mut self) -> _OPTERW {
            _OPTERW { w: self }
        }
        # [ doc = "Bit 6 - Start" ]
        # [ inline ( always ) ]
        pub fn strt(&mut self) -> _STRTW {
            _STRTW { w: self }
        }
        # [ doc = "Bit 7 - Lock" ]
        # [ inline ( always ) ]
        pub fn lock(&mut self) -> _LOCKW {
            _LOCKW { w: self }
        }
        # [ doc = "Bit 9 - Option bytes write enable" ]
        # [ inline ( always ) ]
        pub fn optwre(&mut self) -> _OPTWREW {
            _OPTWREW { w: self }
        }
        # [ doc = "Bit 10 - Error interrupt enable" ]
        # [ inline ( always ) ]
        pub fn errie(&mut self) -> _ERRIEW {
            _ERRIEW { w: self }
        }
        # [ doc = "Bit 12 - End of operation interrupt enable" ]
        # [ inline ( always ) ]
        pub fn eopie(&mut self) -> _EOPIEW {
            _EOPIEW { w: self }
        }
    }
}
# [ doc = "Flash address register" ]
pub struct AR {
    register: VolatileCell<u32>,
}
# [ doc = "Flash address register" ]
pub mod ar {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::AR {
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
    pub struct _FARW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FARW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 4294967295;
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
        # [ doc = "Bits 0:31 - Flash Address" ]
        # [ inline ( always ) ]
        pub fn far(&mut self) -> _FARW {
            _FARW { w: self }
        }
    }
}
# [ doc = "Option byte register" ]
pub struct OBR {
    register: VolatileCell<u32>,
}
# [ doc = "Option byte register" ]
pub mod obr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::OBR {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct OPTERRR {
        bits: bool,
    }
    impl OPTERRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RDPRTR {
        bits: bool,
    }
    impl RDPRTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WDG_SWR {
        bits: bool,
    }
    impl WDG_SWR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct NRST_STOPR {
        bits: bool,
    }
    impl NRST_STOPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct NRST_STDBYR {
        bits: bool,
    }
    impl NRST_STDBYR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATA0R {
        bits: u8,
    }
    impl DATA0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATA1R {
        bits: u8,
    }
    impl DATA1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Option byte error" ]
        # [ inline ( always ) ]
        pub fn opterr(&self) -> OPTERRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OPTERRR { bits }
        }
        # [ doc = "Bit 1 - Read protection" ]
        # [ inline ( always ) ]
        pub fn rdprt(&self) -> RDPRTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RDPRTR { bits }
        }
        # [ doc = "Bit 2 - WDG_SW" ]
        # [ inline ( always ) ]
        pub fn wdg_sw(&self) -> WDG_SWR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WDG_SWR { bits }
        }
        # [ doc = "Bit 3 - nRST_STOP" ]
        # [ inline ( always ) ]
        pub fn n_rst_stop(&self) -> NRST_STOPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            NRST_STOPR { bits }
        }
        # [ doc = "Bit 4 - nRST_STDBY" ]
        # [ inline ( always ) ]
        pub fn n_rst_stdby(&self) -> NRST_STDBYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            NRST_STDBYR { bits }
        }
        # [ doc = "Bits 10:17 - Data0" ]
        # [ inline ( always ) ]
        pub fn data0(&self) -> DATA0R {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATA0R { bits }
        }
        # [ doc = "Bits 18:25 - Data1" ]
        # [ inline ( always ) ]
        pub fn data1(&self) -> DATA1R {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATA1R { bits }
        }
    }
}
# [ doc = "Write protection register" ]
pub struct WRPR {
    register: VolatileCell<u32>,
}
# [ doc = "Write protection register" ]
pub mod wrpr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::WRPR {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WRPR {
        bits: u32,
    }
    impl WRPR {
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
        # [ doc = "Bits 0:31 - Write protect" ]
        # [ inline ( always ) ]
        pub fn wrp(&self) -> WRPR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            WRPR { bits }
        }
    }
}
# [ doc = "FLASH" ]
pub struct FLASH {
    register_block: RegisterBlock,
}
impl Deref for FLASH {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
