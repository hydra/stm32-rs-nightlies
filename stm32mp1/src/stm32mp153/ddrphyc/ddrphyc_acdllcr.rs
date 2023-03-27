///Register `DDRPHYC_ACDLLCR` reader
pub struct R(crate::R<DDRPHYC_ACDLLCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_ACDLLCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_ACDLLCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_ACDLLCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DDRPHYC_ACDLLCR` writer
pub struct W(crate::W<DDRPHYC_ACDLLCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_ACDLLCR_SPEC>;
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
impl From<crate::W<DDRPHYC_ACDLLCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_ACDLLCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MFBDLY` reader - MFBDLY
pub type MFBDLY_R = crate::FieldReader<u8, u8>;
///Field `MFBDLY` writer - MFBDLY
pub type MFBDLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_ACDLLCR_SPEC, u8, u8, 3, O>;
///Field `MFWDLY` reader - MFWDLY
pub type MFWDLY_R = crate::FieldReader<u8, u8>;
///Field `MFWDLY` writer - MFWDLY
pub type MFWDLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_ACDLLCR_SPEC, u8, u8, 3, O>;
///Field `ATESTEN` reader - ATESTEN
pub type ATESTEN_R = crate::BitReader<bool>;
///Field `ATESTEN` writer - ATESTEN
pub type ATESTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_ACDLLCR_SPEC, bool, O>;
///Field `DLLSRST` reader - DLLSRST
pub type DLLSRST_R = crate::BitReader<bool>;
///Field `DLLSRST` writer - DLLSRST
pub type DLLSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_ACDLLCR_SPEC, bool, O>;
///Field `DLLDIS` reader - DLLDIS
pub type DLLDIS_R = crate::BitReader<bool>;
///Field `DLLDIS` writer - DLLDIS
pub type DLLDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_ACDLLCR_SPEC, bool, O>;
impl R {
    ///Bits 6:8 - MFBDLY
    #[inline(always)]
    pub fn mfbdly(&self) -> MFBDLY_R {
        MFBDLY_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - MFWDLY
    #[inline(always)]
    pub fn mfwdly(&self) -> MFWDLY_R {
        MFWDLY_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bit 18 - ATESTEN
    #[inline(always)]
    pub fn atesten(&self) -> ATESTEN_R {
        ATESTEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 30 - DLLSRST
    #[inline(always)]
    pub fn dllsrst(&self) -> DLLSRST_R {
        DLLSRST_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - DLLDIS
    #[inline(always)]
    pub fn dlldis(&self) -> DLLDIS_R {
        DLLDIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 6:8 - MFBDLY
    #[inline(always)]
    #[must_use]
    pub fn mfbdly(&mut self) -> MFBDLY_W<6> {
        MFBDLY_W::new(self)
    }
    ///Bits 9:11 - MFWDLY
    #[inline(always)]
    #[must_use]
    pub fn mfwdly(&mut self) -> MFWDLY_W<9> {
        MFWDLY_W::new(self)
    }
    ///Bit 18 - ATESTEN
    #[inline(always)]
    #[must_use]
    pub fn atesten(&mut self) -> ATESTEN_W<18> {
        ATESTEN_W::new(self)
    }
    ///Bit 30 - DLLSRST
    #[inline(always)]
    #[must_use]
    pub fn dllsrst(&mut self) -> DLLSRST_W<30> {
        DLLSRST_W::new(self)
    }
    ///Bit 31 - DLLDIS
    #[inline(always)]
    #[must_use]
    pub fn dlldis(&mut self) -> DLLDIS_W<31> {
        DLLDIS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DDRPHYC AC DLL control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrphyc_acdllcr](index.html) module
pub struct DDRPHYC_ACDLLCR_SPEC;
impl crate::RegisterSpec for DDRPHYC_ACDLLCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrphyc_acdllcr::R](R) reader structure
impl crate::Readable for DDRPHYC_ACDLLCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ddrphyc_acdllcr::W](W) writer structure
impl crate::Writable for DDRPHYC_ACDLLCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DDRPHYC_ACDLLCR to value 0x4000_0000
impl crate::Resettable for DDRPHYC_ACDLLCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0000;
}
