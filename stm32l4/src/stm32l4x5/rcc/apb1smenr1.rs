///Register `APB1SMENR1` reader
pub struct R(crate::R<APB1SMENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1SMENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1SMENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1SMENR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APB1SMENR1` writer
pub struct W(crate::W<APB1SMENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1SMENR1_SPEC>;
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
impl From<crate::W<APB1SMENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1SMENR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM2SMEN` reader - TIM2 timer clocks enable during Sleep and Stop modes
pub type TIM2SMEN_R = crate::BitReader<bool>;
///Field `TIM2SMEN` writer - TIM2 timer clocks enable during Sleep and Stop modes
pub type TIM2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
///Field `TIM3SMEN` reader - TIM3 timer clocks enable during Sleep and Stop modes
pub type TIM3SMEN_R = crate::BitReader<bool>;
///Field `TIM3SMEN` writer - TIM3 timer clocks enable during Sleep and Stop modes
pub type TIM3SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
///Field `TIM4SMEN` reader - TIM4 timer clocks enable during Sleep and Stop modes
pub type TIM4SMEN_R = crate::BitReader<bool>;
///Field `TIM4SMEN` writer - TIM4 timer clocks enable during Sleep and Stop modes
pub type TIM4SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
///Field `TIM5SMEN` reader - TIM5 timer clocks enable during Sleep and Stop modes
pub type TIM5SMEN_R = crate::BitReader<bool>;
///Field `TIM5SMEN` writer - TIM5 timer clocks enable during Sleep and Stop modes
pub type TIM5SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
///Field `TIM6SMEN` reader - TIM6 timer clocks enable during Sleep and Stop modes
pub type TIM6SMEN_R = crate::BitReader<bool>;
///Field `TIM6SMEN` writer - TIM6 timer clocks enable during Sleep and Stop modes
pub type TIM6SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
///Field `TIM7SMEN` reader - TIM7 timer clocks enable during Sleep and Stop modes
pub type TIM7SMEN_R = crate::BitReader<bool>;
///Field `TIM7SMEN` writer - TIM7 timer clocks enable during Sleep and Stop modes
pub type TIM7SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
///Field `LCDSMEN` reader - LCD clocks enable during Sleep and Stop modes
pub type LCDSMEN_R = crate::BitReader<bool>;
///Field `LCDSMEN` writer - LCD clocks enable during Sleep and Stop modes
pub type LCDSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
///Field `RTCAPBSMEN` reader - RTC APB clock enable during Sleep and Stop modes
pub type RTCAPBSMEN_R = crate::BitReader<bool>;
///Field `RTCAPBSMEN` writer - RTC APB clock enable during Sleep and Stop modes
pub type RTCAPBSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
///Field `WWDGSMEN` reader - Window watchdog clocks enable during Sleep and Stop modes
pub type WWDGSMEN_R = crate::BitReader<bool>;
///Field `WWDGSMEN` writer - Window watchdog clocks enable during Sleep and Stop modes
pub type WWDGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
///Field `SPI2SMEN` reader - SPI2 clocks enable during Sleep and Stop modes
pub type SPI2SMEN_R = crate::BitReader<bool>;
///Field `SPI2SMEN` writer - SPI2 clocks enable during Sleep and Stop modes
pub type SPI2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
///Field `SP3SMEN` reader - SPI3 clocks enable during Sleep and Stop modes
pub type SP3SMEN_R = crate::BitReader<bool>;
///Field `SP3SMEN` writer - SPI3 clocks enable during Sleep and Stop modes
pub type SP3SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
///Field `USART2SMEN` reader - USART2 clocks enable during Sleep and Stop modes
pub type USART2SMEN_R = crate::BitReader<bool>;
///Field `USART2SMEN` writer - USART2 clocks enable during Sleep and Stop modes
pub type USART2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
///Field `USART3SMEN` reader - USART3 clocks enable during Sleep and Stop modes
pub type USART3SMEN_R = crate::BitReader<bool>;
///Field `USART3SMEN` writer - USART3 clocks enable during Sleep and Stop modes
pub type USART3SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
///Field `UART4SMEN` reader - UART4 clocks enable during Sleep and Stop modes
pub type UART4SMEN_R = crate::BitReader<bool>;
///Field `UART4SMEN` writer - UART4 clocks enable during Sleep and Stop modes
pub type UART4SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
///Field `UART5SMEN` reader - UART5 clocks enable during Sleep and Stop modes
pub type UART5SMEN_R = crate::BitReader<bool>;
///Field `UART5SMEN` writer - UART5 clocks enable during Sleep and Stop modes
pub type UART5SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
///Field `I2C1SMEN` reader - I2C1 clocks enable during Sleep and Stop modes
pub type I2C1SMEN_R = crate::BitReader<bool>;
///Field `I2C1SMEN` writer - I2C1 clocks enable during Sleep and Stop modes
pub type I2C1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
///Field `I2C2SMEN` reader - I2C2 clocks enable during Sleep and Stop modes
pub type I2C2SMEN_R = crate::BitReader<bool>;
///Field `I2C2SMEN` writer - I2C2 clocks enable during Sleep and Stop modes
pub type I2C2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
///Field `I2C3SMEN` reader - I2C3 clocks enable during Sleep and Stop modes
pub type I2C3SMEN_R = crate::BitReader<bool>;
///Field `I2C3SMEN` writer - I2C3 clocks enable during Sleep and Stop modes
pub type I2C3SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
///Field `CAN1SMEN` reader - CAN1 clocks enable during Sleep and Stop modes
pub type CAN1SMEN_R = crate::BitReader<bool>;
///Field `CAN1SMEN` writer - CAN1 clocks enable during Sleep and Stop modes
pub type CAN1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
///Field `PWRSMEN` reader - Power interface clocks enable during Sleep and Stop modes
pub type PWRSMEN_R = crate::BitReader<bool>;
///Field `PWRSMEN` writer - Power interface clocks enable during Sleep and Stop modes
pub type PWRSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
///Field `DAC1SMEN` reader - DAC1 interface clocks enable during Sleep and Stop modes
pub type DAC1SMEN_R = crate::BitReader<bool>;
///Field `DAC1SMEN` writer - DAC1 interface clocks enable during Sleep and Stop modes
pub type DAC1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
///Field `OPAMPSMEN` reader - OPAMP interface clocks enable during Sleep and Stop modes
pub type OPAMPSMEN_R = crate::BitReader<bool>;
///Field `OPAMPSMEN` writer - OPAMP interface clocks enable during Sleep and Stop modes
pub type OPAMPSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
///Field `LPTIM1SMEN` reader - Low power timer 1 clocks enable during Sleep and Stop modes
pub type LPTIM1SMEN_R = crate::BitReader<bool>;
///Field `LPTIM1SMEN` writer - Low power timer 1 clocks enable during Sleep and Stop modes
pub type LPTIM1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - TIM2 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim3smen(&self) -> TIM3SMEN_R {
        TIM3SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM4 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim4smen(&self) -> TIM4SMEN_R {
        TIM4SMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim5smen(&self) -> TIM5SMEN_R {
        TIM5SMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim6smen(&self) -> TIM6SMEN_R {
        TIM6SMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim7smen(&self) -> TIM7SMEN_R {
        TIM7SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - LCD clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn lcdsmen(&self) -> LCDSMEN_R {
        LCDSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RTC APB clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Window watchdog clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI3 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sp3smen(&self) -> SP3SMEN_R {
        SP3SMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - USART2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn usart3smen(&self) -> USART3SMEN_R {
        USART3SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - UART4 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn uart4smen(&self) -> UART4SMEN_R {
        UART4SMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - UART5 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn uart5smen(&self) -> UART5SMEN_R {
        UART5SMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2C2SMEN_R {
        I2C2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I2C3 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - CAN1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn can1smen(&self) -> CAN1SMEN_R {
        CAN1SMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 28 - Power interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DAC1 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dac1smen(&self) -> DAC1SMEN_R {
        DAC1SMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - OPAMP interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn opampsmen(&self) -> OPAMPSMEN_R {
        OPAMPSMEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low power timer 1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn lptim1smen(&self) -> LPTIM1SMEN_R {
        LPTIM1SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TIM2 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W<0> {
        TIM2SMEN_W::new(self)
    }
    ///Bit 1 - TIM3 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn tim3smen(&mut self) -> TIM3SMEN_W<1> {
        TIM3SMEN_W::new(self)
    }
    ///Bit 2 - TIM4 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn tim4smen(&mut self) -> TIM4SMEN_W<2> {
        TIM4SMEN_W::new(self)
    }
    ///Bit 3 - TIM5 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn tim5smen(&mut self) -> TIM5SMEN_W<3> {
        TIM5SMEN_W::new(self)
    }
    ///Bit 4 - TIM6 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn tim6smen(&mut self) -> TIM6SMEN_W<4> {
        TIM6SMEN_W::new(self)
    }
    ///Bit 5 - TIM7 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn tim7smen(&mut self) -> TIM7SMEN_W<5> {
        TIM7SMEN_W::new(self)
    }
    ///Bit 9 - LCD clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn lcdsmen(&mut self) -> LCDSMEN_W<9> {
        LCDSMEN_W::new(self)
    }
    ///Bit 10 - RTC APB clock enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W<10> {
        RTCAPBSMEN_W::new(self)
    }
    ///Bit 11 - Window watchdog clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<11> {
        WWDGSMEN_W::new(self)
    }
    ///Bit 14 - SPI2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W<14> {
        SPI2SMEN_W::new(self)
    }
    ///Bit 15 - SPI3 clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn sp3smen(&mut self) -> SP3SMEN_W<15> {
        SP3SMEN_W::new(self)
    }
    ///Bit 17 - USART2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn usart2smen(&mut self) -> USART2SMEN_W<17> {
        USART2SMEN_W::new(self)
    }
    ///Bit 18 - USART3 clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn usart3smen(&mut self) -> USART3SMEN_W<18> {
        USART3SMEN_W::new(self)
    }
    ///Bit 19 - UART4 clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn uart4smen(&mut self) -> UART4SMEN_W<19> {
        UART4SMEN_W::new(self)
    }
    ///Bit 20 - UART5 clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn uart5smen(&mut self) -> UART5SMEN_W<20> {
        UART5SMEN_W::new(self)
    }
    ///Bit 21 - I2C1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<21> {
        I2C1SMEN_W::new(self)
    }
    ///Bit 22 - I2C2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W<22> {
        I2C2SMEN_W::new(self)
    }
    ///Bit 23 - I2C3 clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W<23> {
        I2C3SMEN_W::new(self)
    }
    ///Bit 25 - CAN1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn can1smen(&mut self) -> CAN1SMEN_W<25> {
        CAN1SMEN_W::new(self)
    }
    ///Bit 28 - Power interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W<28> {
        PWRSMEN_W::new(self)
    }
    ///Bit 29 - DAC1 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn dac1smen(&mut self) -> DAC1SMEN_W<29> {
        DAC1SMEN_W::new(self)
    }
    ///Bit 30 - OPAMP interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn opampsmen(&mut self) -> OPAMPSMEN_W<30> {
        OPAMPSMEN_W::new(self)
    }
    ///Bit 31 - Low power timer 1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn lptim1smen(&mut self) -> LPTIM1SMEN_W<31> {
        LPTIM1SMEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///APB1SMENR1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1smenr1](index.html) module
pub struct APB1SMENR1_SPEC;
impl crate::RegisterSpec for APB1SMENR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [apb1smenr1::R](R) reader structure
impl crate::Readable for APB1SMENR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apb1smenr1::W](W) writer structure
impl crate::Writable for APB1SMENR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets APB1SMENR1 to value 0xf2fe_ca3f
impl crate::Resettable for APB1SMENR1_SPEC {
    const RESET_VALUE: Self::Ux = 0xf2fe_ca3f;
}
