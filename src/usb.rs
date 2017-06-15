# ! [ doc = "Universal serial bus full-speed device interface" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;

# [ doc = "Universal serial bus full-speed device interface" ]
pub const USB: Peripheral<USB> = unsafe { Peripheral::new(1073765376) };
use vcell::VolatileCell;
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - endpoint 0 register" ]
    pub ep0r: EP0R,
    # [ doc = "0x04 - endpoint 1 register" ]
    pub ep1r: EP1R,
    # [ doc = "0x08 - endpoint 2 register" ]
    pub ep2r: EP2R,
    # [ doc = "0x0c - endpoint 3 register" ]
    pub ep3r: EP3R,
    # [ doc = "0x10 - endpoint 4 register" ]
    pub ep4r: EP4R,
    # [ doc = "0x14 - endpoint 5 register" ]
    pub ep5r: EP5R,
    # [ doc = "0x18 - endpoint 6 register" ]
    pub ep6r: EP6R,
    # [ doc = "0x1c - endpoint 7 register" ]
    pub ep7r: EP7R,
    _reserved0: [u8; 32usize],
    # [ doc = "0x40 - control register" ]
    pub cntr: CNTR,
    # [ doc = "0x44 - interrupt status register" ]
    pub istr: ISTR,
    # [ doc = "0x48 - frame number register" ]
    pub fnr: FNR,
    # [ doc = "0x4c - device address" ]
    pub daddr: DADDR,
    # [ doc = "0x50 - Buffer table address" ]
    pub btable: BTABLE,
}
# [ doc = "endpoint 0 register" ]
pub struct EP0R {
    register: VolatileCell<u32>,
}
# [ doc = "endpoint 0 register" ]
pub mod ep0r {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::EP0R {
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
    pub struct EAR {
        bits: u8,
    }
    impl EAR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct STAT_TXR {
        bits: u8,
    }
    impl STAT_TXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DTOG_TXR {
        bits: bool,
    }
    impl DTOG_TXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CTR_TXR {
        bits: bool,
    }
    impl CTR_TXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EP_KINDR {
        bits: bool,
    }
    impl EP_KINDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EP_TYPER {
        bits: u8,
    }
    impl EP_TYPER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SETUPR {
        bits: bool,
    }
    impl SETUPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct STAT_RXR {
        bits: u8,
    }
    impl STAT_RXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DTOG_RXR {
        bits: bool,
    }
    impl DTOG_RXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CTR_RXR {
        bits: bool,
    }
    impl CTR_RXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _EAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EAW<'a> {
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
    pub struct _STAT_TXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STAT_TXW<'a> {
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
    pub struct _DTOG_TXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTOG_TXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CTR_TXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CTR_TXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EP_KINDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EP_KINDW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EP_TYPEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EP_TYPEW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SETUPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SETUPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _STAT_RXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STAT_RXW<'a> {
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
    pub struct _DTOG_RXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTOG_RXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CTR_RXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CTR_RXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:3 - Endpoint address" ]
        # [ inline ( always ) ]
        pub fn ea(&self) -> EAR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EAR { bits }
        }
        # [ doc = "Bits 4:5 - Status bits, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn stat_tx(&self) -> STAT_TXR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            STAT_TXR { bits }
        }
        # [ doc = "Bit 6 - Data Toggle, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_tx(&self) -> DTOG_TXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DTOG_TXR { bits }
        }
        # [ doc = "Bit 7 - Correct Transfer for transmission" ]
        # [ inline ( always ) ]
        pub fn ctr_tx(&self) -> CTR_TXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CTR_TXR { bits }
        }
        # [ doc = "Bit 8 - Endpoint kind" ]
        # [ inline ( always ) ]
        pub fn ep_kind(&self) -> EP_KINDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EP_KINDR { bits }
        }
        # [ doc = "Bits 9:10 - Endpoint type" ]
        # [ inline ( always ) ]
        pub fn ep_type(&self) -> EP_TYPER {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EP_TYPER { bits }
        }
        # [ doc = "Bit 11 - Setup transaction completed" ]
        # [ inline ( always ) ]
        pub fn setup(&self) -> SETUPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SETUPR { bits }
        }
        # [ doc = "Bits 12:13 - Status bits, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn stat_rx(&self) -> STAT_RXR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            STAT_RXR { bits }
        }
        # [ doc = "Bit 14 - Data Toggle, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_rx(&self) -> DTOG_RXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DTOG_RXR { bits }
        }
        # [ doc = "Bit 15 - Correct transfer for reception" ]
        # [ inline ( always ) ]
        pub fn ctr_rx(&self) -> CTR_RXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CTR_RXR { bits }
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
        # [ doc = "Bits 0:3 - Endpoint address" ]
        # [ inline ( always ) ]
        pub fn ea(&mut self) -> _EAW {
            _EAW { w: self }
        }
        # [ doc = "Bits 4:5 - Status bits, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn stat_tx(&mut self) -> _STAT_TXW {
            _STAT_TXW { w: self }
        }
        # [ doc = "Bit 6 - Data Toggle, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_tx(&mut self) -> _DTOG_TXW {
            _DTOG_TXW { w: self }
        }
        # [ doc = "Bit 7 - Correct Transfer for transmission" ]
        # [ inline ( always ) ]
        pub fn ctr_tx(&mut self) -> _CTR_TXW {
            _CTR_TXW { w: self }
        }
        # [ doc = "Bit 8 - Endpoint kind" ]
        # [ inline ( always ) ]
        pub fn ep_kind(&mut self) -> _EP_KINDW {
            _EP_KINDW { w: self }
        }
        # [ doc = "Bits 9:10 - Endpoint type" ]
        # [ inline ( always ) ]
        pub fn ep_type(&mut self) -> _EP_TYPEW {
            _EP_TYPEW { w: self }
        }
        # [ doc = "Bit 11 - Setup transaction completed" ]
        # [ inline ( always ) ]
        pub fn setup(&mut self) -> _SETUPW {
            _SETUPW { w: self }
        }
        # [ doc = "Bits 12:13 - Status bits, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn stat_rx(&mut self) -> _STAT_RXW {
            _STAT_RXW { w: self }
        }
        # [ doc = "Bit 14 - Data Toggle, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_rx(&mut self) -> _DTOG_RXW {
            _DTOG_RXW { w: self }
        }
        # [ doc = "Bit 15 - Correct transfer for reception" ]
        # [ inline ( always ) ]
        pub fn ctr_rx(&mut self) -> _CTR_RXW {
            _CTR_RXW { w: self }
        }
    }
}
# [ doc = "endpoint 1 register" ]
pub struct EP1R {
    register: VolatileCell<u32>,
}
# [ doc = "endpoint 1 register" ]
pub mod ep1r {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::EP1R {
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
    pub struct EAR {
        bits: u8,
    }
    impl EAR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct STAT_TXR {
        bits: u8,
    }
    impl STAT_TXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DTOG_TXR {
        bits: bool,
    }
    impl DTOG_TXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CTR_TXR {
        bits: bool,
    }
    impl CTR_TXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EP_KINDR {
        bits: bool,
    }
    impl EP_KINDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EP_TYPER {
        bits: u8,
    }
    impl EP_TYPER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SETUPR {
        bits: bool,
    }
    impl SETUPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct STAT_RXR {
        bits: u8,
    }
    impl STAT_RXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DTOG_RXR {
        bits: bool,
    }
    impl DTOG_RXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CTR_RXR {
        bits: bool,
    }
    impl CTR_RXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _EAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EAW<'a> {
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
    pub struct _STAT_TXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STAT_TXW<'a> {
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
    pub struct _DTOG_TXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTOG_TXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CTR_TXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CTR_TXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EP_KINDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EP_KINDW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EP_TYPEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EP_TYPEW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SETUPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SETUPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _STAT_RXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STAT_RXW<'a> {
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
    pub struct _DTOG_RXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTOG_RXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CTR_RXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CTR_RXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:3 - Endpoint address" ]
        # [ inline ( always ) ]
        pub fn ea(&self) -> EAR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EAR { bits }
        }
        # [ doc = "Bits 4:5 - Status bits, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn stat_tx(&self) -> STAT_TXR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            STAT_TXR { bits }
        }
        # [ doc = "Bit 6 - Data Toggle, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_tx(&self) -> DTOG_TXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DTOG_TXR { bits }
        }
        # [ doc = "Bit 7 - Correct Transfer for transmission" ]
        # [ inline ( always ) ]
        pub fn ctr_tx(&self) -> CTR_TXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CTR_TXR { bits }
        }
        # [ doc = "Bit 8 - Endpoint kind" ]
        # [ inline ( always ) ]
        pub fn ep_kind(&self) -> EP_KINDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EP_KINDR { bits }
        }
        # [ doc = "Bits 9:10 - Endpoint type" ]
        # [ inline ( always ) ]
        pub fn ep_type(&self) -> EP_TYPER {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EP_TYPER { bits }
        }
        # [ doc = "Bit 11 - Setup transaction completed" ]
        # [ inline ( always ) ]
        pub fn setup(&self) -> SETUPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SETUPR { bits }
        }
        # [ doc = "Bits 12:13 - Status bits, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn stat_rx(&self) -> STAT_RXR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            STAT_RXR { bits }
        }
        # [ doc = "Bit 14 - Data Toggle, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_rx(&self) -> DTOG_RXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DTOG_RXR { bits }
        }
        # [ doc = "Bit 15 - Correct transfer for reception" ]
        # [ inline ( always ) ]
        pub fn ctr_rx(&self) -> CTR_RXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CTR_RXR { bits }
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
        # [ doc = "Bits 0:3 - Endpoint address" ]
        # [ inline ( always ) ]
        pub fn ea(&mut self) -> _EAW {
            _EAW { w: self }
        }
        # [ doc = "Bits 4:5 - Status bits, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn stat_tx(&mut self) -> _STAT_TXW {
            _STAT_TXW { w: self }
        }
        # [ doc = "Bit 6 - Data Toggle, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_tx(&mut self) -> _DTOG_TXW {
            _DTOG_TXW { w: self }
        }
        # [ doc = "Bit 7 - Correct Transfer for transmission" ]
        # [ inline ( always ) ]
        pub fn ctr_tx(&mut self) -> _CTR_TXW {
            _CTR_TXW { w: self }
        }
        # [ doc = "Bit 8 - Endpoint kind" ]
        # [ inline ( always ) ]
        pub fn ep_kind(&mut self) -> _EP_KINDW {
            _EP_KINDW { w: self }
        }
        # [ doc = "Bits 9:10 - Endpoint type" ]
        # [ inline ( always ) ]
        pub fn ep_type(&mut self) -> _EP_TYPEW {
            _EP_TYPEW { w: self }
        }
        # [ doc = "Bit 11 - Setup transaction completed" ]
        # [ inline ( always ) ]
        pub fn setup(&mut self) -> _SETUPW {
            _SETUPW { w: self }
        }
        # [ doc = "Bits 12:13 - Status bits, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn stat_rx(&mut self) -> _STAT_RXW {
            _STAT_RXW { w: self }
        }
        # [ doc = "Bit 14 - Data Toggle, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_rx(&mut self) -> _DTOG_RXW {
            _DTOG_RXW { w: self }
        }
        # [ doc = "Bit 15 - Correct transfer for reception" ]
        # [ inline ( always ) ]
        pub fn ctr_rx(&mut self) -> _CTR_RXW {
            _CTR_RXW { w: self }
        }
    }
}
# [ doc = "endpoint 2 register" ]
pub struct EP2R {
    register: VolatileCell<u32>,
}
# [ doc = "endpoint 2 register" ]
pub mod ep2r {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::EP2R {
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
    pub struct EAR {
        bits: u8,
    }
    impl EAR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct STAT_TXR {
        bits: u8,
    }
    impl STAT_TXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DTOG_TXR {
        bits: bool,
    }
    impl DTOG_TXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CTR_TXR {
        bits: bool,
    }
    impl CTR_TXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EP_KINDR {
        bits: bool,
    }
    impl EP_KINDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EP_TYPER {
        bits: u8,
    }
    impl EP_TYPER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SETUPR {
        bits: bool,
    }
    impl SETUPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct STAT_RXR {
        bits: u8,
    }
    impl STAT_RXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DTOG_RXR {
        bits: bool,
    }
    impl DTOG_RXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CTR_RXR {
        bits: bool,
    }
    impl CTR_RXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _EAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EAW<'a> {
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
    pub struct _STAT_TXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STAT_TXW<'a> {
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
    pub struct _DTOG_TXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTOG_TXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CTR_TXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CTR_TXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EP_KINDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EP_KINDW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EP_TYPEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EP_TYPEW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SETUPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SETUPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _STAT_RXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STAT_RXW<'a> {
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
    pub struct _DTOG_RXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTOG_RXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CTR_RXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CTR_RXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:3 - Endpoint address" ]
        # [ inline ( always ) ]
        pub fn ea(&self) -> EAR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EAR { bits }
        }
        # [ doc = "Bits 4:5 - Status bits, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn stat_tx(&self) -> STAT_TXR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            STAT_TXR { bits }
        }
        # [ doc = "Bit 6 - Data Toggle, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_tx(&self) -> DTOG_TXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DTOG_TXR { bits }
        }
        # [ doc = "Bit 7 - Correct Transfer for transmission" ]
        # [ inline ( always ) ]
        pub fn ctr_tx(&self) -> CTR_TXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CTR_TXR { bits }
        }
        # [ doc = "Bit 8 - Endpoint kind" ]
        # [ inline ( always ) ]
        pub fn ep_kind(&self) -> EP_KINDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EP_KINDR { bits }
        }
        # [ doc = "Bits 9:10 - Endpoint type" ]
        # [ inline ( always ) ]
        pub fn ep_type(&self) -> EP_TYPER {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EP_TYPER { bits }
        }
        # [ doc = "Bit 11 - Setup transaction completed" ]
        # [ inline ( always ) ]
        pub fn setup(&self) -> SETUPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SETUPR { bits }
        }
        # [ doc = "Bits 12:13 - Status bits, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn stat_rx(&self) -> STAT_RXR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            STAT_RXR { bits }
        }
        # [ doc = "Bit 14 - Data Toggle, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_rx(&self) -> DTOG_RXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DTOG_RXR { bits }
        }
        # [ doc = "Bit 15 - Correct transfer for reception" ]
        # [ inline ( always ) ]
        pub fn ctr_rx(&self) -> CTR_RXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CTR_RXR { bits }
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
        # [ doc = "Bits 0:3 - Endpoint address" ]
        # [ inline ( always ) ]
        pub fn ea(&mut self) -> _EAW {
            _EAW { w: self }
        }
        # [ doc = "Bits 4:5 - Status bits, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn stat_tx(&mut self) -> _STAT_TXW {
            _STAT_TXW { w: self }
        }
        # [ doc = "Bit 6 - Data Toggle, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_tx(&mut self) -> _DTOG_TXW {
            _DTOG_TXW { w: self }
        }
        # [ doc = "Bit 7 - Correct Transfer for transmission" ]
        # [ inline ( always ) ]
        pub fn ctr_tx(&mut self) -> _CTR_TXW {
            _CTR_TXW { w: self }
        }
        # [ doc = "Bit 8 - Endpoint kind" ]
        # [ inline ( always ) ]
        pub fn ep_kind(&mut self) -> _EP_KINDW {
            _EP_KINDW { w: self }
        }
        # [ doc = "Bits 9:10 - Endpoint type" ]
        # [ inline ( always ) ]
        pub fn ep_type(&mut self) -> _EP_TYPEW {
            _EP_TYPEW { w: self }
        }
        # [ doc = "Bit 11 - Setup transaction completed" ]
        # [ inline ( always ) ]
        pub fn setup(&mut self) -> _SETUPW {
            _SETUPW { w: self }
        }
        # [ doc = "Bits 12:13 - Status bits, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn stat_rx(&mut self) -> _STAT_RXW {
            _STAT_RXW { w: self }
        }
        # [ doc = "Bit 14 - Data Toggle, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_rx(&mut self) -> _DTOG_RXW {
            _DTOG_RXW { w: self }
        }
        # [ doc = "Bit 15 - Correct transfer for reception" ]
        # [ inline ( always ) ]
        pub fn ctr_rx(&mut self) -> _CTR_RXW {
            _CTR_RXW { w: self }
        }
    }
}
# [ doc = "endpoint 3 register" ]
pub struct EP3R {
    register: VolatileCell<u32>,
}
# [ doc = "endpoint 3 register" ]
pub mod ep3r {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::EP3R {
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
    pub struct EAR {
        bits: u8,
    }
    impl EAR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct STAT_TXR {
        bits: u8,
    }
    impl STAT_TXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DTOG_TXR {
        bits: bool,
    }
    impl DTOG_TXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CTR_TXR {
        bits: bool,
    }
    impl CTR_TXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EP_KINDR {
        bits: bool,
    }
    impl EP_KINDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EP_TYPER {
        bits: u8,
    }
    impl EP_TYPER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SETUPR {
        bits: bool,
    }
    impl SETUPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct STAT_RXR {
        bits: u8,
    }
    impl STAT_RXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DTOG_RXR {
        bits: bool,
    }
    impl DTOG_RXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CTR_RXR {
        bits: bool,
    }
    impl CTR_RXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _EAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EAW<'a> {
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
    pub struct _STAT_TXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STAT_TXW<'a> {
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
    pub struct _DTOG_TXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTOG_TXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CTR_TXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CTR_TXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EP_KINDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EP_KINDW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EP_TYPEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EP_TYPEW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SETUPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SETUPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _STAT_RXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STAT_RXW<'a> {
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
    pub struct _DTOG_RXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTOG_RXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CTR_RXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CTR_RXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:3 - Endpoint address" ]
        # [ inline ( always ) ]
        pub fn ea(&self) -> EAR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EAR { bits }
        }
        # [ doc = "Bits 4:5 - Status bits, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn stat_tx(&self) -> STAT_TXR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            STAT_TXR { bits }
        }
        # [ doc = "Bit 6 - Data Toggle, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_tx(&self) -> DTOG_TXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DTOG_TXR { bits }
        }
        # [ doc = "Bit 7 - Correct Transfer for transmission" ]
        # [ inline ( always ) ]
        pub fn ctr_tx(&self) -> CTR_TXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CTR_TXR { bits }
        }
        # [ doc = "Bit 8 - Endpoint kind" ]
        # [ inline ( always ) ]
        pub fn ep_kind(&self) -> EP_KINDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EP_KINDR { bits }
        }
        # [ doc = "Bits 9:10 - Endpoint type" ]
        # [ inline ( always ) ]
        pub fn ep_type(&self) -> EP_TYPER {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EP_TYPER { bits }
        }
        # [ doc = "Bit 11 - Setup transaction completed" ]
        # [ inline ( always ) ]
        pub fn setup(&self) -> SETUPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SETUPR { bits }
        }
        # [ doc = "Bits 12:13 - Status bits, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn stat_rx(&self) -> STAT_RXR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            STAT_RXR { bits }
        }
        # [ doc = "Bit 14 - Data Toggle, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_rx(&self) -> DTOG_RXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DTOG_RXR { bits }
        }
        # [ doc = "Bit 15 - Correct transfer for reception" ]
        # [ inline ( always ) ]
        pub fn ctr_rx(&self) -> CTR_RXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CTR_RXR { bits }
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
        # [ doc = "Bits 0:3 - Endpoint address" ]
        # [ inline ( always ) ]
        pub fn ea(&mut self) -> _EAW {
            _EAW { w: self }
        }
        # [ doc = "Bits 4:5 - Status bits, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn stat_tx(&mut self) -> _STAT_TXW {
            _STAT_TXW { w: self }
        }
        # [ doc = "Bit 6 - Data Toggle, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_tx(&mut self) -> _DTOG_TXW {
            _DTOG_TXW { w: self }
        }
        # [ doc = "Bit 7 - Correct Transfer for transmission" ]
        # [ inline ( always ) ]
        pub fn ctr_tx(&mut self) -> _CTR_TXW {
            _CTR_TXW { w: self }
        }
        # [ doc = "Bit 8 - Endpoint kind" ]
        # [ inline ( always ) ]
        pub fn ep_kind(&mut self) -> _EP_KINDW {
            _EP_KINDW { w: self }
        }
        # [ doc = "Bits 9:10 - Endpoint type" ]
        # [ inline ( always ) ]
        pub fn ep_type(&mut self) -> _EP_TYPEW {
            _EP_TYPEW { w: self }
        }
        # [ doc = "Bit 11 - Setup transaction completed" ]
        # [ inline ( always ) ]
        pub fn setup(&mut self) -> _SETUPW {
            _SETUPW { w: self }
        }
        # [ doc = "Bits 12:13 - Status bits, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn stat_rx(&mut self) -> _STAT_RXW {
            _STAT_RXW { w: self }
        }
        # [ doc = "Bit 14 - Data Toggle, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_rx(&mut self) -> _DTOG_RXW {
            _DTOG_RXW { w: self }
        }
        # [ doc = "Bit 15 - Correct transfer for reception" ]
        # [ inline ( always ) ]
        pub fn ctr_rx(&mut self) -> _CTR_RXW {
            _CTR_RXW { w: self }
        }
    }
}
# [ doc = "endpoint 4 register" ]
pub struct EP4R {
    register: VolatileCell<u32>,
}
# [ doc = "endpoint 4 register" ]
pub mod ep4r {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::EP4R {
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
    pub struct EAR {
        bits: u8,
    }
    impl EAR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct STAT_TXR {
        bits: u8,
    }
    impl STAT_TXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DTOG_TXR {
        bits: bool,
    }
    impl DTOG_TXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CTR_TXR {
        bits: bool,
    }
    impl CTR_TXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EP_KINDR {
        bits: bool,
    }
    impl EP_KINDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EP_TYPER {
        bits: u8,
    }
    impl EP_TYPER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SETUPR {
        bits: bool,
    }
    impl SETUPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct STAT_RXR {
        bits: u8,
    }
    impl STAT_RXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DTOG_RXR {
        bits: bool,
    }
    impl DTOG_RXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CTR_RXR {
        bits: bool,
    }
    impl CTR_RXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _EAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EAW<'a> {
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
    pub struct _STAT_TXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STAT_TXW<'a> {
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
    pub struct _DTOG_TXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTOG_TXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CTR_TXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CTR_TXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EP_KINDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EP_KINDW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EP_TYPEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EP_TYPEW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SETUPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SETUPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _STAT_RXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STAT_RXW<'a> {
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
    pub struct _DTOG_RXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTOG_RXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CTR_RXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CTR_RXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:3 - Endpoint address" ]
        # [ inline ( always ) ]
        pub fn ea(&self) -> EAR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EAR { bits }
        }
        # [ doc = "Bits 4:5 - Status bits, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn stat_tx(&self) -> STAT_TXR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            STAT_TXR { bits }
        }
        # [ doc = "Bit 6 - Data Toggle, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_tx(&self) -> DTOG_TXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DTOG_TXR { bits }
        }
        # [ doc = "Bit 7 - Correct Transfer for transmission" ]
        # [ inline ( always ) ]
        pub fn ctr_tx(&self) -> CTR_TXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CTR_TXR { bits }
        }
        # [ doc = "Bit 8 - Endpoint kind" ]
        # [ inline ( always ) ]
        pub fn ep_kind(&self) -> EP_KINDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EP_KINDR { bits }
        }
        # [ doc = "Bits 9:10 - Endpoint type" ]
        # [ inline ( always ) ]
        pub fn ep_type(&self) -> EP_TYPER {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EP_TYPER { bits }
        }
        # [ doc = "Bit 11 - Setup transaction completed" ]
        # [ inline ( always ) ]
        pub fn setup(&self) -> SETUPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SETUPR { bits }
        }
        # [ doc = "Bits 12:13 - Status bits, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn stat_rx(&self) -> STAT_RXR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            STAT_RXR { bits }
        }
        # [ doc = "Bit 14 - Data Toggle, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_rx(&self) -> DTOG_RXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DTOG_RXR { bits }
        }
        # [ doc = "Bit 15 - Correct transfer for reception" ]
        # [ inline ( always ) ]
        pub fn ctr_rx(&self) -> CTR_RXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CTR_RXR { bits }
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
        # [ doc = "Bits 0:3 - Endpoint address" ]
        # [ inline ( always ) ]
        pub fn ea(&mut self) -> _EAW {
            _EAW { w: self }
        }
        # [ doc = "Bits 4:5 - Status bits, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn stat_tx(&mut self) -> _STAT_TXW {
            _STAT_TXW { w: self }
        }
        # [ doc = "Bit 6 - Data Toggle, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_tx(&mut self) -> _DTOG_TXW {
            _DTOG_TXW { w: self }
        }
        # [ doc = "Bit 7 - Correct Transfer for transmission" ]
        # [ inline ( always ) ]
        pub fn ctr_tx(&mut self) -> _CTR_TXW {
            _CTR_TXW { w: self }
        }
        # [ doc = "Bit 8 - Endpoint kind" ]
        # [ inline ( always ) ]
        pub fn ep_kind(&mut self) -> _EP_KINDW {
            _EP_KINDW { w: self }
        }
        # [ doc = "Bits 9:10 - Endpoint type" ]
        # [ inline ( always ) ]
        pub fn ep_type(&mut self) -> _EP_TYPEW {
            _EP_TYPEW { w: self }
        }
        # [ doc = "Bit 11 - Setup transaction completed" ]
        # [ inline ( always ) ]
        pub fn setup(&mut self) -> _SETUPW {
            _SETUPW { w: self }
        }
        # [ doc = "Bits 12:13 - Status bits, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn stat_rx(&mut self) -> _STAT_RXW {
            _STAT_RXW { w: self }
        }
        # [ doc = "Bit 14 - Data Toggle, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_rx(&mut self) -> _DTOG_RXW {
            _DTOG_RXW { w: self }
        }
        # [ doc = "Bit 15 - Correct transfer for reception" ]
        # [ inline ( always ) ]
        pub fn ctr_rx(&mut self) -> _CTR_RXW {
            _CTR_RXW { w: self }
        }
    }
}
# [ doc = "endpoint 5 register" ]
pub struct EP5R {
    register: VolatileCell<u32>,
}
# [ doc = "endpoint 5 register" ]
pub mod ep5r {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::EP5R {
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
    pub struct EAR {
        bits: u8,
    }
    impl EAR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct STAT_TXR {
        bits: u8,
    }
    impl STAT_TXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DTOG_TXR {
        bits: bool,
    }
    impl DTOG_TXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CTR_TXR {
        bits: bool,
    }
    impl CTR_TXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EP_KINDR {
        bits: bool,
    }
    impl EP_KINDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EP_TYPER {
        bits: u8,
    }
    impl EP_TYPER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SETUPR {
        bits: bool,
    }
    impl SETUPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct STAT_RXR {
        bits: u8,
    }
    impl STAT_RXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DTOG_RXR {
        bits: bool,
    }
    impl DTOG_RXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CTR_RXR {
        bits: bool,
    }
    impl CTR_RXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _EAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EAW<'a> {
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
    pub struct _STAT_TXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STAT_TXW<'a> {
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
    pub struct _DTOG_TXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTOG_TXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CTR_TXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CTR_TXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EP_KINDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EP_KINDW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EP_TYPEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EP_TYPEW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SETUPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SETUPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _STAT_RXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STAT_RXW<'a> {
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
    pub struct _DTOG_RXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTOG_RXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CTR_RXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CTR_RXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:3 - Endpoint address" ]
        # [ inline ( always ) ]
        pub fn ea(&self) -> EAR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EAR { bits }
        }
        # [ doc = "Bits 4:5 - Status bits, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn stat_tx(&self) -> STAT_TXR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            STAT_TXR { bits }
        }
        # [ doc = "Bit 6 - Data Toggle, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_tx(&self) -> DTOG_TXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DTOG_TXR { bits }
        }
        # [ doc = "Bit 7 - Correct Transfer for transmission" ]
        # [ inline ( always ) ]
        pub fn ctr_tx(&self) -> CTR_TXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CTR_TXR { bits }
        }
        # [ doc = "Bit 8 - Endpoint kind" ]
        # [ inline ( always ) ]
        pub fn ep_kind(&self) -> EP_KINDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EP_KINDR { bits }
        }
        # [ doc = "Bits 9:10 - Endpoint type" ]
        # [ inline ( always ) ]
        pub fn ep_type(&self) -> EP_TYPER {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EP_TYPER { bits }
        }
        # [ doc = "Bit 11 - Setup transaction completed" ]
        # [ inline ( always ) ]
        pub fn setup(&self) -> SETUPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SETUPR { bits }
        }
        # [ doc = "Bits 12:13 - Status bits, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn stat_rx(&self) -> STAT_RXR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            STAT_RXR { bits }
        }
        # [ doc = "Bit 14 - Data Toggle, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_rx(&self) -> DTOG_RXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DTOG_RXR { bits }
        }
        # [ doc = "Bit 15 - Correct transfer for reception" ]
        # [ inline ( always ) ]
        pub fn ctr_rx(&self) -> CTR_RXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CTR_RXR { bits }
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
        # [ doc = "Bits 0:3 - Endpoint address" ]
        # [ inline ( always ) ]
        pub fn ea(&mut self) -> _EAW {
            _EAW { w: self }
        }
        # [ doc = "Bits 4:5 - Status bits, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn stat_tx(&mut self) -> _STAT_TXW {
            _STAT_TXW { w: self }
        }
        # [ doc = "Bit 6 - Data Toggle, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_tx(&mut self) -> _DTOG_TXW {
            _DTOG_TXW { w: self }
        }
        # [ doc = "Bit 7 - Correct Transfer for transmission" ]
        # [ inline ( always ) ]
        pub fn ctr_tx(&mut self) -> _CTR_TXW {
            _CTR_TXW { w: self }
        }
        # [ doc = "Bit 8 - Endpoint kind" ]
        # [ inline ( always ) ]
        pub fn ep_kind(&mut self) -> _EP_KINDW {
            _EP_KINDW { w: self }
        }
        # [ doc = "Bits 9:10 - Endpoint type" ]
        # [ inline ( always ) ]
        pub fn ep_type(&mut self) -> _EP_TYPEW {
            _EP_TYPEW { w: self }
        }
        # [ doc = "Bit 11 - Setup transaction completed" ]
        # [ inline ( always ) ]
        pub fn setup(&mut self) -> _SETUPW {
            _SETUPW { w: self }
        }
        # [ doc = "Bits 12:13 - Status bits, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn stat_rx(&mut self) -> _STAT_RXW {
            _STAT_RXW { w: self }
        }
        # [ doc = "Bit 14 - Data Toggle, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_rx(&mut self) -> _DTOG_RXW {
            _DTOG_RXW { w: self }
        }
        # [ doc = "Bit 15 - Correct transfer for reception" ]
        # [ inline ( always ) ]
        pub fn ctr_rx(&mut self) -> _CTR_RXW {
            _CTR_RXW { w: self }
        }
    }
}
# [ doc = "endpoint 6 register" ]
pub struct EP6R {
    register: VolatileCell<u32>,
}
# [ doc = "endpoint 6 register" ]
pub mod ep6r {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::EP6R {
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
    pub struct EAR {
        bits: u8,
    }
    impl EAR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct STAT_TXR {
        bits: u8,
    }
    impl STAT_TXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DTOG_TXR {
        bits: bool,
    }
    impl DTOG_TXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CTR_TXR {
        bits: bool,
    }
    impl CTR_TXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EP_KINDR {
        bits: bool,
    }
    impl EP_KINDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EP_TYPER {
        bits: u8,
    }
    impl EP_TYPER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SETUPR {
        bits: bool,
    }
    impl SETUPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct STAT_RXR {
        bits: u8,
    }
    impl STAT_RXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DTOG_RXR {
        bits: bool,
    }
    impl DTOG_RXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CTR_RXR {
        bits: bool,
    }
    impl CTR_RXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _EAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EAW<'a> {
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
    pub struct _STAT_TXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STAT_TXW<'a> {
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
    pub struct _DTOG_TXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTOG_TXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CTR_TXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CTR_TXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EP_KINDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EP_KINDW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EP_TYPEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EP_TYPEW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SETUPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SETUPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _STAT_RXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STAT_RXW<'a> {
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
    pub struct _DTOG_RXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTOG_RXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CTR_RXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CTR_RXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:3 - Endpoint address" ]
        # [ inline ( always ) ]
        pub fn ea(&self) -> EAR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EAR { bits }
        }
        # [ doc = "Bits 4:5 - Status bits, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn stat_tx(&self) -> STAT_TXR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            STAT_TXR { bits }
        }
        # [ doc = "Bit 6 - Data Toggle, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_tx(&self) -> DTOG_TXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DTOG_TXR { bits }
        }
        # [ doc = "Bit 7 - Correct Transfer for transmission" ]
        # [ inline ( always ) ]
        pub fn ctr_tx(&self) -> CTR_TXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CTR_TXR { bits }
        }
        # [ doc = "Bit 8 - Endpoint kind" ]
        # [ inline ( always ) ]
        pub fn ep_kind(&self) -> EP_KINDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EP_KINDR { bits }
        }
        # [ doc = "Bits 9:10 - Endpoint type" ]
        # [ inline ( always ) ]
        pub fn ep_type(&self) -> EP_TYPER {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EP_TYPER { bits }
        }
        # [ doc = "Bit 11 - Setup transaction completed" ]
        # [ inline ( always ) ]
        pub fn setup(&self) -> SETUPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SETUPR { bits }
        }
        # [ doc = "Bits 12:13 - Status bits, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn stat_rx(&self) -> STAT_RXR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            STAT_RXR { bits }
        }
        # [ doc = "Bit 14 - Data Toggle, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_rx(&self) -> DTOG_RXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DTOG_RXR { bits }
        }
        # [ doc = "Bit 15 - Correct transfer for reception" ]
        # [ inline ( always ) ]
        pub fn ctr_rx(&self) -> CTR_RXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CTR_RXR { bits }
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
        # [ doc = "Bits 0:3 - Endpoint address" ]
        # [ inline ( always ) ]
        pub fn ea(&mut self) -> _EAW {
            _EAW { w: self }
        }
        # [ doc = "Bits 4:5 - Status bits, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn stat_tx(&mut self) -> _STAT_TXW {
            _STAT_TXW { w: self }
        }
        # [ doc = "Bit 6 - Data Toggle, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_tx(&mut self) -> _DTOG_TXW {
            _DTOG_TXW { w: self }
        }
        # [ doc = "Bit 7 - Correct Transfer for transmission" ]
        # [ inline ( always ) ]
        pub fn ctr_tx(&mut self) -> _CTR_TXW {
            _CTR_TXW { w: self }
        }
        # [ doc = "Bit 8 - Endpoint kind" ]
        # [ inline ( always ) ]
        pub fn ep_kind(&mut self) -> _EP_KINDW {
            _EP_KINDW { w: self }
        }
        # [ doc = "Bits 9:10 - Endpoint type" ]
        # [ inline ( always ) ]
        pub fn ep_type(&mut self) -> _EP_TYPEW {
            _EP_TYPEW { w: self }
        }
        # [ doc = "Bit 11 - Setup transaction completed" ]
        # [ inline ( always ) ]
        pub fn setup(&mut self) -> _SETUPW {
            _SETUPW { w: self }
        }
        # [ doc = "Bits 12:13 - Status bits, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn stat_rx(&mut self) -> _STAT_RXW {
            _STAT_RXW { w: self }
        }
        # [ doc = "Bit 14 - Data Toggle, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_rx(&mut self) -> _DTOG_RXW {
            _DTOG_RXW { w: self }
        }
        # [ doc = "Bit 15 - Correct transfer for reception" ]
        # [ inline ( always ) ]
        pub fn ctr_rx(&mut self) -> _CTR_RXW {
            _CTR_RXW { w: self }
        }
    }
}
# [ doc = "endpoint 7 register" ]
pub struct EP7R {
    register: VolatileCell<u32>,
}
# [ doc = "endpoint 7 register" ]
pub mod ep7r {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::EP7R {
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
    pub struct EAR {
        bits: u8,
    }
    impl EAR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct STAT_TXR {
        bits: u8,
    }
    impl STAT_TXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DTOG_TXR {
        bits: bool,
    }
    impl DTOG_TXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CTR_TXR {
        bits: bool,
    }
    impl CTR_TXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EP_KINDR {
        bits: bool,
    }
    impl EP_KINDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EP_TYPER {
        bits: u8,
    }
    impl EP_TYPER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SETUPR {
        bits: bool,
    }
    impl SETUPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct STAT_RXR {
        bits: u8,
    }
    impl STAT_RXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DTOG_RXR {
        bits: bool,
    }
    impl DTOG_RXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CTR_RXR {
        bits: bool,
    }
    impl CTR_RXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _EAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EAW<'a> {
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
    pub struct _STAT_TXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STAT_TXW<'a> {
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
    pub struct _DTOG_TXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTOG_TXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CTR_TXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CTR_TXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EP_KINDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EP_KINDW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EP_TYPEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EP_TYPEW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SETUPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SETUPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _STAT_RXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STAT_RXW<'a> {
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
    pub struct _DTOG_RXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTOG_RXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CTR_RXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CTR_RXW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:3 - Endpoint address" ]
        # [ inline ( always ) ]
        pub fn ea(&self) -> EAR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EAR { bits }
        }
        # [ doc = "Bits 4:5 - Status bits, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn stat_tx(&self) -> STAT_TXR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            STAT_TXR { bits }
        }
        # [ doc = "Bit 6 - Data Toggle, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_tx(&self) -> DTOG_TXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DTOG_TXR { bits }
        }
        # [ doc = "Bit 7 - Correct Transfer for transmission" ]
        # [ inline ( always ) ]
        pub fn ctr_tx(&self) -> CTR_TXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CTR_TXR { bits }
        }
        # [ doc = "Bit 8 - Endpoint kind" ]
        # [ inline ( always ) ]
        pub fn ep_kind(&self) -> EP_KINDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EP_KINDR { bits }
        }
        # [ doc = "Bits 9:10 - Endpoint type" ]
        # [ inline ( always ) ]
        pub fn ep_type(&self) -> EP_TYPER {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EP_TYPER { bits }
        }
        # [ doc = "Bit 11 - Setup transaction completed" ]
        # [ inline ( always ) ]
        pub fn setup(&self) -> SETUPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SETUPR { bits }
        }
        # [ doc = "Bits 12:13 - Status bits, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn stat_rx(&self) -> STAT_RXR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            STAT_RXR { bits }
        }
        # [ doc = "Bit 14 - Data Toggle, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_rx(&self) -> DTOG_RXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DTOG_RXR { bits }
        }
        # [ doc = "Bit 15 - Correct transfer for reception" ]
        # [ inline ( always ) ]
        pub fn ctr_rx(&self) -> CTR_RXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CTR_RXR { bits }
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
        # [ doc = "Bits 0:3 - Endpoint address" ]
        # [ inline ( always ) ]
        pub fn ea(&mut self) -> _EAW {
            _EAW { w: self }
        }
        # [ doc = "Bits 4:5 - Status bits, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn stat_tx(&mut self) -> _STAT_TXW {
            _STAT_TXW { w: self }
        }
        # [ doc = "Bit 6 - Data Toggle, for transmission transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_tx(&mut self) -> _DTOG_TXW {
            _DTOG_TXW { w: self }
        }
        # [ doc = "Bit 7 - Correct Transfer for transmission" ]
        # [ inline ( always ) ]
        pub fn ctr_tx(&mut self) -> _CTR_TXW {
            _CTR_TXW { w: self }
        }
        # [ doc = "Bit 8 - Endpoint kind" ]
        # [ inline ( always ) ]
        pub fn ep_kind(&mut self) -> _EP_KINDW {
            _EP_KINDW { w: self }
        }
        # [ doc = "Bits 9:10 - Endpoint type" ]
        # [ inline ( always ) ]
        pub fn ep_type(&mut self) -> _EP_TYPEW {
            _EP_TYPEW { w: self }
        }
        # [ doc = "Bit 11 - Setup transaction completed" ]
        # [ inline ( always ) ]
        pub fn setup(&mut self) -> _SETUPW {
            _SETUPW { w: self }
        }
        # [ doc = "Bits 12:13 - Status bits, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn stat_rx(&mut self) -> _STAT_RXW {
            _STAT_RXW { w: self }
        }
        # [ doc = "Bit 14 - Data Toggle, for reception transfers" ]
        # [ inline ( always ) ]
        pub fn dtog_rx(&mut self) -> _DTOG_RXW {
            _DTOG_RXW { w: self }
        }
        # [ doc = "Bit 15 - Correct transfer for reception" ]
        # [ inline ( always ) ]
        pub fn ctr_rx(&mut self) -> _CTR_RXW {
            _CTR_RXW { w: self }
        }
    }
}
# [ doc = "control register" ]
pub struct CNTR {
    register: VolatileCell<u32>,
}
# [ doc = "control register" ]
pub mod cntr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CNTR {
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
    pub struct FRESR {
        bits: bool,
    }
    impl FRESR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct PDWNR {
        bits: bool,
    }
    impl PDWNR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct LPMODER {
        bits: bool,
    }
    impl LPMODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct FSUSPR {
        bits: bool,
    }
    impl FSUSPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RESUMER {
        bits: bool,
    }
    impl RESUMER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ESOFMR {
        bits: bool,
    }
    impl ESOFMR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SOFMR {
        bits: bool,
    }
    impl SOFMR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RESETMR {
        bits: bool,
    }
    impl RESETMR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SUSPMR {
        bits: bool,
    }
    impl SUSPMR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WKUPMR {
        bits: bool,
    }
    impl WKUPMR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ERRMR {
        bits: bool,
    }
    impl ERRMR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct PMAOVRMR {
        bits: bool,
    }
    impl PMAOVRMR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CTRMR {
        bits: bool,
    }
    impl CTRMR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _FRESW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FRESW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _PDWNW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PDWNW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LPMODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LPMODEW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _FSUSPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FSUSPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _RESUMEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RESUMEW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ESOFMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ESOFMW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SOFMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SOFMW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _RESETMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RESETMW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SUSPMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SUSPMW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _WKUPMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WKUPMW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ERRMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ERRMW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _PMAOVRMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PMAOVRMW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CTRMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CTRMW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Force USB Reset" ]
        # [ inline ( always ) ]
        pub fn fres(&self) -> FRESR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FRESR { bits }
        }
        # [ doc = "Bit 1 - Power down" ]
        # [ inline ( always ) ]
        pub fn pdwn(&self) -> PDWNR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PDWNR { bits }
        }
        # [ doc = "Bit 2 - Low-power mode" ]
        # [ inline ( always ) ]
        pub fn lpmode(&self) -> LPMODER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LPMODER { bits }
        }
        # [ doc = "Bit 3 - Force suspend" ]
        # [ inline ( always ) ]
        pub fn fsusp(&self) -> FSUSPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FSUSPR { bits }
        }
        # [ doc = "Bit 4 - Resume request" ]
        # [ inline ( always ) ]
        pub fn resume(&self) -> RESUMER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RESUMER { bits }
        }
        # [ doc = "Bit 8 - Expected start of frame interrupt mask" ]
        # [ inline ( always ) ]
        pub fn esofm(&self) -> ESOFMR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ESOFMR { bits }
        }
        # [ doc = "Bit 9 - Start of frame interrupt mask" ]
        # [ inline ( always ) ]
        pub fn sofm(&self) -> SOFMR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SOFMR { bits }
        }
        # [ doc = "Bit 10 - USB reset interrupt mask" ]
        # [ inline ( always ) ]
        pub fn resetm(&self) -> RESETMR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RESETMR { bits }
        }
        # [ doc = "Bit 11 - Suspend mode interrupt mask" ]
        # [ inline ( always ) ]
        pub fn suspm(&self) -> SUSPMR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SUSPMR { bits }
        }
        # [ doc = "Bit 12 - Wakeup interrupt mask" ]
        # [ inline ( always ) ]
        pub fn wkupm(&self) -> WKUPMR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WKUPMR { bits }
        }
        # [ doc = "Bit 13 - Error interrupt mask" ]
        # [ inline ( always ) ]
        pub fn errm(&self) -> ERRMR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ERRMR { bits }
        }
        # [ doc = "Bit 14 - Packet memory area over / underrun interrupt mask" ]
        # [ inline ( always ) ]
        pub fn pmaovrm(&self) -> PMAOVRMR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PMAOVRMR { bits }
        }
        # [ doc = "Bit 15 - Correct transfer interrupt mask" ]
        # [ inline ( always ) ]
        pub fn ctrm(&self) -> CTRMR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CTRMR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 3 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 0 - Force USB Reset" ]
        # [ inline ( always ) ]
        pub fn fres(&mut self) -> _FRESW {
            _FRESW { w: self }
        }
        # [ doc = "Bit 1 - Power down" ]
        # [ inline ( always ) ]
        pub fn pdwn(&mut self) -> _PDWNW {
            _PDWNW { w: self }
        }
        # [ doc = "Bit 2 - Low-power mode" ]
        # [ inline ( always ) ]
        pub fn lpmode(&mut self) -> _LPMODEW {
            _LPMODEW { w: self }
        }
        # [ doc = "Bit 3 - Force suspend" ]
        # [ inline ( always ) ]
        pub fn fsusp(&mut self) -> _FSUSPW {
            _FSUSPW { w: self }
        }
        # [ doc = "Bit 4 - Resume request" ]
        # [ inline ( always ) ]
        pub fn resume(&mut self) -> _RESUMEW {
            _RESUMEW { w: self }
        }
        # [ doc = "Bit 8 - Expected start of frame interrupt mask" ]
        # [ inline ( always ) ]
        pub fn esofm(&mut self) -> _ESOFMW {
            _ESOFMW { w: self }
        }
        # [ doc = "Bit 9 - Start of frame interrupt mask" ]
        # [ inline ( always ) ]
        pub fn sofm(&mut self) -> _SOFMW {
            _SOFMW { w: self }
        }
        # [ doc = "Bit 10 - USB reset interrupt mask" ]
        # [ inline ( always ) ]
        pub fn resetm(&mut self) -> _RESETMW {
            _RESETMW { w: self }
        }
        # [ doc = "Bit 11 - Suspend mode interrupt mask" ]
        # [ inline ( always ) ]
        pub fn suspm(&mut self) -> _SUSPMW {
            _SUSPMW { w: self }
        }
        # [ doc = "Bit 12 - Wakeup interrupt mask" ]
        # [ inline ( always ) ]
        pub fn wkupm(&mut self) -> _WKUPMW {
            _WKUPMW { w: self }
        }
        # [ doc = "Bit 13 - Error interrupt mask" ]
        # [ inline ( always ) ]
        pub fn errm(&mut self) -> _ERRMW {
            _ERRMW { w: self }
        }
        # [ doc = "Bit 14 - Packet memory area over / underrun interrupt mask" ]
        # [ inline ( always ) ]
        pub fn pmaovrm(&mut self) -> _PMAOVRMW {
            _PMAOVRMW { w: self }
        }
        # [ doc = "Bit 15 - Correct transfer interrupt mask" ]
        # [ inline ( always ) ]
        pub fn ctrm(&mut self) -> _CTRMW {
            _CTRMW { w: self }
        }
    }
}
# [ doc = "interrupt status register" ]
pub struct ISTR {
    register: VolatileCell<u32>,
}
# [ doc = "interrupt status register" ]
pub mod istr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::ISTR {
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
    pub struct EP_IDR {
        bits: u8,
    }
    impl EP_IDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DIRR {
        bits: bool,
    }
    impl DIRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ESOFR {
        bits: bool,
    }
    impl ESOFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SOFR {
        bits: bool,
    }
    impl SOFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RESETR {
        bits: bool,
    }
    impl RESETR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SUSPR {
        bits: bool,
    }
    impl SUSPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WKUPR {
        bits: bool,
    }
    impl WKUPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ERRR {
        bits: bool,
    }
    impl ERRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct PMAOVRR {
        bits: bool,
    }
    impl PMAOVRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CTRR {
        bits: bool,
    }
    impl CTRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _EP_IDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EP_IDW<'a> {
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
    pub struct _DIRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DIRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ESOFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ESOFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SOFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SOFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _RESETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RESETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SUSPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SUSPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _WKUPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WKUPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ERRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ERRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _PMAOVRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PMAOVRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CTRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CTRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:3 - Endpoint Identifier" ]
        # [ inline ( always ) ]
        pub fn ep_id(&self) -> EP_IDR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EP_IDR { bits }
        }
        # [ doc = "Bit 4 - Direction of transaction" ]
        # [ inline ( always ) ]
        pub fn dir(&self) -> DIRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DIRR { bits }
        }
        # [ doc = "Bit 8 - Expected start frame" ]
        # [ inline ( always ) ]
        pub fn esof(&self) -> ESOFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ESOFR { bits }
        }
        # [ doc = "Bit 9 - start of frame" ]
        # [ inline ( always ) ]
        pub fn sof(&self) -> SOFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SOFR { bits }
        }
        # [ doc = "Bit 10 - reset request" ]
        # [ inline ( always ) ]
        pub fn reset(&self) -> RESETR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RESETR { bits }
        }
        # [ doc = "Bit 11 - Suspend mode request" ]
        # [ inline ( always ) ]
        pub fn susp(&self) -> SUSPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SUSPR { bits }
        }
        # [ doc = "Bit 12 - Wakeup" ]
        # [ inline ( always ) ]
        pub fn wkup(&self) -> WKUPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WKUPR { bits }
        }
        # [ doc = "Bit 13 - Error" ]
        # [ inline ( always ) ]
        pub fn err(&self) -> ERRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ERRR { bits }
        }
        # [ doc = "Bit 14 - Packet memory area over / underrun" ]
        # [ inline ( always ) ]
        pub fn pmaovr(&self) -> PMAOVRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PMAOVRR { bits }
        }
        # [ doc = "Bit 15 - Correct transfer" ]
        # [ inline ( always ) ]
        pub fn ctr(&self) -> CTRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CTRR { bits }
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
        # [ doc = "Bits 0:3 - Endpoint Identifier" ]
        # [ inline ( always ) ]
        pub fn ep_id(&mut self) -> _EP_IDW {
            _EP_IDW { w: self }
        }
        # [ doc = "Bit 4 - Direction of transaction" ]
        # [ inline ( always ) ]
        pub fn dir(&mut self) -> _DIRW {
            _DIRW { w: self }
        }
        # [ doc = "Bit 8 - Expected start frame" ]
        # [ inline ( always ) ]
        pub fn esof(&mut self) -> _ESOFW {
            _ESOFW { w: self }
        }
        # [ doc = "Bit 9 - start of frame" ]
        # [ inline ( always ) ]
        pub fn sof(&mut self) -> _SOFW {
            _SOFW { w: self }
        }
        # [ doc = "Bit 10 - reset request" ]
        # [ inline ( always ) ]
        pub fn reset(&mut self) -> _RESETW {
            _RESETW { w: self }
        }
        # [ doc = "Bit 11 - Suspend mode request" ]
        # [ inline ( always ) ]
        pub fn susp(&mut self) -> _SUSPW {
            _SUSPW { w: self }
        }
        # [ doc = "Bit 12 - Wakeup" ]
        # [ inline ( always ) ]
        pub fn wkup(&mut self) -> _WKUPW {
            _WKUPW { w: self }
        }
        # [ doc = "Bit 13 - Error" ]
        # [ inline ( always ) ]
        pub fn err(&mut self) -> _ERRW {
            _ERRW { w: self }
        }
        # [ doc = "Bit 14 - Packet memory area over / underrun" ]
        # [ inline ( always ) ]
        pub fn pmaovr(&mut self) -> _PMAOVRW {
            _PMAOVRW { w: self }
        }
        # [ doc = "Bit 15 - Correct transfer" ]
        # [ inline ( always ) ]
        pub fn ctr(&mut self) -> _CTRW {
            _CTRW { w: self }
        }
    }
}
# [ doc = "frame number register" ]
pub struct FNR {
    register: VolatileCell<u32>,
}
# [ doc = "frame number register" ]
pub mod fnr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::FNR {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct FNR {
        bits: u16,
    }
    impl FNR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct LSOFR {
        bits: u8,
    }
    impl LSOFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct LCKR {
        bits: bool,
    }
    impl LCKR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RXDMR {
        bits: bool,
    }
    impl RXDMR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RXDPR {
        bits: bool,
    }
    impl RXDPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:10 - Frame number" ]
        # [ inline ( always ) ]
        pub fn fn_(&self) -> FNR {
            let bits = {
                const MASK: u16 = 2047;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            FNR { bits }
        }
        # [ doc = "Bits 11:12 - Lost SOF" ]
        # [ inline ( always ) ]
        pub fn lsof(&self) -> LSOFR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            LSOFR { bits }
        }
        # [ doc = "Bit 13 - Locked" ]
        # [ inline ( always ) ]
        pub fn lck(&self) -> LCKR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LCKR { bits }
        }
        # [ doc = "Bit 14 - Receive data - line status" ]
        # [ inline ( always ) ]
        pub fn rxdm(&self) -> RXDMR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RXDMR { bits }
        }
        # [ doc = "Bit 15 - Receive data + line status" ]
        # [ inline ( always ) ]
        pub fn rxdp(&self) -> RXDPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RXDPR { bits }
        }
    }
}
# [ doc = "device address" ]
pub struct DADDR {
    register: VolatileCell<u32>,
}
# [ doc = "device address" ]
pub mod daddr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DADDR {
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
    pub struct ADDR {
        bits: u8,
    }
    impl ADDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EFR {
        bits: bool,
    }
    impl EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _ADDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADDW<'a> {
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
    pub struct _EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
        # [ doc = "Bits 0:6 - Device address" ]
        # [ inline ( always ) ]
        pub fn add(&self) -> ADDR {
            let bits = {
                const MASK: u8 = 127;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ADDR { bits }
        }
        # [ doc = "Bit 7 - Enable function" ]
        # [ inline ( always ) ]
        pub fn ef(&self) -> EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EFR { bits }
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
        # [ doc = "Bits 0:6 - Device address" ]
        # [ inline ( always ) ]
        pub fn add(&mut self) -> _ADDW {
            _ADDW { w: self }
        }
        # [ doc = "Bit 7 - Enable function" ]
        # [ inline ( always ) ]
        pub fn ef(&mut self) -> _EFW {
            _EFW { w: self }
        }
    }
}
# [ doc = "Buffer table address" ]
pub struct BTABLE {
    register: VolatileCell<u32>,
}
# [ doc = "Buffer table address" ]
pub mod btable {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::BTABLE {
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
    pub struct BTABLER {
        bits: u16,
    }
    impl BTABLER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _BTABLEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BTABLEW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 8191;
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
        # [ doc = "Bits 3:15 - Buffer table" ]
        # [ inline ( always ) ]
        pub fn btable(&self) -> BTABLER {
            let bits = {
                const MASK: u16 = 8191;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            BTABLER { bits }
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
        # [ doc = "Bits 3:15 - Buffer table" ]
        # [ inline ( always ) ]
        pub fn btable(&mut self) -> _BTABLEW {
            _BTABLEW { w: self }
        }
    }
}
# [ doc = "Universal serial bus full-speed device interface" ]
pub struct USB {
    register_block: RegisterBlock,
}
impl Deref for USB {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
