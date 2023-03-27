///Register `WCFGR` reader
pub struct R(crate::R<WCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `WCFGR` writer
pub struct W(crate::W<WCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WCFGR_SPEC>;
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
impl From<crate::W<WCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DSIM` reader - DSI mode
pub type DSIM_R = crate::BitReader<bool>;
///Field `DSIM` writer - DSI mode
pub type DSIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCFGR_SPEC, bool, O>;
///Field `COLMUX` reader - Color multiplexing
pub type COLMUX_R = crate::FieldReader<u8, u8>;
///Field `COLMUX` writer - Color multiplexing
pub type COLMUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WCFGR_SPEC, u8, u8, 3, O>;
///Field `TESRC` reader - TE source
pub type TESRC_R = crate::BitReader<bool>;
///Field `TESRC` writer - TE source
pub type TESRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCFGR_SPEC, bool, O>;
///Field `TEPOL` reader - TE polarity
pub type TEPOL_R = crate::BitReader<bool>;
///Field `TEPOL` writer - TE polarity
pub type TEPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCFGR_SPEC, bool, O>;
///Field `AR` reader - Automatic refresh
pub type AR_R = crate::BitReader<bool>;
///Field `AR` writer - Automatic refresh
pub type AR_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCFGR_SPEC, bool, O>;
///Field `VSPOL` reader - VSync polarity
pub type VSPOL_R = crate::BitReader<bool>;
///Field `VSPOL` writer - VSync polarity
pub type VSPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCFGR_SPEC, bool, O>;
impl R {
    ///Bit 0 - DSI mode
    #[inline(always)]
    pub fn dsim(&self) -> DSIM_R {
        DSIM_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - Color multiplexing
    #[inline(always)]
    pub fn colmux(&self) -> COLMUX_R {
        COLMUX_R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bit 4 - TE source
    #[inline(always)]
    pub fn tesrc(&self) -> TESRC_R {
        TESRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TE polarity
    #[inline(always)]
    pub fn tepol(&self) -> TEPOL_R {
        TEPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Automatic refresh
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - VSync polarity
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DSI mode
    #[inline(always)]
    #[must_use]
    pub fn dsim(&mut self) -> DSIM_W<0> {
        DSIM_W::new(self)
    }
    ///Bits 1:3 - Color multiplexing
    #[inline(always)]
    #[must_use]
    pub fn colmux(&mut self) -> COLMUX_W<1> {
        COLMUX_W::new(self)
    }
    ///Bit 4 - TE source
    #[inline(always)]
    #[must_use]
    pub fn tesrc(&mut self) -> TESRC_W<4> {
        TESRC_W::new(self)
    }
    ///Bit 5 - TE polarity
    #[inline(always)]
    #[must_use]
    pub fn tepol(&mut self) -> TEPOL_W<5> {
        TEPOL_W::new(self)
    }
    ///Bit 6 - Automatic refresh
    #[inline(always)]
    #[must_use]
    pub fn ar(&mut self) -> AR_W<6> {
        AR_W::new(self)
    }
    ///Bit 7 - VSync polarity
    #[inline(always)]
    #[must_use]
    pub fn vspol(&mut self) -> VSPOL_W<7> {
        VSPOL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI wrapper configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wcfgr](index.html) module
pub struct WCFGR_SPEC;
impl crate::RegisterSpec for WCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [wcfgr::R](R) reader structure
impl crate::Readable for WCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [wcfgr::W](W) writer structure
impl crate::Writable for WCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets WCFGR to value 0
impl crate::Resettable for WCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
