///Register `TZSC_SECCFGR1` reader
pub struct R(crate::R<TZSC_SECCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZSC_SECCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZSC_SECCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZSC_SECCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TZSC_SECCFGR1` writer
pub struct W(crate::W<TZSC_SECCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZSC_SECCFGR1_SPEC>;
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
impl From<crate::W<TZSC_SECCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZSC_SECCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM2SEC` reader - secure access mode for TIM2
pub type TIM2SEC_R = crate::BitReader<bool>;
///Field `TIM2SEC` writer - secure access mode for TIM2
pub type TIM2SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `TIM3SEC` reader - secure access mode for TIM3
pub type TIM3SEC_R = crate::BitReader<bool>;
///Field `TIM3SEC` writer - secure access mode for TIM3
pub type TIM3SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `TIM4SEC` reader - secure access mode for TIM4
pub type TIM4SEC_R = crate::BitReader<bool>;
///Field `TIM4SEC` writer - secure access mode for TIM4
pub type TIM4SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `TIM5SEC` reader - secure access mode for TIM5
pub type TIM5SEC_R = crate::BitReader<bool>;
///Field `TIM5SEC` writer - secure access mode for TIM5
pub type TIM5SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `TIM6SEC` reader - secure access mode for TIM6
pub type TIM6SEC_R = crate::BitReader<bool>;
///Field `TIM6SEC` writer - secure access mode for TIM6
pub type TIM6SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `TIM7SEC` reader - secure access mode for TIM7
pub type TIM7SEC_R = crate::BitReader<bool>;
///Field `TIM7SEC` writer - secure access mode for TIM7
pub type TIM7SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `WWDGSEC` reader - secure access mode for WWDG
pub type WWDGSEC_R = crate::BitReader<bool>;
///Field `WWDGSEC` writer - secure access mode for WWDG
pub type WWDGSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `IWDGSEC` reader - secure access mode for IWDG
pub type IWDGSEC_R = crate::BitReader<bool>;
///Field `IWDGSEC` writer - secure access mode for IWDG
pub type IWDGSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `SPI2SEC` reader - secure access mode for SPI2
pub type SPI2SEC_R = crate::BitReader<bool>;
///Field `SPI2SEC` writer - secure access mode for SPI2
pub type SPI2SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `USART2SEC` reader - secure access mode for USART2
pub type USART2SEC_R = crate::BitReader<bool>;
///Field `USART2SEC` writer - secure access mode for USART2
pub type USART2SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `USART3SEC` reader - secure access mode for USART3
pub type USART3SEC_R = crate::BitReader<bool>;
///Field `USART3SEC` writer - secure access mode for USART3
pub type USART3SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `UART4SEC` reader - secure access mode for UART4
pub type UART4SEC_R = crate::BitReader<bool>;
///Field `UART4SEC` writer - secure access mode for UART4
pub type UART4SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `UART5SEC` reader - secure access mode for UART5
pub type UART5SEC_R = crate::BitReader<bool>;
///Field `UART5SEC` writer - secure access mode for UART5
pub type UART5SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `I2C1SEC` reader - secure access mode for I2C1
pub type I2C1SEC_R = crate::BitReader<bool>;
///Field `I2C1SEC` writer - secure access mode for I2C1
pub type I2C1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `I2C2SEC` reader - secure access mode for I2C2
pub type I2C2SEC_R = crate::BitReader<bool>;
///Field `I2C2SEC` writer - secure access mode for I2C2
pub type I2C2SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `CRSSEC` reader - secure access mode for CRS
pub type CRSSEC_R = crate::BitReader<bool>;
///Field `CRSSEC` writer - secure access mode for CRS
pub type CRSSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `I2C4SEC` reader - secure access mode for I2C4
pub type I2C4SEC_R = crate::BitReader<bool>;
///Field `I2C4SEC` writer - secure access mode for I2C4
pub type I2C4SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `LPTIM2SEC` reader - secure access mode for LPTIM2
pub type LPTIM2SEC_R = crate::BitReader<bool>;
///Field `LPTIM2SEC` writer - secure access mode for LPTIM2
pub type LPTIM2SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `FDCAN1SEC` reader - secure access mode for FDCAN1
pub type FDCAN1SEC_R = crate::BitReader<bool>;
///Field `FDCAN1SEC` writer - secure access mode for FDCAN1
pub type FDCAN1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `UCPD1SEC` reader - secure access mode for UCPD1
pub type UCPD1SEC_R = crate::BitReader<bool>;
///Field `UCPD1SEC` writer - secure access mode for UCPD1
pub type UCPD1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - secure access mode for TIM2
    #[inline(always)]
    pub fn tim2sec(&self) -> TIM2SEC_R {
        TIM2SEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - secure access mode for TIM3
    #[inline(always)]
    pub fn tim3sec(&self) -> TIM3SEC_R {
        TIM3SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - secure access mode for TIM4
    #[inline(always)]
    pub fn tim4sec(&self) -> TIM4SEC_R {
        TIM4SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - secure access mode for TIM5
    #[inline(always)]
    pub fn tim5sec(&self) -> TIM5SEC_R {
        TIM5SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - secure access mode for TIM6
    #[inline(always)]
    pub fn tim6sec(&self) -> TIM6SEC_R {
        TIM6SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - secure access mode for TIM7
    #[inline(always)]
    pub fn tim7sec(&self) -> TIM7SEC_R {
        TIM7SEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - secure access mode for WWDG
    #[inline(always)]
    pub fn wwdgsec(&self) -> WWDGSEC_R {
        WWDGSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - secure access mode for IWDG
    #[inline(always)]
    pub fn iwdgsec(&self) -> IWDGSEC_R {
        IWDGSEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - secure access mode for SPI2
    #[inline(always)]
    pub fn spi2sec(&self) -> SPI2SEC_R {
        SPI2SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - secure access mode for USART2
    #[inline(always)]
    pub fn usart2sec(&self) -> USART2SEC_R {
        USART2SEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - secure access mode for USART3
    #[inline(always)]
    pub fn usart3sec(&self) -> USART3SEC_R {
        USART3SEC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - secure access mode for UART4
    #[inline(always)]
    pub fn uart4sec(&self) -> UART4SEC_R {
        UART4SEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - secure access mode for UART5
    #[inline(always)]
    pub fn uart5sec(&self) -> UART5SEC_R {
        UART5SEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - secure access mode for I2C1
    #[inline(always)]
    pub fn i2c1sec(&self) -> I2C1SEC_R {
        I2C1SEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - secure access mode for I2C2
    #[inline(always)]
    pub fn i2c2sec(&self) -> I2C2SEC_R {
        I2C2SEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - secure access mode for CRS
    #[inline(always)]
    pub fn crssec(&self) -> CRSSEC_R {
        CRSSEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - secure access mode for I2C4
    #[inline(always)]
    pub fn i2c4sec(&self) -> I2C4SEC_R {
        I2C4SEC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - secure access mode for LPTIM2
    #[inline(always)]
    pub fn lptim2sec(&self) -> LPTIM2SEC_R {
        LPTIM2SEC_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - secure access mode for FDCAN1
    #[inline(always)]
    pub fn fdcan1sec(&self) -> FDCAN1SEC_R {
        FDCAN1SEC_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - secure access mode for UCPD1
    #[inline(always)]
    pub fn ucpd1sec(&self) -> UCPD1SEC_R {
        UCPD1SEC_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - secure access mode for TIM2
    #[inline(always)]
    #[must_use]
    pub fn tim2sec(&mut self) -> TIM2SEC_W<0> {
        TIM2SEC_W::new(self)
    }
    ///Bit 1 - secure access mode for TIM3
    #[inline(always)]
    #[must_use]
    pub fn tim3sec(&mut self) -> TIM3SEC_W<1> {
        TIM3SEC_W::new(self)
    }
    ///Bit 2 - secure access mode for TIM4
    #[inline(always)]
    #[must_use]
    pub fn tim4sec(&mut self) -> TIM4SEC_W<2> {
        TIM4SEC_W::new(self)
    }
    ///Bit 3 - secure access mode for TIM5
    #[inline(always)]
    #[must_use]
    pub fn tim5sec(&mut self) -> TIM5SEC_W<3> {
        TIM5SEC_W::new(self)
    }
    ///Bit 4 - secure access mode for TIM6
    #[inline(always)]
    #[must_use]
    pub fn tim6sec(&mut self) -> TIM6SEC_W<4> {
        TIM6SEC_W::new(self)
    }
    ///Bit 5 - secure access mode for TIM7
    #[inline(always)]
    #[must_use]
    pub fn tim7sec(&mut self) -> TIM7SEC_W<5> {
        TIM7SEC_W::new(self)
    }
    ///Bit 6 - secure access mode for WWDG
    #[inline(always)]
    #[must_use]
    pub fn wwdgsec(&mut self) -> WWDGSEC_W<6> {
        WWDGSEC_W::new(self)
    }
    ///Bit 7 - secure access mode for IWDG
    #[inline(always)]
    #[must_use]
    pub fn iwdgsec(&mut self) -> IWDGSEC_W<7> {
        IWDGSEC_W::new(self)
    }
    ///Bit 8 - secure access mode for SPI2
    #[inline(always)]
    #[must_use]
    pub fn spi2sec(&mut self) -> SPI2SEC_W<8> {
        SPI2SEC_W::new(self)
    }
    ///Bit 9 - secure access mode for USART2
    #[inline(always)]
    #[must_use]
    pub fn usart2sec(&mut self) -> USART2SEC_W<9> {
        USART2SEC_W::new(self)
    }
    ///Bit 10 - secure access mode for USART3
    #[inline(always)]
    #[must_use]
    pub fn usart3sec(&mut self) -> USART3SEC_W<10> {
        USART3SEC_W::new(self)
    }
    ///Bit 11 - secure access mode for UART4
    #[inline(always)]
    #[must_use]
    pub fn uart4sec(&mut self) -> UART4SEC_W<11> {
        UART4SEC_W::new(self)
    }
    ///Bit 12 - secure access mode for UART5
    #[inline(always)]
    #[must_use]
    pub fn uart5sec(&mut self) -> UART5SEC_W<12> {
        UART5SEC_W::new(self)
    }
    ///Bit 13 - secure access mode for I2C1
    #[inline(always)]
    #[must_use]
    pub fn i2c1sec(&mut self) -> I2C1SEC_W<13> {
        I2C1SEC_W::new(self)
    }
    ///Bit 14 - secure access mode for I2C2
    #[inline(always)]
    #[must_use]
    pub fn i2c2sec(&mut self) -> I2C2SEC_W<14> {
        I2C2SEC_W::new(self)
    }
    ///Bit 15 - secure access mode for CRS
    #[inline(always)]
    #[must_use]
    pub fn crssec(&mut self) -> CRSSEC_W<15> {
        CRSSEC_W::new(self)
    }
    ///Bit 16 - secure access mode for I2C4
    #[inline(always)]
    #[must_use]
    pub fn i2c4sec(&mut self) -> I2C4SEC_W<16> {
        I2C4SEC_W::new(self)
    }
    ///Bit 17 - secure access mode for LPTIM2
    #[inline(always)]
    #[must_use]
    pub fn lptim2sec(&mut self) -> LPTIM2SEC_W<17> {
        LPTIM2SEC_W::new(self)
    }
    ///Bit 18 - secure access mode for FDCAN1
    #[inline(always)]
    #[must_use]
    pub fn fdcan1sec(&mut self) -> FDCAN1SEC_W<18> {
        FDCAN1SEC_W::new(self)
    }
    ///Bit 19 - secure access mode for UCPD1
    #[inline(always)]
    #[must_use]
    pub fn ucpd1sec(&mut self) -> UCPD1SEC_W<19> {
        UCPD1SEC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZSC secure configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzsc_seccfgr1](index.html) module
pub struct TZSC_SECCFGR1_SPEC;
impl crate::RegisterSpec for TZSC_SECCFGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzsc_seccfgr1::R](R) reader structure
impl crate::Readable for TZSC_SECCFGR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tzsc_seccfgr1::W](W) writer structure
impl crate::Writable for TZSC_SECCFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TZSC_SECCFGR1 to value 0
impl crate::Resettable for TZSC_SECCFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
