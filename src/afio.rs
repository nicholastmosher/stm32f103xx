# ! [ doc = "Alternate function I/O" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;
use vcell::VolatileCell;

# [ doc = "Alternate function I/O" ]
pub const AFIO: Peripheral<AFIO> = unsafe { Peripheral::new(1073807360) };

# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - Event Control Register (AFIO_EVCR)" ]
    pub evcr: EVCR,
    # [ doc = "0x04 - AF remap and debug I/O configuration register (AFIO_MAPR)" ]
    pub mapr: MAPR,
    # [ doc = "0x08 - External interrupt configuration register 1 (AFIO_EXTICR1)" ]
    pub exticr1: EXTICR1,
    # [ doc = "0x0c - External interrupt configuration register 2 (AFIO_EXTICR2)" ]
    pub exticr2: EXTICR2,
    # [ doc = "0x10 - External interrupt configuration register 3 (AFIO_EXTICR3)" ]
    pub exticr3: EXTICR3,
    # [ doc = "0x14 - External interrupt configuration register 4 (AFIO_EXTICR4)" ]
    pub exticr4: EXTICR4,
    _reserved0: [u8; 4usize],
    # [ doc = "0x1c - AF remap and debug I/O configuration register" ]
    pub mapr2: MAPR2,
}
# [ doc = "Event Control Register (AFIO_EVCR)" ]
pub struct EVCR {
    register: VolatileCell<u32>,
}
# [ doc = "Event Control Register (AFIO_EVCR)" ]
pub mod evcr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::EVCR {
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
    pub struct PINR {
        bits: u8,
    }
    impl PINR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct PORTR {
        bits: u8,
    }
    impl PORTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EVOER {
        bits: bool,
    }
    impl EVOER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _PINW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINW<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _PORTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PORTW<'a> {
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
    pub struct _EVOEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVOEW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:3 - Pin selection" ]
        # [ inline ( always ) ]
        pub fn pin(&self) -> PINR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            PINR { bits }
        }
        # [ doc = "Bits 4:6 - Port selection" ]
        # [ inline ( always ) ]
        pub fn port(&self) -> PORTR {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            PORTR { bits }
        }
        # [ doc = "Bit 7 - Event Output Enable" ]
        # [ inline ( always ) ]
        pub fn evoe(&self) -> EVOER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVOER { bits }
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
        # [ doc = "Bits 0:3 - Pin selection" ]
        # [ inline ( always ) ]
        pub fn pin(&mut self) -> _PINW {
            _PINW { w: self }
        }
        # [ doc = "Bits 4:6 - Port selection" ]
        # [ inline ( always ) ]
        pub fn port(&mut self) -> _PORTW {
            _PORTW { w: self }
        }
        # [ doc = "Bit 7 - Event Output Enable" ]
        # [ inline ( always ) ]
        pub fn evoe(&mut self) -> _EVOEW {
            _EVOEW { w: self }
        }
    }
}
# [ doc = "AF remap and debug I/O configuration register (AFIO_MAPR)" ]
pub struct MAPR {
    register: VolatileCell<u32>,
}
# [ doc = "AF remap and debug I/O configuration register (AFIO_MAPR)" ]
pub mod mapr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::MAPR {
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
    pub struct SPI1_REMAPR {
        bits: bool,
    }
    impl SPI1_REMAPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct I2C1_REMAPR {
        bits: bool,
    }
    impl I2C1_REMAPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct USART1_REMAPR {
        bits: bool,
    }
    impl USART1_REMAPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct USART2_REMAPR {
        bits: bool,
    }
    impl USART2_REMAPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct USART3_REMAPR {
        bits: u8,
    }
    impl USART3_REMAPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TIM1_REMAPR {
        bits: u8,
    }
    impl TIM1_REMAPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TIM2_REMAPR {
        bits: u8,
    }
    impl TIM2_REMAPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TIM3_REMAPR {
        bits: u8,
    }
    impl TIM3_REMAPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TIM4_REMAPR {
        bits: bool,
    }
    impl TIM4_REMAPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CAN_REMAPR {
        bits: u8,
    }
    impl CAN_REMAPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct PD01_REMAPR {
        bits: bool,
    }
    impl PD01_REMAPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TIM5CH4_IREMAPR {
        bits: bool,
    }
    impl TIM5CH4_IREMAPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ADC1_ETRGINJ_REMAPR {
        bits: bool,
    }
    impl ADC1_ETRGINJ_REMAPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ADC1_ETRGREG_REMAPR {
        bits: bool,
    }
    impl ADC1_ETRGREG_REMAPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ADC2_ETRGINJ_REMAPR {
        bits: bool,
    }
    impl ADC2_ETRGINJ_REMAPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ADC2_ETRGREG_REMAPR {
        bits: bool,
    }
    impl ADC2_ETRGREG_REMAPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _SPI1_REMAPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SPI1_REMAPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _I2C1_REMAPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _I2C1_REMAPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _USART1_REMAPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _USART1_REMAPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _USART2_REMAPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _USART2_REMAPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _USART3_REMAPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _USART3_REMAPW<'a> {
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
    pub struct _TIM1_REMAPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM1_REMAPW<'a> {
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
    pub struct _TIM2_REMAPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM2_REMAPW<'a> {
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
    pub struct _TIM3_REMAPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM3_REMAPW<'a> {
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
    pub struct _TIM4_REMAPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM4_REMAPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAN_REMAPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAN_REMAPW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PD01_REMAPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PD01_REMAPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TIM5CH4_IREMAPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM5CH4_IREMAPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ADC1_ETRGINJ_REMAPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADC1_ETRGINJ_REMAPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ADC1_ETRGREG_REMAPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADC1_ETRGREG_REMAPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ADC2_ETRGINJ_REMAPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADC2_ETRGINJ_REMAPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ADC2_ETRGREG_REMAPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADC2_ETRGREG_REMAPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SWJ_CFGW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWJ_CFGW<'a> {
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
        # [ doc = "Bit 0 - SPI1 remapping" ]
        # [ inline ( always ) ]
        pub fn spi1_remap(&self) -> SPI1_REMAPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SPI1_REMAPR { bits }
        }
        # [ doc = "Bit 1 - I2C1 remapping" ]
        # [ inline ( always ) ]
        pub fn i2c1_remap(&self) -> I2C1_REMAPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            I2C1_REMAPR { bits }
        }
        # [ doc = "Bit 2 - USART1 remapping" ]
        # [ inline ( always ) ]
        pub fn usart1_remap(&self) -> USART1_REMAPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USART1_REMAPR { bits }
        }
        # [ doc = "Bit 3 - USART2 remapping" ]
        # [ inline ( always ) ]
        pub fn usart2_remap(&self) -> USART2_REMAPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USART2_REMAPR { bits }
        }
        # [ doc = "Bits 4:5 - USART3 remapping" ]
        # [ inline ( always ) ]
        pub fn usart3_remap(&self) -> USART3_REMAPR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            USART3_REMAPR { bits }
        }
        # [ doc = "Bits 6:7 - TIM1 remapping" ]
        # [ inline ( always ) ]
        pub fn tim1_remap(&self) -> TIM1_REMAPR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            TIM1_REMAPR { bits }
        }
        # [ doc = "Bits 8:9 - TIM2 remapping" ]
        # [ inline ( always ) ]
        pub fn tim2_remap(&self) -> TIM2_REMAPR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            TIM2_REMAPR { bits }
        }
        # [ doc = "Bits 10:11 - TIM3 remapping" ]
        # [ inline ( always ) ]
        pub fn tim3_remap(&self) -> TIM3_REMAPR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            TIM3_REMAPR { bits }
        }
        # [ doc = "Bit 12 - TIM4 remapping" ]
        # [ inline ( always ) ]
        pub fn tim4_remap(&self) -> TIM4_REMAPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIM4_REMAPR { bits }
        }
        # [ doc = "Bits 13:14 - CAN1 remapping" ]
        # [ inline ( always ) ]
        pub fn can_remap(&self) -> CAN_REMAPR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CAN_REMAPR { bits }
        }
        # [ doc = "Bit 15 - Port D0/Port D1 mapping on OSCIN/OSCOUT" ]
        # [ inline ( always ) ]
        pub fn pd01_remap(&self) -> PD01_REMAPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PD01_REMAPR { bits }
        }
        # [ doc = "Bit 16 - Set and cleared by software" ]
        # [ inline ( always ) ]
        pub fn tim5ch4_iremap(&self) -> TIM5CH4_IREMAPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIM5CH4_IREMAPR { bits }
        }
        # [ doc = "Bit 17 - ADC 1 External trigger injected conversion remapping" ]
        # [ inline ( always ) ]
        pub fn adc1_etrginj_remap(&self) -> ADC1_ETRGINJ_REMAPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ADC1_ETRGINJ_REMAPR { bits }
        }
        # [ doc = "Bit 18 - ADC 1 external trigger regular conversion remapping" ]
        # [ inline ( always ) ]
        pub fn adc1_etrgreg_remap(&self) -> ADC1_ETRGREG_REMAPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ADC1_ETRGREG_REMAPR { bits }
        }
        # [ doc = "Bit 19 - ADC 2 external trigger injected conversion remapping" ]
        # [ inline ( always ) ]
        pub fn adc2_etrginj_remap(&self) -> ADC2_ETRGINJ_REMAPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ADC2_ETRGINJ_REMAPR { bits }
        }
        # [ doc = "Bit 20 - ADC 2 external trigger regular conversion remapping" ]
        # [ inline ( always ) ]
        pub fn adc2_etrgreg_remap(&self) -> ADC2_ETRGREG_REMAPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ADC2_ETRGREG_REMAPR { bits }
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
        # [ doc = "Bit 0 - SPI1 remapping" ]
        # [ inline ( always ) ]
        pub fn spi1_remap(&mut self) -> _SPI1_REMAPW {
            _SPI1_REMAPW { w: self }
        }
        # [ doc = "Bit 1 - I2C1 remapping" ]
        # [ inline ( always ) ]
        pub fn i2c1_remap(&mut self) -> _I2C1_REMAPW {
            _I2C1_REMAPW { w: self }
        }
        # [ doc = "Bit 2 - USART1 remapping" ]
        # [ inline ( always ) ]
        pub fn usart1_remap(&mut self) -> _USART1_REMAPW {
            _USART1_REMAPW { w: self }
        }
        # [ doc = "Bit 3 - USART2 remapping" ]
        # [ inline ( always ) ]
        pub fn usart2_remap(&mut self) -> _USART2_REMAPW {
            _USART2_REMAPW { w: self }
        }
        # [ doc = "Bits 4:5 - USART3 remapping" ]
        # [ inline ( always ) ]
        pub fn usart3_remap(&mut self) -> _USART3_REMAPW {
            _USART3_REMAPW { w: self }
        }
        # [ doc = "Bits 6:7 - TIM1 remapping" ]
        # [ inline ( always ) ]
        pub fn tim1_remap(&mut self) -> _TIM1_REMAPW {
            _TIM1_REMAPW { w: self }
        }
        # [ doc = "Bits 8:9 - TIM2 remapping" ]
        # [ inline ( always ) ]
        pub fn tim2_remap(&mut self) -> _TIM2_REMAPW {
            _TIM2_REMAPW { w: self }
        }
        # [ doc = "Bits 10:11 - TIM3 remapping" ]
        # [ inline ( always ) ]
        pub fn tim3_remap(&mut self) -> _TIM3_REMAPW {
            _TIM3_REMAPW { w: self }
        }
        # [ doc = "Bit 12 - TIM4 remapping" ]
        # [ inline ( always ) ]
        pub fn tim4_remap(&mut self) -> _TIM4_REMAPW {
            _TIM4_REMAPW { w: self }
        }
        # [ doc = "Bits 13:14 - CAN1 remapping" ]
        # [ inline ( always ) ]
        pub fn can_remap(&mut self) -> _CAN_REMAPW {
            _CAN_REMAPW { w: self }
        }
        # [ doc = "Bit 15 - Port D0/Port D1 mapping on OSCIN/OSCOUT" ]
        # [ inline ( always ) ]
        pub fn pd01_remap(&mut self) -> _PD01_REMAPW {
            _PD01_REMAPW { w: self }
        }
        # [ doc = "Bit 16 - Set and cleared by software" ]
        # [ inline ( always ) ]
        pub fn tim5ch4_iremap(&mut self) -> _TIM5CH4_IREMAPW {
            _TIM5CH4_IREMAPW { w: self }
        }
        # [ doc = "Bit 17 - ADC 1 External trigger injected conversion remapping" ]
        # [ inline ( always ) ]
        pub fn adc1_etrginj_remap(&mut self) -> _ADC1_ETRGINJ_REMAPW {
            _ADC1_ETRGINJ_REMAPW { w: self }
        }
        # [ doc = "Bit 18 - ADC 1 external trigger regular conversion remapping" ]
        # [ inline ( always ) ]
        pub fn adc1_etrgreg_remap(&mut self) -> _ADC1_ETRGREG_REMAPW {
            _ADC1_ETRGREG_REMAPW { w: self }
        }
        # [ doc = "Bit 19 - ADC 2 external trigger injected conversion remapping" ]
        # [ inline ( always ) ]
        pub fn adc2_etrginj_remap(&mut self) -> _ADC2_ETRGINJ_REMAPW {
            _ADC2_ETRGINJ_REMAPW { w: self }
        }
        # [ doc = "Bit 20 - ADC 2 external trigger regular conversion remapping" ]
        # [ inline ( always ) ]
        pub fn adc2_etrgreg_remap(&mut self) -> _ADC2_ETRGREG_REMAPW {
            _ADC2_ETRGREG_REMAPW { w: self }
        }
        # [ doc = "Bits 24:26 - Serial wire JTAG configuration" ]
        # [ inline ( always ) ]
        pub fn swj_cfg(&mut self) -> _SWJ_CFGW {
            _SWJ_CFGW { w: self }
        }
    }
}
# [ doc = "External interrupt configuration register 1 (AFIO_EXTICR1)" ]
pub struct EXTICR1 {
    register: VolatileCell<u32>,
}
# [ doc = "External interrupt configuration register 1 (AFIO_EXTICR1)" ]
pub mod exticr1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::EXTICR1 {
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
    pub struct EXTI0R {
        bits: u8,
    }
    impl EXTI0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EXTI1R {
        bits: u8,
    }
    impl EXTI1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EXTI2R {
        bits: u8,
    }
    impl EXTI2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EXTI3R {
        bits: u8,
    }
    impl EXTI3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EXTI0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTI0W<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _EXTI1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTI1W<'a> {
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
    pub struct _EXTI2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTI2W<'a> {
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
    pub struct _EXTI3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTI3W<'a> {
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:3 - EXTI0 configuration" ]
        # [ inline ( always ) ]
        pub fn exti0(&self) -> EXTI0R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EXTI0R { bits }
        }
        # [ doc = "Bits 4:7 - EXTI1 configuration" ]
        # [ inline ( always ) ]
        pub fn exti1(&self) -> EXTI1R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EXTI1R { bits }
        }
        # [ doc = "Bits 8:11 - EXTI2 configuration" ]
        # [ inline ( always ) ]
        pub fn exti2(&self) -> EXTI2R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EXTI2R { bits }
        }
        # [ doc = "Bits 12:15 - EXTI3 configuration" ]
        # [ inline ( always ) ]
        pub fn exti3(&self) -> EXTI3R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EXTI3R { bits }
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
        # [ doc = "Bits 0:3 - EXTI0 configuration" ]
        # [ inline ( always ) ]
        pub fn exti0(&mut self) -> _EXTI0W {
            _EXTI0W { w: self }
        }
        # [ doc = "Bits 4:7 - EXTI1 configuration" ]
        # [ inline ( always ) ]
        pub fn exti1(&mut self) -> _EXTI1W {
            _EXTI1W { w: self }
        }
        # [ doc = "Bits 8:11 - EXTI2 configuration" ]
        # [ inline ( always ) ]
        pub fn exti2(&mut self) -> _EXTI2W {
            _EXTI2W { w: self }
        }
        # [ doc = "Bits 12:15 - EXTI3 configuration" ]
        # [ inline ( always ) ]
        pub fn exti3(&mut self) -> _EXTI3W {
            _EXTI3W { w: self }
        }
    }
}
# [ doc = "External interrupt configuration register 2 (AFIO_EXTICR2)" ]
pub struct EXTICR2 {
    register: VolatileCell<u32>,
}
# [ doc = "External interrupt configuration register 2 (AFIO_EXTICR2)" ]
pub mod exticr2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::EXTICR2 {
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
    pub struct EXTI4R {
        bits: u8,
    }
    impl EXTI4R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EXTI5R {
        bits: u8,
    }
    impl EXTI5R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EXTI6R {
        bits: u8,
    }
    impl EXTI6R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EXTI7R {
        bits: u8,
    }
    impl EXTI7R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EXTI4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTI4W<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _EXTI5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTI5W<'a> {
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
    pub struct _EXTI6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTI6W<'a> {
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
    pub struct _EXTI7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTI7W<'a> {
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:3 - EXTI4 configuration" ]
        # [ inline ( always ) ]
        pub fn exti4(&self) -> EXTI4R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EXTI4R { bits }
        }
        # [ doc = "Bits 4:7 - EXTI5 configuration" ]
        # [ inline ( always ) ]
        pub fn exti5(&self) -> EXTI5R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EXTI5R { bits }
        }
        # [ doc = "Bits 8:11 - EXTI6 configuration" ]
        # [ inline ( always ) ]
        pub fn exti6(&self) -> EXTI6R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EXTI6R { bits }
        }
        # [ doc = "Bits 12:15 - EXTI7 configuration" ]
        # [ inline ( always ) ]
        pub fn exti7(&self) -> EXTI7R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EXTI7R { bits }
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
        # [ doc = "Bits 0:3 - EXTI4 configuration" ]
        # [ inline ( always ) ]
        pub fn exti4(&mut self) -> _EXTI4W {
            _EXTI4W { w: self }
        }
        # [ doc = "Bits 4:7 - EXTI5 configuration" ]
        # [ inline ( always ) ]
        pub fn exti5(&mut self) -> _EXTI5W {
            _EXTI5W { w: self }
        }
        # [ doc = "Bits 8:11 - EXTI6 configuration" ]
        # [ inline ( always ) ]
        pub fn exti6(&mut self) -> _EXTI6W {
            _EXTI6W { w: self }
        }
        # [ doc = "Bits 12:15 - EXTI7 configuration" ]
        # [ inline ( always ) ]
        pub fn exti7(&mut self) -> _EXTI7W {
            _EXTI7W { w: self }
        }
    }
}
# [ doc = "External interrupt configuration register 3 (AFIO_EXTICR3)" ]
pub struct EXTICR3 {
    register: VolatileCell<u32>,
}
# [ doc = "External interrupt configuration register 3 (AFIO_EXTICR3)" ]
pub mod exticr3 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::EXTICR3 {
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
    pub struct EXTI8R {
        bits: u8,
    }
    impl EXTI8R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EXTI9R {
        bits: u8,
    }
    impl EXTI9R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EXTI10R {
        bits: u8,
    }
    impl EXTI10R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EXTI11R {
        bits: u8,
    }
    impl EXTI11R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EXTI8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTI8W<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _EXTI9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTI9W<'a> {
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
    pub struct _EXTI10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTI10W<'a> {
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
    pub struct _EXTI11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTI11W<'a> {
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:3 - EXTI8 configuration" ]
        # [ inline ( always ) ]
        pub fn exti8(&self) -> EXTI8R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EXTI8R { bits }
        }
        # [ doc = "Bits 4:7 - EXTI9 configuration" ]
        # [ inline ( always ) ]
        pub fn exti9(&self) -> EXTI9R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EXTI9R { bits }
        }
        # [ doc = "Bits 8:11 - EXTI10 configuration" ]
        # [ inline ( always ) ]
        pub fn exti10(&self) -> EXTI10R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EXTI10R { bits }
        }
        # [ doc = "Bits 12:15 - EXTI11 configuration" ]
        # [ inline ( always ) ]
        pub fn exti11(&self) -> EXTI11R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EXTI11R { bits }
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
        # [ doc = "Bits 0:3 - EXTI8 configuration" ]
        # [ inline ( always ) ]
        pub fn exti8(&mut self) -> _EXTI8W {
            _EXTI8W { w: self }
        }
        # [ doc = "Bits 4:7 - EXTI9 configuration" ]
        # [ inline ( always ) ]
        pub fn exti9(&mut self) -> _EXTI9W {
            _EXTI9W { w: self }
        }
        # [ doc = "Bits 8:11 - EXTI10 configuration" ]
        # [ inline ( always ) ]
        pub fn exti10(&mut self) -> _EXTI10W {
            _EXTI10W { w: self }
        }
        # [ doc = "Bits 12:15 - EXTI11 configuration" ]
        # [ inline ( always ) ]
        pub fn exti11(&mut self) -> _EXTI11W {
            _EXTI11W { w: self }
        }
    }
}
# [ doc = "External interrupt configuration register 4 (AFIO_EXTICR4)" ]
pub struct EXTICR4 {
    register: VolatileCell<u32>,
}
# [ doc = "External interrupt configuration register 4 (AFIO_EXTICR4)" ]
pub mod exticr4 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::EXTICR4 {
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
    pub struct EXTI12R {
        bits: u8,
    }
    impl EXTI12R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EXTI13R {
        bits: u8,
    }
    impl EXTI13R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EXTI14R {
        bits: u8,
    }
    impl EXTI14R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EXTI15R {
        bits: u8,
    }
    impl EXTI15R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EXTI12W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTI12W<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _EXTI13W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTI13W<'a> {
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
    pub struct _EXTI14W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTI14W<'a> {
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
    pub struct _EXTI15W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTI15W<'a> {
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:3 - EXTI12 configuration" ]
        # [ inline ( always ) ]
        pub fn exti12(&self) -> EXTI12R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EXTI12R { bits }
        }
        # [ doc = "Bits 4:7 - EXTI13 configuration" ]
        # [ inline ( always ) ]
        pub fn exti13(&self) -> EXTI13R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EXTI13R { bits }
        }
        # [ doc = "Bits 8:11 - EXTI14 configuration" ]
        # [ inline ( always ) ]
        pub fn exti14(&self) -> EXTI14R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EXTI14R { bits }
        }
        # [ doc = "Bits 12:15 - EXTI15 configuration" ]
        # [ inline ( always ) ]
        pub fn exti15(&self) -> EXTI15R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EXTI15R { bits }
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
        # [ doc = "Bits 0:3 - EXTI12 configuration" ]
        # [ inline ( always ) ]
        pub fn exti12(&mut self) -> _EXTI12W {
            _EXTI12W { w: self }
        }
        # [ doc = "Bits 4:7 - EXTI13 configuration" ]
        # [ inline ( always ) ]
        pub fn exti13(&mut self) -> _EXTI13W {
            _EXTI13W { w: self }
        }
        # [ doc = "Bits 8:11 - EXTI14 configuration" ]
        # [ inline ( always ) ]
        pub fn exti14(&mut self) -> _EXTI14W {
            _EXTI14W { w: self }
        }
        # [ doc = "Bits 12:15 - EXTI15 configuration" ]
        # [ inline ( always ) ]
        pub fn exti15(&mut self) -> _EXTI15W {
            _EXTI15W { w: self }
        }
    }
}
# [ doc = "AF remap and debug I/O configuration register" ]
pub struct MAPR2 {
    register: VolatileCell<u32>,
}
# [ doc = "AF remap and debug I/O configuration register" ]
pub mod mapr2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::MAPR2 {
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
    pub struct TIM9_REMAPR {
        bits: bool,
    }
    impl TIM9_REMAPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TIM10_REMAPR {
        bits: bool,
    }
    impl TIM10_REMAPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TIM11_REMAPR {
        bits: bool,
    }
    impl TIM11_REMAPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TIM13_REMAPR {
        bits: bool,
    }
    impl TIM13_REMAPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TIM14_REMAPR {
        bits: bool,
    }
    impl TIM14_REMAPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct FSMC_NADVR {
        bits: bool,
    }
    impl FSMC_NADVR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _TIM9_REMAPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM9_REMAPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TIM10_REMAPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM10_REMAPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TIM11_REMAPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM11_REMAPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TIM13_REMAPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM13_REMAPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TIM14_REMAPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIM14_REMAPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _FSMC_NADVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FSMC_NADVW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 5 - TIM9 remapping" ]
        # [ inline ( always ) ]
        pub fn tim9_remap(&self) -> TIM9_REMAPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIM9_REMAPR { bits }
        }
        # [ doc = "Bit 6 - TIM10 remapping" ]
        # [ inline ( always ) ]
        pub fn tim10_remap(&self) -> TIM10_REMAPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIM10_REMAPR { bits }
        }
        # [ doc = "Bit 7 - TIM11 remapping" ]
        # [ inline ( always ) ]
        pub fn tim11_remap(&self) -> TIM11_REMAPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIM11_REMAPR { bits }
        }
        # [ doc = "Bit 8 - TIM13 remapping" ]
        # [ inline ( always ) ]
        pub fn tim13_remap(&self) -> TIM13_REMAPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIM13_REMAPR { bits }
        }
        # [ doc = "Bit 9 - TIM14 remapping" ]
        # [ inline ( always ) ]
        pub fn tim14_remap(&self) -> TIM14_REMAPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIM14_REMAPR { bits }
        }
        # [ doc = "Bit 10 - NADV connect/disconnect" ]
        # [ inline ( always ) ]
        pub fn fsmc_nadv(&self) -> FSMC_NADVR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FSMC_NADVR { bits }
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
        # [ doc = "Bit 5 - TIM9 remapping" ]
        # [ inline ( always ) ]
        pub fn tim9_remap(&mut self) -> _TIM9_REMAPW {
            _TIM9_REMAPW { w: self }
        }
        # [ doc = "Bit 6 - TIM10 remapping" ]
        # [ inline ( always ) ]
        pub fn tim10_remap(&mut self) -> _TIM10_REMAPW {
            _TIM10_REMAPW { w: self }
        }
        # [ doc = "Bit 7 - TIM11 remapping" ]
        # [ inline ( always ) ]
        pub fn tim11_remap(&mut self) -> _TIM11_REMAPW {
            _TIM11_REMAPW { w: self }
        }
        # [ doc = "Bit 8 - TIM13 remapping" ]
        # [ inline ( always ) ]
        pub fn tim13_remap(&mut self) -> _TIM13_REMAPW {
            _TIM13_REMAPW { w: self }
        }
        # [ doc = "Bit 9 - TIM14 remapping" ]
        # [ inline ( always ) ]
        pub fn tim14_remap(&mut self) -> _TIM14_REMAPW {
            _TIM14_REMAPW { w: self }
        }
        # [ doc = "Bit 10 - NADV connect/disconnect" ]
        # [ inline ( always ) ]
        pub fn fsmc_nadv(&mut self) -> _FSMC_NADVW {
            _FSMC_NADVW { w: self }
        }
    }
}
# [ doc = "Alternate function I/O" ]
pub struct AFIO {
    register_block: RegisterBlock,
}
impl Deref for AFIO {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
