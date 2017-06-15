# ! [ doc = r" Interrupts" ]

use cortex_m::ctxt::Context;
use cortex_m::exception;
use cortex_m::interrupt::Nr;

# [ doc = "0 - Window Watchdog interrupt" ]
pub struct WWDG {
    _0: (),
}
unsafe impl Context for WWDG {}
unsafe impl Nr for WWDG {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        0
    }
}
impl !Send for WWDG {}
# [ doc = "1 - PVD through EXTI line detection interrupt" ]
pub struct PVD {
    _0: (),
}
unsafe impl Context for PVD {}
unsafe impl Nr for PVD {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        1
    }
}
impl !Send for PVD {}
# [ doc = "2 - Tamper interrupt" ]
pub struct TAMPER {
    _0: (),
}
unsafe impl Context for TAMPER {}
unsafe impl Nr for TAMPER {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        2
    }
}
impl !Send for TAMPER {}
# [ doc = "3 - RTC global interrupt" ]
pub struct RTC {
    _0: (),
}
unsafe impl Context for RTC {}
unsafe impl Nr for RTC {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        3
    }
}
impl !Send for RTC {}
# [ doc = "4 - Flash global interrupt" ]
pub struct FLASH {
    _0: (),
}
unsafe impl Context for FLASH {}
unsafe impl Nr for FLASH {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        4
    }
}
impl !Send for FLASH {}
# [ doc = "5 - RCC global interrupt" ]
pub struct RCC {
    _0: (),
}
unsafe impl Context for RCC {}
unsafe impl Nr for RCC {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        5
    }
}
impl !Send for RCC {}
# [ doc = "6 - EXTI Line0 interrupt" ]
pub struct EXTI0 {
    _0: (),
}
unsafe impl Context for EXTI0 {}
unsafe impl Nr for EXTI0 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        6
    }
}
impl !Send for EXTI0 {}
# [ doc = "7 - EXTI Line1 interrupt" ]
pub struct EXTI1 {
    _0: (),
}
unsafe impl Context for EXTI1 {}
unsafe impl Nr for EXTI1 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        7
    }
}
impl !Send for EXTI1 {}
# [ doc = "8 - EXTI Line2 interrupt" ]
pub struct EXTI2 {
    _0: (),
}
unsafe impl Context for EXTI2 {}
unsafe impl Nr for EXTI2 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        8
    }
}
impl !Send for EXTI2 {}
# [ doc = "9 - EXTI Line3 interrupt" ]
pub struct EXTI3 {
    _0: (),
}
unsafe impl Context for EXTI3 {}
unsafe impl Nr for EXTI3 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        9
    }
}
impl !Send for EXTI3 {}
# [ doc = "10 - EXTI Line4 interrupt" ]
pub struct EXTI4 {
    _0: (),
}
unsafe impl Context for EXTI4 {}
unsafe impl Nr for EXTI4 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        10
    }
}
impl !Send for EXTI4 {}
# [ doc = "11 - DMA1 Channel1 global interrupt" ]
pub struct DMA1_CHANNEL1 {
    _0: (),
}
unsafe impl Context for DMA1_CHANNEL1 {}
unsafe impl Nr for DMA1_CHANNEL1 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        11
    }
}
impl !Send for DMA1_CHANNEL1 {}
# [ doc = "12 - DMA1 Channel2 global interrupt" ]
pub struct DMA1_CHANNEL2 {
    _0: (),
}
unsafe impl Context for DMA1_CHANNEL2 {}
unsafe impl Nr for DMA1_CHANNEL2 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        12
    }
}
impl !Send for DMA1_CHANNEL2 {}
# [ doc = "13 - DMA1 Channel3 global interrupt" ]
pub struct DMA1_CHANNEL3 {
    _0: (),
}
unsafe impl Context for DMA1_CHANNEL3 {}
unsafe impl Nr for DMA1_CHANNEL3 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        13
    }
}
impl !Send for DMA1_CHANNEL3 {}
# [ doc = "14 - DMA1 Channel4 global interrupt" ]
pub struct DMA1_CHANNEL4 {
    _0: (),
}
unsafe impl Context for DMA1_CHANNEL4 {}
unsafe impl Nr for DMA1_CHANNEL4 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        14
    }
}
impl !Send for DMA1_CHANNEL4 {}
# [ doc = "15 - DMA1 Channel5 global interrupt" ]
pub struct DMA1_CHANNEL5 {
    _0: (),
}
unsafe impl Context for DMA1_CHANNEL5 {}
unsafe impl Nr for DMA1_CHANNEL5 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        15
    }
}
impl !Send for DMA1_CHANNEL5 {}
# [ doc = "16 - DMA1 Channel6 global interrupt" ]
pub struct DMA1_CHANNEL6 {
    _0: (),
}
unsafe impl Context for DMA1_CHANNEL6 {}
unsafe impl Nr for DMA1_CHANNEL6 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        16
    }
}
impl !Send for DMA1_CHANNEL6 {}
# [ doc = "17 - DMA1 Channel7 global interrupt" ]
pub struct DMA1_CHANNEL7 {
    _0: (),
}
unsafe impl Context for DMA1_CHANNEL7 {}
unsafe impl Nr for DMA1_CHANNEL7 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        17
    }
}
impl !Send for DMA1_CHANNEL7 {}
# [ doc = "18 - ADC2 global interrupt" ]
pub struct ADC {
    _0: (),
}
unsafe impl Context for ADC {}
unsafe impl Nr for ADC {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        18
    }
}
impl !Send for ADC {}
# [ doc = "19 - CAN1 TX interrupts" ]
pub struct CAN1_TX {
    _0: (),
}
unsafe impl Context for CAN1_TX {}
unsafe impl Nr for CAN1_TX {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        19
    }
}
impl !Send for CAN1_TX {}
# [ doc = "20 - CAN1 RX0 interrupts" ]
pub struct CAN1_RX0 {
    _0: (),
}
unsafe impl Context for CAN1_RX0 {}
unsafe impl Nr for CAN1_RX0 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        20
    }
}
impl !Send for CAN1_RX0 {}
# [ doc = "21 - CAN1 RX1 interrupt" ]
pub struct CAN1_RX1 {
    _0: (),
}
unsafe impl Context for CAN1_RX1 {}
unsafe impl Nr for CAN1_RX1 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        21
    }
}
impl !Send for CAN1_RX1 {}
# [ doc = "22 - CAN1 SCE interrupt" ]
pub struct CAN1_SCE {
    _0: (),
}
unsafe impl Context for CAN1_SCE {}
unsafe impl Nr for CAN1_SCE {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        22
    }
}
impl !Send for CAN1_SCE {}
# [ doc = "23 - EXTI Line[9:5] interrupts" ]
pub struct EXTI9_5 {
    _0: (),
}
unsafe impl Context for EXTI9_5 {}
unsafe impl Nr for EXTI9_5 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        23
    }
}
impl !Send for EXTI9_5 {}
# [ doc = "24 - TIM1 Break interrupt and TIM9 global interrupt" ]
pub struct TIM1_BRK_TIM9 {
    _0: (),
}
unsafe impl Context for TIM1_BRK_TIM9 {}
unsafe impl Nr for TIM1_BRK_TIM9 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        24
    }
}
impl !Send for TIM1_BRK_TIM9 {}
# [ doc = "25 - TIM1 Update interrupt and TIM10 global interrupt" ]
pub struct TIM1_UP_TIM10 {
    _0: (),
}
unsafe impl Context for TIM1_UP_TIM10 {}
unsafe impl Nr for TIM1_UP_TIM10 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        25
    }
}
impl !Send for TIM1_UP_TIM10 {}
# [ doc = "26 - TIM1 Trigger and Commutation interrupts and TIM11 global interrupt" ]
pub struct TIM1_TRG_COM_TIM11 {
    _0: (),
}
unsafe impl Context for TIM1_TRG_COM_TIM11 {}
unsafe impl Nr for TIM1_TRG_COM_TIM11 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        26
    }
}
impl !Send for TIM1_TRG_COM_TIM11 {}
# [ doc = "27 - TIM1 Capture Compare interrupt" ]
pub struct TIM1_CC {
    _0: (),
}
unsafe impl Context for TIM1_CC {}
unsafe impl Nr for TIM1_CC {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        27
    }
}
impl !Send for TIM1_CC {}
# [ doc = "28 - TIM2 global interrupt" ]
pub struct TIM2 {
    _0: (),
}
unsafe impl Context for TIM2 {}
unsafe impl Nr for TIM2 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        28
    }
}
impl !Send for TIM2 {}
# [ doc = "29 - TIM3 global interrupt" ]
pub struct TIM3 {
    _0: (),
}
unsafe impl Context for TIM3 {}
unsafe impl Nr for TIM3 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        29
    }
}
impl !Send for TIM3 {}
# [ doc = "30 - TIM4 global interrupt" ]
pub struct TIM4 {
    _0: (),
}
unsafe impl Context for TIM4 {}
unsafe impl Nr for TIM4 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        30
    }
}
impl !Send for TIM4 {}
# [ doc = "31 - I2C1 event interrupt" ]
pub struct I2C1_EV {
    _0: (),
}
unsafe impl Context for I2C1_EV {}
unsafe impl Nr for I2C1_EV {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        31
    }
}
impl !Send for I2C1_EV {}
# [ doc = "32 - I2C1 error interrupt" ]
pub struct I2C1_ER {
    _0: (),
}
unsafe impl Context for I2C1_ER {}
unsafe impl Nr for I2C1_ER {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        32
    }
}
impl !Send for I2C1_ER {}
# [ doc = "33 - I2C2 event interrupt" ]
pub struct I2C2_EV {
    _0: (),
}
unsafe impl Context for I2C2_EV {}
unsafe impl Nr for I2C2_EV {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        33
    }
}
impl !Send for I2C2_EV {}
# [ doc = "34 - I2C2 error interrupt" ]
pub struct I2C2_ER {
    _0: (),
}
unsafe impl Context for I2C2_ER {}
unsafe impl Nr for I2C2_ER {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        34
    }
}
impl !Send for I2C2_ER {}
# [ doc = "35 - SPI1 global interrupt" ]
pub struct SPI1 {
    _0: (),
}
unsafe impl Context for SPI1 {}
unsafe impl Nr for SPI1 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        35
    }
}
impl !Send for SPI1 {}
# [ doc = "36 - SPI2 global interrupt" ]
pub struct SPI2 {
    _0: (),
}
unsafe impl Context for SPI2 {}
unsafe impl Nr for SPI2 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        36
    }
}
impl !Send for SPI2 {}
# [ doc = "37 - USART1 global interrupt" ]
pub struct USART1 {
    _0: (),
}
unsafe impl Context for USART1 {}
unsafe impl Nr for USART1 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        37
    }
}
impl !Send for USART1 {}
# [ doc = "38 - USART2 global interrupt" ]
pub struct USART2 {
    _0: (),
}
unsafe impl Context for USART2 {}
unsafe impl Nr for USART2 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        38
    }
}
impl !Send for USART2 {}
# [ doc = "39 - USART3 global interrupt" ]
pub struct USART3 {
    _0: (),
}
unsafe impl Context for USART3 {}
unsafe impl Nr for USART3 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        39
    }
}
impl !Send for USART3 {}
# [ doc = "40 - EXTI Line[15:10] interrupts" ]
pub struct EXTI15_10 {
    _0: (),
}
unsafe impl Context for EXTI15_10 {}
unsafe impl Nr for EXTI15_10 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        40
    }
}
impl !Send for EXTI15_10 {}
# [ doc = "41 - RTC Alarms through EXTI line interrupt" ]
pub struct RTCALARM {
    _0: (),
}
unsafe impl Context for RTCALARM {}
unsafe impl Nr for RTCALARM {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        41
    }
}
impl !Send for RTCALARM {}
# [ doc = "42 - USB Device FS Wakeup through EXTI line interrupt" ]
pub struct USB_FS_WKUP {
    _0: (),
}
unsafe impl Context for USB_FS_WKUP {}
unsafe impl Nr for USB_FS_WKUP {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        42
    }
}
impl !Send for USB_FS_WKUP {}
# [ doc = "43 - TIM8 Break interrupt and TIM12 global interrupt" ]
pub struct TIM8_BRK_TIM12 {
    _0: (),
}
unsafe impl Context for TIM8_BRK_TIM12 {}
unsafe impl Nr for TIM8_BRK_TIM12 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        43
    }
}
impl !Send for TIM8_BRK_TIM12 {}
# [ doc = "44 - TIM8 Update interrupt and TIM13 global interrupt" ]
pub struct TIM8_UP_TIM13 {
    _0: (),
}
unsafe impl Context for TIM8_UP_TIM13 {}
unsafe impl Nr for TIM8_UP_TIM13 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        44
    }
}
impl !Send for TIM8_UP_TIM13 {}
# [ doc = "45 - TIM8 Trigger and Commutation interrupts and TIM14 global interrupt" ]
pub struct TIM8_TRG_COM_TIM14 {
    _0: (),
}
unsafe impl Context for TIM8_TRG_COM_TIM14 {}
unsafe impl Nr for TIM8_TRG_COM_TIM14 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        45
    }
}
impl !Send for TIM8_TRG_COM_TIM14 {}
# [ doc = "46 - TIM8 Capture Compare interrupt" ]
pub struct TIM8_CC {
    _0: (),
}
unsafe impl Context for TIM8_CC {}
unsafe impl Nr for TIM8_CC {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        46
    }
}
impl !Send for TIM8_CC {}
# [ doc = "47 - ADC3 global interrupt" ]
pub struct ADC3 {
    _0: (),
}
unsafe impl Context for ADC3 {}
unsafe impl Nr for ADC3 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        47
    }
}
impl !Send for ADC3 {}
# [ doc = "48 - FSMC global interrupt" ]
pub struct FSMC {
    _0: (),
}
unsafe impl Context for FSMC {}
unsafe impl Nr for FSMC {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        48
    }
}
impl !Send for FSMC {}
# [ doc = "49 - SDIO global interrupt" ]
pub struct SDIO {
    _0: (),
}
unsafe impl Context for SDIO {}
unsafe impl Nr for SDIO {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        49
    }
}
impl !Send for SDIO {}
# [ doc = "50 - TIM5 global interrupt" ]
pub struct TIM5 {
    _0: (),
}
unsafe impl Context for TIM5 {}
unsafe impl Nr for TIM5 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        50
    }
}
impl !Send for TIM5 {}
# [ doc = "51 - SPI3 global interrupt" ]
pub struct SPI3 {
    _0: (),
}
unsafe impl Context for SPI3 {}
unsafe impl Nr for SPI3 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        51
    }
}
impl !Send for SPI3 {}
# [ doc = "52 - UART4 global interrupt" ]
pub struct UART4 {
    _0: (),
}
unsafe impl Context for UART4 {}
unsafe impl Nr for UART4 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        52
    }
}
impl !Send for UART4 {}
# [ doc = "53 - UART5 global interrupt" ]
pub struct UART5 {
    _0: (),
}
unsafe impl Context for UART5 {}
unsafe impl Nr for UART5 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        53
    }
}
impl !Send for UART5 {}
# [ doc = "54 - TIM6 global interrupt" ]
pub struct TIM6 {
    _0: (),
}
unsafe impl Context for TIM6 {}
unsafe impl Nr for TIM6 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        54
    }
}
impl !Send for TIM6 {}
# [ doc = "55 - TIM7 global interrupt" ]
pub struct TIM7 {
    _0: (),
}
unsafe impl Context for TIM7 {}
unsafe impl Nr for TIM7 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        55
    }
}
impl !Send for TIM7 {}
# [ doc = "56 - DMA2 Channel1 global interrupt" ]
pub struct DMA2_CHANNEL1 {
    _0: (),
}
unsafe impl Context for DMA2_CHANNEL1 {}
unsafe impl Nr for DMA2_CHANNEL1 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        56
    }
}
impl !Send for DMA2_CHANNEL1 {}
# [ doc = "57 - DMA2 Channel2 global interrupt" ]
pub struct DMA2_CHANNEL2 {
    _0: (),
}
unsafe impl Context for DMA2_CHANNEL2 {}
unsafe impl Nr for DMA2_CHANNEL2 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        57
    }
}
impl !Send for DMA2_CHANNEL2 {}
# [ doc = "58 - DMA2 Channel3 global interrupt" ]
pub struct DMA2_CHANNEL3 {
    _0: (),
}
unsafe impl Context for DMA2_CHANNEL3 {}
unsafe impl Nr for DMA2_CHANNEL3 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        58
    }
}
impl !Send for DMA2_CHANNEL3 {}
# [ doc = "59 - DMA2 Channel4 and DMA2 Channel5 global interrupt" ]
pub struct DMA2_CHANNEL4_5 {
    _0: (),
}
unsafe impl Context for DMA2_CHANNEL4_5 {}
unsafe impl Nr for DMA2_CHANNEL4_5 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        59
    }
}
impl !Send for DMA2_CHANNEL4_5 {}
# [ doc = r" Interrupt handlers" ]
# [ allow ( non_snake_case ) ]
# [ repr ( C ) ]
pub struct Handlers {
    # [ doc = "0 - Window Watchdog interrupt" ]
    pub WWDG: extern "C" fn(WWDG),
    # [ doc = "1 - PVD through EXTI line detection interrupt" ]
    pub PVD: extern "C" fn(PVD),
    # [ doc = "2 - Tamper interrupt" ]
    pub TAMPER: extern "C" fn(TAMPER),
    # [ doc = "3 - RTC global interrupt" ]
    pub RTC: extern "C" fn(RTC),
    # [ doc = "4 - Flash global interrupt" ]
    pub FLASH: extern "C" fn(FLASH),
    # [ doc = "5 - RCC global interrupt" ]
    pub RCC: extern "C" fn(RCC),
    # [ doc = "6 - EXTI Line0 interrupt" ]
    pub EXTI0: extern "C" fn(EXTI0),
    # [ doc = "7 - EXTI Line1 interrupt" ]
    pub EXTI1: extern "C" fn(EXTI1),
    # [ doc = "8 - EXTI Line2 interrupt" ]
    pub EXTI2: extern "C" fn(EXTI2),
    # [ doc = "9 - EXTI Line3 interrupt" ]
    pub EXTI3: extern "C" fn(EXTI3),
    # [ doc = "10 - EXTI Line4 interrupt" ]
    pub EXTI4: extern "C" fn(EXTI4),
    # [ doc = "11 - DMA1 Channel1 global interrupt" ]
    pub DMA1_CHANNEL1: extern "C" fn(DMA1_CHANNEL1),
    # [ doc = "12 - DMA1 Channel2 global interrupt" ]
    pub DMA1_CHANNEL2: extern "C" fn(DMA1_CHANNEL2),
    # [ doc = "13 - DMA1 Channel3 global interrupt" ]
    pub DMA1_CHANNEL3: extern "C" fn(DMA1_CHANNEL3),
    # [ doc = "14 - DMA1 Channel4 global interrupt" ]
    pub DMA1_CHANNEL4: extern "C" fn(DMA1_CHANNEL4),
    # [ doc = "15 - DMA1 Channel5 global interrupt" ]
    pub DMA1_CHANNEL5: extern "C" fn(DMA1_CHANNEL5),
    # [ doc = "16 - DMA1 Channel6 global interrupt" ]
    pub DMA1_CHANNEL6: extern "C" fn(DMA1_CHANNEL6),
    # [ doc = "17 - DMA1 Channel7 global interrupt" ]
    pub DMA1_CHANNEL7: extern "C" fn(DMA1_CHANNEL7),
    # [ doc = "18 - ADC2 global interrupt" ]
    pub ADC: extern "C" fn(ADC),
    # [ doc = "19 - CAN1 TX interrupts" ]
    pub CAN1_TX: extern "C" fn(CAN1_TX),
    # [ doc = "20 - CAN1 RX0 interrupts" ]
    pub CAN1_RX0: extern "C" fn(CAN1_RX0),
    # [ doc = "21 - CAN1 RX1 interrupt" ]
    pub CAN1_RX1: extern "C" fn(CAN1_RX1),
    # [ doc = "22 - CAN1 SCE interrupt" ]
    pub CAN1_SCE: extern "C" fn(CAN1_SCE),
    # [ doc = "23 - EXTI Line[9:5] interrupts" ]
    pub EXTI9_5: extern "C" fn(EXTI9_5),
    # [ doc = "24 - TIM1 Break interrupt and TIM9 global interrupt" ]
    pub TIM1_BRK_TIM9: extern "C" fn(TIM1_BRK_TIM9),
    # [ doc = "25 - TIM1 Update interrupt and TIM10 global interrupt" ]
    pub TIM1_UP_TIM10: extern "C" fn(TIM1_UP_TIM10),
    # [ doc = "26 - TIM1 Trigger and Commutation interrupts and TIM11 global interrupt" ]
    pub TIM1_TRG_COM_TIM11: extern "C" fn(TIM1_TRG_COM_TIM11),
    # [ doc = "27 - TIM1 Capture Compare interrupt" ]
    pub TIM1_CC: extern "C" fn(TIM1_CC),
    # [ doc = "28 - TIM2 global interrupt" ]
    pub TIM2: extern "C" fn(TIM2),
    # [ doc = "29 - TIM3 global interrupt" ]
    pub TIM3: extern "C" fn(TIM3),
    # [ doc = "30 - TIM4 global interrupt" ]
    pub TIM4: extern "C" fn(TIM4),
    # [ doc = "31 - I2C1 event interrupt" ]
    pub I2C1_EV: extern "C" fn(I2C1_EV),
    # [ doc = "32 - I2C1 error interrupt" ]
    pub I2C1_ER: extern "C" fn(I2C1_ER),
    # [ doc = "33 - I2C2 event interrupt" ]
    pub I2C2_EV: extern "C" fn(I2C2_EV),
    # [ doc = "34 - I2C2 error interrupt" ]
    pub I2C2_ER: extern "C" fn(I2C2_ER),
    # [ doc = "35 - SPI1 global interrupt" ]
    pub SPI1: extern "C" fn(SPI1),
    # [ doc = "36 - SPI2 global interrupt" ]
    pub SPI2: extern "C" fn(SPI2),
    # [ doc = "37 - USART1 global interrupt" ]
    pub USART1: extern "C" fn(USART1),
    # [ doc = "38 - USART2 global interrupt" ]
    pub USART2: extern "C" fn(USART2),
    # [ doc = "39 - USART3 global interrupt" ]
    pub USART3: extern "C" fn(USART3),
    # [ doc = "40 - EXTI Line[15:10] interrupts" ]
    pub EXTI15_10: extern "C" fn(EXTI15_10),
    # [ doc = "41 - RTC Alarms through EXTI line interrupt" ]
    pub RTCALARM: extern "C" fn(RTCALARM),
    # [ doc = "42 - USB Device FS Wakeup through EXTI line interrupt" ]
    pub USB_FS_WKUP: extern "C" fn(USB_FS_WKUP),
    # [ doc = "43 - TIM8 Break interrupt and TIM12 global interrupt" ]
    pub TIM8_BRK_TIM12: extern "C" fn(TIM8_BRK_TIM12),
    # [ doc = "44 - TIM8 Update interrupt and TIM13 global interrupt" ]
    pub TIM8_UP_TIM13: extern "C" fn(TIM8_UP_TIM13),
    # [ doc = "45 - TIM8 Trigger and Commutation interrupts and TIM14 global interrupt" ]
    pub TIM8_TRG_COM_TIM14: extern "C" fn(TIM8_TRG_COM_TIM14),
    # [ doc = "46 - TIM8 Capture Compare interrupt" ]
    pub TIM8_CC: extern "C" fn(TIM8_CC),
    # [ doc = "47 - ADC3 global interrupt" ]
    pub ADC3: extern "C" fn(ADC3),
    # [ doc = "48 - FSMC global interrupt" ]
    pub FSMC: extern "C" fn(FSMC),
    # [ doc = "49 - SDIO global interrupt" ]
    pub SDIO: extern "C" fn(SDIO),
    # [ doc = "50 - TIM5 global interrupt" ]
    pub TIM5: extern "C" fn(TIM5),
    # [ doc = "51 - SPI3 global interrupt" ]
    pub SPI3: extern "C" fn(SPI3),
    # [ doc = "52 - UART4 global interrupt" ]
    pub UART4: extern "C" fn(UART4),
    # [ doc = "53 - UART5 global interrupt" ]
    pub UART5: extern "C" fn(UART5),
    # [ doc = "54 - TIM6 global interrupt" ]
    pub TIM6: extern "C" fn(TIM6),
    # [ doc = "55 - TIM7 global interrupt" ]
    pub TIM7: extern "C" fn(TIM7),
    # [ doc = "56 - DMA2 Channel1 global interrupt" ]
    pub DMA2_CHANNEL1: extern "C" fn(DMA2_CHANNEL1),
    # [ doc = "57 - DMA2 Channel2 global interrupt" ]
    pub DMA2_CHANNEL2: extern "C" fn(DMA2_CHANNEL2),
    # [ doc = "58 - DMA2 Channel3 global interrupt" ]
    pub DMA2_CHANNEL3: extern "C" fn(DMA2_CHANNEL3),
    # [ doc = "59 - DMA2 Channel4 and DMA2 Channel5 global interrupt" ]
    pub DMA2_CHANNEL4_5: extern "C" fn(DMA2_CHANNEL4_5),
}
# [ doc = r" Default interrupt handlers" ]
pub const DEFAULT_HANDLERS: Handlers = Handlers {
    WWDG: exception::default_handler,
    PVD: exception::default_handler,
    TAMPER: exception::default_handler,
    RTC: exception::default_handler,
    FLASH: exception::default_handler,
    RCC: exception::default_handler,
    EXTI0: exception::default_handler,
    EXTI1: exception::default_handler,
    EXTI2: exception::default_handler,
    EXTI3: exception::default_handler,
    EXTI4: exception::default_handler,
    DMA1_CHANNEL1: exception::default_handler,
    DMA1_CHANNEL2: exception::default_handler,
    DMA1_CHANNEL3: exception::default_handler,
    DMA1_CHANNEL4: exception::default_handler,
    DMA1_CHANNEL5: exception::default_handler,
    DMA1_CHANNEL6: exception::default_handler,
    DMA1_CHANNEL7: exception::default_handler,
    ADC: exception::default_handler,
    CAN1_TX: exception::default_handler,
    CAN1_RX0: exception::default_handler,
    CAN1_RX1: exception::default_handler,
    CAN1_SCE: exception::default_handler,
    EXTI9_5: exception::default_handler,
    TIM1_BRK_TIM9: exception::default_handler,
    TIM1_UP_TIM10: exception::default_handler,
    TIM1_TRG_COM_TIM11: exception::default_handler,
    TIM1_CC: exception::default_handler,
    TIM2: exception::default_handler,
    TIM3: exception::default_handler,
    TIM4: exception::default_handler,
    I2C1_EV: exception::default_handler,
    I2C1_ER: exception::default_handler,
    I2C2_EV: exception::default_handler,
    I2C2_ER: exception::default_handler,
    SPI1: exception::default_handler,
    SPI2: exception::default_handler,
    USART1: exception::default_handler,
    USART2: exception::default_handler,
    USART3: exception::default_handler,
    EXTI15_10: exception::default_handler,
    RTCALARM: exception::default_handler,
    USB_FS_WKUP: exception::default_handler,
    TIM8_BRK_TIM12: exception::default_handler,
    TIM8_UP_TIM13: exception::default_handler,
    TIM8_TRG_COM_TIM14: exception::default_handler,
    TIM8_CC: exception::default_handler,
    ADC3: exception::default_handler,
    FSMC: exception::default_handler,
    SDIO: exception::default_handler,
    TIM5: exception::default_handler,
    SPI3: exception::default_handler,
    UART4: exception::default_handler,
    UART5: exception::default_handler,
    TIM6: exception::default_handler,
    TIM7: exception::default_handler,
    DMA2_CHANNEL1: exception::default_handler,
    DMA2_CHANNEL2: exception::default_handler,
    DMA2_CHANNEL3: exception::default_handler,
    DMA2_CHANNEL4_5: exception::default_handler,
};
# [ doc = r" Enumeration of all the interrupts" ]
pub enum Interrupt {
    # [ doc = "0 - Window Watchdog interrupt" ]
    WWDG,
    # [ doc = "1 - PVD through EXTI line detection interrupt" ]
    PVD,
    # [ doc = "2 - Tamper interrupt" ]
    TAMPER,
    # [ doc = "3 - RTC global interrupt" ]
    RTC,
    # [ doc = "4 - Flash global interrupt" ]
    FLASH,
    # [ doc = "5 - RCC global interrupt" ]
    RCC,
    # [ doc = "6 - EXTI Line0 interrupt" ]
    EXTI0,
    # [ doc = "7 - EXTI Line1 interrupt" ]
    EXTI1,
    # [ doc = "8 - EXTI Line2 interrupt" ]
    EXTI2,
    # [ doc = "9 - EXTI Line3 interrupt" ]
    EXTI3,
    # [ doc = "10 - EXTI Line4 interrupt" ]
    EXTI4,
    # [ doc = "11 - DMA1 Channel1 global interrupt" ]
    DMA1_CHANNEL1,
    # [ doc = "12 - DMA1 Channel2 global interrupt" ]
    DMA1_CHANNEL2,
    # [ doc = "13 - DMA1 Channel3 global interrupt" ]
    DMA1_CHANNEL3,
    # [ doc = "14 - DMA1 Channel4 global interrupt" ]
    DMA1_CHANNEL4,
    # [ doc = "15 - DMA1 Channel5 global interrupt" ]
    DMA1_CHANNEL5,
    # [ doc = "16 - DMA1 Channel6 global interrupt" ]
    DMA1_CHANNEL6,
    # [ doc = "17 - DMA1 Channel7 global interrupt" ]
    DMA1_CHANNEL7,
    # [ doc = "18 - ADC2 global interrupt" ]
    ADC,
    # [ doc = "19 - CAN1 TX interrupts" ]
    CAN1_TX,
    # [ doc = "20 - CAN1 RX0 interrupts" ]
    CAN1_RX0,
    # [ doc = "21 - CAN1 RX1 interrupt" ]
    CAN1_RX1,
    # [ doc = "22 - CAN1 SCE interrupt" ]
    CAN1_SCE,
    # [ doc = "23 - EXTI Line[9:5] interrupts" ]
    EXTI9_5,
    # [ doc = "24 - TIM1 Break interrupt and TIM9 global interrupt" ]
    TIM1_BRK_TIM9,
    # [ doc = "25 - TIM1 Update interrupt and TIM10 global interrupt" ]
    TIM1_UP_TIM10,
    # [ doc = "26 - TIM1 Trigger and Commutation interrupts and TIM11 global interrupt" ]
    TIM1_TRG_COM_TIM11,
    # [ doc = "27 - TIM1 Capture Compare interrupt" ]
    TIM1_CC,
    # [ doc = "28 - TIM2 global interrupt" ]
    TIM2,
    # [ doc = "29 - TIM3 global interrupt" ]
    TIM3,
    # [ doc = "30 - TIM4 global interrupt" ]
    TIM4,
    # [ doc = "31 - I2C1 event interrupt" ]
    I2C1_EV,
    # [ doc = "32 - I2C1 error interrupt" ]
    I2C1_ER,
    # [ doc = "33 - I2C2 event interrupt" ]
    I2C2_EV,
    # [ doc = "34 - I2C2 error interrupt" ]
    I2C2_ER,
    # [ doc = "35 - SPI1 global interrupt" ]
    SPI1,
    # [ doc = "36 - SPI2 global interrupt" ]
    SPI2,
    # [ doc = "37 - USART1 global interrupt" ]
    USART1,
    # [ doc = "38 - USART2 global interrupt" ]
    USART2,
    # [ doc = "39 - USART3 global interrupt" ]
    USART3,
    # [ doc = "40 - EXTI Line[15:10] interrupts" ]
    EXTI15_10,
    # [ doc = "41 - RTC Alarms through EXTI line interrupt" ]
    RTCALARM,
    # [ doc = "42 - USB Device FS Wakeup through EXTI line interrupt" ]
    USB_FS_WKUP,
    # [ doc = "43 - TIM8 Break interrupt and TIM12 global interrupt" ]
    TIM8_BRK_TIM12,
    # [ doc = "44 - TIM8 Update interrupt and TIM13 global interrupt" ]
    TIM8_UP_TIM13,
    # [ doc = "45 - TIM8 Trigger and Commutation interrupts and TIM14 global interrupt" ]
    TIM8_TRG_COM_TIM14,
    # [ doc = "46 - TIM8 Capture Compare interrupt" ]
    TIM8_CC,
    # [ doc = "47 - ADC3 global interrupt" ]
    ADC3,
    # [ doc = "48 - FSMC global interrupt" ]
    FSMC,
    # [ doc = "49 - SDIO global interrupt" ]
    SDIO,
    # [ doc = "50 - TIM5 global interrupt" ]
    TIM5,
    # [ doc = "51 - SPI3 global interrupt" ]
    SPI3,
    # [ doc = "52 - UART4 global interrupt" ]
    UART4,
    # [ doc = "53 - UART5 global interrupt" ]
    UART5,
    # [ doc = "54 - TIM6 global interrupt" ]
    TIM6,
    # [ doc = "55 - TIM7 global interrupt" ]
    TIM7,
    # [ doc = "56 - DMA2 Channel1 global interrupt" ]
    DMA2_CHANNEL1,
    # [ doc = "57 - DMA2 Channel2 global interrupt" ]
    DMA2_CHANNEL2,
    # [ doc = "58 - DMA2 Channel3 global interrupt" ]
    DMA2_CHANNEL3,
    # [ doc = "59 - DMA2 Channel4 and DMA2 Channel5 global interrupt" ]
    DMA2_CHANNEL4_5,
}
unsafe impl Nr for Interrupt {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::WWDG => 0,
            Interrupt::PVD => 1,
            Interrupt::TAMPER => 2,
            Interrupt::RTC => 3,
            Interrupt::FLASH => 4,
            Interrupt::RCC => 5,
            Interrupt::EXTI0 => 6,
            Interrupt::EXTI1 => 7,
            Interrupt::EXTI2 => 8,
            Interrupt::EXTI3 => 9,
            Interrupt::EXTI4 => 10,
            Interrupt::DMA1_CHANNEL1 => 11,
            Interrupt::DMA1_CHANNEL2 => 12,
            Interrupt::DMA1_CHANNEL3 => 13,
            Interrupt::DMA1_CHANNEL4 => 14,
            Interrupt::DMA1_CHANNEL5 => 15,
            Interrupt::DMA1_CHANNEL6 => 16,
            Interrupt::DMA1_CHANNEL7 => 17,
            Interrupt::ADC => 18,
            Interrupt::CAN1_TX => 19,
            Interrupt::CAN1_RX0 => 20,
            Interrupt::CAN1_RX1 => 21,
            Interrupt::CAN1_SCE => 22,
            Interrupt::EXTI9_5 => 23,
            Interrupt::TIM1_BRK_TIM9 => 24,
            Interrupt::TIM1_UP_TIM10 => 25,
            Interrupt::TIM1_TRG_COM_TIM11 => 26,
            Interrupt::TIM1_CC => 27,
            Interrupt::TIM2 => 28,
            Interrupt::TIM3 => 29,
            Interrupt::TIM4 => 30,
            Interrupt::I2C1_EV => 31,
            Interrupt::I2C1_ER => 32,
            Interrupt::I2C2_EV => 33,
            Interrupt::I2C2_ER => 34,
            Interrupt::SPI1 => 35,
            Interrupt::SPI2 => 36,
            Interrupt::USART1 => 37,
            Interrupt::USART2 => 38,
            Interrupt::USART3 => 39,
            Interrupt::EXTI15_10 => 40,
            Interrupt::RTCALARM => 41,
            Interrupt::USB_FS_WKUP => 42,
            Interrupt::TIM8_BRK_TIM12 => 43,
            Interrupt::TIM8_UP_TIM13 => 44,
            Interrupt::TIM8_TRG_COM_TIM14 => 45,
            Interrupt::TIM8_CC => 46,
            Interrupt::ADC3 => 47,
            Interrupt::FSMC => 48,
            Interrupt::SDIO => 49,
            Interrupt::TIM5 => 50,
            Interrupt::SPI3 => 51,
            Interrupt::UART4 => 52,
            Interrupt::UART5 => 53,
            Interrupt::TIM6 => 54,
            Interrupt::TIM7 => 55,
            Interrupt::DMA2_CHANNEL1 => 56,
            Interrupt::DMA2_CHANNEL2 => 57,
            Interrupt::DMA2_CHANNEL3 => 58,
            Interrupt::DMA2_CHANNEL4_5 => 59,
        }
    }
}
