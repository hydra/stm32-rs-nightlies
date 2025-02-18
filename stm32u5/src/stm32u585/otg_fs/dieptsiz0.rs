///Register `DIEPTSIZ0` reader
pub struct R(crate::R<DIEPTSIZ0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPTSIZ0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPTSIZ0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPTSIZ0_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DIEPTSIZ0` writer
pub struct W(crate::W<DIEPTSIZ0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPTSIZ0_SPEC>;
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
impl From<crate::W<DIEPTSIZ0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPTSIZ0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `XFRSIZ` reader - XFRSIZ
pub type XFRSIZ_R = crate::FieldReader<u8, u8>;
///Field `XFRSIZ` writer - XFRSIZ
pub type XFRSIZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEPTSIZ0_SPEC, u8, u8, 7, O>;
///Field `PKTCNT` reader - PKTCNT
pub type PKTCNT_R = crate::FieldReader<u8, u8>;
///Field `PKTCNT` writer - PKTCNT
pub type PKTCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEPTSIZ0_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:6 - XFRSIZ
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 19:20 - PKTCNT
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 3) as u8)
    }
}
impl W {
    ///Bits 0:6 - XFRSIZ
    #[inline(always)]
    #[must_use]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<0> {
        XFRSIZ_W::new(self)
    }
    ///Bits 19:20 - PKTCNT
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<19> {
        PKTCNT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///The application must modify this register before enabling endpoint 0.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dieptsiz0](index.html) module
pub struct DIEPTSIZ0_SPEC;
impl crate::RegisterSpec for DIEPTSIZ0_SPEC {
    type Ux = u32;
}
///`read()` method returns [dieptsiz0::R](R) reader structure
impl crate::Readable for DIEPTSIZ0_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dieptsiz0::W](W) writer structure
impl crate::Writable for DIEPTSIZ0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DIEPTSIZ0 to value 0
impl crate::Resettable for DIEPTSIZ0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
