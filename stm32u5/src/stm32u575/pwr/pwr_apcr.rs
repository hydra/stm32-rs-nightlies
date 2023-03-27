///Register `PWR_APCR` reader
pub struct R(crate::R<PWR_APCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_APCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_APCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_APCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PWR_APCR` writer
pub struct W(crate::W<PWR_APCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_APCR_SPEC>;
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
impl From<crate::W<PWR_APCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_APCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `APC` reader - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PWR_PUCRx and PWR_PDCRx are applied. When this bit is cleared, PWR_PUCRx and PWR_PDCRx are not applied to the I/Os.
pub type APC_R = crate::BitReader<bool>;
///Field `APC` writer - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PWR_PUCRx and PWR_PDCRx are applied. When this bit is cleared, PWR_PUCRx and PWR_PDCRx are not applied to the I/Os.
pub type APC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_APCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PWR_PUCRx and PWR_PDCRx are applied. When this bit is cleared, PWR_PUCRx and PWR_PDCRx are not applied to the I/Os.
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PWR_PUCRx and PWR_PDCRx are applied. When this bit is cleared, PWR_PUCRx and PWR_PDCRx are not applied to the I/Os.
    #[inline(always)]
    #[must_use]
    pub fn apc(&mut self) -> APC_W<0> {
        APC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PWR apply pull configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pwr_apcr](index.html) module
pub struct PWR_APCR_SPEC;
impl crate::RegisterSpec for PWR_APCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pwr_apcr::R](R) reader structure
impl crate::Readable for PWR_APCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pwr_apcr::W](W) writer structure
impl crate::Writable for PWR_APCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PWR_APCR to value 0
impl crate::Resettable for PWR_APCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
