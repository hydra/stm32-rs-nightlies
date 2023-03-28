///Register `PCTLR` reader
pub struct R(crate::R<PCTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCTLR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PCTLR` writer
pub struct W(crate::W<PCTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCTLR_SPEC>;
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
impl From<crate::W<PCTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCTLR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DEN` reader - Digital Enable
pub type DEN_R = crate::BitReader<bool>;
///Field `DEN` writer - Digital Enable
pub type DEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCTLR_SPEC, bool, O>;
///Field `CKE` reader - Clock Enable
pub type CKE_R = crate::BitReader<bool>;
///Field `CKE` writer - Clock Enable
pub type CKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCTLR_SPEC, bool, O>;
impl R {
    ///Bit 1 - Digital Enable
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clock Enable
    #[inline(always)]
    pub fn cke(&self) -> CKE_R {
        CKE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - Digital Enable
    #[inline(always)]
    #[must_use]
    pub fn den(&mut self) -> DEN_W<1> {
        DEN_W::new(self)
    }
    ///Bit 2 - Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn cke(&mut self) -> CKE_W<2> {
        CKE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host PHY Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pctlr](index.html) module
pub struct PCTLR_SPEC;
impl crate::RegisterSpec for PCTLR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pctlr::R](R) reader structure
impl crate::Readable for PCTLR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pctlr::W](W) writer structure
impl crate::Writable for PCTLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PCTLR to value 0
impl crate::Resettable for PCTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
