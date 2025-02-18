///Register `APBRSTR2` reader
pub struct R(crate::R<APBRSTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBRSTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBRSTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBRSTR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APBRSTR2` writer
pub struct W(crate::W<APBRSTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBRSTR2_SPEC>;
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
impl From<crate::W<APBRSTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBRSTR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SYSCFGRST` reader - SYSCFG, COMP and VREFBUF reset
pub type SYSCFGRST_R = crate::BitReader<bool>;
///Field `SYSCFGRST` writer - SYSCFG, COMP and VREFBUF reset
pub type SYSCFGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR2_SPEC, bool, O>;
///Field `TIM1RST` reader - TIM1 timer reset
pub type TIM1RST_R = crate::BitReader<bool>;
///Field `TIM1RST` writer - TIM1 timer reset
pub type TIM1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR2_SPEC, bool, O>;
///Field `SPI1RST` reader - SPI1 reset
pub type SPI1RST_R = crate::BitReader<bool>;
///Field `SPI1RST` writer - SPI1 reset
pub type SPI1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR2_SPEC, bool, O>;
///Field `USART1RST` reader - USART1 reset
pub type USART1RST_R = crate::BitReader<bool>;
///Field `USART1RST` writer - USART1 reset
pub type USART1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR2_SPEC, bool, O>;
///Field `TIM14RST` reader - TIM14 timer reset
pub type TIM14RST_R = crate::BitReader<bool>;
///Field `TIM14RST` writer - TIM14 timer reset
pub type TIM14RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR2_SPEC, bool, O>;
///Field `TIM15RST` reader - TIM15 timer reset
pub type TIM15RST_R = crate::BitReader<bool>;
///Field `TIM15RST` writer - TIM15 timer reset
pub type TIM15RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR2_SPEC, bool, O>;
///Field `TIM16RST` reader - TIM16 timer reset
pub type TIM16RST_R = crate::BitReader<bool>;
///Field `TIM16RST` writer - TIM16 timer reset
pub type TIM16RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR2_SPEC, bool, O>;
///Field `TIM17RST` reader - TIM17 timer reset
pub type TIM17RST_R = crate::BitReader<bool>;
///Field `TIM17RST` writer - TIM17 timer reset
pub type TIM17RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR2_SPEC, bool, O>;
///Field `ADCRST` reader - ADC reset
pub type ADCRST_R = crate::BitReader<bool>;
///Field `ADCRST` writer - ADC reset
pub type ADCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR2_SPEC, bool, O>;
impl R {
    ///Bit 0 - SYSCFG, COMP and VREFBUF reset
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 11 - TIM1 timer reset
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1 reset
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USART1 reset
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TIM14 timer reset
    #[inline(always)]
    pub fn tim14rst(&self) -> TIM14RST_R {
        TIM14RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - TIM15 timer reset
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 timer reset
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 timer reset
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - ADC reset
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SYSCFG, COMP and VREFBUF reset
    #[inline(always)]
    #[must_use]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<0> {
        SYSCFGRST_W::new(self)
    }
    ///Bit 11 - TIM1 timer reset
    #[inline(always)]
    #[must_use]
    pub fn tim1rst(&mut self) -> TIM1RST_W<11> {
        TIM1RST_W::new(self)
    }
    ///Bit 12 - SPI1 reset
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<12> {
        SPI1RST_W::new(self)
    }
    ///Bit 14 - USART1 reset
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<14> {
        USART1RST_W::new(self)
    }
    ///Bit 15 - TIM14 timer reset
    #[inline(always)]
    #[must_use]
    pub fn tim14rst(&mut self) -> TIM14RST_W<15> {
        TIM14RST_W::new(self)
    }
    ///Bit 16 - TIM15 timer reset
    #[inline(always)]
    #[must_use]
    pub fn tim15rst(&mut self) -> TIM15RST_W<16> {
        TIM15RST_W::new(self)
    }
    ///Bit 17 - TIM16 timer reset
    #[inline(always)]
    #[must_use]
    pub fn tim16rst(&mut self) -> TIM16RST_W<17> {
        TIM16RST_W::new(self)
    }
    ///Bit 18 - TIM17 timer reset
    #[inline(always)]
    #[must_use]
    pub fn tim17rst(&mut self) -> TIM17RST_W<18> {
        TIM17RST_W::new(self)
    }
    ///Bit 20 - ADC reset
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> ADCRST_W<20> {
        ADCRST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///APB peripheral reset register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apbrstr2](index.html) module
pub struct APBRSTR2_SPEC;
impl crate::RegisterSpec for APBRSTR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [apbrstr2::R](R) reader structure
impl crate::Readable for APBRSTR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apbrstr2::W](W) writer structure
impl crate::Writable for APBRSTR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets APBRSTR2 to value 0
impl crate::Resettable for APBRSTR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
