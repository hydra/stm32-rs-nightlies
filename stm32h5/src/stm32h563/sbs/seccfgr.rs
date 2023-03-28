///Register `SECCFGR` reader
pub struct R(crate::R<SECCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SECCFGR` writer
pub struct W(crate::W<SECCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECCFGR_SPEC>;
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
impl From<crate::W<SECCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SBSSEC` reader - SBS clock control, memory-erase status register and compensation cell register security enable
pub type SBSSEC_R = crate::BitReader<bool>;
///Field `SBSSEC` writer - SBS clock control, memory-erase status register and compensation cell register security enable
pub type SBSSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
///Field `CLASSBSEC` reader - ClassB security enable
pub type CLASSBSEC_R = crate::BitReader<bool>;
///Field `CLASSBSEC` writer - ClassB security enable
pub type CLASSBSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
///Field `FPUSEC` reader - FPU security enable Note: This bit can only be written through privilege transaction.
pub type FPUSEC_R = crate::BitReader<bool>;
///Field `FPUSEC` writer - FPU security enable Note: This bit can only be written through privilege transaction.
pub type FPUSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
///Field `SDCE_SEC_EN` reader - control accessibility of SMPS_DIV_CLOCK _EN in SBS_PMCR
pub type SDCE_SEC_EN_R = crate::BitReader<bool>;
///Field `SDCE_SEC_EN` writer - control accessibility of SMPS_DIV_CLOCK _EN in SBS_PMCR
pub type SDCE_SEC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
impl R {
    ///Bit 0 - SBS clock control, memory-erase status register and compensation cell register security enable
    #[inline(always)]
    pub fn sbssec(&self) -> SBSSEC_R {
        SBSSEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ClassB security enable
    #[inline(always)]
    pub fn classbsec(&self) -> CLASSBSEC_R {
        CLASSBSEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - FPU security enable Note: This bit can only be written through privilege transaction.
    #[inline(always)]
    pub fn fpusec(&self) -> FPUSEC_R {
        FPUSEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 31 - control accessibility of SMPS_DIV_CLOCK _EN in SBS_PMCR
    #[inline(always)]
    pub fn sdce_sec_en(&self) -> SDCE_SEC_EN_R {
        SDCE_SEC_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SBS clock control, memory-erase status register and compensation cell register security enable
    #[inline(always)]
    #[must_use]
    pub fn sbssec(&mut self) -> SBSSEC_W<0> {
        SBSSEC_W::new(self)
    }
    ///Bit 1 - ClassB security enable
    #[inline(always)]
    #[must_use]
    pub fn classbsec(&mut self) -> CLASSBSEC_W<1> {
        CLASSBSEC_W::new(self)
    }
    ///Bit 3 - FPU security enable Note: This bit can only be written through privilege transaction.
    #[inline(always)]
    #[must_use]
    pub fn fpusec(&mut self) -> FPUSEC_W<3> {
        FPUSEC_W::new(self)
    }
    ///Bit 31 - control accessibility of SMPS_DIV_CLOCK _EN in SBS_PMCR
    #[inline(always)]
    #[must_use]
    pub fn sdce_sec_en(&mut self) -> SDCE_SEC_EN_W<31> {
        SDCE_SEC_EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SBS security mode configuration control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [seccfgr](index.html) module
pub struct SECCFGR_SPEC;
impl crate::RegisterSpec for SECCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [seccfgr::R](R) reader structure
impl crate::Readable for SECCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [seccfgr::W](W) writer structure
impl crate::Writable for SECCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SECCFGR to value 0
impl crate::Resettable for SECCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
