///Register `ADF_SITF0CR` reader
pub struct R(crate::R<ADF_SITF0CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADF_SITF0CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADF_SITF0CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADF_SITF0CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADF_SITF0CR` writer
pub struct W(crate::W<ADF_SITF0CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADF_SITF0CR_SPEC>;
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
impl From<crate::W<ADF_SITF0CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADF_SITF0CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SITFEN` reader - SITFEN
pub type SITFEN_R = crate::BitReader<bool>;
///Field `SITFEN` writer - SITFEN
pub type SITFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_SITF0CR_SPEC, bool, O>;
///Field `SCKSRC` reader - SCKSRC
pub type SCKSRC_R = crate::FieldReader<u8, u8>;
///Field `SCKSRC` writer - SCKSRC
pub type SCKSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADF_SITF0CR_SPEC, u8, u8, 2, O>;
///Field `SITFMOD` reader - SITFMOD
pub type SITFMOD_R = crate::FieldReader<u8, u8>;
///Field `SITFMOD` writer - SITFMOD
pub type SITFMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADF_SITF0CR_SPEC, u8, u8, 2, O>;
///Field `STH` reader - STH
pub type STH_R = crate::FieldReader<u8, u8>;
///Field `STH` writer - STH
pub type STH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADF_SITF0CR_SPEC, u8, u8, 5, O>;
///Field `SITFACTIVE` reader - SITFACTIVE
pub type SITFACTIVE_R = crate::BitReader<bool>;
///Field `SITFACTIVE` writer - SITFACTIVE
pub type SITFACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_SITF0CR_SPEC, bool, O>;
impl R {
    ///Bit 0 - SITFEN
    #[inline(always)]
    pub fn sitfen(&self) -> SITFEN_R {
        SITFEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - SCKSRC
    #[inline(always)]
    pub fn scksrc(&self) -> SCKSRC_R {
        SCKSRC_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 4:5 - SITFMOD
    #[inline(always)]
    pub fn sitfmod(&self) -> SITFMOD_R {
        SITFMOD_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:12 - STH
    #[inline(always)]
    pub fn sth(&self) -> STH_R {
        STH_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bit 31 - SITFACTIVE
    #[inline(always)]
    pub fn sitfactive(&self) -> SITFACTIVE_R {
        SITFACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SITFEN
    #[inline(always)]
    #[must_use]
    pub fn sitfen(&mut self) -> SITFEN_W<0> {
        SITFEN_W::new(self)
    }
    ///Bits 1:2 - SCKSRC
    #[inline(always)]
    #[must_use]
    pub fn scksrc(&mut self) -> SCKSRC_W<1> {
        SCKSRC_W::new(self)
    }
    ///Bits 4:5 - SITFMOD
    #[inline(always)]
    #[must_use]
    pub fn sitfmod(&mut self) -> SITFMOD_W<4> {
        SITFMOD_W::new(self)
    }
    ///Bits 8:12 - STH
    #[inline(always)]
    #[must_use]
    pub fn sth(&mut self) -> STH_W<8> {
        STH_W::new(self)
    }
    ///Bit 31 - SITFACTIVE
    #[inline(always)]
    #[must_use]
    pub fn sitfactive(&mut self) -> SITFACTIVE_W<31> {
        SITFACTIVE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADF serial interface control register 0
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adf_sitf0cr](index.html) module
pub struct ADF_SITF0CR_SPEC;
impl crate::RegisterSpec for ADF_SITF0CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [adf_sitf0cr::R](R) reader structure
impl crate::Readable for ADF_SITF0CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adf_sitf0cr::W](W) writer structure
impl crate::Writable for ADF_SITF0CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADF_SITF0CR to value 0x1f00
impl crate::Resettable for ADF_SITF0CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f00;
}
