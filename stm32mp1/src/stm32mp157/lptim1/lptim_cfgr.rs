///Register `LPTIM_CFGR` reader
pub struct R(crate::R<LPTIM_CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM_CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM_CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM_CFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LPTIM_CFGR` writer
pub struct W(crate::W<LPTIM_CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPTIM_CFGR_SPEC>;
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
impl From<crate::W<LPTIM_CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPTIM_CFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CKSEL` reader - CKSEL
pub type CKSEL_R = crate::BitReader<bool>;
///Field `CKSEL` writer - CKSEL
pub type CKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_CFGR_SPEC, bool, O>;
///Field `CKPOL` reader - CKPOL
pub type CKPOL_R = crate::FieldReader<u8, u8>;
///Field `CKPOL` writer - CKPOL
pub type CKPOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTIM_CFGR_SPEC, u8, u8, 2, O>;
///Field `CKFLT` reader - CKFLT
pub type CKFLT_R = crate::FieldReader<u8, u8>;
///Field `CKFLT` writer - CKFLT
pub type CKFLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTIM_CFGR_SPEC, u8, u8, 2, O>;
///Field `TRGFLT` reader - TRGFLT
pub type TRGFLT_R = crate::FieldReader<u8, u8>;
///Field `TRGFLT` writer - TRGFLT
pub type TRGFLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTIM_CFGR_SPEC, u8, u8, 2, O>;
///Field `PRESC` reader - PRESC
pub type PRESC_R = crate::FieldReader<u8, u8>;
///Field `PRESC` writer - PRESC
pub type PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTIM_CFGR_SPEC, u8, u8, 3, O>;
///Field `TRIGSEL` reader - TRIGSEL
pub type TRIGSEL_R = crate::FieldReader<u8, u8>;
///Field `TRIGSEL` writer - TRIGSEL
pub type TRIGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTIM_CFGR_SPEC, u8, u8, 3, O>;
///Field `TRIGEN` reader - TRIGEN
pub type TRIGEN_R = crate::FieldReader<u8, u8>;
///Field `TRIGEN` writer - TRIGEN
pub type TRIGEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTIM_CFGR_SPEC, u8, u8, 2, O>;
///Field `TIMOUT` reader - TIMOUT
pub type TIMOUT_R = crate::BitReader<bool>;
///Field `TIMOUT` writer - TIMOUT
pub type TIMOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_CFGR_SPEC, bool, O>;
///Field `WAVE` reader - WAVE
pub type WAVE_R = crate::BitReader<bool>;
///Field `WAVE` writer - WAVE
pub type WAVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_CFGR_SPEC, bool, O>;
///Field `WAVPOL` reader - WAVPOL
pub type WAVPOL_R = crate::BitReader<bool>;
///Field `WAVPOL` writer - WAVPOL
pub type WAVPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_CFGR_SPEC, bool, O>;
///Field `PRELOAD` reader - PRELOAD
pub type PRELOAD_R = crate::BitReader<bool>;
///Field `PRELOAD` writer - PRELOAD
pub type PRELOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_CFGR_SPEC, bool, O>;
///Field `COUNTMODE` reader - COUNTMODE
pub type COUNTMODE_R = crate::BitReader<bool>;
///Field `COUNTMODE` writer - COUNTMODE
pub type COUNTMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_CFGR_SPEC, bool, O>;
///Field `ENC` reader - ENC
pub type ENC_R = crate::BitReader<bool>;
///Field `ENC` writer - ENC
pub type ENC_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_CFGR_SPEC, bool, O>;
impl R {
    ///Bit 0 - CKSEL
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - CKPOL
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - CKFLT
    #[inline(always)]
    pub fn ckflt(&self) -> CKFLT_R {
        CKFLT_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 6:7 - TRGFLT
    #[inline(always)]
    pub fn trgflt(&self) -> TRGFLT_R {
        TRGFLT_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 9:11 - PRESC
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 13:15 - TRIGSEL
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 17:18 - TRIGEN
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bit 19 - TIMOUT
    #[inline(always)]
    pub fn timout(&self) -> TIMOUT_R {
        TIMOUT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - WAVE
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - WAVPOL
    #[inline(always)]
    pub fn wavpol(&self) -> WAVPOL_R {
        WAVPOL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - PRELOAD
    #[inline(always)]
    pub fn preload(&self) -> PRELOAD_R {
        PRELOAD_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - COUNTMODE
    #[inline(always)]
    pub fn countmode(&self) -> COUNTMODE_R {
        COUNTMODE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - ENC
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CKSEL
    #[inline(always)]
    #[must_use]
    pub fn cksel(&mut self) -> CKSEL_W<0> {
        CKSEL_W::new(self)
    }
    ///Bits 1:2 - CKPOL
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CKPOL_W<1> {
        CKPOL_W::new(self)
    }
    ///Bits 3:4 - CKFLT
    #[inline(always)]
    #[must_use]
    pub fn ckflt(&mut self) -> CKFLT_W<3> {
        CKFLT_W::new(self)
    }
    ///Bits 6:7 - TRGFLT
    #[inline(always)]
    #[must_use]
    pub fn trgflt(&mut self) -> TRGFLT_W<6> {
        TRGFLT_W::new(self)
    }
    ///Bits 9:11 - PRESC
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<9> {
        PRESC_W::new(self)
    }
    ///Bits 13:15 - TRIGSEL
    #[inline(always)]
    #[must_use]
    pub fn trigsel(&mut self) -> TRIGSEL_W<13> {
        TRIGSEL_W::new(self)
    }
    ///Bits 17:18 - TRIGEN
    #[inline(always)]
    #[must_use]
    pub fn trigen(&mut self) -> TRIGEN_W<17> {
        TRIGEN_W::new(self)
    }
    ///Bit 19 - TIMOUT
    #[inline(always)]
    #[must_use]
    pub fn timout(&mut self) -> TIMOUT_W<19> {
        TIMOUT_W::new(self)
    }
    ///Bit 20 - WAVE
    #[inline(always)]
    #[must_use]
    pub fn wave(&mut self) -> WAVE_W<20> {
        WAVE_W::new(self)
    }
    ///Bit 21 - WAVPOL
    #[inline(always)]
    #[must_use]
    pub fn wavpol(&mut self) -> WAVPOL_W<21> {
        WAVPOL_W::new(self)
    }
    ///Bit 22 - PRELOAD
    #[inline(always)]
    #[must_use]
    pub fn preload(&mut self) -> PRELOAD_W<22> {
        PRELOAD_W::new(self)
    }
    ///Bit 23 - COUNTMODE
    #[inline(always)]
    #[must_use]
    pub fn countmode(&mut self) -> COUNTMODE_W<23> {
        COUNTMODE_W::new(self)
    }
    ///Bit 24 - ENC
    #[inline(always)]
    #[must_use]
    pub fn enc(&mut self) -> ENC_W<24> {
        ENC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LPTIM configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lptim_cfgr](index.html) module
pub struct LPTIM_CFGR_SPEC;
impl crate::RegisterSpec for LPTIM_CFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lptim_cfgr::R](R) reader structure
impl crate::Readable for LPTIM_CFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lptim_cfgr::W](W) writer structure
impl crate::Writable for LPTIM_CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LPTIM_CFGR to value 0
impl crate::Resettable for LPTIM_CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
