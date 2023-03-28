///Register `GTZC1_TZIC_IER1` reader
pub struct R(crate::R<GTZC1_TZIC_IER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTZC1_TZIC_IER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTZC1_TZIC_IER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTZC1_TZIC_IER1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GTZC1_TZIC_IER1` writer
pub struct W(crate::W<GTZC1_TZIC_IER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTZC1_TZIC_IER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GTZC1_TZIC_IER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTZC1_TZIC_IER1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM2IE` reader - illegal access interrupt enable for TIM2
pub type TIM2IE_R = crate::BitReader<bool>;
///Field `TIM2IE` writer - illegal access interrupt enable for TIM2
pub type TIM2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `TIM3IE` reader - illegal access interrupt enable for TIM3
pub type TIM3IE_R = crate::BitReader<bool>;
///Field `TIM3IE` writer - illegal access interrupt enable for TIM3
pub type TIM3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `TIM4IE` reader - illegal access interrupt enable for TIM4
pub type TIM4IE_R = crate::BitReader<bool>;
///Field `TIM4IE` writer - illegal access interrupt enable for TIM4
pub type TIM4IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `TIM5IE` reader - illegal access interrupt enable for TIM5
pub type TIM5IE_R = crate::BitReader<bool>;
///Field `TIM5IE` writer - illegal access interrupt enable for TIM5
pub type TIM5IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `TIM6IE` reader - illegal access interrupt enable for TIM6
pub type TIM6IE_R = crate::BitReader<bool>;
///Field `TIM6IE` writer - illegal access interrupt enable for TIM6
pub type TIM6IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `TIM7IE` reader - illegal access interrupt enable for TIM7
pub type TIM7IE_R = crate::BitReader<bool>;
///Field `TIM7IE` writer - illegal access interrupt enable for TIM7
pub type TIM7IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `TIM12IE` reader - illegal access interrupt enable for TIM12
pub type TIM12IE_R = crate::BitReader<bool>;
///Field `TIM12IE` writer - illegal access interrupt enable for TIM12
pub type TIM12IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `TIM13IE` reader - illegal access interrupt enable for TIM13
pub type TIM13IE_R = crate::BitReader<bool>;
///Field `TIM13IE` writer - illegal access interrupt enable for TIM13
pub type TIM13IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `TIM14IE` reader - illegal access interrupt enable for TIM14
pub type TIM14IE_R = crate::BitReader<bool>;
///Field `TIM14IE` writer - illegal access interrupt enable for TIM14
pub type TIM14IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `WWDGIE` reader - illegal access interrupt enable for WWDG
pub type WWDGIE_R = crate::BitReader<bool>;
///Field `WWDGIE` writer - illegal access interrupt enable for WWDG
pub type WWDGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `IWDGIE` reader - illegal access interrupt enable for IWDG
pub type IWDGIE_R = crate::BitReader<bool>;
///Field `IWDGIE` writer - illegal access interrupt enable for IWDG
pub type IWDGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `SPI2IE` reader - illegal access interrupt enable for SPI2
pub type SPI2IE_R = crate::BitReader<bool>;
///Field `SPI2IE` writer - illegal access interrupt enable for SPI2
pub type SPI2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `SPI3IE` reader - illegal access interrupt enable for SPI3
pub type SPI3IE_R = crate::BitReader<bool>;
///Field `SPI3IE` writer - illegal access interrupt enable for SPI3
pub type SPI3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `USART2IE` reader - illegal access interrupt enable for USART2
pub type USART2IE_R = crate::BitReader<bool>;
///Field `USART2IE` writer - illegal access interrupt enable for USART2
pub type USART2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `USART3IE` reader - illegal access interrupt enable for USART3
pub type USART3IE_R = crate::BitReader<bool>;
///Field `USART3IE` writer - illegal access interrupt enable for USART3
pub type USART3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `UART4IE` reader - illegal access interrupt enable for UART4
pub type UART4IE_R = crate::BitReader<bool>;
///Field `UART4IE` writer - illegal access interrupt enable for UART4
pub type UART4IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `UART5IE` reader - illegal access interrupt enable for UART5
pub type UART5IE_R = crate::BitReader<bool>;
///Field `UART5IE` writer - illegal access interrupt enable for UART5
pub type UART5IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `I2C1IE` reader - illegal access interrupt enable for I2C1
pub type I2C1IE_R = crate::BitReader<bool>;
///Field `I2C1IE` writer - illegal access interrupt enable for I2C1
pub type I2C1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `I2C2IE` reader - illegal access interrupt enable for I2C2
pub type I2C2IE_R = crate::BitReader<bool>;
///Field `I2C2IE` writer - illegal access interrupt enable for I2C2
pub type I2C2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `I3C1IE` reader - illegal access interrupt enable for I3C1
pub type I3C1IE_R = crate::BitReader<bool>;
///Field `I3C1IE` writer - illegal access interrupt enable for I3C1
pub type I3C1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `CRSIE` reader - illegal access interrupt enable for CRS
pub type CRSIE_R = crate::BitReader<bool>;
///Field `CRSIE` writer - illegal access interrupt enable for CRS
pub type CRSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `USART6IE` reader - illegal access interrupt enable for USART6
pub type USART6IE_R = crate::BitReader<bool>;
///Field `USART6IE` writer - illegal access interrupt enable for USART6
pub type USART6IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `USART10IE` reader - illegal access interrupt enable for USART10
pub type USART10IE_R = crate::BitReader<bool>;
///Field `USART10IE` writer - illegal access interrupt enable for USART10
pub type USART10IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `USART11IE` reader - illegal access interrupt enable for USART11
pub type USART11IE_R = crate::BitReader<bool>;
///Field `USART11IE` writer - illegal access interrupt enable for USART11
pub type USART11IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `HDMICECIE` reader - illegal access interrupt enable for HDMICEC
pub type HDMICECIE_R = crate::BitReader<bool>;
///Field `HDMICECIE` writer - illegal access interrupt enable for HDMICEC
pub type HDMICECIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `DAC1IE` reader - illegal access interrupt enable for DAC1
pub type DAC1IE_R = crate::BitReader<bool>;
///Field `DAC1IE` writer - illegal access interrupt enable for DAC1
pub type DAC1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `UART7IE` reader - illegal access interrupt enable for UART7
pub type UART7IE_R = crate::BitReader<bool>;
///Field `UART7IE` writer - illegal access interrupt enable for UART7
pub type UART7IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `UART8IE` reader - illegal access interrupt enable for UART8
pub type UART8IE_R = crate::BitReader<bool>;
///Field `UART8IE` writer - illegal access interrupt enable for UART8
pub type UART8IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `UART9IE` reader - illegal access interrupt enable for UART9
pub type UART9IE_R = crate::BitReader<bool>;
///Field `UART9IE` writer - illegal access interrupt enable for UART9
pub type UART9IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `UART12IE` reader - illegal access interrupt enable for UART12
pub type UART12IE_R = crate::BitReader<bool>;
///Field `UART12IE` writer - illegal access interrupt enable for UART12
pub type UART12IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `DTSIE` reader - illegal access interrupt enable for DTS
pub type DTSIE_R = crate::BitReader<bool>;
///Field `DTSIE` writer - illegal access interrupt enable for DTS
pub type DTSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
///Field `LPTIM2IE` reader - illegal access interrupt enable for LPTIM2
pub type LPTIM2IE_R = crate::BitReader<bool>;
///Field `LPTIM2IE` writer - illegal access interrupt enable for LPTIM2
pub type LPTIM2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER1_SPEC, bool, O>;
impl R {
    ///Bit 0 - illegal access interrupt enable for TIM2
    #[inline(always)]
    pub fn tim2ie(&self) -> TIM2IE_R {
        TIM2IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access interrupt enable for TIM3
    #[inline(always)]
    pub fn tim3ie(&self) -> TIM3IE_R {
        TIM3IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access interrupt enable for TIM4
    #[inline(always)]
    pub fn tim4ie(&self) -> TIM4IE_R {
        TIM4IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access interrupt enable for TIM5
    #[inline(always)]
    pub fn tim5ie(&self) -> TIM5IE_R {
        TIM5IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access interrupt enable for TIM6
    #[inline(always)]
    pub fn tim6ie(&self) -> TIM6IE_R {
        TIM6IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access interrupt enable for TIM7
    #[inline(always)]
    pub fn tim7ie(&self) -> TIM7IE_R {
        TIM7IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access interrupt enable for TIM12
    #[inline(always)]
    pub fn tim12ie(&self) -> TIM12IE_R {
        TIM12IE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access interrupt enable for TIM13
    #[inline(always)]
    pub fn tim13ie(&self) -> TIM13IE_R {
        TIM13IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access interrupt enable for TIM14
    #[inline(always)]
    pub fn tim14ie(&self) -> TIM14IE_R {
        TIM14IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access interrupt enable for WWDG
    #[inline(always)]
    pub fn wwdgie(&self) -> WWDGIE_R {
        WWDGIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access interrupt enable for IWDG
    #[inline(always)]
    pub fn iwdgie(&self) -> IWDGIE_R {
        IWDGIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access interrupt enable for SPI2
    #[inline(always)]
    pub fn spi2ie(&self) -> SPI2IE_R {
        SPI2IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access interrupt enable for SPI3
    #[inline(always)]
    pub fn spi3ie(&self) -> SPI3IE_R {
        SPI3IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access interrupt enable for USART2
    #[inline(always)]
    pub fn usart2ie(&self) -> USART2IE_R {
        USART2IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access interrupt enable for USART3
    #[inline(always)]
    pub fn usart3ie(&self) -> USART3IE_R {
        USART3IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access interrupt enable for UART4
    #[inline(always)]
    pub fn uart4ie(&self) -> UART4IE_R {
        UART4IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access interrupt enable for UART5
    #[inline(always)]
    pub fn uart5ie(&self) -> UART5IE_R {
        UART5IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access interrupt enable for I2C1
    #[inline(always)]
    pub fn i2c1ie(&self) -> I2C1IE_R {
        I2C1IE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access interrupt enable for I2C2
    #[inline(always)]
    pub fn i2c2ie(&self) -> I2C2IE_R {
        I2C2IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access interrupt enable for I3C1
    #[inline(always)]
    pub fn i3c1ie(&self) -> I3C1IE_R {
        I3C1IE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - illegal access interrupt enable for CRS
    #[inline(always)]
    pub fn crsie(&self) -> CRSIE_R {
        CRSIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - illegal access interrupt enable for USART6
    #[inline(always)]
    pub fn usart6ie(&self) -> USART6IE_R {
        USART6IE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - illegal access interrupt enable for USART10
    #[inline(always)]
    pub fn usart10ie(&self) -> USART10IE_R {
        USART10IE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - illegal access interrupt enable for USART11
    #[inline(always)]
    pub fn usart11ie(&self) -> USART11IE_R {
        USART11IE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - illegal access interrupt enable for HDMICEC
    #[inline(always)]
    pub fn hdmicecie(&self) -> HDMICECIE_R {
        HDMICECIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access interrupt enable for DAC1
    #[inline(always)]
    pub fn dac1ie(&self) -> DAC1IE_R {
        DAC1IE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - illegal access interrupt enable for UART7
    #[inline(always)]
    pub fn uart7ie(&self) -> UART7IE_R {
        UART7IE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - illegal access interrupt enable for UART8
    #[inline(always)]
    pub fn uart8ie(&self) -> UART8IE_R {
        UART8IE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - illegal access interrupt enable for UART9
    #[inline(always)]
    pub fn uart9ie(&self) -> UART9IE_R {
        UART9IE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - illegal access interrupt enable for UART12
    #[inline(always)]
    pub fn uart12ie(&self) -> UART12IE_R {
        UART12IE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - illegal access interrupt enable for DTS
    #[inline(always)]
    pub fn dtsie(&self) -> DTSIE_R {
        DTSIE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - illegal access interrupt enable for LPTIM2
    #[inline(always)]
    pub fn lptim2ie(&self) -> LPTIM2IE_R {
        LPTIM2IE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - illegal access interrupt enable for TIM2
    #[inline(always)]
    #[must_use]
    pub fn tim2ie(&mut self) -> TIM2IE_W<0> {
        TIM2IE_W::new(self)
    }
    ///Bit 1 - illegal access interrupt enable for TIM3
    #[inline(always)]
    #[must_use]
    pub fn tim3ie(&mut self) -> TIM3IE_W<1> {
        TIM3IE_W::new(self)
    }
    ///Bit 2 - illegal access interrupt enable for TIM4
    #[inline(always)]
    #[must_use]
    pub fn tim4ie(&mut self) -> TIM4IE_W<2> {
        TIM4IE_W::new(self)
    }
    ///Bit 3 - illegal access interrupt enable for TIM5
    #[inline(always)]
    #[must_use]
    pub fn tim5ie(&mut self) -> TIM5IE_W<3> {
        TIM5IE_W::new(self)
    }
    ///Bit 4 - illegal access interrupt enable for TIM6
    #[inline(always)]
    #[must_use]
    pub fn tim6ie(&mut self) -> TIM6IE_W<4> {
        TIM6IE_W::new(self)
    }
    ///Bit 5 - illegal access interrupt enable for TIM7
    #[inline(always)]
    #[must_use]
    pub fn tim7ie(&mut self) -> TIM7IE_W<5> {
        TIM7IE_W::new(self)
    }
    ///Bit 6 - illegal access interrupt enable for TIM12
    #[inline(always)]
    #[must_use]
    pub fn tim12ie(&mut self) -> TIM12IE_W<6> {
        TIM12IE_W::new(self)
    }
    ///Bit 7 - illegal access interrupt enable for TIM13
    #[inline(always)]
    #[must_use]
    pub fn tim13ie(&mut self) -> TIM13IE_W<7> {
        TIM13IE_W::new(self)
    }
    ///Bit 8 - illegal access interrupt enable for TIM14
    #[inline(always)]
    #[must_use]
    pub fn tim14ie(&mut self) -> TIM14IE_W<8> {
        TIM14IE_W::new(self)
    }
    ///Bit 9 - illegal access interrupt enable for WWDG
    #[inline(always)]
    #[must_use]
    pub fn wwdgie(&mut self) -> WWDGIE_W<9> {
        WWDGIE_W::new(self)
    }
    ///Bit 10 - illegal access interrupt enable for IWDG
    #[inline(always)]
    #[must_use]
    pub fn iwdgie(&mut self) -> IWDGIE_W<10> {
        IWDGIE_W::new(self)
    }
    ///Bit 11 - illegal access interrupt enable for SPI2
    #[inline(always)]
    #[must_use]
    pub fn spi2ie(&mut self) -> SPI2IE_W<11> {
        SPI2IE_W::new(self)
    }
    ///Bit 12 - illegal access interrupt enable for SPI3
    #[inline(always)]
    #[must_use]
    pub fn spi3ie(&mut self) -> SPI3IE_W<12> {
        SPI3IE_W::new(self)
    }
    ///Bit 13 - illegal access interrupt enable for USART2
    #[inline(always)]
    #[must_use]
    pub fn usart2ie(&mut self) -> USART2IE_W<13> {
        USART2IE_W::new(self)
    }
    ///Bit 14 - illegal access interrupt enable for USART3
    #[inline(always)]
    #[must_use]
    pub fn usart3ie(&mut self) -> USART3IE_W<14> {
        USART3IE_W::new(self)
    }
    ///Bit 15 - illegal access interrupt enable for UART4
    #[inline(always)]
    #[must_use]
    pub fn uart4ie(&mut self) -> UART4IE_W<15> {
        UART4IE_W::new(self)
    }
    ///Bit 16 - illegal access interrupt enable for UART5
    #[inline(always)]
    #[must_use]
    pub fn uart5ie(&mut self) -> UART5IE_W<16> {
        UART5IE_W::new(self)
    }
    ///Bit 17 - illegal access interrupt enable for I2C1
    #[inline(always)]
    #[must_use]
    pub fn i2c1ie(&mut self) -> I2C1IE_W<17> {
        I2C1IE_W::new(self)
    }
    ///Bit 18 - illegal access interrupt enable for I2C2
    #[inline(always)]
    #[must_use]
    pub fn i2c2ie(&mut self) -> I2C2IE_W<18> {
        I2C2IE_W::new(self)
    }
    ///Bit 19 - illegal access interrupt enable for I3C1
    #[inline(always)]
    #[must_use]
    pub fn i3c1ie(&mut self) -> I3C1IE_W<19> {
        I3C1IE_W::new(self)
    }
    ///Bit 20 - illegal access interrupt enable for CRS
    #[inline(always)]
    #[must_use]
    pub fn crsie(&mut self) -> CRSIE_W<20> {
        CRSIE_W::new(self)
    }
    ///Bit 21 - illegal access interrupt enable for USART6
    #[inline(always)]
    #[must_use]
    pub fn usart6ie(&mut self) -> USART6IE_W<21> {
        USART6IE_W::new(self)
    }
    ///Bit 22 - illegal access interrupt enable for USART10
    #[inline(always)]
    #[must_use]
    pub fn usart10ie(&mut self) -> USART10IE_W<22> {
        USART10IE_W::new(self)
    }
    ///Bit 23 - illegal access interrupt enable for USART11
    #[inline(always)]
    #[must_use]
    pub fn usart11ie(&mut self) -> USART11IE_W<23> {
        USART11IE_W::new(self)
    }
    ///Bit 24 - illegal access interrupt enable for HDMICEC
    #[inline(always)]
    #[must_use]
    pub fn hdmicecie(&mut self) -> HDMICECIE_W<24> {
        HDMICECIE_W::new(self)
    }
    ///Bit 25 - illegal access interrupt enable for DAC1
    #[inline(always)]
    #[must_use]
    pub fn dac1ie(&mut self) -> DAC1IE_W<25> {
        DAC1IE_W::new(self)
    }
    ///Bit 26 - illegal access interrupt enable for UART7
    #[inline(always)]
    #[must_use]
    pub fn uart7ie(&mut self) -> UART7IE_W<26> {
        UART7IE_W::new(self)
    }
    ///Bit 27 - illegal access interrupt enable for UART8
    #[inline(always)]
    #[must_use]
    pub fn uart8ie(&mut self) -> UART8IE_W<27> {
        UART8IE_W::new(self)
    }
    ///Bit 28 - illegal access interrupt enable for UART9
    #[inline(always)]
    #[must_use]
    pub fn uart9ie(&mut self) -> UART9IE_W<28> {
        UART9IE_W::new(self)
    }
    ///Bit 29 - illegal access interrupt enable for UART12
    #[inline(always)]
    #[must_use]
    pub fn uart12ie(&mut self) -> UART12IE_W<29> {
        UART12IE_W::new(self)
    }
    ///Bit 30 - illegal access interrupt enable for DTS
    #[inline(always)]
    #[must_use]
    pub fn dtsie(&mut self) -> DTSIE_W<30> {
        DTSIE_W::new(self)
    }
    ///Bit 31 - illegal access interrupt enable for LPTIM2
    #[inline(always)]
    #[must_use]
    pub fn lptim2ie(&mut self) -> LPTIM2IE_W<31> {
        LPTIM2IE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZIC interrupt enable register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gtzc1_tzic_ier1](index.html) module
pub struct GTZC1_TZIC_IER1_SPEC;
impl crate::RegisterSpec for GTZC1_TZIC_IER1_SPEC {
    type Ux = u32;
}
///`read()` method returns [gtzc1_tzic_ier1::R](R) reader structure
impl crate::Readable for GTZC1_TZIC_IER1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gtzc1_tzic_ier1::W](W) writer structure
impl crate::Writable for GTZC1_TZIC_IER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GTZC1_TZIC_IER1 to value 0
impl crate::Resettable for GTZC1_TZIC_IER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
