///Register `AUTOCR` reader
pub struct R(crate::R<AUTOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTOCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AUTOCR` writer
pub struct W(crate::W<AUTOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUTOCR_SPEC>;
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
impl From<crate::W<AUTOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUTOCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TDN` reader - TDN
pub type TDN_R = crate::FieldReader<u16, u16>;
///Field `TDN` writer - TDN
pub type TDN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AUTOCR_SPEC, u16, u16, 16, O>;
///Field `TRIGPOL` reader - TRIGPOL
pub type TRIGPOL_R = crate::BitReader<bool>;
///Field `TRIGPOL` writer - TRIGPOL
pub type TRIGPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUTOCR_SPEC, bool, O>;
///Field `TRIGEN` reader - TRIGEN
pub type TRIGEN_R = crate::BitReader<bool>;
///Field `TRIGEN` writer - TRIGEN
pub type TRIGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUTOCR_SPEC, bool, O>;
///Field `IDLEDIS` reader - IDLEDIS
pub type IDLEDIS_R = crate::BitReader<bool>;
///Field `IDLEDIS` writer - IDLEDIS
pub type IDLEDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUTOCR_SPEC, bool, O>;
///Field `TRIGSEL` reader - TRIGSEL
pub type TRIGSEL_R = crate::FieldReader<u8, u8>;
///Field `TRIGSEL` writer - TRIGSEL
pub type TRIGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AUTOCR_SPEC, u8, u8, 4, O>;
///Field `TECLREN` reader - TECLREN
pub type TECLREN_R = crate::BitReader<bool>;
///Field `TECLREN` writer - TECLREN
pub type TECLREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUTOCR_SPEC, bool, O>;
impl R {
    ///Bits 0:15 - TDN
    #[inline(always)]
    pub fn tdn(&self) -> TDN_R {
        TDN_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - TRIGPOL
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TRIGEN
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - IDLEDIS
    #[inline(always)]
    pub fn idledis(&self) -> IDLEDIS_R {
        IDLEDIS_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:22 - TRIGSEL
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    ///Bit 31 - TECLREN
    #[inline(always)]
    pub fn teclren(&self) -> TECLREN_R {
        TECLREN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:15 - TDN
    #[inline(always)]
    #[must_use]
    pub fn tdn(&mut self) -> TDN_W<0> {
        TDN_W::new(self)
    }
    ///Bit 16 - TRIGPOL
    #[inline(always)]
    #[must_use]
    pub fn trigpol(&mut self) -> TRIGPOL_W<16> {
        TRIGPOL_W::new(self)
    }
    ///Bit 17 - TRIGEN
    #[inline(always)]
    #[must_use]
    pub fn trigen(&mut self) -> TRIGEN_W<17> {
        TRIGEN_W::new(self)
    }
    ///Bit 18 - IDLEDIS
    #[inline(always)]
    #[must_use]
    pub fn idledis(&mut self) -> IDLEDIS_W<18> {
        IDLEDIS_W::new(self)
    }
    ///Bits 19:22 - TRIGSEL
    #[inline(always)]
    #[must_use]
    pub fn trigsel(&mut self) -> TRIGSEL_W<19> {
        TRIGSEL_W::new(self)
    }
    ///Bit 31 - TECLREN
    #[inline(always)]
    #[must_use]
    pub fn teclren(&mut self) -> TECLREN_W<31> {
        TECLREN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Autonomous mode control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [autocr](index.html) module
pub struct AUTOCR_SPEC;
impl crate::RegisterSpec for AUTOCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [autocr::R](R) reader structure
impl crate::Readable for AUTOCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [autocr::W](W) writer structure
impl crate::Writable for AUTOCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AUTOCR to value 0x8000_0000
impl crate::Resettable for AUTOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
