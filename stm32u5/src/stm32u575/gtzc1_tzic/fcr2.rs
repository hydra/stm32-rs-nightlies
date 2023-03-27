///Register `FCR2` writer
pub struct W(crate::W<FCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR2_SPEC>;
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
impl From<crate::W<FCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CTIM1F` writer - clear the illegal access flag for TIM1
pub type CTIM1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
///Field `CSPI1F` writer - clear the illegal access flag for SPI1
pub type CSPI1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
///Field `CTIM8F` writer - clear the illegal access flag for TIM8
pub type CTIM8F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
///Field `CUSART1F` writer - clear the illegal access flag for USART1
pub type CUSART1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
///Field `CTIM15F` writer - clear the illegal access flag for TIM5
pub type CTIM15F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
///Field `CTIM16F` writer - clear the illegal access flag for TIM6
pub type CTIM16F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
///Field `CTIM17F` writer - clear the illegal access flag for TIM7
pub type CTIM17F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
///Field `CSAI1F` writer - clear the illegal access flag for SAI1
pub type CSAI1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
///Field `CSAI2F` writer - clear the illegal access flag for SAI2
pub type CSAI2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
impl W {
    ///Bit 0 - clear the illegal access flag for TIM1
    #[inline(always)]
    #[must_use]
    pub fn ctim1f(&mut self) -> CTIM1F_W<0> {
        CTIM1F_W::new(self)
    }
    ///Bit 1 - clear the illegal access flag for SPI1
    #[inline(always)]
    #[must_use]
    pub fn cspi1f(&mut self) -> CSPI1F_W<1> {
        CSPI1F_W::new(self)
    }
    ///Bit 2 - clear the illegal access flag for TIM8
    #[inline(always)]
    #[must_use]
    pub fn ctim8f(&mut self) -> CTIM8F_W<2> {
        CTIM8F_W::new(self)
    }
    ///Bit 3 - clear the illegal access flag for USART1
    #[inline(always)]
    #[must_use]
    pub fn cusart1f(&mut self) -> CUSART1F_W<3> {
        CUSART1F_W::new(self)
    }
    ///Bit 4 - clear the illegal access flag for TIM5
    #[inline(always)]
    #[must_use]
    pub fn ctim15f(&mut self) -> CTIM15F_W<4> {
        CTIM15F_W::new(self)
    }
    ///Bit 5 - clear the illegal access flag for TIM6
    #[inline(always)]
    #[must_use]
    pub fn ctim16f(&mut self) -> CTIM16F_W<5> {
        CTIM16F_W::new(self)
    }
    ///Bit 6 - clear the illegal access flag for TIM7
    #[inline(always)]
    #[must_use]
    pub fn ctim17f(&mut self) -> CTIM17F_W<6> {
        CTIM17F_W::new(self)
    }
    ///Bit 7 - clear the illegal access flag for SAI1
    #[inline(always)]
    #[must_use]
    pub fn csai1f(&mut self) -> CSAI1F_W<7> {
        CSAI1F_W::new(self)
    }
    ///Bit 8 - clear the illegal access flag for SAI2
    #[inline(always)]
    #[must_use]
    pub fn csai2f(&mut self) -> CSAI2F_W<8> {
        CSAI2F_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZIC flag clear register 2
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fcr2](index.html) module
pub struct FCR2_SPEC;
impl crate::RegisterSpec for FCR2_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [fcr2::W](W) writer structure
impl crate::Writable for FCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FCR2 to value 0
impl crate::Resettable for FCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
