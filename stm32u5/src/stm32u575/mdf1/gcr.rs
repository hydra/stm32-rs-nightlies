///Register `GCR` reader
pub struct R(crate::R<GCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GCR` writer
pub struct W(crate::W<GCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCR_SPEC>;
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
impl From<crate::W<GCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TRGO` reader - TRGO
pub type TRGO_R = crate::BitReader<bool>;
///Field `TRGO` writer - TRGO
pub type TRGO_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
///Field `ILVNB` reader - ILVNB
pub type ILVNB_R = crate::FieldReader<u8, u8>;
///Field `ILVNB` writer - ILVNB
pub type ILVNB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCR_SPEC, u8, u8, 4, O>;
impl R {
    ///Bit 0 - TRGO
    #[inline(always)]
    pub fn trgo(&self) -> TRGO_R {
        TRGO_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:7 - ILVNB
    #[inline(always)]
    pub fn ilvnb(&self) -> ILVNB_R {
        ILVNB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - TRGO
    #[inline(always)]
    #[must_use]
    pub fn trgo(&mut self) -> TRGO_W<0> {
        TRGO_W::new(self)
    }
    ///Bits 4:7 - ILVNB
    #[inline(always)]
    #[must_use]
    pub fn ilvnb(&mut self) -> ILVNB_W<4> {
        ILVNB_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MDF global control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gcr](index.html) module
pub struct GCR_SPEC;
impl crate::RegisterSpec for GCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gcr::R](R) reader structure
impl crate::Readable for GCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gcr::W](W) writer structure
impl crate::Writable for GCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GCR to value 0
impl crate::Resettable for GCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
