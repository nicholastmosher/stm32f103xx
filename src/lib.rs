# ! [ doc = "Peripheral access API for STM32F103XX microcontrollers (generated using svd2rust v0.9.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.9.1/svd2rust/#peripheral-api" ]

# ! [ deny ( missing_docs ) ]
# ! [ deny ( warnings ) ]
# ! [ allow ( non_camel_case_types ) ]
# ! [ feature ( const_fn ) ]
# ! [ feature ( optin_builtin_traits ) ]
# ! [ no_std ]

extern crate cortex_m;
extern crate vcell;

pub use cortex_m::peripheral::Cpuid as CPUID;
pub use cortex_m::peripheral::CPUID;
pub use cortex_m::peripheral::Dcb as DCB;
pub use cortex_m::peripheral::DCB;
pub use cortex_m::peripheral::Dwt as DWT;
pub use cortex_m::peripheral::DWT;
pub use cortex_m::peripheral::Fpb as FPB;
pub use cortex_m::peripheral::FPB;
pub use cortex_m::peripheral::Fpu as FPU;
pub use cortex_m::peripheral::FPU;
pub use cortex_m::peripheral::Itm as ITM;
pub use cortex_m::peripheral::ITM;
pub use cortex_m::peripheral::Mpu as MPU;
pub use cortex_m::peripheral::MPU;
pub use cortex_m::peripheral::Nvic as NVIC;
pub use cortex_m::peripheral::NVIC;
pub use cortex_m::peripheral::Scb as SCB;
pub use cortex_m::peripheral::SCB;
pub use cortex_m::peripheral::Syst as SYST;
pub use cortex_m::peripheral::SYST;
pub use cortex_m::peripheral::Tpiu as TPIU;
pub use cortex_m::peripheral::TPIU;

pub mod interrupt;
pub mod fsmc;
pub mod pwr;
pub mod rcc;
pub mod gpioa;
pub mod afio;
pub mod exti;
pub mod dma1;
pub mod sdio;
pub mod rtc;
pub mod bkp;
pub mod iwdg;
pub mod wwdg;
pub mod tim1;
pub mod tim2;
pub mod tim6;
pub mod tim9;
pub mod tim10;
pub mod i2c1;
pub mod spi1;
pub mod usart1;
pub mod adc1;
pub mod adc2;
pub mod can;
pub mod dac;
pub mod dbg;
pub mod uart4;
pub mod uart5;
pub mod crc;
pub mod flash;
pub mod usb;

pub use fsmc::FSMC;
pub use pwr::PWR;
pub use rcc::RCC;
pub use gpioa::{GPIOA, GPIOB, GPIOC, GPIOD, GPIOE, GPIOF, GPIOG};
pub use afio::AFIO;
pub use exti::EXTI;
pub use dma1::{DMA1, DMA2};
pub use sdio::SDIO;
pub use rtc::RTC;
pub use bkp::BKP;
pub use iwdg::IWDG;
pub use wwdg::WWDG;
pub use tim1::{TIM1, TIM8};
pub use tim2::{TIM2, TIM3, TIM4, TIM5};
pub use tim6::{TIM6, TIM7};
pub use tim9::{TIM9, TIM12};
pub use tim10::{TIM10, TIM11, TIM13, TIM14};
pub use i2c1::{I2C1, I2C2};
pub use spi1::{SPI1, SPI2, SPI3};
pub use usart1::{USART1, USART2, USART3};
pub use adc1::ADC1;
pub use adc2::{ADC2, ADC3};
pub use can::CAN;
pub use dac::DAC;
pub use dbg::DBG;
pub use uart4::UART4;
pub use uart5::UART5;
pub use crc::CRC;
pub use flash::FLASH;
pub use usb::USB;
