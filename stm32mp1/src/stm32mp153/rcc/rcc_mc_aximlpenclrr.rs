///Register `RCC_MC_AXIMLPENCLRR` reader
pub struct R(crate::R<RCC_MC_AXIMLPENCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_AXIMLPENCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_AXIMLPENCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_AXIMLPENCLRR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_MC_AXIMLPENCLRR` writer
pub struct W(crate::W<RCC_MC_AXIMLPENCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_AXIMLPENCLRR_SPEC>;
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
impl From<crate::W<RCC_MC_AXIMLPENCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_AXIMLPENCLRR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SYSRAMLPEN` reader - SYSRAMLPEN
pub type SYSRAMLPEN_R = crate::BitReader<bool>;
///Field `SYSRAMLPEN` writer - SYSRAMLPEN
pub type SYSRAMLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MC_AXIMLPENCLRR_SPEC, bool, O>;
impl R {
    ///Bit 0 - SYSRAMLPEN
    #[inline(always)]
    pub fn sysramlpen(&self) -> SYSRAMLPEN_R {
        SYSRAMLPEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SYSRAMLPEN
    #[inline(always)]
    #[must_use]
    pub fn sysramlpen(&mut self) -> SYSRAMLPEN_W<0> {
        SYSRAMLPEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_mc_aximlpenclrr](index.html) module
pub struct RCC_MC_AXIMLPENCLRR_SPEC;
impl crate::RegisterSpec for RCC_MC_AXIMLPENCLRR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_mc_aximlpenclrr::R](R) reader structure
impl crate::Readable for RCC_MC_AXIMLPENCLRR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_mc_aximlpenclrr::W](W) writer structure
impl crate::Writable for RCC_MC_AXIMLPENCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_MC_AXIMLPENCLRR to value 0x01
impl crate::Resettable for RCC_MC_AXIMLPENCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
