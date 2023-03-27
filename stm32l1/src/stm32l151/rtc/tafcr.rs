///Register `TAFCR` reader
pub struct R(crate::R<TAFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAFCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TAFCR` writer
pub struct W(crate::W<TAFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAFCR_SPEC>;
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
impl From<crate::W<TAFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAFCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TAMP1E` reader - Tamper 1 detection enable
pub type TAMP1E_R = crate::BitReader<bool>;
///Field `TAMP1E` writer - Tamper 1 detection enable
pub type TAMP1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
///Field `TAMP1ETRG` reader - Active level for tamper 1
pub type TAMP1ETRG_R = crate::BitReader<bool>;
///Field `TAMP1ETRG` writer - Active level for tamper 1
pub type TAMP1ETRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
///Field `TAMPIE` reader - Tamper interrupt enable
pub type TAMPIE_R = crate::BitReader<bool>;
///Field `TAMPIE` writer - Tamper interrupt enable
pub type TAMPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
///Field `TAMP2E` reader - Tamper 2 detection enable
pub type TAMP2E_R = crate::BitReader<bool>;
///Field `TAMP2E` writer - Tamper 2 detection enable
pub type TAMP2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
///Field `TAMP2TRG` reader - Active level for tamper 2
pub type TAMP2TRG_R = crate::BitReader<bool>;
///Field `TAMP2TRG` writer - Active level for tamper 2
pub type TAMP2TRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
///Field `TAMP3E` reader - TIMESTAMP mapping
pub type TAMP3E_R = crate::BitReader<bool>;
///Field `TAMP3E` writer - TIMESTAMP mapping
pub type TAMP3E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
///Field `TAMP3TRG` reader - TAMPER1 mapping
pub type TAMP3TRG_R = crate::BitReader<bool>;
///Field `TAMP3TRG` writer - TAMPER1 mapping
pub type TAMP3TRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
///Field `TAMPTS` reader - Activate timestamp on tamper detection event
pub type TAMPTS_R = crate::BitReader<bool>;
///Field `TAMPTS` writer - Activate timestamp on tamper detection event
pub type TAMPTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
///Field `TAMPFREQ` reader - Tamper sampling frequency
pub type TAMPFREQ_R = crate::FieldReader<u8, u8>;
///Field `TAMPFREQ` writer - Tamper sampling frequency
pub type TAMPFREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAFCR_SPEC, u8, u8, 3, O>;
///Field `TAMPFLT` reader - Tamper filter count
pub type TAMPFLT_R = crate::FieldReader<u8, u8>;
///Field `TAMPFLT` writer - Tamper filter count
pub type TAMPFLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAFCR_SPEC, u8, u8, 2, O>;
///Field `TAMPPRCH` reader - Tamper precharge duration
pub type TAMPPRCH_R = crate::FieldReader<u8, u8>;
///Field `TAMPPRCH` writer - Tamper precharge duration
pub type TAMPPRCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAFCR_SPEC, u8, u8, 2, O>;
///Field `TAMPPUDIS` reader - TAMPER pull-up disable
pub type TAMPPUDIS_R = crate::BitReader<bool>;
///Field `TAMPPUDIS` writer - TAMPER pull-up disable
pub type TAMPPUDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
///Field `ALARMOUTTYPE` reader - AFO_ALARM output type
pub type ALARMOUTTYPE_R = crate::BitReader<bool>;
///Field `ALARMOUTTYPE` writer - AFO_ALARM output type
pub type ALARMOUTTYPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Tamper 1 detection enable
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Active level for tamper 1
    #[inline(always)]
    pub fn tamp1etrg(&self) -> TAMP1ETRG_R {
        TAMP1ETRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Tamper interrupt enable
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Tamper 2 detection enable
    #[inline(always)]
    pub fn tamp2e(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Active level for tamper 2
    #[inline(always)]
    pub fn tamp2trg(&self) -> TAMP2TRG_R {
        TAMP2TRG_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIMESTAMP mapping
    #[inline(always)]
    pub fn tamp3e(&self) -> TAMP3E_R {
        TAMP3E_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TAMPER1 mapping
    #[inline(always)]
    pub fn tamp3trg(&self) -> TAMP3TRG_R {
        TAMP3TRG_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Activate timestamp on tamper detection event
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:10 - Tamper sampling frequency
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:12 - Tamper filter count
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bits 13:14 - Tamper precharge duration
    #[inline(always)]
    pub fn tampprch(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - TAMPER pull-up disable
    #[inline(always)]
    pub fn tamppudis(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 18 - AFO_ALARM output type
    #[inline(always)]
    pub fn alarmouttype(&self) -> ALARMOUTTYPE_R {
        ALARMOUTTYPE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Tamper 1 detection enable
    #[inline(always)]
    #[must_use]
    pub fn tamp1e(&mut self) -> TAMP1E_W<0> {
        TAMP1E_W::new(self)
    }
    ///Bit 1 - Active level for tamper 1
    #[inline(always)]
    #[must_use]
    pub fn tamp1etrg(&mut self) -> TAMP1ETRG_W<1> {
        TAMP1ETRG_W::new(self)
    }
    ///Bit 2 - Tamper interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tampie(&mut self) -> TAMPIE_W<2> {
        TAMPIE_W::new(self)
    }
    ///Bit 3 - Tamper 2 detection enable
    #[inline(always)]
    #[must_use]
    pub fn tamp2e(&mut self) -> TAMP2E_W<3> {
        TAMP2E_W::new(self)
    }
    ///Bit 4 - Active level for tamper 2
    #[inline(always)]
    #[must_use]
    pub fn tamp2trg(&mut self) -> TAMP2TRG_W<4> {
        TAMP2TRG_W::new(self)
    }
    ///Bit 5 - TIMESTAMP mapping
    #[inline(always)]
    #[must_use]
    pub fn tamp3e(&mut self) -> TAMP3E_W<5> {
        TAMP3E_W::new(self)
    }
    ///Bit 6 - TAMPER1 mapping
    #[inline(always)]
    #[must_use]
    pub fn tamp3trg(&mut self) -> TAMP3TRG_W<6> {
        TAMP3TRG_W::new(self)
    }
    ///Bit 7 - Activate timestamp on tamper detection event
    #[inline(always)]
    #[must_use]
    pub fn tampts(&mut self) -> TAMPTS_W<7> {
        TAMPTS_W::new(self)
    }
    ///Bits 8:10 - Tamper sampling frequency
    #[inline(always)]
    #[must_use]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W<8> {
        TAMPFREQ_W::new(self)
    }
    ///Bits 11:12 - Tamper filter count
    #[inline(always)]
    #[must_use]
    pub fn tampflt(&mut self) -> TAMPFLT_W<11> {
        TAMPFLT_W::new(self)
    }
    ///Bits 13:14 - Tamper precharge duration
    #[inline(always)]
    #[must_use]
    pub fn tampprch(&mut self) -> TAMPPRCH_W<13> {
        TAMPPRCH_W::new(self)
    }
    ///Bit 15 - TAMPER pull-up disable
    #[inline(always)]
    #[must_use]
    pub fn tamppudis(&mut self) -> TAMPPUDIS_W<15> {
        TAMPPUDIS_W::new(self)
    }
    ///Bit 18 - AFO_ALARM output type
    #[inline(always)]
    #[must_use]
    pub fn alarmouttype(&mut self) -> ALARMOUTTYPE_W<18> {
        ALARMOUTTYPE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///tamper and alternate function configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tafcr](index.html) module
pub struct TAFCR_SPEC;
impl crate::RegisterSpec for TAFCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tafcr::R](R) reader structure
impl crate::Readable for TAFCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tafcr::W](W) writer structure
impl crate::Writable for TAFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TAFCR to value 0
impl crate::Resettable for TAFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
