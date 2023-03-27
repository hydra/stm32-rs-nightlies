///Register `WCR` reader
pub struct R(crate::R<WCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `WCR` writer
pub struct W(crate::W<WCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WCR_SPEC>;
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
impl From<crate::W<WCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `COLM` reader - COLM
pub type COLM_R = crate::BitReader<bool>;
///Field `COLM` writer - COLM
pub type COLM_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCR_SPEC, bool, O>;
///Field `SHTDN` reader - SHTDN
pub type SHTDN_R = crate::BitReader<bool>;
///Field `SHTDN` writer - SHTDN
pub type SHTDN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCR_SPEC, bool, O>;
///Field `LTDCEN` reader - LTDCEN
pub type LTDCEN_R = crate::BitReader<bool>;
///Field `LTDCEN` writer - LTDCEN
pub type LTDCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCR_SPEC, bool, O>;
///Field `DSIEN` reader - DSIEN
pub type DSIEN_R = crate::BitReader<bool>;
///Field `DSIEN` writer - DSIEN
pub type DSIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - COLM
    #[inline(always)]
    pub fn colm(&self) -> COLM_R {
        COLM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SHTDN
    #[inline(always)]
    pub fn shtdn(&self) -> SHTDN_R {
        SHTDN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LTDCEN
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DSIEN
    #[inline(always)]
    pub fn dsien(&self) -> DSIEN_R {
        DSIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - COLM
    #[inline(always)]
    #[must_use]
    pub fn colm(&mut self) -> COLM_W<0> {
        COLM_W::new(self)
    }
    ///Bit 1 - SHTDN
    #[inline(always)]
    #[must_use]
    pub fn shtdn(&mut self) -> SHTDN_W<1> {
        SHTDN_W::new(self)
    }
    ///Bit 2 - LTDCEN
    #[inline(always)]
    #[must_use]
    pub fn ltdcen(&mut self) -> LTDCEN_W<2> {
        LTDCEN_W::new(self)
    }
    ///Bit 3 - DSIEN
    #[inline(always)]
    #[must_use]
    pub fn dsien(&mut self) -> DSIEN_W<3> {
        DSIEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI wrapper control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wcr](index.html) module
pub struct WCR_SPEC;
impl crate::RegisterSpec for WCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [wcr::R](R) reader structure
impl crate::Readable for WCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [wcr::W](W) writer structure
impl crate::Writable for WCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets WCR to value 0
impl crate::Resettable for WCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
