# ! [ doc = "Peripheral access API for STM32F103XX microcontrollers (generated using svd2rust v0.9.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.9.1/svd2rust/#peripheral-api" ]

# ! [ deny ( missing_docs ) ]
# ! [ deny ( warnings ) ]
# ! [ allow ( non_camel_case_types ) ]
# ! [ feature ( const_fn ) ]
# ! [ feature ( optin_builtin_traits ) ]
# ! [ no_std ]

extern crate cortex_m ;
extern crate vcell ;

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
pub mod tim9;
pub mod tim10;
pub mod tim6;
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

