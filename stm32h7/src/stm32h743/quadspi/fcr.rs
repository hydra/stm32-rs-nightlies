///Register `FCR` reader
pub struct R(crate::R<FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FCR` writer
pub struct W(crate::W<FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR_SPEC>;
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
impl From<crate::W<FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CTEF` reader - Clear transfer error flag Writing 1 clears the TEF flag in the QUADSPI_SR register
pub type CTEF_R = crate::BitReader<bool>;
///Field `CTEF` writer - Clear transfer error flag Writing 1 clears the TEF flag in the QUADSPI_SR register
pub type CTEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
///Field `CTCF` reader - Clear transfer complete flag Writing 1 clears the TCF flag in the QUADSPI_SR register
pub type CTCF_R = crate::BitReader<bool>;
///Field `CTCF` writer - Clear transfer complete flag Writing 1 clears the TCF flag in the QUADSPI_SR register
pub type CTCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
///Field `CSMF` reader - Clear status match flag Writing 1 clears the SMF flag in the QUADSPI_SR register
pub type CSMF_R = crate::BitReader<bool>;
///Field `CSMF` writer - Clear status match flag Writing 1 clears the SMF flag in the QUADSPI_SR register
pub type CSMF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
///Field `CTOF` reader - Clear timeout flag Writing 1 clears the TOF flag in the QUADSPI_SR register
pub type CTOF_R = crate::BitReader<bool>;
///Field `CTOF` writer - Clear timeout flag Writing 1 clears the TOF flag in the QUADSPI_SR register
pub type CTOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Clear transfer error flag Writing 1 clears the TEF flag in the QUADSPI_SR register
    #[inline(always)]
    pub fn ctef(&self) -> CTEF_R {
        CTEF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clear transfer complete flag Writing 1 clears the TCF flag in the QUADSPI_SR register
    #[inline(always)]
    pub fn ctcf(&self) -> CTCF_R {
        CTCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Clear status match flag Writing 1 clears the SMF flag in the QUADSPI_SR register
    #[inline(always)]
    pub fn csmf(&self) -> CSMF_R {
        CSMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Clear timeout flag Writing 1 clears the TOF flag in the QUADSPI_SR register
    #[inline(always)]
    pub fn ctof(&self) -> CTOF_R {
        CTOF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Clear transfer error flag Writing 1 clears the TEF flag in the QUADSPI_SR register
    #[inline(always)]
    #[must_use]
    pub fn ctef(&mut self) -> CTEF_W<0> {
        CTEF_W::new(self)
    }
    ///Bit 1 - Clear transfer complete flag Writing 1 clears the TCF flag in the QUADSPI_SR register
    #[inline(always)]
    #[must_use]
    pub fn ctcf(&mut self) -> CTCF_W<1> {
        CTCF_W::new(self)
    }
    ///Bit 3 - Clear status match flag Writing 1 clears the SMF flag in the QUADSPI_SR register
    #[inline(always)]
    #[must_use]
    pub fn csmf(&mut self) -> CSMF_W<3> {
        CSMF_W::new(self)
    }
    ///Bit 4 - Clear timeout flag Writing 1 clears the TOF flag in the QUADSPI_SR register
    #[inline(always)]
    #[must_use]
    pub fn ctof(&mut self) -> CTOF_W<4> {
        CTOF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///QUADSPI flag clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fcr](index.html) module
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [fcr::R](R) reader structure
impl crate::Readable for FCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fcr::W](W) writer structure
impl crate::Writable for FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FCR to value 0
impl crate::Resettable for FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
