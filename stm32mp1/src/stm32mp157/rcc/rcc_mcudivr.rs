///Register `RCC_MCUDIVR` reader
pub struct R(crate::R<RCC_MCUDIVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MCUDIVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MCUDIVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MCUDIVR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_MCUDIVR` writer
pub struct W(crate::W<RCC_MCUDIVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MCUDIVR_SPEC>;
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
impl From<crate::W<RCC_MCUDIVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MCUDIVR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MCUDIV` reader - MCUDIV
pub type MCUDIV_R = crate::FieldReader<u8, u8>;
///Field `MCUDIV` writer - MCUDIV
pub type MCUDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_MCUDIVR_SPEC, u8, u8, 4, O>;
///Field `MCUDIVRDY` reader - MCUDIVRDY
pub type MCUDIVRDY_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:3 - MCUDIV
    #[inline(always)]
    pub fn mcudiv(&self) -> MCUDIV_R {
        MCUDIV_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 31 - MCUDIVRDY
    #[inline(always)]
    pub fn mcudivrdy(&self) -> MCUDIVRDY_R {
        MCUDIVRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - MCUDIV
    #[inline(always)]
    #[must_use]
    pub fn mcudiv(&mut self) -> MCUDIV_W<0> {
        MCUDIV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to control the MCU sub-system clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_mcudivr](index.html) module
pub struct RCC_MCUDIVR_SPEC;
impl crate::RegisterSpec for RCC_MCUDIVR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_mcudivr::R](R) reader structure
impl crate::Readable for RCC_MCUDIVR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_mcudivr::W](W) writer structure
impl crate::Writable for RCC_MCUDIVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_MCUDIVR to value 0x8000_0000
impl crate::Resettable for RCC_MCUDIVR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
