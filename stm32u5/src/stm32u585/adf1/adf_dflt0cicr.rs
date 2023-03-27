///Register `ADF_DFLT0CICR` reader
pub struct R(crate::R<ADF_DFLT0CICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADF_DFLT0CICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADF_DFLT0CICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADF_DFLT0CICR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADF_DFLT0CICR` writer
pub struct W(crate::W<ADF_DFLT0CICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADF_DFLT0CICR_SPEC>;
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
impl From<crate::W<ADF_DFLT0CICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADF_DFLT0CICR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DATSRC` reader - Source data for the digital filter
pub type DATSRC_R = crate::FieldReader<u8, u8>;
///Field `DATSRC` writer - Source data for the digital filter
pub type DATSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADF_DFLT0CICR_SPEC, u8, u8, 2, O>;
///Field `CICMOD` reader - Select the CIC order
pub type CICMOD_R = crate::FieldReader<u8, u8>;
///Field `CICMOD` writer - Select the CIC order
pub type CICMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADF_DFLT0CICR_SPEC, u8, u8, 3, O>;
///Field `MCICD` reader - CIC decimation ratio selection
pub type MCICD_R = crate::FieldReader<u16, u16>;
///Field `MCICD` writer - CIC decimation ratio selection
pub type MCICD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADF_DFLT0CICR_SPEC, u16, u16, 9, O>;
///Field `SCALE` reader - Scaling factor selection
pub type SCALE_R = crate::FieldReader<u8, u8>;
///Field `SCALE` writer - Scaling factor selection
pub type SCALE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADF_DFLT0CICR_SPEC, u8, u8, 6, O>;
impl R {
    ///Bits 0:1 - Source data for the digital filter
    #[inline(always)]
    pub fn datsrc(&self) -> DATSRC_R {
        DATSRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:6 - Select the CIC order
    #[inline(always)]
    pub fn cicmod(&self) -> CICMOD_R {
        CICMOD_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:16 - CIC decimation ratio selection
    #[inline(always)]
    pub fn mcicd(&self) -> MCICD_R {
        MCICD_R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    ///Bits 20:25 - Scaling factor selection
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
impl W {
    ///Bits 0:1 - Source data for the digital filter
    #[inline(always)]
    #[must_use]
    pub fn datsrc(&mut self) -> DATSRC_W<0> {
        DATSRC_W::new(self)
    }
    ///Bits 4:6 - Select the CIC order
    #[inline(always)]
    #[must_use]
    pub fn cicmod(&mut self) -> CICMOD_W<4> {
        CICMOD_W::new(self)
    }
    ///Bits 8:16 - CIC decimation ratio selection
    #[inline(always)]
    #[must_use]
    pub fn mcicd(&mut self) -> MCICD_W<8> {
        MCICD_W::new(self)
    }
    ///Bits 20:25 - Scaling factor selection
    #[inline(always)]
    #[must_use]
    pub fn scale(&mut self) -> SCALE_W<20> {
        SCALE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADF digital filer configuration register 0
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adf_dflt0cicr](index.html) module
pub struct ADF_DFLT0CICR_SPEC;
impl crate::RegisterSpec for ADF_DFLT0CICR_SPEC {
    type Ux = u32;
}
///`read()` method returns [adf_dflt0cicr::R](R) reader structure
impl crate::Readable for ADF_DFLT0CICR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adf_dflt0cicr::W](W) writer structure
impl crate::Writable for ADF_DFLT0CICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADF_DFLT0CICR to value 0
impl crate::Resettable for ADF_DFLT0CICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
