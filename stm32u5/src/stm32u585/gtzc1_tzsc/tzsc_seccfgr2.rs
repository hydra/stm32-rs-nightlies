///Register `TZSC_SECCFGR2` reader
pub struct R(crate::R<TZSC_SECCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZSC_SECCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZSC_SECCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZSC_SECCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TZSC_SECCFGR2` writer
pub struct W(crate::W<TZSC_SECCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZSC_SECCFGR2_SPEC>;
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
impl From<crate::W<TZSC_SECCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZSC_SECCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM1SEC` reader - secure access mode for TIM1
pub type TIM1SEC_R = crate::BitReader<bool>;
///Field `TIM1SEC` writer - secure access mode for TIM1
pub type TIM1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR2_SPEC, bool, O>;
///Field `SPI1SEC` reader - secure access mode for SPI1
pub type SPI1SEC_R = crate::BitReader<bool>;
///Field `SPI1SEC` writer - secure access mode for SPI1
pub type SPI1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR2_SPEC, bool, O>;
///Field `TIM8SEC` reader - secure access mode for TIM8
pub type TIM8SEC_R = crate::BitReader<bool>;
///Field `TIM8SEC` writer - secure access mode for TIM8
pub type TIM8SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR2_SPEC, bool, O>;
///Field `USART1SEC` reader - secure access mode for USART1
pub type USART1SEC_R = crate::BitReader<bool>;
///Field `USART1SEC` writer - secure access mode for USART1
pub type USART1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR2_SPEC, bool, O>;
///Field `TIM15SEC` reader - secure access mode for TIM5
pub type TIM15SEC_R = crate::BitReader<bool>;
///Field `TIM15SEC` writer - secure access mode for TIM5
pub type TIM15SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR2_SPEC, bool, O>;
///Field `TIM16SEC` reader - secure access mode for TIM6
pub type TIM16SEC_R = crate::BitReader<bool>;
///Field `TIM16SEC` writer - secure access mode for TIM6
pub type TIM16SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR2_SPEC, bool, O>;
///Field `TIM17SEC` reader - secure access mode for TIM7
pub type TIM17SEC_R = crate::BitReader<bool>;
///Field `TIM17SEC` writer - secure access mode for TIM7
pub type TIM17SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR2_SPEC, bool, O>;
///Field `SAI1SEC` reader - secure access mode for SAI1
pub type SAI1SEC_R = crate::BitReader<bool>;
///Field `SAI1SEC` writer - secure access mode for SAI1
pub type SAI1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR2_SPEC, bool, O>;
///Field `SAI2SEC` reader - secure access mode for SAI2
pub type SAI2SEC_R = crate::BitReader<bool>;
///Field `SAI2SEC` writer - secure access mode for SAI2
pub type SAI2SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR2_SPEC, bool, O>;
impl R {
    ///Bit 0 - secure access mode for TIM1
    #[inline(always)]
    pub fn tim1sec(&self) -> TIM1SEC_R {
        TIM1SEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - secure access mode for SPI1
    #[inline(always)]
    pub fn spi1sec(&self) -> SPI1SEC_R {
        SPI1SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - secure access mode for TIM8
    #[inline(always)]
    pub fn tim8sec(&self) -> TIM8SEC_R {
        TIM8SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - secure access mode for USART1
    #[inline(always)]
    pub fn usart1sec(&self) -> USART1SEC_R {
        USART1SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - secure access mode for TIM5
    #[inline(always)]
    pub fn tim15sec(&self) -> TIM15SEC_R {
        TIM15SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - secure access mode for TIM6
    #[inline(always)]
    pub fn tim16sec(&self) -> TIM16SEC_R {
        TIM16SEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - secure access mode for TIM7
    #[inline(always)]
    pub fn tim17sec(&self) -> TIM17SEC_R {
        TIM17SEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - secure access mode for SAI1
    #[inline(always)]
    pub fn sai1sec(&self) -> SAI1SEC_R {
        SAI1SEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - secure access mode for SAI2
    #[inline(always)]
    pub fn sai2sec(&self) -> SAI2SEC_R {
        SAI2SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - secure access mode for TIM1
    #[inline(always)]
    #[must_use]
    pub fn tim1sec(&mut self) -> TIM1SEC_W<0> {
        TIM1SEC_W::new(self)
    }
    ///Bit 1 - secure access mode for SPI1
    #[inline(always)]
    #[must_use]
    pub fn spi1sec(&mut self) -> SPI1SEC_W<1> {
        SPI1SEC_W::new(self)
    }
    ///Bit 2 - secure access mode for TIM8
    #[inline(always)]
    #[must_use]
    pub fn tim8sec(&mut self) -> TIM8SEC_W<2> {
        TIM8SEC_W::new(self)
    }
    ///Bit 3 - secure access mode for USART1
    #[inline(always)]
    #[must_use]
    pub fn usart1sec(&mut self) -> USART1SEC_W<3> {
        USART1SEC_W::new(self)
    }
    ///Bit 4 - secure access mode for TIM5
    #[inline(always)]
    #[must_use]
    pub fn tim15sec(&mut self) -> TIM15SEC_W<4> {
        TIM15SEC_W::new(self)
    }
    ///Bit 5 - secure access mode for TIM6
    #[inline(always)]
    #[must_use]
    pub fn tim16sec(&mut self) -> TIM16SEC_W<5> {
        TIM16SEC_W::new(self)
    }
    ///Bit 6 - secure access mode for TIM7
    #[inline(always)]
    #[must_use]
    pub fn tim17sec(&mut self) -> TIM17SEC_W<6> {
        TIM17SEC_W::new(self)
    }
    ///Bit 7 - secure access mode for SAI1
    #[inline(always)]
    #[must_use]
    pub fn sai1sec(&mut self) -> SAI1SEC_W<7> {
        SAI1SEC_W::new(self)
    }
    ///Bit 8 - secure access mode for SAI2
    #[inline(always)]
    #[must_use]
    pub fn sai2sec(&mut self) -> SAI2SEC_W<8> {
        SAI2SEC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZSC secure configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzsc_seccfgr2](index.html) module
pub struct TZSC_SECCFGR2_SPEC;
impl crate::RegisterSpec for TZSC_SECCFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzsc_seccfgr2::R](R) reader structure
impl crate::Readable for TZSC_SECCFGR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tzsc_seccfgr2::W](W) writer structure
impl crate::Writable for TZSC_SECCFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TZSC_SECCFGR2 to value 0
impl crate::Resettable for TZSC_SECCFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
