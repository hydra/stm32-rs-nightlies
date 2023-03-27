///Register `DLENR` reader
pub struct R(crate::R<DLENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DLENR` writer
pub struct W(crate::W<DLENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLENR_SPEC>;
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
impl From<crate::W<DLENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DATALENGTH` reader - Data length value This register can only be written by firmware when DPSM is inactive (DPSMACT = 0). Number of data bytes to be transferred. When DDR = 1 DATALENGTH is truncated to a multiple of 2. (The last odd byte is not transfered) When DATALENGTH = 0 no data will be transfered, when requested by a CPSMEN and CMDTRANS = 1 also no command will be transfered. DTEN and CPSMEN are cleared to 0.
pub type DATALENGTH_R = crate::FieldReader<u32, u32>;
///Field `DATALENGTH` writer - Data length value This register can only be written by firmware when DPSM is inactive (DPSMACT = 0). Number of data bytes to be transferred. When DDR = 1 DATALENGTH is truncated to a multiple of 2. (The last odd byte is not transfered) When DATALENGTH = 0 no data will be transfered, when requested by a CPSMEN and CMDTRANS = 1 also no command will be transfered. DTEN and CPSMEN are cleared to 0.
pub type DATALENGTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLENR_SPEC, u32, u32, 25, O>;
impl R {
    ///Bits 0:24 - Data length value This register can only be written by firmware when DPSM is inactive (DPSMACT = 0). Number of data bytes to be transferred. When DDR = 1 DATALENGTH is truncated to a multiple of 2. (The last odd byte is not transfered) When DATALENGTH = 0 no data will be transfered, when requested by a CPSMEN and CMDTRANS = 1 also no command will be transfered. DTEN and CPSMEN are cleared to 0.
    #[inline(always)]
    pub fn datalength(&self) -> DATALENGTH_R {
        DATALENGTH_R::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    ///Bits 0:24 - Data length value This register can only be written by firmware when DPSM is inactive (DPSMACT = 0). Number of data bytes to be transferred. When DDR = 1 DATALENGTH is truncated to a multiple of 2. (The last odd byte is not transfered) When DATALENGTH = 0 no data will be transfered, when requested by a CPSMEN and CMDTRANS = 1 also no command will be transfered. DTEN and CPSMEN are cleared to 0.
    #[inline(always)]
    #[must_use]
    pub fn datalength(&mut self) -> DATALENGTH_W<0> {
        DATALENGTH_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dlenr](index.html) module
pub struct DLENR_SPEC;
impl crate::RegisterSpec for DLENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dlenr::R](R) reader structure
impl crate::Readable for DLENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dlenr::W](W) writer structure
impl crate::Writable for DLENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DLENR to value 0
impl crate::Resettable for DLENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
