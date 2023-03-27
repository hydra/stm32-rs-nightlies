///Register `ADF_DFLT0ISR` reader
pub struct R(crate::R<ADF_DFLT0ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADF_DFLT0ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADF_DFLT0ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADF_DFLT0ISR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADF_DFLT0ISR` writer
pub struct W(crate::W<ADF_DFLT0ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADF_DFLT0ISR_SPEC>;
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
impl From<crate::W<ADF_DFLT0ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADF_DFLT0ISR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FTHF` reader - RXFIFO threshold flag
pub type FTHF_R = crate::BitReader<bool>;
///Field `DOVRF` reader - Data overflow flag
pub type DOVRF_R = crate::BitReader<bool>;
///Field `DOVRF` writer - Data overflow flag
pub type DOVRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_DFLT0ISR_SPEC, bool, O>;
///Field `RXNEF` reader - RXFIFO not empty flag
pub type RXNEF_R = crate::BitReader<bool>;
///Field `SATF` reader - Saturation detection flag
pub type SATF_R = crate::BitReader<bool>;
///Field `SATF` writer - Saturation detection flag
pub type SATF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_DFLT0ISR_SPEC, bool, O>;
///Field `CKABF` reader - Clock absence detection flag
pub type CKABF_R = crate::BitReader<bool>;
///Field `CKABF` writer - Clock absence detection flag
pub type CKABF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_DFLT0ISR_SPEC, bool, O>;
///Field `RFOVRF` reader - Reshape filter overrun detection flag
pub type RFOVRF_R = crate::BitReader<bool>;
///Field `RFOVRF` writer - Reshape filter overrun detection flag
pub type RFOVRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_DFLT0ISR_SPEC, bool, O>;
///Field `SDDETF` reader - Sound activity detection flag
pub type SDDETF_R = crate::BitReader<bool>;
///Field `SDDETF` writer - Sound activity detection flag
pub type SDDETF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_DFLT0ISR_SPEC, bool, O>;
///Field `SDLVLF` reader - Sound level value ready flag
pub type SDLVLF_R = crate::BitReader<bool>;
///Field `SDLVLF` writer - Sound level value ready flag
pub type SDLVLF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_DFLT0ISR_SPEC, bool, O>;
impl R {
    ///Bit 0 - RXFIFO threshold flag
    #[inline(always)]
    pub fn fthf(&self) -> FTHF_R {
        FTHF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data overflow flag
    #[inline(always)]
    pub fn dovrf(&self) -> DOVRF_R {
        DOVRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - RXFIFO not empty flag
    #[inline(always)]
    pub fn rxnef(&self) -> RXNEF_R {
        RXNEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 9 - Saturation detection flag
    #[inline(always)]
    pub fn satf(&self) -> SATF_R {
        SATF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Clock absence detection flag
    #[inline(always)]
    pub fn ckabf(&self) -> CKABF_R {
        CKABF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Reshape filter overrun detection flag
    #[inline(always)]
    pub fn rfovrf(&self) -> RFOVRF_R {
        RFOVRF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Sound activity detection flag
    #[inline(always)]
    pub fn sddetf(&self) -> SDDETF_R {
        SDDETF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Sound level value ready flag
    #[inline(always)]
    pub fn sdlvlf(&self) -> SDLVLF_R {
        SDLVLF_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - Data overflow flag
    #[inline(always)]
    #[must_use]
    pub fn dovrf(&mut self) -> DOVRF_W<1> {
        DOVRF_W::new(self)
    }
    ///Bit 9 - Saturation detection flag
    #[inline(always)]
    #[must_use]
    pub fn satf(&mut self) -> SATF_W<9> {
        SATF_W::new(self)
    }
    ///Bit 10 - Clock absence detection flag
    #[inline(always)]
    #[must_use]
    pub fn ckabf(&mut self) -> CKABF_W<10> {
        CKABF_W::new(self)
    }
    ///Bit 11 - Reshape filter overrun detection flag
    #[inline(always)]
    #[must_use]
    pub fn rfovrf(&mut self) -> RFOVRF_W<11> {
        RFOVRF_W::new(self)
    }
    ///Bit 12 - Sound activity detection flag
    #[inline(always)]
    #[must_use]
    pub fn sddetf(&mut self) -> SDDETF_W<12> {
        SDDETF_W::new(self)
    }
    ///Bit 13 - Sound level value ready flag
    #[inline(always)]
    #[must_use]
    pub fn sdlvlf(&mut self) -> SDLVLF_W<13> {
        SDLVLF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADF DFLT0 interrupt status register 0
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adf_dflt0isr](index.html) module
pub struct ADF_DFLT0ISR_SPEC;
impl crate::RegisterSpec for ADF_DFLT0ISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [adf_dflt0isr::R](R) reader structure
impl crate::Readable for ADF_DFLT0ISR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adf_dflt0isr::W](W) writer structure
impl crate::Writable for ADF_DFLT0ISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADF_DFLT0ISR to value 0
impl crate::Resettable for ADF_DFLT0ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
