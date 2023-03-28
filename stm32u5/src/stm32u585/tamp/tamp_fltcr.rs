///Register `TAMP_FLTCR` reader
pub struct R(crate::R<TAMP_FLTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMP_FLTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMP_FLTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMP_FLTCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TAMP_FLTCR` writer
pub struct W(crate::W<TAMP_FLTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMP_FLTCR_SPEC>;
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
impl From<crate::W<TAMP_FLTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMP_FLTCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TAMPFREQ` reader - Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled.
pub type TAMPFREQ_R = crate::FieldReader<u8, u8>;
///Field `TAMPFREQ` writer - Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled.
pub type TAMPFREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_FLTCR_SPEC, u8, u8, 3, O>;
///Field `TAMPFLT` reader - TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs.
pub type TAMPFLT_R = crate::FieldReader<u8, u8>;
///Field `TAMPFLT` writer - TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs.
pub type TAMPFLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_FLTCR_SPEC, u8, u8, 2, O>;
///Field `TAMPPRCH` reader - TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs.
pub type TAMPPRCH_R = crate::FieldReader<u8, u8>;
///Field `TAMPPRCH` writer - TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs.
pub type TAMPPRCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_FLTCR_SPEC, u8, u8, 2, O>;
///Field `TAMPPUDIS` reader - TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample.
pub type TAMPPUDIS_R = crate::BitReader<bool>;
///Field `TAMPPUDIS` writer - TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample.
pub type TAMPPUDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_FLTCR_SPEC, bool, O>;
impl R {
    ///Bits 0:2 - Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled.
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:4 - TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs.
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6 - TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs.
    #[inline(always)]
    pub fn tampprch(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample.
    #[inline(always)]
    pub fn tamppudis(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled.
    #[inline(always)]
    #[must_use]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W<0> {
        TAMPFREQ_W::new(self)
    }
    ///Bits 3:4 - TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs.
    #[inline(always)]
    #[must_use]
    pub fn tampflt(&mut self) -> TAMPFLT_W<3> {
        TAMPFLT_W::new(self)
    }
    ///Bits 5:6 - TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs.
    #[inline(always)]
    #[must_use]
    pub fn tampprch(&mut self) -> TAMPPRCH_W<5> {
        TAMPPRCH_W::new(self)
    }
    ///Bit 7 - TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample.
    #[inline(always)]
    #[must_use]
    pub fn tamppudis(&mut self) -> TAMPPUDIS_W<7> {
        TAMPPUDIS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TAMP filter control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tamp_fltcr](index.html) module
pub struct TAMP_FLTCR_SPEC;
impl crate::RegisterSpec for TAMP_FLTCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tamp_fltcr::R](R) reader structure
impl crate::Readable for TAMP_FLTCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tamp_fltcr::W](W) writer structure
impl crate::Writable for TAMP_FLTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TAMP_FLTCR to value 0
impl crate::Resettable for TAMP_FLTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
