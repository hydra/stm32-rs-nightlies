///Register `PWR_SR` reader
pub struct R(crate::R<PWR_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_SR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PWR_SR` writer
pub struct W(crate::W<PWR_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_SR_SPEC>;
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
impl From<crate::W<PWR_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_SR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CSSF` writer - Clear Stop and Standby flags This bit is protected against non-secure access when LPMSEC = 1 in PWR_SECCFGR. This bit is protected against unprivileged access when LPMSEC = 1 and SPRIV = 1 in PWR_PRIVCFGR, or when LPMSEC = 0 and NSPRIV = 1. Writing 1 to this bit clears the STOPF and SBF flags.
pub type CSSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_SR_SPEC, bool, O>;
///Field `STOPF` reader - Stop flag This bit is set by hardware when the device enters a Stop mode, and is cleared by software by writing 1 to the CSSF bit.
pub type STOPF_R = crate::BitReader<bool>;
///Field `SBF` reader - Standby flag This bit is set by hardware when the device enters the Standby mode, and is cleared by writing 1 to the CSSF bit, or by a power-on reset. It is not cleared by the system reset.
pub type SBF_R = crate::BitReader<bool>;
impl R {
    ///Bit 1 - Stop flag This bit is set by hardware when the device enters a Stop mode, and is cleared by software by writing 1 to the CSSF bit.
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Standby flag This bit is set by hardware when the device enters the Standby mode, and is cleared by writing 1 to the CSSF bit, or by a power-on reset. It is not cleared by the system reset.
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Clear Stop and Standby flags This bit is protected against non-secure access when LPMSEC = 1 in PWR_SECCFGR. This bit is protected against unprivileged access when LPMSEC = 1 and SPRIV = 1 in PWR_PRIVCFGR, or when LPMSEC = 0 and NSPRIV = 1. Writing 1 to this bit clears the STOPF and SBF flags.
    #[inline(always)]
    #[must_use]
    pub fn cssf(&mut self) -> CSSF_W<0> {
        CSSF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PWR status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pwr_sr](index.html) module
pub struct PWR_SR_SPEC;
impl crate::RegisterSpec for PWR_SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pwr_sr::R](R) reader structure
impl crate::Readable for PWR_SR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pwr_sr::W](W) writer structure
impl crate::Writable for PWR_SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PWR_SR to value 0
impl crate::Resettable for PWR_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
