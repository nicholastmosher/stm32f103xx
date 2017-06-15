# ! [ doc = "General purpose I/O" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;
use vcell::VolatileCell;

# [ doc = "General purpose I/O" ]
pub const GPIOA: Peripheral<GPIOA> = unsafe { Peripheral::new(1073809408) };

# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - Port configuration register low (GPIOn_CRL)" ]
    pub crl: CRL,
    # [ doc = "0x04 - Port configuration register high (GPIOn_CRL)" ]
    pub crh: CRH,
    # [ doc = "0x08 - Port input data register (GPIOn_IDR)" ]
    pub idr: IDR,
    # [ doc = "0x0c - Port output data register (GPIOn_ODR)" ]
    pub odr: ODR,
    # [ doc = "0x10 - Port bit set/reset register (GPIOn_BSRR)" ]
    pub bsrr: BSRR,
    # [ doc = "0x14 - Port bit reset register (GPIOn_BRR)" ]
    pub brr: BRR,
    # [ doc = "0x18 - Port configuration lock register" ]
    pub lckr: LCKR,
}
# [ doc = "Port configuration register low (GPIOn_CRL)" ]
pub struct CRL {
    register: VolatileCell<u32>,
}
# [ doc = "Port configuration register low (GPIOn_CRL)" ]
pub mod crl {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CRL {
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
    # [ doc = "Possible values of the field `MODE0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum MODE0R {
        # [ doc = "Input mode" ]
        INPUT,
        # [ doc = "Output mode 10 MHz" ]
        OUTPUT,
        # [ doc = "Output mode 2 MHz" ]
        OUTPUT2,
        # [ doc = "Output mode 50 MHz" ]
        OUTPUT50,
    }
    impl MODE0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                MODE0R::INPUT => 0,
                MODE0R::OUTPUT => 1,
                MODE0R::OUTPUT2 => 2,
                MODE0R::OUTPUT50 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> MODE0R {
            match value {
                0 => MODE0R::INPUT,
                1 => MODE0R::OUTPUT,
                2 => MODE0R::OUTPUT2,
                3 => MODE0R::OUTPUT50,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `INPUT`" ]
        # [ inline ( always ) ]
        pub fn is_input(&self) -> bool {
            *self == MODE0R::INPUT
        }
        # [ doc = "Checks if the value of the field is `OUTPUT`" ]
        # [ inline ( always ) ]
        pub fn is_output(&self) -> bool {
            *self == MODE0R::OUTPUT
        }
        # [ doc = "Checks if the value of the field is `OUTPUT2`" ]
        # [ inline ( always ) ]
        pub fn is_output2(&self) -> bool {
            *self == MODE0R::OUTPUT2
        }
        # [ doc = "Checks if the value of the field is `OUTPUT50`" ]
        # [ inline ( always ) ]
        pub fn is_output50(&self) -> bool {
            *self == MODE0R::OUTPUT50
        }
    }
    # [ doc = "Possible values of the field `CNF0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CNF0R {
        # [ doc = "Push-Pull mode" ]
        PUSH,
        # [ doc = "Open Drain-Mode" ]
        OPEN,
        # [ doc = "Alternate Function Push-Pull Mode" ]
        ALTPUSH,
        # [ doc = "Alternate Function Open-Drain Mode" ]
        ALTOPEN,
    }
    impl CNF0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                CNF0R::PUSH => 0,
                CNF0R::OPEN => 1,
                CNF0R::ALTPUSH => 2,
                CNF0R::ALTOPEN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> CNF0R {
            match value {
                0 => CNF0R::PUSH,
                1 => CNF0R::OPEN,
                2 => CNF0R::ALTPUSH,
                3 => CNF0R::ALTOPEN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PUSH`" ]
        # [ inline ( always ) ]
        pub fn is_push(&self) -> bool {
            *self == CNF0R::PUSH
        }
        # [ doc = "Checks if the value of the field is `OPEN`" ]
        # [ inline ( always ) ]
        pub fn is_open(&self) -> bool {
            *self == CNF0R::OPEN
        }
        # [ doc = "Checks if the value of the field is `ALTPUSH`" ]
        # [ inline ( always ) ]
        pub fn is_alt_push(&self) -> bool {
            *self == CNF0R::ALTPUSH
        }
        # [ doc = "Checks if the value of the field is `ALTOPEN`" ]
        # [ inline ( always ) ]
        pub fn is_alt_open(&self) -> bool {
            *self == CNF0R::ALTOPEN
        }
    }
    # [ doc = "Possible values of the field `MODE1`" ]
    pub type MODE1R = MODE0R;
    # [ doc = "Possible values of the field `CNF1`" ]
    pub type CNF1R = CNF0R;
    # [ doc = "Possible values of the field `MODE2`" ]
    pub type MODE2R = MODE0R;
    # [ doc = "Possible values of the field `CNF2`" ]
    pub type CNF2R = CNF0R;
    # [ doc = "Possible values of the field `MODE3`" ]
    pub type MODE3R = MODE0R;
    # [ doc = "Possible values of the field `CNF3`" ]
    pub type CNF3R = CNF0R;
    # [ doc = "Possible values of the field `MODE4`" ]
    pub type MODE4R = MODE0R;
    # [ doc = "Possible values of the field `CNF4`" ]
    pub type CNF4R = CNF0R;
    # [ doc = "Possible values of the field `MODE5`" ]
    pub type MODE5R = MODE0R;
    # [ doc = "Possible values of the field `CNF5`" ]
    pub type CNF5R = CNF0R;
    # [ doc = "Possible values of the field `MODE6`" ]
    pub type MODE6R = MODE0R;
    # [ doc = "Possible values of the field `CNF6`" ]
    pub type CNF6R = CNF0R;
    # [ doc = "Possible values of the field `MODE7`" ]
    pub type MODE7R = MODE0R;
    # [ doc = "Possible values of the field `CNF7`" ]
    pub type CNF7R = CNF0R;
    # [ doc = "Values that can be written to the field `MODE0`" ]
    pub enum MODE0W {
        # [ doc = "Input mode" ]
        INPUT,
        # [ doc = "Output mode 10 MHz" ]
        OUTPUT,
        # [ doc = "Output mode 2 MHz" ]
        OUTPUT2,
        # [ doc = "Output mode 50 MHz" ]
        OUTPUT50,
    }
    impl MODE0W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                MODE0W::INPUT => 0,
                MODE0W::OUTPUT => 1,
                MODE0W::OUTPUT2 => 2,
                MODE0W::OUTPUT50 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MODE0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MODE0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MODE0W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Input mode" ]
        # [ inline ( always ) ]
        pub fn input(self) -> &'a mut W {
            self.variant(MODE0W::INPUT)
        }
        # [ doc = "Output mode 10 MHz" ]
        # [ inline ( always ) ]
        pub fn output(self) -> &'a mut W {
            self.variant(MODE0W::OUTPUT)
        }
        # [ doc = "Output mode 2 MHz" ]
        # [ inline ( always ) ]
        pub fn output2(self) -> &'a mut W {
            self.variant(MODE0W::OUTPUT2)
        }
        # [ doc = "Output mode 50 MHz" ]
        # [ inline ( always ) ]
        pub fn output50(self) -> &'a mut W {
            self.variant(MODE0W::OUTPUT50)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `CNF0`" ]
    pub enum CNF0W {
        # [ doc = "Push-Pull mode" ]
        PUSH,
        # [ doc = "Open Drain-Mode" ]
        OPEN,
        # [ doc = "Alternate Function Push-Pull Mode" ]
        ALTPUSH,
        # [ doc = "Alternate Function Open-Drain Mode" ]
        ALTOPEN,
    }
    impl CNF0W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                CNF0W::PUSH => 0,
                CNF0W::OPEN => 1,
                CNF0W::ALTPUSH => 2,
                CNF0W::ALTOPEN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CNF0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNF0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CNF0W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Push-Pull mode" ]
        # [ inline ( always ) ]
        pub fn push(self) -> &'a mut W {
            self.variant(CNF0W::PUSH)
        }
        # [ doc = "Open Drain-Mode" ]
        # [ inline ( always ) ]
        pub fn open(self) -> &'a mut W {
            self.variant(CNF0W::OPEN)
        }
        # [ doc = "Alternate Function Push-Pull Mode" ]
        # [ inline ( always ) ]
        pub fn alt_push(self) -> &'a mut W {
            self.variant(CNF0W::ALTPUSH)
        }
        # [ doc = "Alternate Function Open-Drain Mode" ]
        # [ inline ( always ) ]
        pub fn alt_open(self) -> &'a mut W {
            self.variant(CNF0W::ALTOPEN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `MODE1`" ]
    pub type MODE1W = MODE0W;
    # [ doc = r" Proxy" ]
    pub struct _MODE1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MODE1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MODE1W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Input mode" ]
        # [ inline ( always ) ]
        pub fn input(self) -> &'a mut W {
            self.variant(MODE0W::INPUT)
        }
        # [ doc = "Output mode 10 MHz" ]
        # [ inline ( always ) ]
        pub fn output(self) -> &'a mut W {
            self.variant(MODE0W::OUTPUT)
        }
        # [ doc = "Output mode 2 MHz" ]
        # [ inline ( always ) ]
        pub fn output2(self) -> &'a mut W {
            self.variant(MODE0W::OUTPUT2)
        }
        # [ doc = "Output mode 50 MHz" ]
        # [ inline ( always ) ]
        pub fn output50(self) -> &'a mut W {
            self.variant(MODE0W::OUTPUT50)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `CNF1`" ]
    pub type CNF1W = CNF0W;
    # [ doc = r" Proxy" ]
    pub struct _CNF1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNF1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CNF1W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Push-Pull mode" ]
        # [ inline ( always ) ]
        pub fn push(self) -> &'a mut W {
            self.variant(CNF0W::PUSH)
        }
        # [ doc = "Open Drain-Mode" ]
        # [ inline ( always ) ]
        pub fn open(self) -> &'a mut W {
            self.variant(CNF0W::OPEN)
        }
        # [ doc = "Alternate Function Push-Pull Mode" ]
        # [ inline ( always ) ]
        pub fn alt_push(self) -> &'a mut W {
            self.variant(CNF0W::ALTPUSH)
        }
        # [ doc = "Alternate Function Open-Drain Mode" ]
        # [ inline ( always ) ]
        pub fn alt_open(self) -> &'a mut W {
            self.variant(CNF0W::ALTOPEN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `MODE2`" ]
    pub type MODE2W = MODE0W;
    # [ doc = r" Proxy" ]
    pub struct _MODE2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MODE2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MODE2W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Input mode" ]
        # [ inline ( always ) ]
        pub fn input(self) -> &'a mut W {
            self.variant(MODE0W::INPUT)
        }
        # [ doc = "Output mode 10 MHz" ]
        # [ inline ( always ) ]
        pub fn output(self) -> &'a mut W {
            self.variant(MODE0W::OUTPUT)
        }
        # [ doc = "Output mode 2 MHz" ]
        # [ inline ( always ) ]
        pub fn output2(self) -> &'a mut W {
            self.variant(MODE0W::OUTPUT2)
        }
        # [ doc = "Output mode 50 MHz" ]
        # [ inline ( always ) ]
        pub fn output50(self) -> &'a mut W {
            self.variant(MODE0W::OUTPUT50)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `CNF2`" ]
    pub type CNF2W = CNF0W;
    # [ doc = r" Proxy" ]
    pub struct _CNF2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNF2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CNF2W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Push-Pull mode" ]
        # [ inline ( always ) ]
        pub fn push(self) -> &'a mut W {
            self.variant(CNF0W::PUSH)
        }
        # [ doc = "Open Drain-Mode" ]
        # [ inline ( always ) ]
        pub fn open(self) -> &'a mut W {
            self.variant(CNF0W::OPEN)
        }
        # [ doc = "Alternate Function Push-Pull Mode" ]
        # [ inline ( always ) ]
        pub fn alt_push(self) -> &'a mut W {
            self.variant(CNF0W::ALTPUSH)
        }
        # [ doc = "Alternate Function Open-Drain Mode" ]
        # [ inline ( always ) ]
        pub fn alt_open(self) -> &'a mut W {
            self.variant(CNF0W::ALTOPEN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `MODE3`" ]
    pub type MODE3W = MODE0W;
    # [ doc = r" Proxy" ]
    pub struct _MODE3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MODE3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MODE3W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Input mode" ]
        # [ inline ( always ) ]
        pub fn input(self) -> &'a mut W {
            self.variant(MODE0W::INPUT)
        }
        # [ doc = "Output mode 10 MHz" ]
        # [ inline ( always ) ]
        pub fn output(self) -> &'a mut W {
            self.variant(MODE0W::OUTPUT)
        }
        # [ doc = "Output mode 2 MHz" ]
        # [ inline ( always ) ]
        pub fn output2(self) -> &'a mut W {
            self.variant(MODE0W::OUTPUT2)
        }
        # [ doc = "Output mode 50 MHz" ]
        # [ inline ( always ) ]
        pub fn output50(self) -> &'a mut W {
            self.variant(MODE0W::OUTPUT50)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `CNF3`" ]
    pub type CNF3W = CNF0W;
    # [ doc = r" Proxy" ]
    pub struct _CNF3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNF3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CNF3W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Push-Pull mode" ]
        # [ inline ( always ) ]
        pub fn push(self) -> &'a mut W {
            self.variant(CNF0W::PUSH)
        }
        # [ doc = "Open Drain-Mode" ]
        # [ inline ( always ) ]
        pub fn open(self) -> &'a mut W {
            self.variant(CNF0W::OPEN)
        }
        # [ doc = "Alternate Function Push-Pull Mode" ]
        # [ inline ( always ) ]
        pub fn alt_push(self) -> &'a mut W {
            self.variant(CNF0W::ALTPUSH)
        }
        # [ doc = "Alternate Function Open-Drain Mode" ]
        # [ inline ( always ) ]
        pub fn alt_open(self) -> &'a mut W {
            self.variant(CNF0W::ALTOPEN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `MODE4`" ]
    pub type MODE4W = MODE0W;
    # [ doc = r" Proxy" ]
    pub struct _MODE4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MODE4W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MODE4W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Input mode" ]
        # [ inline ( always ) ]
        pub fn input(self) -> &'a mut W {
            self.variant(MODE0W::INPUT)
        }
        # [ doc = "Output mode 10 MHz" ]
        # [ inline ( always ) ]
        pub fn output(self) -> &'a mut W {
            self.variant(MODE0W::OUTPUT)
        }
        # [ doc = "Output mode 2 MHz" ]
        # [ inline ( always ) ]
        pub fn output2(self) -> &'a mut W {
            self.variant(MODE0W::OUTPUT2)
        }
        # [ doc = "Output mode 50 MHz" ]
        # [ inline ( always ) ]
        pub fn output50(self) -> &'a mut W {
            self.variant(MODE0W::OUTPUT50)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `CNF4`" ]
    pub type CNF4W = CNF0W;
    # [ doc = r" Proxy" ]
    pub struct _CNF4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNF4W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CNF4W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Push-Pull mode" ]
        # [ inline ( always ) ]
        pub fn push(self) -> &'a mut W {
            self.variant(CNF0W::PUSH)
        }
        # [ doc = "Open Drain-Mode" ]
        # [ inline ( always ) ]
        pub fn open(self) -> &'a mut W {
            self.variant(CNF0W::OPEN)
        }
        # [ doc = "Alternate Function Push-Pull Mode" ]
        # [ inline ( always ) ]
        pub fn alt_push(self) -> &'a mut W {
            self.variant(CNF0W::ALTPUSH)
        }
        # [ doc = "Alternate Function Open-Drain Mode" ]
        # [ inline ( always ) ]
        pub fn alt_open(self) -> &'a mut W {
            self.variant(CNF0W::ALTOPEN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `MODE5`" ]
    pub type MODE5W = MODE0W;
    # [ doc = r" Proxy" ]
    pub struct _MODE5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MODE5W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MODE5W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Input mode" ]
        # [ inline ( always ) ]
        pub fn input(self) -> &'a mut W {
            self.variant(MODE0W::INPUT)
        }
        # [ doc = "Output mode 10 MHz" ]
        # [ inline ( always ) ]
        pub fn output(self) -> &'a mut W {
            self.variant(MODE0W::OUTPUT)
        }
        # [ doc = "Output mode 2 MHz" ]
        # [ inline ( always ) ]
        pub fn output2(self) -> &'a mut W {
            self.variant(MODE0W::OUTPUT2)
        }
        # [ doc = "Output mode 50 MHz" ]
        # [ inline ( always ) ]
        pub fn output50(self) -> &'a mut W {
            self.variant(MODE0W::OUTPUT50)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `CNF5`" ]
    pub type CNF5W = CNF0W;
    # [ doc = r" Proxy" ]
    pub struct _CNF5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNF5W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CNF5W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Push-Pull mode" ]
        # [ inline ( always ) ]
        pub fn push(self) -> &'a mut W {
            self.variant(CNF0W::PUSH)
        }
        # [ doc = "Open Drain-Mode" ]
        # [ inline ( always ) ]
        pub fn open(self) -> &'a mut W {
            self.variant(CNF0W::OPEN)
        }
        # [ doc = "Alternate Function Push-Pull Mode" ]
        # [ inline ( always ) ]
        pub fn alt_push(self) -> &'a mut W {
            self.variant(CNF0W::ALTPUSH)
        }
        # [ doc = "Alternate Function Open-Drain Mode" ]
        # [ inline ( always ) ]
        pub fn alt_open(self) -> &'a mut W {
            self.variant(CNF0W::ALTOPEN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `MODE6`" ]
    pub type MODE6W = MODE0W;
    # [ doc = r" Proxy" ]
    pub struct _MODE6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MODE6W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MODE6W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Input mode" ]
        # [ inline ( always ) ]
        pub fn input(self) -> &'a mut W {
            self.variant(MODE0W::INPUT)
        }
        # [ doc = "Output mode 10 MHz" ]
        # [ inline ( always ) ]
        pub fn output(self) -> &'a mut W {
            self.variant(MODE0W::OUTPUT)
        }
        # [ doc = "Output mode 2 MHz" ]
        # [ inline ( always ) ]
        pub fn output2(self) -> &'a mut W {
            self.variant(MODE0W::OUTPUT2)
        }
        # [ doc = "Output mode 50 MHz" ]
        # [ inline ( always ) ]
        pub fn output50(self) -> &'a mut W {
            self.variant(MODE0W::OUTPUT50)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `CNF6`" ]
    pub type CNF6W = CNF0W;
    # [ doc = r" Proxy" ]
    pub struct _CNF6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNF6W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CNF6W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Push-Pull mode" ]
        # [ inline ( always ) ]
        pub fn push(self) -> &'a mut W {
            self.variant(CNF0W::PUSH)
        }
        # [ doc = "Open Drain-Mode" ]
        # [ inline ( always ) ]
        pub fn open(self) -> &'a mut W {
            self.variant(CNF0W::OPEN)
        }
        # [ doc = "Alternate Function Push-Pull Mode" ]
        # [ inline ( always ) ]
        pub fn alt_push(self) -> &'a mut W {
            self.variant(CNF0W::ALTPUSH)
        }
        # [ doc = "Alternate Function Open-Drain Mode" ]
        # [ inline ( always ) ]
        pub fn alt_open(self) -> &'a mut W {
            self.variant(CNF0W::ALTOPEN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `MODE7`" ]
    pub type MODE7W = MODE0W;
    # [ doc = r" Proxy" ]
    pub struct _MODE7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MODE7W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MODE7W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Input mode" ]
        # [ inline ( always ) ]
        pub fn input(self) -> &'a mut W {
            self.variant(MODE0W::INPUT)
        }
        # [ doc = "Output mode 10 MHz" ]
        # [ inline ( always ) ]
        pub fn output(self) -> &'a mut W {
            self.variant(MODE0W::OUTPUT)
        }
        # [ doc = "Output mode 2 MHz" ]
        # [ inline ( always ) ]
        pub fn output2(self) -> &'a mut W {
            self.variant(MODE0W::OUTPUT2)
        }
        # [ doc = "Output mode 50 MHz" ]
        # [ inline ( always ) ]
        pub fn output50(self) -> &'a mut W {
            self.variant(MODE0W::OUTPUT50)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `CNF7`" ]
    pub type CNF7W = CNF0W;
    # [ doc = r" Proxy" ]
    pub struct _CNF7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNF7W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CNF7W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Push-Pull mode" ]
        # [ inline ( always ) ]
        pub fn push(self) -> &'a mut W {
            self.variant(CNF0W::PUSH)
        }
        # [ doc = "Open Drain-Mode" ]
        # [ inline ( always ) ]
        pub fn open(self) -> &'a mut W {
            self.variant(CNF0W::OPEN)
        }
        # [ doc = "Alternate Function Push-Pull Mode" ]
        # [ inline ( always ) ]
        pub fn alt_push(self) -> &'a mut W {
            self.variant(CNF0W::ALTPUSH)
        }
        # [ doc = "Alternate Function Open-Drain Mode" ]
        # [ inline ( always ) ]
        pub fn alt_open(self) -> &'a mut W {
            self.variant(CNF0W::ALTOPEN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
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
        # [ doc = "Bits 0:1 - Port n.0 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode0(&self) -> MODE0R {
            MODE0R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 2:3 - Port n.0 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf0(&self) -> CNF0R {
            CNF0R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 4:5 - Port n.1 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode1(&self) -> MODE1R {
            MODE1R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 6:7 - Port n.1 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf1(&self) -> CNF1R {
            CNF1R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 8:9 - Port n.2 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode2(&self) -> MODE2R {
            MODE2R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 10:11 - Port n.2 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf2(&self) -> CNF2R {
            CNF2R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 12:13 - Port n.3 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode3(&self) -> MODE3R {
            MODE3R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 14:15 - Port n.3 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf3(&self) -> CNF3R {
            CNF3R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 16:17 - Port n.4 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode4(&self) -> MODE4R {
            MODE4R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 18:19 - Port n.4 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf4(&self) -> CNF4R {
            CNF4R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 20:21 - Port n.5 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode5(&self) -> MODE5R {
            MODE5R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 22:23 - Port n.5 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf5(&self) -> CNF5R {
            CNF5R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 22;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 24:25 - Port n.6 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode6(&self) -> MODE6R {
            MODE6R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 26:27 - Port n.6 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf6(&self) -> CNF6R {
            CNF6R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 28:29 - Port n.7 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode7(&self) -> MODE7R {
            MODE7R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 30:31 - Port n.7 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf7(&self) -> CNF7R {
            CNF7R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 30;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 1145324612 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:1 - Port n.0 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode0(&mut self) -> _MODE0W {
            _MODE0W { w: self }
        }
        # [ doc = "Bits 2:3 - Port n.0 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf0(&mut self) -> _CNF0W {
            _CNF0W { w: self }
        }
        # [ doc = "Bits 4:5 - Port n.1 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode1(&mut self) -> _MODE1W {
            _MODE1W { w: self }
        }
        # [ doc = "Bits 6:7 - Port n.1 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf1(&mut self) -> _CNF1W {
            _CNF1W { w: self }
        }
        # [ doc = "Bits 8:9 - Port n.2 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode2(&mut self) -> _MODE2W {
            _MODE2W { w: self }
        }
        # [ doc = "Bits 10:11 - Port n.2 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf2(&mut self) -> _CNF2W {
            _CNF2W { w: self }
        }
        # [ doc = "Bits 12:13 - Port n.3 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode3(&mut self) -> _MODE3W {
            _MODE3W { w: self }
        }
        # [ doc = "Bits 14:15 - Port n.3 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf3(&mut self) -> _CNF3W {
            _CNF3W { w: self }
        }
        # [ doc = "Bits 16:17 - Port n.4 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode4(&mut self) -> _MODE4W {
            _MODE4W { w: self }
        }
        # [ doc = "Bits 18:19 - Port n.4 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf4(&mut self) -> _CNF4W {
            _CNF4W { w: self }
        }
        # [ doc = "Bits 20:21 - Port n.5 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode5(&mut self) -> _MODE5W {
            _MODE5W { w: self }
        }
        # [ doc = "Bits 22:23 - Port n.5 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf5(&mut self) -> _CNF5W {
            _CNF5W { w: self }
        }
        # [ doc = "Bits 24:25 - Port n.6 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode6(&mut self) -> _MODE6W {
            _MODE6W { w: self }
        }
        # [ doc = "Bits 26:27 - Port n.6 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf6(&mut self) -> _CNF6W {
            _CNF6W { w: self }
        }
        # [ doc = "Bits 28:29 - Port n.7 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode7(&mut self) -> _MODE7W {
            _MODE7W { w: self }
        }
        # [ doc = "Bits 30:31 - Port n.7 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf7(&mut self) -> _CNF7W {
            _CNF7W { w: self }
        }
    }
}
# [ doc = "Port configuration register high (GPIOn_CRL)" ]
pub struct CRH {
    register: VolatileCell<u32>,
}
# [ doc = "Port configuration register high (GPIOn_CRL)" ]
pub mod crh {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CRH {
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
    # [ doc = "Possible values of the field `MODE8`" ]
    pub type MODE8R = super::crl::MODE0R;
    # [ doc = "Possible values of the field `CNF8`" ]
    pub type CNF8R = super::crl::CNF0R;
    # [ doc = "Possible values of the field `MODE9`" ]
    pub type MODE9R = super::crl::MODE0R;
    # [ doc = "Possible values of the field `CNF9`" ]
    pub type CNF9R = super::crl::CNF0R;
    # [ doc = "Possible values of the field `MODE10`" ]
    pub type MODE10R = super::crl::MODE0R;
    # [ doc = "Possible values of the field `CNF10`" ]
    pub type CNF10R = super::crl::CNF0R;
    # [ doc = "Possible values of the field `MODE11`" ]
    pub type MODE11R = super::crl::MODE0R;
    # [ doc = "Possible values of the field `CNF11`" ]
    pub type CNF11R = super::crl::CNF0R;
    # [ doc = "Possible values of the field `MODE12`" ]
    pub type MODE12R = super::crl::MODE0R;
    # [ doc = "Possible values of the field `CNF12`" ]
    pub type CNF12R = super::crl::CNF0R;
    # [ doc = "Possible values of the field `MODE13`" ]
    pub type MODE13R = super::crl::MODE0R;
    # [ doc = "Possible values of the field `CNF13`" ]
    pub type CNF13R = super::crl::CNF0R;
    # [ doc = "Possible values of the field `MODE14`" ]
    pub type MODE14R = super::crl::MODE0R;
    # [ doc = "Possible values of the field `CNF14`" ]
    pub type CNF14R = super::crl::CNF0R;
    # [ doc = "Possible values of the field `MODE15`" ]
    pub type MODE15R = super::crl::MODE0R;
    # [ doc = "Possible values of the field `CNF15`" ]
    pub type CNF15R = super::crl::CNF0R;
    # [ doc = "Values that can be written to the field `MODE8`" ]
    pub type MODE8W = super::crl::MODE0W;
    # [ doc = r" Proxy" ]
    pub struct _MODE8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MODE8W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MODE8W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Input mode" ]
        # [ inline ( always ) ]
        pub fn input(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::INPUT)
        }
        # [ doc = "Output mode 10 MHz" ]
        # [ inline ( always ) ]
        pub fn output(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::OUTPUT)
        }
        # [ doc = "Output mode 2 MHz" ]
        # [ inline ( always ) ]
        pub fn output2(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::OUTPUT2)
        }
        # [ doc = "Output mode 50 MHz" ]
        # [ inline ( always ) ]
        pub fn output50(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::OUTPUT50)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `CNF8`" ]
    pub type CNF8W = super::crl::CNF0W;
    # [ doc = r" Proxy" ]
    pub struct _CNF8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNF8W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CNF8W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Push-Pull mode" ]
        # [ inline ( always ) ]
        pub fn push(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::PUSH)
        }
        # [ doc = "Open Drain-Mode" ]
        # [ inline ( always ) ]
        pub fn open(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::OPEN)
        }
        # [ doc = "Alternate Function Push-Pull Mode" ]
        # [ inline ( always ) ]
        pub fn alt_push(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::ALTPUSH)
        }
        # [ doc = "Alternate Function Open-Drain Mode" ]
        # [ inline ( always ) ]
        pub fn alt_open(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::ALTOPEN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `MODE9`" ]
    pub type MODE9W = super::crl::MODE0W;
    # [ doc = r" Proxy" ]
    pub struct _MODE9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MODE9W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MODE9W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Input mode" ]
        # [ inline ( always ) ]
        pub fn input(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::INPUT)
        }
        # [ doc = "Output mode 10 MHz" ]
        # [ inline ( always ) ]
        pub fn output(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::OUTPUT)
        }
        # [ doc = "Output mode 2 MHz" ]
        # [ inline ( always ) ]
        pub fn output2(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::OUTPUT2)
        }
        # [ doc = "Output mode 50 MHz" ]
        # [ inline ( always ) ]
        pub fn output50(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::OUTPUT50)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `CNF9`" ]
    pub type CNF9W = super::crl::CNF0W;
    # [ doc = r" Proxy" ]
    pub struct _CNF9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNF9W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CNF9W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Push-Pull mode" ]
        # [ inline ( always ) ]
        pub fn push(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::PUSH)
        }
        # [ doc = "Open Drain-Mode" ]
        # [ inline ( always ) ]
        pub fn open(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::OPEN)
        }
        # [ doc = "Alternate Function Push-Pull Mode" ]
        # [ inline ( always ) ]
        pub fn alt_push(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::ALTPUSH)
        }
        # [ doc = "Alternate Function Open-Drain Mode" ]
        # [ inline ( always ) ]
        pub fn alt_open(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::ALTOPEN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `MODE10`" ]
    pub type MODE10W = super::crl::MODE0W;
    # [ doc = r" Proxy" ]
    pub struct _MODE10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MODE10W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MODE10W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Input mode" ]
        # [ inline ( always ) ]
        pub fn input(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::INPUT)
        }
        # [ doc = "Output mode 10 MHz" ]
        # [ inline ( always ) ]
        pub fn output(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::OUTPUT)
        }
        # [ doc = "Output mode 2 MHz" ]
        # [ inline ( always ) ]
        pub fn output2(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::OUTPUT2)
        }
        # [ doc = "Output mode 50 MHz" ]
        # [ inline ( always ) ]
        pub fn output50(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::OUTPUT50)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `CNF10`" ]
    pub type CNF10W = super::crl::CNF0W;
    # [ doc = r" Proxy" ]
    pub struct _CNF10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNF10W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CNF10W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Push-Pull mode" ]
        # [ inline ( always ) ]
        pub fn push(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::PUSH)
        }
        # [ doc = "Open Drain-Mode" ]
        # [ inline ( always ) ]
        pub fn open(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::OPEN)
        }
        # [ doc = "Alternate Function Push-Pull Mode" ]
        # [ inline ( always ) ]
        pub fn alt_push(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::ALTPUSH)
        }
        # [ doc = "Alternate Function Open-Drain Mode" ]
        # [ inline ( always ) ]
        pub fn alt_open(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::ALTOPEN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `MODE11`" ]
    pub type MODE11W = super::crl::MODE0W;
    # [ doc = r" Proxy" ]
    pub struct _MODE11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MODE11W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MODE11W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Input mode" ]
        # [ inline ( always ) ]
        pub fn input(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::INPUT)
        }
        # [ doc = "Output mode 10 MHz" ]
        # [ inline ( always ) ]
        pub fn output(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::OUTPUT)
        }
        # [ doc = "Output mode 2 MHz" ]
        # [ inline ( always ) ]
        pub fn output2(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::OUTPUT2)
        }
        # [ doc = "Output mode 50 MHz" ]
        # [ inline ( always ) ]
        pub fn output50(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::OUTPUT50)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `CNF11`" ]
    pub type CNF11W = super::crl::CNF0W;
    # [ doc = r" Proxy" ]
    pub struct _CNF11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNF11W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CNF11W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Push-Pull mode" ]
        # [ inline ( always ) ]
        pub fn push(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::PUSH)
        }
        # [ doc = "Open Drain-Mode" ]
        # [ inline ( always ) ]
        pub fn open(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::OPEN)
        }
        # [ doc = "Alternate Function Push-Pull Mode" ]
        # [ inline ( always ) ]
        pub fn alt_push(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::ALTPUSH)
        }
        # [ doc = "Alternate Function Open-Drain Mode" ]
        # [ inline ( always ) ]
        pub fn alt_open(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::ALTOPEN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `MODE12`" ]
    pub type MODE12W = super::crl::MODE0W;
    # [ doc = r" Proxy" ]
    pub struct _MODE12W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MODE12W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MODE12W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Input mode" ]
        # [ inline ( always ) ]
        pub fn input(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::INPUT)
        }
        # [ doc = "Output mode 10 MHz" ]
        # [ inline ( always ) ]
        pub fn output(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::OUTPUT)
        }
        # [ doc = "Output mode 2 MHz" ]
        # [ inline ( always ) ]
        pub fn output2(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::OUTPUT2)
        }
        # [ doc = "Output mode 50 MHz" ]
        # [ inline ( always ) ]
        pub fn output50(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::OUTPUT50)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `CNF12`" ]
    pub type CNF12W = super::crl::CNF0W;
    # [ doc = r" Proxy" ]
    pub struct _CNF12W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNF12W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CNF12W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Push-Pull mode" ]
        # [ inline ( always ) ]
        pub fn push(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::PUSH)
        }
        # [ doc = "Open Drain-Mode" ]
        # [ inline ( always ) ]
        pub fn open(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::OPEN)
        }
        # [ doc = "Alternate Function Push-Pull Mode" ]
        # [ inline ( always ) ]
        pub fn alt_push(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::ALTPUSH)
        }
        # [ doc = "Alternate Function Open-Drain Mode" ]
        # [ inline ( always ) ]
        pub fn alt_open(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::ALTOPEN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `MODE13`" ]
    pub type MODE13W = super::crl::MODE0W;
    # [ doc = r" Proxy" ]
    pub struct _MODE13W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MODE13W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MODE13W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Input mode" ]
        # [ inline ( always ) ]
        pub fn input(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::INPUT)
        }
        # [ doc = "Output mode 10 MHz" ]
        # [ inline ( always ) ]
        pub fn output(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::OUTPUT)
        }
        # [ doc = "Output mode 2 MHz" ]
        # [ inline ( always ) ]
        pub fn output2(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::OUTPUT2)
        }
        # [ doc = "Output mode 50 MHz" ]
        # [ inline ( always ) ]
        pub fn output50(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::OUTPUT50)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `CNF13`" ]
    pub type CNF13W = super::crl::CNF0W;
    # [ doc = r" Proxy" ]
    pub struct _CNF13W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNF13W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CNF13W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Push-Pull mode" ]
        # [ inline ( always ) ]
        pub fn push(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::PUSH)
        }
        # [ doc = "Open Drain-Mode" ]
        # [ inline ( always ) ]
        pub fn open(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::OPEN)
        }
        # [ doc = "Alternate Function Push-Pull Mode" ]
        # [ inline ( always ) ]
        pub fn alt_push(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::ALTPUSH)
        }
        # [ doc = "Alternate Function Open-Drain Mode" ]
        # [ inline ( always ) ]
        pub fn alt_open(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::ALTOPEN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `MODE14`" ]
    pub type MODE14W = super::crl::MODE0W;
    # [ doc = r" Proxy" ]
    pub struct _MODE14W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MODE14W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MODE14W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Input mode" ]
        # [ inline ( always ) ]
        pub fn input(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::INPUT)
        }
        # [ doc = "Output mode 10 MHz" ]
        # [ inline ( always ) ]
        pub fn output(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::OUTPUT)
        }
        # [ doc = "Output mode 2 MHz" ]
        # [ inline ( always ) ]
        pub fn output2(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::OUTPUT2)
        }
        # [ doc = "Output mode 50 MHz" ]
        # [ inline ( always ) ]
        pub fn output50(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::OUTPUT50)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `CNF14`" ]
    pub type CNF14W = super::crl::CNF0W;
    # [ doc = r" Proxy" ]
    pub struct _CNF14W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNF14W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CNF14W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Push-Pull mode" ]
        # [ inline ( always ) ]
        pub fn push(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::PUSH)
        }
        # [ doc = "Open Drain-Mode" ]
        # [ inline ( always ) ]
        pub fn open(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::OPEN)
        }
        # [ doc = "Alternate Function Push-Pull Mode" ]
        # [ inline ( always ) ]
        pub fn alt_push(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::ALTPUSH)
        }
        # [ doc = "Alternate Function Open-Drain Mode" ]
        # [ inline ( always ) ]
        pub fn alt_open(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::ALTOPEN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `MODE15`" ]
    pub type MODE15W = super::crl::MODE0W;
    # [ doc = r" Proxy" ]
    pub struct _MODE15W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MODE15W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MODE15W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Input mode" ]
        # [ inline ( always ) ]
        pub fn input(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::INPUT)
        }
        # [ doc = "Output mode 10 MHz" ]
        # [ inline ( always ) ]
        pub fn output(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::OUTPUT)
        }
        # [ doc = "Output mode 2 MHz" ]
        # [ inline ( always ) ]
        pub fn output2(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::OUTPUT2)
        }
        # [ doc = "Output mode 50 MHz" ]
        # [ inline ( always ) ]
        pub fn output50(self) -> &'a mut W {
            self.variant(super::crl::MODE0W::OUTPUT50)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `CNF15`" ]
    pub type CNF15W = super::crl::CNF0W;
    # [ doc = r" Proxy" ]
    pub struct _CNF15W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNF15W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CNF15W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Push-Pull mode" ]
        # [ inline ( always ) ]
        pub fn push(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::PUSH)
        }
        # [ doc = "Open Drain-Mode" ]
        # [ inline ( always ) ]
        pub fn open(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::OPEN)
        }
        # [ doc = "Alternate Function Push-Pull Mode" ]
        # [ inline ( always ) ]
        pub fn alt_push(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::ALTPUSH)
        }
        # [ doc = "Alternate Function Open-Drain Mode" ]
        # [ inline ( always ) ]
        pub fn alt_open(self) -> &'a mut W {
            self.variant(super::crl::CNF0W::ALTOPEN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
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
        # [ doc = "Bits 0:1 - Port n.8 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode8(&self) -> MODE8R {
            MODE8R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 2:3 - Port n.8 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf8(&self) -> CNF8R {
            CNF8R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 4:5 - Port n.9 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode9(&self) -> MODE9R {
            MODE9R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 6:7 - Port n.9 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf9(&self) -> CNF9R {
            CNF9R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 8:9 - Port n.10 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode10(&self) -> MODE10R {
            MODE10R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 10:11 - Port n.10 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf10(&self) -> CNF10R {
            CNF10R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 12:13 - Port n.11 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode11(&self) -> MODE11R {
            MODE11R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 14:15 - Port n.11 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf11(&self) -> CNF11R {
            CNF11R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 16:17 - Port n.12 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode12(&self) -> MODE12R {
            MODE12R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 18:19 - Port n.12 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf12(&self) -> CNF12R {
            CNF12R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 20:21 - Port n.13 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode13(&self) -> MODE13R {
            MODE13R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 22:23 - Port n.13 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf13(&self) -> CNF13R {
            CNF13R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 22;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 24:25 - Port n.14 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode14(&self) -> MODE14R {
            MODE14R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 26:27 - Port n.14 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf14(&self) -> CNF14R {
            CNF14R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 28:29 - Port n.15 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode15(&self) -> MODE15R {
            MODE15R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        # [ doc = "Bits 30:31 - Port n.15 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf15(&self) -> CNF15R {
            CNF15R::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 30;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 1145324612 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:1 - Port n.8 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode8(&mut self) -> _MODE8W {
            _MODE8W { w: self }
        }
        # [ doc = "Bits 2:3 - Port n.8 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf8(&mut self) -> _CNF8W {
            _CNF8W { w: self }
        }
        # [ doc = "Bits 4:5 - Port n.9 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode9(&mut self) -> _MODE9W {
            _MODE9W { w: self }
        }
        # [ doc = "Bits 6:7 - Port n.9 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf9(&mut self) -> _CNF9W {
            _CNF9W { w: self }
        }
        # [ doc = "Bits 8:9 - Port n.10 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode10(&mut self) -> _MODE10W {
            _MODE10W { w: self }
        }
        # [ doc = "Bits 10:11 - Port n.10 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf10(&mut self) -> _CNF10W {
            _CNF10W { w: self }
        }
        # [ doc = "Bits 12:13 - Port n.11 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode11(&mut self) -> _MODE11W {
            _MODE11W { w: self }
        }
        # [ doc = "Bits 14:15 - Port n.11 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf11(&mut self) -> _CNF11W {
            _CNF11W { w: self }
        }
        # [ doc = "Bits 16:17 - Port n.12 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode12(&mut self) -> _MODE12W {
            _MODE12W { w: self }
        }
        # [ doc = "Bits 18:19 - Port n.12 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf12(&mut self) -> _CNF12W {
            _CNF12W { w: self }
        }
        # [ doc = "Bits 20:21 - Port n.13 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode13(&mut self) -> _MODE13W {
            _MODE13W { w: self }
        }
        # [ doc = "Bits 22:23 - Port n.13 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf13(&mut self) -> _CNF13W {
            _CNF13W { w: self }
        }
        # [ doc = "Bits 24:25 - Port n.14 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode14(&mut self) -> _MODE14W {
            _MODE14W { w: self }
        }
        # [ doc = "Bits 26:27 - Port n.14 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf14(&mut self) -> _CNF14W {
            _CNF14W { w: self }
        }
        # [ doc = "Bits 28:29 - Port n.15 mode bits" ]
        # [ inline ( always ) ]
        pub fn mode15(&mut self) -> _MODE15W {
            _MODE15W { w: self }
        }
        # [ doc = "Bits 30:31 - Port n.15 configuration bits" ]
        # [ inline ( always ) ]
        pub fn cnf15(&mut self) -> _CNF15W {
            _CNF15W { w: self }
        }
    }
}
# [ doc = "Port input data register (GPIOn_IDR)" ]
pub struct IDR {
    register: VolatileCell<u32>,
}
# [ doc = "Port input data register (GPIOn_IDR)" ]
pub mod idr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::IDR {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IDR0R {
        bits: bool,
    }
    impl IDR0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IDR1R {
        bits: bool,
    }
    impl IDR1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IDR2R {
        bits: bool,
    }
    impl IDR2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IDR3R {
        bits: bool,
    }
    impl IDR3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IDR4R {
        bits: bool,
    }
    impl IDR4R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IDR5R {
        bits: bool,
    }
    impl IDR5R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IDR6R {
        bits: bool,
    }
    impl IDR6R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IDR7R {
        bits: bool,
    }
    impl IDR7R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IDR8R {
        bits: bool,
    }
    impl IDR8R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IDR9R {
        bits: bool,
    }
    impl IDR9R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IDR10R {
        bits: bool,
    }
    impl IDR10R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IDR11R {
        bits: bool,
    }
    impl IDR11R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IDR12R {
        bits: bool,
    }
    impl IDR12R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IDR13R {
        bits: bool,
    }
    impl IDR13R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IDR14R {
        bits: bool,
    }
    impl IDR14R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IDR15R {
        bits: bool,
    }
    impl IDR15R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
        # [ doc = "Bit 0 - Port input data" ]
        # [ inline ( always ) ]
        pub fn idr0(&self) -> IDR0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IDR0R { bits }
        }
        # [ doc = "Bit 1 - Port input data" ]
        # [ inline ( always ) ]
        pub fn idr1(&self) -> IDR1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IDR1R { bits }
        }
        # [ doc = "Bit 2 - Port input data" ]
        # [ inline ( always ) ]
        pub fn idr2(&self) -> IDR2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IDR2R { bits }
        }
        # [ doc = "Bit 3 - Port input data" ]
        # [ inline ( always ) ]
        pub fn idr3(&self) -> IDR3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IDR3R { bits }
        }
        # [ doc = "Bit 4 - Port input data" ]
        # [ inline ( always ) ]
        pub fn idr4(&self) -> IDR4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IDR4R { bits }
        }
        # [ doc = "Bit 5 - Port input data" ]
        # [ inline ( always ) ]
        pub fn idr5(&self) -> IDR5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IDR5R { bits }
        }
        # [ doc = "Bit 6 - Port input data" ]
        # [ inline ( always ) ]
        pub fn idr6(&self) -> IDR6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IDR6R { bits }
        }
        # [ doc = "Bit 7 - Port input data" ]
        # [ inline ( always ) ]
        pub fn idr7(&self) -> IDR7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IDR7R { bits }
        }
        # [ doc = "Bit 8 - Port input data" ]
        # [ inline ( always ) ]
        pub fn idr8(&self) -> IDR8R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IDR8R { bits }
        }
        # [ doc = "Bit 9 - Port input data" ]
        # [ inline ( always ) ]
        pub fn idr9(&self) -> IDR9R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IDR9R { bits }
        }
        # [ doc = "Bit 10 - Port input data" ]
        # [ inline ( always ) ]
        pub fn idr10(&self) -> IDR10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IDR10R { bits }
        }
        # [ doc = "Bit 11 - Port input data" ]
        # [ inline ( always ) ]
        pub fn idr11(&self) -> IDR11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IDR11R { bits }
        }
        # [ doc = "Bit 12 - Port input data" ]
        # [ inline ( always ) ]
        pub fn idr12(&self) -> IDR12R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IDR12R { bits }
        }
        # [ doc = "Bit 13 - Port input data" ]
        # [ inline ( always ) ]
        pub fn idr13(&self) -> IDR13R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IDR13R { bits }
        }
        # [ doc = "Bit 14 - Port input data" ]
        # [ inline ( always ) ]
        pub fn idr14(&self) -> IDR14R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IDR14R { bits }
        }
        # [ doc = "Bit 15 - Port input data" ]
        # [ inline ( always ) ]
        pub fn idr15(&self) -> IDR15R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IDR15R { bits }
        }
    }
}
# [ doc = "Port output data register (GPIOn_ODR)" ]
pub struct ODR {
    register: VolatileCell<u32>,
}
# [ doc = "Port output data register (GPIOn_ODR)" ]
pub mod odr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::ODR {
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
    pub struct ODR0R {
        bits: bool,
    }
    impl ODR0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ODR1R {
        bits: bool,
    }
    impl ODR1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ODR2R {
        bits: bool,
    }
    impl ODR2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ODR3R {
        bits: bool,
    }
    impl ODR3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ODR4R {
        bits: bool,
    }
    impl ODR4R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ODR5R {
        bits: bool,
    }
    impl ODR5R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ODR6R {
        bits: bool,
    }
    impl ODR6R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ODR7R {
        bits: bool,
    }
    impl ODR7R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ODR8R {
        bits: bool,
    }
    impl ODR8R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ODR9R {
        bits: bool,
    }
    impl ODR9R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ODR10R {
        bits: bool,
    }
    impl ODR10R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ODR11R {
        bits: bool,
    }
    impl ODR11R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ODR12R {
        bits: bool,
    }
    impl ODR12R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ODR13R {
        bits: bool,
    }
    impl ODR13R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ODR14R {
        bits: bool,
    }
    impl ODR14R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ODR15R {
        bits: bool,
    }
    impl ODR15R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _ODR0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _ODR0W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ODR1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _ODR1W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ODR2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _ODR2W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ODR3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _ODR3W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ODR4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _ODR4W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ODR5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _ODR5W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ODR6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _ODR6W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ODR7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _ODR7W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ODR8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _ODR8W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ODR9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _ODR9W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ODR10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _ODR10W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ODR11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _ODR11W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ODR12W<'a> {
        w: &'a mut W,
    }
    impl<'a> _ODR12W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ODR13W<'a> {
        w: &'a mut W,
    }
    impl<'a> _ODR13W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ODR14W<'a> {
        w: &'a mut W,
    }
    impl<'a> _ODR14W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ODR15W<'a> {
        w: &'a mut W,
    }
    impl<'a> _ODR15W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
        # [ doc = "Bit 0 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr0(&self) -> ODR0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ODR0R { bits }
        }
        # [ doc = "Bit 1 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr1(&self) -> ODR1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ODR1R { bits }
        }
        # [ doc = "Bit 2 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr2(&self) -> ODR2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ODR2R { bits }
        }
        # [ doc = "Bit 3 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr3(&self) -> ODR3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ODR3R { bits }
        }
        # [ doc = "Bit 4 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr4(&self) -> ODR4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ODR4R { bits }
        }
        # [ doc = "Bit 5 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr5(&self) -> ODR5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ODR5R { bits }
        }
        # [ doc = "Bit 6 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr6(&self) -> ODR6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ODR6R { bits }
        }
        # [ doc = "Bit 7 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr7(&self) -> ODR7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ODR7R { bits }
        }
        # [ doc = "Bit 8 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr8(&self) -> ODR8R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ODR8R { bits }
        }
        # [ doc = "Bit 9 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr9(&self) -> ODR9R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ODR9R { bits }
        }
        # [ doc = "Bit 10 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr10(&self) -> ODR10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ODR10R { bits }
        }
        # [ doc = "Bit 11 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr11(&self) -> ODR11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ODR11R { bits }
        }
        # [ doc = "Bit 12 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr12(&self) -> ODR12R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ODR12R { bits }
        }
        # [ doc = "Bit 13 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr13(&self) -> ODR13R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ODR13R { bits }
        }
        # [ doc = "Bit 14 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr14(&self) -> ODR14R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ODR14R { bits }
        }
        # [ doc = "Bit 15 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr15(&self) -> ODR15R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ODR15R { bits }
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
        # [ doc = "Bit 0 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr0(&mut self) -> _ODR0W {
            _ODR0W { w: self }
        }
        # [ doc = "Bit 1 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr1(&mut self) -> _ODR1W {
            _ODR1W { w: self }
        }
        # [ doc = "Bit 2 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr2(&mut self) -> _ODR2W {
            _ODR2W { w: self }
        }
        # [ doc = "Bit 3 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr3(&mut self) -> _ODR3W {
            _ODR3W { w: self }
        }
        # [ doc = "Bit 4 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr4(&mut self) -> _ODR4W {
            _ODR4W { w: self }
        }
        # [ doc = "Bit 5 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr5(&mut self) -> _ODR5W {
            _ODR5W { w: self }
        }
        # [ doc = "Bit 6 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr6(&mut self) -> _ODR6W {
            _ODR6W { w: self }
        }
        # [ doc = "Bit 7 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr7(&mut self) -> _ODR7W {
            _ODR7W { w: self }
        }
        # [ doc = "Bit 8 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr8(&mut self) -> _ODR8W {
            _ODR8W { w: self }
        }
        # [ doc = "Bit 9 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr9(&mut self) -> _ODR9W {
            _ODR9W { w: self }
        }
        # [ doc = "Bit 10 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr10(&mut self) -> _ODR10W {
            _ODR10W { w: self }
        }
        # [ doc = "Bit 11 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr11(&mut self) -> _ODR11W {
            _ODR11W { w: self }
        }
        # [ doc = "Bit 12 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr12(&mut self) -> _ODR12W {
            _ODR12W { w: self }
        }
        # [ doc = "Bit 13 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr13(&mut self) -> _ODR13W {
            _ODR13W { w: self }
        }
        # [ doc = "Bit 14 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr14(&mut self) -> _ODR14W {
            _ODR14W { w: self }
        }
        # [ doc = "Bit 15 - Port output data" ]
        # [ inline ( always ) ]
        pub fn odr15(&mut self) -> _ODR15W {
            _ODR15W { w: self }
        }
    }
}
# [ doc = "Port bit set/reset register (GPIOn_BSRR)" ]
pub struct BSRR {
    register: VolatileCell<u32>,
}
# [ doc = "Port bit set/reset register (GPIOn_BSRR)" ]
pub mod bsrr {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::BSRR {
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
    # [ doc = "Values that can be written to the field `BS0`" ]
    pub enum BS0W {
        # [ doc = "Sets the corresponding ODRx bit" ]
        SET,
    }
    impl BS0W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                BS0W::SET => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _BS0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BS0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BS0W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Sets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn set(self) -> &'a mut W {
            self.variant(BS0W::SET)
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
    # [ doc = "Values that can be written to the field `BS1`" ]
    pub type BS1W = BS0W;
    # [ doc = r" Proxy" ]
    pub struct _BS1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BS1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BS1W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Sets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn set(self) -> &'a mut W {
            self.variant(BS0W::SET)
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
    # [ doc = "Values that can be written to the field `BS2`" ]
    pub type BS2W = BS0W;
    # [ doc = r" Proxy" ]
    pub struct _BS2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BS2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BS2W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Sets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn set(self) -> &'a mut W {
            self.variant(BS0W::SET)
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
    # [ doc = "Values that can be written to the field `BS3`" ]
    pub type BS3W = BS0W;
    # [ doc = r" Proxy" ]
    pub struct _BS3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BS3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BS3W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Sets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn set(self) -> &'a mut W {
            self.variant(BS0W::SET)
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
    # [ doc = "Values that can be written to the field `BS4`" ]
    pub type BS4W = BS0W;
    # [ doc = r" Proxy" ]
    pub struct _BS4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BS4W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BS4W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Sets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn set(self) -> &'a mut W {
            self.variant(BS0W::SET)
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
    # [ doc = "Values that can be written to the field `BS5`" ]
    pub type BS5W = BS0W;
    # [ doc = r" Proxy" ]
    pub struct _BS5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BS5W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BS5W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Sets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn set(self) -> &'a mut W {
            self.variant(BS0W::SET)
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
    # [ doc = "Values that can be written to the field `BS6`" ]
    pub type BS6W = BS0W;
    # [ doc = r" Proxy" ]
    pub struct _BS6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BS6W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BS6W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Sets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn set(self) -> &'a mut W {
            self.variant(BS0W::SET)
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
    # [ doc = "Values that can be written to the field `BS7`" ]
    pub type BS7W = BS0W;
    # [ doc = r" Proxy" ]
    pub struct _BS7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BS7W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BS7W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Sets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn set(self) -> &'a mut W {
            self.variant(BS0W::SET)
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
    # [ doc = "Values that can be written to the field `BS8`" ]
    pub type BS8W = BS0W;
    # [ doc = r" Proxy" ]
    pub struct _BS8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BS8W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BS8W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Sets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn set(self) -> &'a mut W {
            self.variant(BS0W::SET)
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
    # [ doc = "Values that can be written to the field `BS9`" ]
    pub type BS9W = BS0W;
    # [ doc = r" Proxy" ]
    pub struct _BS9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BS9W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BS9W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Sets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn set(self) -> &'a mut W {
            self.variant(BS0W::SET)
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
    # [ doc = "Values that can be written to the field `BS10`" ]
    pub type BS10W = BS0W;
    # [ doc = r" Proxy" ]
    pub struct _BS10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BS10W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BS10W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Sets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn set(self) -> &'a mut W {
            self.variant(BS0W::SET)
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
    # [ doc = "Values that can be written to the field `BS11`" ]
    pub type BS11W = BS0W;
    # [ doc = r" Proxy" ]
    pub struct _BS11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BS11W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BS11W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Sets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn set(self) -> &'a mut W {
            self.variant(BS0W::SET)
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
    # [ doc = "Values that can be written to the field `BS12`" ]
    pub type BS12W = BS0W;
    # [ doc = r" Proxy" ]
    pub struct _BS12W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BS12W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BS12W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Sets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn set(self) -> &'a mut W {
            self.variant(BS0W::SET)
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
    # [ doc = "Values that can be written to the field `BS13`" ]
    pub type BS13W = BS0W;
    # [ doc = r" Proxy" ]
    pub struct _BS13W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BS13W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BS13W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Sets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn set(self) -> &'a mut W {
            self.variant(BS0W::SET)
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
    # [ doc = "Values that can be written to the field `BS14`" ]
    pub type BS14W = BS0W;
    # [ doc = r" Proxy" ]
    pub struct _BS14W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BS14W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BS14W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Sets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn set(self) -> &'a mut W {
            self.variant(BS0W::SET)
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
    # [ doc = "Values that can be written to the field `BS15`" ]
    pub type BS15W = BS0W;
    # [ doc = r" Proxy" ]
    pub struct _BS15W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BS15W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BS15W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Sets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn set(self) -> &'a mut W {
            self.variant(BS0W::SET)
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
    # [ doc = "Values that can be written to the field `BR0`" ]
    pub enum BR0W {
        # [ doc = "Resets the corresponding ODRx bit" ]
        RESET,
    }
    impl BR0W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                BR0W::RESET => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _BR0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BR0W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Resets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn reset(self) -> &'a mut W {
            self.variant(BR0W::RESET)
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
    # [ doc = "Values that can be written to the field `BR1`" ]
    pub type BR1W = BR0W;
    # [ doc = r" Proxy" ]
    pub struct _BR1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BR1W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Resets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn reset(self) -> &'a mut W {
            self.variant(BR0W::RESET)
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
    # [ doc = "Values that can be written to the field `BR2`" ]
    pub type BR2W = BR0W;
    # [ doc = r" Proxy" ]
    pub struct _BR2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BR2W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Resets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn reset(self) -> &'a mut W {
            self.variant(BR0W::RESET)
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
    # [ doc = "Values that can be written to the field `BR3`" ]
    pub type BR3W = BR0W;
    # [ doc = r" Proxy" ]
    pub struct _BR3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BR3W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Resets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn reset(self) -> &'a mut W {
            self.variant(BR0W::RESET)
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
    # [ doc = "Values that can be written to the field `BR4`" ]
    pub type BR4W = BR0W;
    # [ doc = r" Proxy" ]
    pub struct _BR4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR4W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BR4W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Resets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn reset(self) -> &'a mut W {
            self.variant(BR0W::RESET)
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
    # [ doc = "Values that can be written to the field `BR5`" ]
    pub type BR5W = BR0W;
    # [ doc = r" Proxy" ]
    pub struct _BR5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR5W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BR5W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Resets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn reset(self) -> &'a mut W {
            self.variant(BR0W::RESET)
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
    # [ doc = "Values that can be written to the field `BR6`" ]
    pub type BR6W = BR0W;
    # [ doc = r" Proxy" ]
    pub struct _BR6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR6W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BR6W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Resets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn reset(self) -> &'a mut W {
            self.variant(BR0W::RESET)
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
    # [ doc = "Values that can be written to the field `BR7`" ]
    pub type BR7W = BR0W;
    # [ doc = r" Proxy" ]
    pub struct _BR7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR7W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BR7W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Resets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn reset(self) -> &'a mut W {
            self.variant(BR0W::RESET)
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
    # [ doc = "Values that can be written to the field `BR8`" ]
    pub type BR8W = BR0W;
    # [ doc = r" Proxy" ]
    pub struct _BR8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR8W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BR8W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Resets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn reset(self) -> &'a mut W {
            self.variant(BR0W::RESET)
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
    # [ doc = "Values that can be written to the field `BR9`" ]
    pub type BR9W = BR0W;
    # [ doc = r" Proxy" ]
    pub struct _BR9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR9W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BR9W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Resets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn reset(self) -> &'a mut W {
            self.variant(BR0W::RESET)
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
    # [ doc = "Values that can be written to the field `BR10`" ]
    pub type BR10W = BR0W;
    # [ doc = r" Proxy" ]
    pub struct _BR10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR10W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BR10W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Resets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn reset(self) -> &'a mut W {
            self.variant(BR0W::RESET)
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
    # [ doc = "Values that can be written to the field `BR11`" ]
    pub type BR11W = BR0W;
    # [ doc = r" Proxy" ]
    pub struct _BR11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR11W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BR11W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Resets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn reset(self) -> &'a mut W {
            self.variant(BR0W::RESET)
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
    # [ doc = "Values that can be written to the field `BR12`" ]
    pub type BR12W = BR0W;
    # [ doc = r" Proxy" ]
    pub struct _BR12W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR12W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BR12W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Resets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn reset(self) -> &'a mut W {
            self.variant(BR0W::RESET)
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
    # [ doc = "Values that can be written to the field `BR13`" ]
    pub type BR13W = BR0W;
    # [ doc = r" Proxy" ]
    pub struct _BR13W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR13W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BR13W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Resets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn reset(self) -> &'a mut W {
            self.variant(BR0W::RESET)
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
    # [ doc = "Values that can be written to the field `BR14`" ]
    pub type BR14W = BR0W;
    # [ doc = r" Proxy" ]
    pub struct _BR14W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR14W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BR14W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Resets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn reset(self) -> &'a mut W {
            self.variant(BR0W::RESET)
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
    # [ doc = "Values that can be written to the field `BR15`" ]
    pub type BR15W = BR0W;
    # [ doc = r" Proxy" ]
    pub struct _BR15W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR15W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BR15W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Resets the corresponding ODRx bit" ]
        # [ inline ( always ) ]
        pub fn reset(self) -> &'a mut W {
            self.variant(BR0W::RESET)
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
        # [ doc = "Bit 0 - Set bit 0" ]
        # [ inline ( always ) ]
        pub fn bs0(&mut self) -> _BS0W {
            _BS0W { w: self }
        }
        # [ doc = "Bit 1 - Set bit 1" ]
        # [ inline ( always ) ]
        pub fn bs1(&mut self) -> _BS1W {
            _BS1W { w: self }
        }
        # [ doc = "Bit 2 - Set bit 1" ]
        # [ inline ( always ) ]
        pub fn bs2(&mut self) -> _BS2W {
            _BS2W { w: self }
        }
        # [ doc = "Bit 3 - Set bit 3" ]
        # [ inline ( always ) ]
        pub fn bs3(&mut self) -> _BS3W {
            _BS3W { w: self }
        }
        # [ doc = "Bit 4 - Set bit 4" ]
        # [ inline ( always ) ]
        pub fn bs4(&mut self) -> _BS4W {
            _BS4W { w: self }
        }
        # [ doc = "Bit 5 - Set bit 5" ]
        # [ inline ( always ) ]
        pub fn bs5(&mut self) -> _BS5W {
            _BS5W { w: self }
        }
        # [ doc = "Bit 6 - Set bit 6" ]
        # [ inline ( always ) ]
        pub fn bs6(&mut self) -> _BS6W {
            _BS6W { w: self }
        }
        # [ doc = "Bit 7 - Set bit 7" ]
        # [ inline ( always ) ]
        pub fn bs7(&mut self) -> _BS7W {
            _BS7W { w: self }
        }
        # [ doc = "Bit 8 - Set bit 8" ]
        # [ inline ( always ) ]
        pub fn bs8(&mut self) -> _BS8W {
            _BS8W { w: self }
        }
        # [ doc = "Bit 9 - Set bit 9" ]
        # [ inline ( always ) ]
        pub fn bs9(&mut self) -> _BS9W {
            _BS9W { w: self }
        }
        # [ doc = "Bit 10 - Set bit 10" ]
        # [ inline ( always ) ]
        pub fn bs10(&mut self) -> _BS10W {
            _BS10W { w: self }
        }
        # [ doc = "Bit 11 - Set bit 11" ]
        # [ inline ( always ) ]
        pub fn bs11(&mut self) -> _BS11W {
            _BS11W { w: self }
        }
        # [ doc = "Bit 12 - Set bit 12" ]
        # [ inline ( always ) ]
        pub fn bs12(&mut self) -> _BS12W {
            _BS12W { w: self }
        }
        # [ doc = "Bit 13 - Set bit 13" ]
        # [ inline ( always ) ]
        pub fn bs13(&mut self) -> _BS13W {
            _BS13W { w: self }
        }
        # [ doc = "Bit 14 - Set bit 14" ]
        # [ inline ( always ) ]
        pub fn bs14(&mut self) -> _BS14W {
            _BS14W { w: self }
        }
        # [ doc = "Bit 15 - Set bit 15" ]
        # [ inline ( always ) ]
        pub fn bs15(&mut self) -> _BS15W {
            _BS15W { w: self }
        }
        # [ doc = "Bit 16 - Reset bit 0" ]
        # [ inline ( always ) ]
        pub fn br0(&mut self) -> _BR0W {
            _BR0W { w: self }
        }
        # [ doc = "Bit 17 - Reset bit 1" ]
        # [ inline ( always ) ]
        pub fn br1(&mut self) -> _BR1W {
            _BR1W { w: self }
        }
        # [ doc = "Bit 18 - Reset bit 2" ]
        # [ inline ( always ) ]
        pub fn br2(&mut self) -> _BR2W {
            _BR2W { w: self }
        }
        # [ doc = "Bit 19 - Reset bit 3" ]
        # [ inline ( always ) ]
        pub fn br3(&mut self) -> _BR3W {
            _BR3W { w: self }
        }
        # [ doc = "Bit 20 - Reset bit 4" ]
        # [ inline ( always ) ]
        pub fn br4(&mut self) -> _BR4W {
            _BR4W { w: self }
        }
        # [ doc = "Bit 21 - Reset bit 5" ]
        # [ inline ( always ) ]
        pub fn br5(&mut self) -> _BR5W {
            _BR5W { w: self }
        }
        # [ doc = "Bit 22 - Reset bit 6" ]
        # [ inline ( always ) ]
        pub fn br6(&mut self) -> _BR6W {
            _BR6W { w: self }
        }
        # [ doc = "Bit 23 - Reset bit 7" ]
        # [ inline ( always ) ]
        pub fn br7(&mut self) -> _BR7W {
            _BR7W { w: self }
        }
        # [ doc = "Bit 24 - Reset bit 8" ]
        # [ inline ( always ) ]
        pub fn br8(&mut self) -> _BR8W {
            _BR8W { w: self }
        }
        # [ doc = "Bit 25 - Reset bit 9" ]
        # [ inline ( always ) ]
        pub fn br9(&mut self) -> _BR9W {
            _BR9W { w: self }
        }
        # [ doc = "Bit 26 - Reset bit 10" ]
        # [ inline ( always ) ]
        pub fn br10(&mut self) -> _BR10W {
            _BR10W { w: self }
        }
        # [ doc = "Bit 27 - Reset bit 11" ]
        # [ inline ( always ) ]
        pub fn br11(&mut self) -> _BR11W {
            _BR11W { w: self }
        }
        # [ doc = "Bit 28 - Reset bit 12" ]
        # [ inline ( always ) ]
        pub fn br12(&mut self) -> _BR12W {
            _BR12W { w: self }
        }
        # [ doc = "Bit 29 - Reset bit 13" ]
        # [ inline ( always ) ]
        pub fn br13(&mut self) -> _BR13W {
            _BR13W { w: self }
        }
        # [ doc = "Bit 30 - Reset bit 14" ]
        # [ inline ( always ) ]
        pub fn br14(&mut self) -> _BR14W {
            _BR14W { w: self }
        }
        # [ doc = "Bit 31 - Reset bit 15" ]
        # [ inline ( always ) ]
        pub fn br15(&mut self) -> _BR15W {
            _BR15W { w: self }
        }
    }
}
# [ doc = "Port bit reset register (GPIOn_BRR)" ]
pub struct BRR {
    register: VolatileCell<u32>,
}
# [ doc = "Port bit reset register (GPIOn_BRR)" ]
pub mod brr {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::BRR {
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
    pub struct _BR0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR0W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _BR1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR1W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _BR2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR2W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _BR3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR3W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _BR4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR4W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _BR5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR5W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _BR6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR6W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _BR7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR7W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _BR8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR8W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _BR9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR9W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _BR10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR10W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _BR11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR11W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _BR12W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR12W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _BR13W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR13W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _BR14W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR14W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _BR15W<'a> {
        w: &'a mut W,
    }
    impl<'a> _BR15W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
        # [ doc = "Bit 0 - Reset bit 0" ]
        # [ inline ( always ) ]
        pub fn br0(&mut self) -> _BR0W {
            _BR0W { w: self }
        }
        # [ doc = "Bit 1 - Reset bit 1" ]
        # [ inline ( always ) ]
        pub fn br1(&mut self) -> _BR1W {
            _BR1W { w: self }
        }
        # [ doc = "Bit 2 - Reset bit 1" ]
        # [ inline ( always ) ]
        pub fn br2(&mut self) -> _BR2W {
            _BR2W { w: self }
        }
        # [ doc = "Bit 3 - Reset bit 3" ]
        # [ inline ( always ) ]
        pub fn br3(&mut self) -> _BR3W {
            _BR3W { w: self }
        }
        # [ doc = "Bit 4 - Reset bit 4" ]
        # [ inline ( always ) ]
        pub fn br4(&mut self) -> _BR4W {
            _BR4W { w: self }
        }
        # [ doc = "Bit 5 - Reset bit 5" ]
        # [ inline ( always ) ]
        pub fn br5(&mut self) -> _BR5W {
            _BR5W { w: self }
        }
        # [ doc = "Bit 6 - Reset bit 6" ]
        # [ inline ( always ) ]
        pub fn br6(&mut self) -> _BR6W {
            _BR6W { w: self }
        }
        # [ doc = "Bit 7 - Reset bit 7" ]
        # [ inline ( always ) ]
        pub fn br7(&mut self) -> _BR7W {
            _BR7W { w: self }
        }
        # [ doc = "Bit 8 - Reset bit 8" ]
        # [ inline ( always ) ]
        pub fn br8(&mut self) -> _BR8W {
            _BR8W { w: self }
        }
        # [ doc = "Bit 9 - Reset bit 9" ]
        # [ inline ( always ) ]
        pub fn br9(&mut self) -> _BR9W {
            _BR9W { w: self }
        }
        # [ doc = "Bit 10 - Reset bit 10" ]
        # [ inline ( always ) ]
        pub fn br10(&mut self) -> _BR10W {
            _BR10W { w: self }
        }
        # [ doc = "Bit 11 - Reset bit 11" ]
        # [ inline ( always ) ]
        pub fn br11(&mut self) -> _BR11W {
            _BR11W { w: self }
        }
        # [ doc = "Bit 12 - Reset bit 12" ]
        # [ inline ( always ) ]
        pub fn br12(&mut self) -> _BR12W {
            _BR12W { w: self }
        }
        # [ doc = "Bit 13 - Reset bit 13" ]
        # [ inline ( always ) ]
        pub fn br13(&mut self) -> _BR13W {
            _BR13W { w: self }
        }
        # [ doc = "Bit 14 - Reset bit 14" ]
        # [ inline ( always ) ]
        pub fn br14(&mut self) -> _BR14W {
            _BR14W { w: self }
        }
        # [ doc = "Bit 15 - Reset bit 15" ]
        # [ inline ( always ) ]
        pub fn br15(&mut self) -> _BR15W {
            _BR15W { w: self }
        }
    }
}
# [ doc = "Port configuration lock register" ]
pub struct LCKR {
    register: VolatileCell<u32>,
}
# [ doc = "Port configuration lock register" ]
pub mod lckr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::LCKR {
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
    pub struct LCK0R {
        bits: bool,
    }
    impl LCK0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct LCK1R {
        bits: bool,
    }
    impl LCK1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct LCK2R {
        bits: bool,
    }
    impl LCK2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct LCK3R {
        bits: bool,
    }
    impl LCK3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct LCK4R {
        bits: bool,
    }
    impl LCK4R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct LCK5R {
        bits: bool,
    }
    impl LCK5R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct LCK6R {
        bits: bool,
    }
    impl LCK6R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct LCK7R {
        bits: bool,
    }
    impl LCK7R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct LCK8R {
        bits: bool,
    }
    impl LCK8R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct LCK9R {
        bits: bool,
    }
    impl LCK9R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct LCK10R {
        bits: bool,
    }
    impl LCK10R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct LCK11R {
        bits: bool,
    }
    impl LCK11R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct LCK12R {
        bits: bool,
    }
    impl LCK12R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct LCK13R {
        bits: bool,
    }
    impl LCK13R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct LCK14R {
        bits: bool,
    }
    impl LCK14R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct LCK15R {
        bits: bool,
    }
    impl LCK15R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct LCKKR {
        bits: bool,
    }
    impl LCKKR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _LCK0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _LCK0W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LCK1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _LCK1W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LCK2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _LCK2W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LCK3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _LCK3W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LCK4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _LCK4W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LCK5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _LCK5W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LCK6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _LCK6W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LCK7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _LCK7W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LCK8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _LCK8W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LCK9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _LCK9W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LCK10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _LCK10W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LCK11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _LCK11W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LCK12W<'a> {
        w: &'a mut W,
    }
    impl<'a> _LCK12W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LCK13W<'a> {
        w: &'a mut W,
    }
    impl<'a> _LCK13W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LCK14W<'a> {
        w: &'a mut W,
    }
    impl<'a> _LCK14W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LCK15W<'a> {
        w: &'a mut W,
    }
    impl<'a> _LCK15W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LCKKW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LCKKW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
        # [ doc = "Bit 0 - Port A Lock bit 0" ]
        # [ inline ( always ) ]
        pub fn lck0(&self) -> LCK0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LCK0R { bits }
        }
        # [ doc = "Bit 1 - Port A Lock bit 1" ]
        # [ inline ( always ) ]
        pub fn lck1(&self) -> LCK1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LCK1R { bits }
        }
        # [ doc = "Bit 2 - Port A Lock bit 2" ]
        # [ inline ( always ) ]
        pub fn lck2(&self) -> LCK2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LCK2R { bits }
        }
        # [ doc = "Bit 3 - Port A Lock bit 3" ]
        # [ inline ( always ) ]
        pub fn lck3(&self) -> LCK3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LCK3R { bits }
        }
        # [ doc = "Bit 4 - Port A Lock bit 4" ]
        # [ inline ( always ) ]
        pub fn lck4(&self) -> LCK4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LCK4R { bits }
        }
        # [ doc = "Bit 5 - Port A Lock bit 5" ]
        # [ inline ( always ) ]
        pub fn lck5(&self) -> LCK5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LCK5R { bits }
        }
        # [ doc = "Bit 6 - Port A Lock bit 6" ]
        # [ inline ( always ) ]
        pub fn lck6(&self) -> LCK6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LCK6R { bits }
        }
        # [ doc = "Bit 7 - Port A Lock bit 7" ]
        # [ inline ( always ) ]
        pub fn lck7(&self) -> LCK7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LCK7R { bits }
        }
        # [ doc = "Bit 8 - Port A Lock bit 8" ]
        # [ inline ( always ) ]
        pub fn lck8(&self) -> LCK8R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LCK8R { bits }
        }
        # [ doc = "Bit 9 - Port A Lock bit 9" ]
        # [ inline ( always ) ]
        pub fn lck9(&self) -> LCK9R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LCK9R { bits }
        }
        # [ doc = "Bit 10 - Port A Lock bit 10" ]
        # [ inline ( always ) ]
        pub fn lck10(&self) -> LCK10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LCK10R { bits }
        }
        # [ doc = "Bit 11 - Port A Lock bit 11" ]
        # [ inline ( always ) ]
        pub fn lck11(&self) -> LCK11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LCK11R { bits }
        }
        # [ doc = "Bit 12 - Port A Lock bit 12" ]
        # [ inline ( always ) ]
        pub fn lck12(&self) -> LCK12R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LCK12R { bits }
        }
        # [ doc = "Bit 13 - Port A Lock bit 13" ]
        # [ inline ( always ) ]
        pub fn lck13(&self) -> LCK13R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LCK13R { bits }
        }
        # [ doc = "Bit 14 - Port A Lock bit 14" ]
        # [ inline ( always ) ]
        pub fn lck14(&self) -> LCK14R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LCK14R { bits }
        }
        # [ doc = "Bit 15 - Port A Lock bit 15" ]
        # [ inline ( always ) ]
        pub fn lck15(&self) -> LCK15R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LCK15R { bits }
        }
        # [ doc = "Bit 16 - Lock key" ]
        # [ inline ( always ) ]
        pub fn lckk(&self) -> LCKKR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LCKKR { bits }
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
        # [ doc = "Bit 0 - Port A Lock bit 0" ]
        # [ inline ( always ) ]
        pub fn lck0(&mut self) -> _LCK0W {
            _LCK0W { w: self }
        }
        # [ doc = "Bit 1 - Port A Lock bit 1" ]
        # [ inline ( always ) ]
        pub fn lck1(&mut self) -> _LCK1W {
            _LCK1W { w: self }
        }
        # [ doc = "Bit 2 - Port A Lock bit 2" ]
        # [ inline ( always ) ]
        pub fn lck2(&mut self) -> _LCK2W {
            _LCK2W { w: self }
        }
        # [ doc = "Bit 3 - Port A Lock bit 3" ]
        # [ inline ( always ) ]
        pub fn lck3(&mut self) -> _LCK3W {
            _LCK3W { w: self }
        }
        # [ doc = "Bit 4 - Port A Lock bit 4" ]
        # [ inline ( always ) ]
        pub fn lck4(&mut self) -> _LCK4W {
            _LCK4W { w: self }
        }
        # [ doc = "Bit 5 - Port A Lock bit 5" ]
        # [ inline ( always ) ]
        pub fn lck5(&mut self) -> _LCK5W {
            _LCK5W { w: self }
        }
        # [ doc = "Bit 6 - Port A Lock bit 6" ]
        # [ inline ( always ) ]
        pub fn lck6(&mut self) -> _LCK6W {
            _LCK6W { w: self }
        }
        # [ doc = "Bit 7 - Port A Lock bit 7" ]
        # [ inline ( always ) ]
        pub fn lck7(&mut self) -> _LCK7W {
            _LCK7W { w: self }
        }
        # [ doc = "Bit 8 - Port A Lock bit 8" ]
        # [ inline ( always ) ]
        pub fn lck8(&mut self) -> _LCK8W {
            _LCK8W { w: self }
        }
        # [ doc = "Bit 9 - Port A Lock bit 9" ]
        # [ inline ( always ) ]
        pub fn lck9(&mut self) -> _LCK9W {
            _LCK9W { w: self }
        }
        # [ doc = "Bit 10 - Port A Lock bit 10" ]
        # [ inline ( always ) ]
        pub fn lck10(&mut self) -> _LCK10W {
            _LCK10W { w: self }
        }
        # [ doc = "Bit 11 - Port A Lock bit 11" ]
        # [ inline ( always ) ]
        pub fn lck11(&mut self) -> _LCK11W {
            _LCK11W { w: self }
        }
        # [ doc = "Bit 12 - Port A Lock bit 12" ]
        # [ inline ( always ) ]
        pub fn lck12(&mut self) -> _LCK12W {
            _LCK12W { w: self }
        }
        # [ doc = "Bit 13 - Port A Lock bit 13" ]
        # [ inline ( always ) ]
        pub fn lck13(&mut self) -> _LCK13W {
            _LCK13W { w: self }
        }
        # [ doc = "Bit 14 - Port A Lock bit 14" ]
        # [ inline ( always ) ]
        pub fn lck14(&mut self) -> _LCK14W {
            _LCK14W { w: self }
        }
        # [ doc = "Bit 15 - Port A Lock bit 15" ]
        # [ inline ( always ) ]
        pub fn lck15(&mut self) -> _LCK15W {
            _LCK15W { w: self }
        }
        # [ doc = "Bit 16 - Lock key" ]
        # [ inline ( always ) ]
        pub fn lckk(&mut self) -> _LCKKW {
            _LCKKW { w: self }
        }
    }
}
# [ doc = "General purpose I/O" ]
pub struct GPIOA {
    register_block: RegisterBlock,
}
impl Deref for GPIOA {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
# [ doc = "GPIOB" ]
pub const GPIOB: Peripheral<GPIOB> = unsafe { Peripheral::new(1073810432) };
# [ doc = r" Register block" ]
pub struct GPIOB {
    register_block: RegisterBlock,
}
impl Deref for GPIOB {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
# [ doc = "GPIOC" ]
pub const GPIOC: Peripheral<GPIOC> = unsafe { Peripheral::new(1073811456) };
# [ doc = r" Register block" ]
pub struct GPIOC {
    register_block: RegisterBlock,
}
impl Deref for GPIOC {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
# [ doc = "GPIOD" ]
pub const GPIOD: Peripheral<GPIOD> = unsafe { Peripheral::new(1073812480) };
# [ doc = r" Register block" ]
pub struct GPIOD {
    register_block: RegisterBlock,
}
impl Deref for GPIOD {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
# [ doc = "GPIOE" ]
pub const GPIOE: Peripheral<GPIOE> = unsafe { Peripheral::new(1073813504) };
# [ doc = r" Register block" ]
pub struct GPIOE {
    register_block: RegisterBlock,
}
impl Deref for GPIOE {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
# [ doc = "GPIOF" ]
pub const GPIOF: Peripheral<GPIOF> = unsafe { Peripheral::new(1073814528) };
# [ doc = r" Register block" ]
pub struct GPIOF {
    register_block: RegisterBlock,
}
impl Deref for GPIOF {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
# [ doc = "GPIOG" ]
pub const GPIOG: Peripheral<GPIOG> = unsafe { Peripheral::new(1073815552) };
# [ doc = r" Register block" ]
pub struct GPIOG {
    register_block: RegisterBlock,
}
impl Deref for GPIOG {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
