///Register `CLRFR` writer
pub struct W(crate::W<CLRFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLRFR_SPEC>;
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
impl From<crate::W<CLRFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLRFR_SPEC>) -> Self {
        W(writer)
    }
}
///Clear PKA End of Operation flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROCENDFC_AW {
    ///1: Clear PROCENDF flag
    Clear = 1,
}
impl From<PROCENDFC_AW> for bool {
    #[inline(always)]
    fn from(variant: PROCENDFC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `PROCENDFC` writer - Clear PKA End of Operation flag
pub type PROCENDFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, PROCENDFC_AW, O>;
impl<'a, const O: u8> PROCENDFC_W<'a, O> {
    ///Clear PROCENDF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PROCENDFC_AW::Clear)
    }
}
///Clear PKA RAM error flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMERRFC_AW {
    ///1: Clear RAMERRF flag
    Clear = 1,
}
impl From<RAMERRFC_AW> for bool {
    #[inline(always)]
    fn from(variant: RAMERRFC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `RAMERRFC` writer - Clear PKA RAM error flag
pub type RAMERRFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, RAMERRFC_AW, O>;
impl<'a, const O: u8> RAMERRFC_W<'a, O> {
    ///Clear RAMERRF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RAMERRFC_AW::Clear)
    }
}
///Clear Address error flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRERRFC_AW {
    ///1: Clear ADDRERRF flag
    Clear = 1,
}
impl From<ADDRERRFC_AW> for bool {
    #[inline(always)]
    fn from(variant: ADDRERRFC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDRERRFC` writer - Clear Address error flag
pub type ADDRERRFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, ADDRERRFC_AW, O>;
impl<'a, const O: u8> ADDRERRFC_W<'a, O> {
    ///Clear ADDRERRF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ADDRERRFC_AW::Clear)
    }
}
impl W {
    ///Bit 17 - Clear PKA End of Operation flag
    #[inline(always)]
    #[must_use]
    pub fn procendfc(&mut self) -> PROCENDFC_W<17> {
        PROCENDFC_W::new(self)
    }
    ///Bit 19 - Clear PKA RAM error flag
    #[inline(always)]
    #[must_use]
    pub fn ramerrfc(&mut self) -> RAMERRFC_W<19> {
        RAMERRFC_W::new(self)
    }
    ///Bit 20 - Clear Address error flag
    #[inline(always)]
    #[must_use]
    pub fn addrerrfc(&mut self) -> ADDRERRFC_W<20> {
        ADDRERRFC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///clear flag register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [clrfr](index.html) module
pub struct CLRFR_SPEC;
impl crate::RegisterSpec for CLRFR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [clrfr::W](W) writer structure
impl crate::Writable for CLRFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CLRFR to value 0
impl crate::Resettable for CLRFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
