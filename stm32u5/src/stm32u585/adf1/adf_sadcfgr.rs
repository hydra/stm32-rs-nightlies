///Register `ADF_SADCFGR` reader
pub struct R(crate::R<ADF_SADCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADF_SADCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADF_SADCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADF_SADCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADF_SADCFGR` writer
pub struct W(crate::W<ADF_SADCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADF_SADCFGR_SPEC>;
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
impl From<crate::W<ADF_SADCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADF_SADCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SNTHR` reader - SNTHR
pub type SNTHR_R = crate::FieldReader<u8, u8>;
///Field `SNTHR` writer - SNTHR
pub type SNTHR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADF_SADCFGR_SPEC, u8, u8, 4, O>;
///Field `ANSLP` reader - ANSLP
pub type ANSLP_R = crate::FieldReader<u8, u8>;
///Field `ANSLP` writer - ANSLP
pub type ANSLP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADF_SADCFGR_SPEC, u8, u8, 3, O>;
///Field `LFRNB` reader - LFRNB
pub type LFRNB_R = crate::FieldReader<u8, u8>;
///Field `LFRNB` writer - LFRNB
pub type LFRNB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADF_SADCFGR_SPEC, u8, u8, 3, O>;
///Field `HGOVR` reader - Hangover time window
pub type HGOVR_R = crate::FieldReader<u8, u8>;
///Field `HGOVR` writer - Hangover time window
pub type HGOVR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADF_SADCFGR_SPEC, u8, u8, 3, O>;
///Field `ANMIN` reader - ANMIN
pub type ANMIN_R = crate::FieldReader<u16, u16>;
///Field `ANMIN` writer - ANMIN
pub type ANMIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADF_SADCFGR_SPEC, u16, u16, 13, O>;
impl R {
    ///Bits 0:3 - SNTHR
    #[inline(always)]
    pub fn snthr(&self) -> SNTHR_R {
        SNTHR_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6 - ANSLP
    #[inline(always)]
    pub fn anslp(&self) -> ANSLP_R {
        ANSLP_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - LFRNB
    #[inline(always)]
    pub fn lfrnb(&self) -> LFRNB_R {
        LFRNB_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - Hangover time window
    #[inline(always)]
    pub fn hgovr(&self) -> HGOVR_R {
        HGOVR_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:28 - ANMIN
    #[inline(always)]
    pub fn anmin(&self) -> ANMIN_R {
        ANMIN_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    ///Bits 0:3 - SNTHR
    #[inline(always)]
    #[must_use]
    pub fn snthr(&mut self) -> SNTHR_W<0> {
        SNTHR_W::new(self)
    }
    ///Bits 4:6 - ANSLP
    #[inline(always)]
    #[must_use]
    pub fn anslp(&mut self) -> ANSLP_W<4> {
        ANSLP_W::new(self)
    }
    ///Bits 8:10 - LFRNB
    #[inline(always)]
    #[must_use]
    pub fn lfrnb(&mut self) -> LFRNB_W<8> {
        LFRNB_W::new(self)
    }
    ///Bits 12:14 - Hangover time window
    #[inline(always)]
    #[must_use]
    pub fn hgovr(&mut self) -> HGOVR_W<12> {
        HGOVR_W::new(self)
    }
    ///Bits 16:28 - ANMIN
    #[inline(always)]
    #[must_use]
    pub fn anmin(&mut self) -> ANMIN_W<16> {
        ANMIN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADF SAD configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adf_sadcfgr](index.html) module
pub struct ADF_SADCFGR_SPEC;
impl crate::RegisterSpec for ADF_SADCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [adf_sadcfgr::R](R) reader structure
impl crate::Readable for ADF_SADCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adf_sadcfgr::W](W) writer structure
impl crate::Writable for ADF_SADCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADF_SADCFGR to value 0
impl crate::Resettable for ADF_SADCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
