///Register `LPCR` reader
pub struct R(crate::R<LPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LPCR` writer
pub struct W(crate::W<LPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPCR_SPEC>;
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
impl From<crate::W<LPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DEP` reader - DEP
pub type DEP_R = crate::BitReader<bool>;
///Field `DEP` writer - DEP
pub type DEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPCR_SPEC, bool, O>;
///Field `VSP` reader - VSP
pub type VSP_R = crate::BitReader<bool>;
///Field `VSP` writer - VSP
pub type VSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPCR_SPEC, bool, O>;
///Field `HSP` reader - HSP
pub type HSP_R = crate::BitReader<bool>;
///Field `HSP` writer - HSP
pub type HSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - DEP
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - VSP
    #[inline(always)]
    pub fn vsp(&self) -> VSP_R {
        VSP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSP
    #[inline(always)]
    pub fn hsp(&self) -> HSP_R {
        HSP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DEP
    #[inline(always)]
    #[must_use]
    pub fn dep(&mut self) -> DEP_W<0> {
        DEP_W::new(self)
    }
    ///Bit 1 - VSP
    #[inline(always)]
    #[must_use]
    pub fn vsp(&mut self) -> VSP_W<1> {
        VSP_W::new(self)
    }
    ///Bit 2 - HSP
    #[inline(always)]
    #[must_use]
    pub fn hsp(&mut self) -> HSP_W<2> {
        HSP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host LTDC polarity configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lpcr](index.html) module
pub struct LPCR_SPEC;
impl crate::RegisterSpec for LPCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lpcr::R](R) reader structure
impl crate::Readable for LPCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lpcr::W](W) writer structure
impl crate::Writable for LPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LPCR to value 0
impl crate::Resettable for LPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
