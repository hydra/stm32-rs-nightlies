///Register `ADF_DFLT0CR` reader
pub struct R(crate::R<ADF_DFLT0CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADF_DFLT0CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADF_DFLT0CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADF_DFLT0CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADF_DFLT0CR` writer
pub struct W(crate::W<ADF_DFLT0CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADF_DFLT0CR_SPEC>;
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
impl From<crate::W<ADF_DFLT0CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADF_DFLT0CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DFLTEN` reader - DFLT0 enable
pub type DFLTEN_R = crate::BitReader<bool>;
///Field `DFLTEN` writer - DFLT0 enable
pub type DFLTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_DFLT0CR_SPEC, bool, O>;
///Field `DMAEN` reader - DMA requests enable
pub type DMAEN_R = crate::BitReader<bool>;
///Field `DMAEN` writer - DMA requests enable
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_DFLT0CR_SPEC, bool, O>;
///Field `FTH` reader - RXFIFO threshold selection
pub type FTH_R = crate::BitReader<bool>;
///Field `FTH` writer - RXFIFO threshold selection
pub type FTH_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_DFLT0CR_SPEC, bool, O>;
///Field `ACQMOD` reader - DFLT0 trigger mode
pub type ACQMOD_R = crate::FieldReader<u8, u8>;
///Field `ACQMOD` writer - DFLT0 trigger mode
pub type ACQMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADF_DFLT0CR_SPEC, u8, u8, 3, O>;
///Field `TRGSRC` reader - DFLT0 trigger signal selection
pub type TRGSRC_R = crate::FieldReader<u8, u8>;
///Field `TRGSRC` writer - DFLT0 trigger signal selection
pub type TRGSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADF_DFLT0CR_SPEC, u8, u8, 4, O>;
///Field `NBDIS` reader - Number of samples to be discarded
pub type NBDIS_R = crate::FieldReader<u8, u8>;
///Field `NBDIS` writer - Number of samples to be discarded
pub type NBDIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADF_DFLT0CR_SPEC, u8, u8, 8, O>;
///Field `DFLTRUN` reader - DFLT0 run status flag
pub type DFLTRUN_R = crate::BitReader<bool>;
///Field `DFLTRUN` writer - DFLT0 run status flag
pub type DFLTRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_DFLT0CR_SPEC, bool, O>;
///Field `DFLTACTIVE` reader - DFLT0 active flag
pub type DFLTACTIVE_R = crate::BitReader<bool>;
///Field `DFLTACTIVE` writer - DFLT0 active flag
pub type DFLTACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_DFLT0CR_SPEC, bool, O>;
impl R {
    ///Bit 0 - DFLT0 enable
    #[inline(always)]
    pub fn dflten(&self) -> DFLTEN_R {
        DFLTEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA requests enable
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RXFIFO threshold selection
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:6 - DFLT0 trigger mode
    #[inline(always)]
    pub fn acqmod(&self) -> ACQMOD_R {
        ACQMOD_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 12:15 - DFLT0 trigger signal selection
    #[inline(always)]
    pub fn trgsrc(&self) -> TRGSRC_R {
        TRGSRC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 20:27 - Number of samples to be discarded
    #[inline(always)]
    pub fn nbdis(&self) -> NBDIS_R {
        NBDIS_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    ///Bit 30 - DFLT0 run status flag
    #[inline(always)]
    pub fn dfltrun(&self) -> DFLTRUN_R {
        DFLTRUN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - DFLT0 active flag
    #[inline(always)]
    pub fn dfltactive(&self) -> DFLTACTIVE_R {
        DFLTACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DFLT0 enable
    #[inline(always)]
    #[must_use]
    pub fn dflten(&mut self) -> DFLTEN_W<0> {
        DFLTEN_W::new(self)
    }
    ///Bit 1 - DMA requests enable
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<1> {
        DMAEN_W::new(self)
    }
    ///Bit 2 - RXFIFO threshold selection
    #[inline(always)]
    #[must_use]
    pub fn fth(&mut self) -> FTH_W<2> {
        FTH_W::new(self)
    }
    ///Bits 4:6 - DFLT0 trigger mode
    #[inline(always)]
    #[must_use]
    pub fn acqmod(&mut self) -> ACQMOD_W<4> {
        ACQMOD_W::new(self)
    }
    ///Bits 12:15 - DFLT0 trigger signal selection
    #[inline(always)]
    #[must_use]
    pub fn trgsrc(&mut self) -> TRGSRC_W<12> {
        TRGSRC_W::new(self)
    }
    ///Bits 20:27 - Number of samples to be discarded
    #[inline(always)]
    #[must_use]
    pub fn nbdis(&mut self) -> NBDIS_W<20> {
        NBDIS_W::new(self)
    }
    ///Bit 30 - DFLT0 run status flag
    #[inline(always)]
    #[must_use]
    pub fn dfltrun(&mut self) -> DFLTRUN_W<30> {
        DFLTRUN_W::new(self)
    }
    ///Bit 31 - DFLT0 active flag
    #[inline(always)]
    #[must_use]
    pub fn dfltactive(&mut self) -> DFLTACTIVE_W<31> {
        DFLTACTIVE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADF digital filter control register 0
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adf_dflt0cr](index.html) module
pub struct ADF_DFLT0CR_SPEC;
impl crate::RegisterSpec for ADF_DFLT0CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [adf_dflt0cr::R](R) reader structure
impl crate::Readable for ADF_DFLT0CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adf_dflt0cr::W](W) writer structure
impl crate::Writable for ADF_DFLT0CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADF_DFLT0CR to value 0
impl crate::Resettable for ADF_DFLT0CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
