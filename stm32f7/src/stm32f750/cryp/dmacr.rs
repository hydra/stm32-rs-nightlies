///Register `DMACR` reader
pub struct R(crate::R<DMACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMACR` writer
pub struct W(crate::W<DMACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACR_SPEC>;
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
impl From<crate::W<DMACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DIEN` reader - DMA input enable
pub type DIEN_R = crate::BitReader<bool>;
///Field `DIEN` writer - DMA input enable
pub type DIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACR_SPEC, bool, O>;
///Field `DOEN` reader - DMA output enable
pub type DOEN_R = crate::BitReader<bool>;
///Field `DOEN` writer - DMA output enable
pub type DOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACR_SPEC, bool, O>;
impl R {
    ///Bit 0 - DMA input enable
    #[inline(always)]
    pub fn dien(&self) -> DIEN_R {
        DIEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA output enable
    #[inline(always)]
    pub fn doen(&self) -> DOEN_R {
        DOEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DMA input enable
    #[inline(always)]
    #[must_use]
    pub fn dien(&mut self) -> DIEN_W<0> {
        DIEN_W::new(self)
    }
    ///Bit 1 - DMA output enable
    #[inline(always)]
    #[must_use]
    pub fn doen(&mut self) -> DOEN_W<1> {
        DOEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DMA control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmacr](index.html) module
pub struct DMACR_SPEC;
impl crate::RegisterSpec for DMACR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmacr::R](R) reader structure
impl crate::Readable for DMACR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dmacr::W](W) writer structure
impl crate::Writable for DMACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMACR to value 0
impl crate::Resettable for DMACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
