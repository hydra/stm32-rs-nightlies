///Register `ADF_DFLT0RSFR` reader
pub struct R(crate::R<ADF_DFLT0RSFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADF_DFLT0RSFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADF_DFLT0RSFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADF_DFLT0RSFR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADF_DFLT0RSFR` writer
pub struct W(crate::W<ADF_DFLT0RSFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADF_DFLT0RSFR_SPEC>;
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
impl From<crate::W<ADF_DFLT0RSFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADF_DFLT0RSFR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RSFLTBYP` reader - Reshaper filter bypass
pub type RSFLTBYP_R = crate::BitReader<bool>;
///Field `RSFLTBYP` writer - Reshaper filter bypass
pub type RSFLTBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_DFLT0RSFR_SPEC, bool, O>;
///Field `RSFLTD` reader - Reshaper filter decimation ratio
pub type RSFLTD_R = crate::BitReader<bool>;
///Field `RSFLTD` writer - Reshaper filter decimation ratio
pub type RSFLTD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_DFLT0RSFR_SPEC, bool, O>;
///Field `HPFBYP` reader - High-pass filter bypass
pub type HPFBYP_R = crate::BitReader<bool>;
///Field `HPFBYP` writer - High-pass filter bypass
pub type HPFBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_DFLT0RSFR_SPEC, bool, O>;
///Field `HPFC` reader - High-pass filter cut-off frequency
pub type HPFC_R = crate::FieldReader<u8, u8>;
///Field `HPFC` writer - High-pass filter cut-off frequency
pub type HPFC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADF_DFLT0RSFR_SPEC, u8, u8, 2, O>;
impl R {
    ///Bit 0 - Reshaper filter bypass
    #[inline(always)]
    pub fn rsfltbyp(&self) -> RSFLTBYP_R {
        RSFLTBYP_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Reshaper filter decimation ratio
    #[inline(always)]
    pub fn rsfltd(&self) -> RSFLTD_R {
        RSFLTD_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - High-pass filter bypass
    #[inline(always)]
    pub fn hpfbyp(&self) -> HPFBYP_R {
        HPFBYP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - High-pass filter cut-off frequency
    #[inline(always)]
    pub fn hpfc(&self) -> HPFC_R {
        HPFC_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - Reshaper filter bypass
    #[inline(always)]
    #[must_use]
    pub fn rsfltbyp(&mut self) -> RSFLTBYP_W<0> {
        RSFLTBYP_W::new(self)
    }
    ///Bit 4 - Reshaper filter decimation ratio
    #[inline(always)]
    #[must_use]
    pub fn rsfltd(&mut self) -> RSFLTD_W<4> {
        RSFLTD_W::new(self)
    }
    ///Bit 7 - High-pass filter bypass
    #[inline(always)]
    #[must_use]
    pub fn hpfbyp(&mut self) -> HPFBYP_W<7> {
        HPFBYP_W::new(self)
    }
    ///Bits 8:9 - High-pass filter cut-off frequency
    #[inline(always)]
    #[must_use]
    pub fn hpfc(&mut self) -> HPFC_W<8> {
        HPFC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADF reshape filter configuration register 0
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adf_dflt0rsfr](index.html) module
pub struct ADF_DFLT0RSFR_SPEC;
impl crate::RegisterSpec for ADF_DFLT0RSFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [adf_dflt0rsfr::R](R) reader structure
impl crate::Readable for ADF_DFLT0RSFR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adf_dflt0rsfr::W](W) writer structure
impl crate::Writable for ADF_DFLT0RSFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADF_DFLT0RSFR to value 0
impl crate::Resettable for ADF_DFLT0RSFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
