///Register `APBSMENR1` reader
pub struct R(crate::R<APBSMENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBSMENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBSMENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBSMENR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APBSMENR1` writer
pub struct W(crate::W<APBSMENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBSMENR1_SPEC>;
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
impl From<crate::W<APBSMENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBSMENR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM3SMEN` reader - TIM3 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM3SMEN_R = crate::BitReader<bool>;
///Field `TIM3SMEN` writer - TIM3 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM3SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
///Field `RTCAPBSMEN` reader - RTC APB clock enable during Sleep mode Set and cleared by software.
pub type RTCAPBSMEN_R = crate::BitReader<bool>;
///Field `RTCAPBSMEN` writer - RTC APB clock enable during Sleep mode Set and cleared by software.
pub type RTCAPBSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
///Field `WWDGSMEN` reader - WWDG clock enable during Sleep and Stop modes Set and cleared by software.
pub type WWDGSMEN_R = crate::BitReader<bool>;
///Field `WWDGSMEN` writer - WWDG clock enable during Sleep and Stop modes Set and cleared by software.
pub type WWDGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
///Field `USART2SMEN` reader - USART2 clock enable during Sleep and Stop modes Set and cleared by software.
pub type USART2SMEN_R = crate::BitReader<bool>;
///Field `USART2SMEN` writer - USART2 clock enable during Sleep and Stop modes Set and cleared by software.
pub type USART2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
///Field `I2C1SMEN` reader - I2C1 clock enable during Sleep and Stop modes Set and cleared by software.
pub type I2C1SMEN_R = crate::BitReader<bool>;
///Field `I2C1SMEN` writer - I2C1 clock enable during Sleep and Stop modes Set and cleared by software.
pub type I2C1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
///Field `DBGSMEN` reader - Debug support clock enable during Sleep mode Set and cleared by software.
pub type DBGSMEN_R = crate::BitReader<bool>;
///Field `DBGSMEN` writer - Debug support clock enable during Sleep mode Set and cleared by software.
pub type DBGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
///Field `PWRSMEN` reader - Power interface clock enable during Sleep mode Set and cleared by software.
pub type PWRSMEN_R = crate::BitReader<bool>;
///Field `PWRSMEN` writer - Power interface clock enable during Sleep mode Set and cleared by software.
pub type PWRSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
impl R {
    ///Bit 1 - TIM3 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim3smen(&self) -> TIM3SMEN_R {
        TIM3SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 10 - RTC APB clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - WWDG clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 17 - USART2 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 27 - Debug support clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn dbgsmen(&self) -> DBGSMEN_R {
        DBGSMEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Power interface clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - TIM3 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim3smen(&mut self) -> TIM3SMEN_W<1> {
        TIM3SMEN_W::new(self)
    }
    ///Bit 10 - RTC APB clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W<10> {
        RTCAPBSMEN_W::new(self)
    }
    ///Bit 11 - WWDG clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<11> {
        WWDGSMEN_W::new(self)
    }
    ///Bit 17 - USART2 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn usart2smen(&mut self) -> USART2SMEN_W<17> {
        USART2SMEN_W::new(self)
    }
    ///Bit 21 - I2C1 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<21> {
        I2C1SMEN_W::new(self)
    }
    ///Bit 27 - Debug support clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn dbgsmen(&mut self) -> DBGSMEN_W<27> {
        DBGSMEN_W::new(self)
    }
    ///Bit 28 - Power interface clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W<28> {
        PWRSMEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC APB peripheral clock enable in Sleep/Stop mode register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apbsmenr1](index.html) module
pub struct APBSMENR1_SPEC;
impl crate::RegisterSpec for APBSMENR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [apbsmenr1::R](R) reader structure
impl crate::Readable for APBSMENR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apbsmenr1::W](W) writer structure
impl crate::Writable for APBSMENR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets APBSMENR1 to value 0x18ef_7f36
impl crate::Resettable for APBSMENR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x18ef_7f36;
}
