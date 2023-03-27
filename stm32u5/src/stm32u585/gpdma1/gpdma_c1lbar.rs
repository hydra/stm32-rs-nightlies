///Register `GPDMA_C1LBAR` reader
pub struct R(crate::R<GPDMA_C1LBAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPDMA_C1LBAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPDMA_C1LBAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPDMA_C1LBAR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GPDMA_C1LBAR` writer
pub struct W(crate::W<GPDMA_C1LBAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPDMA_C1LBAR_SPEC>;
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
impl From<crate::W<GPDMA_C1LBAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPDMA_C1LBAR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LBA` reader - linked-list base address of GPDMA channel x
pub type LBA_R = crate::FieldReader<u16, u16>;
///Field `LBA` writer - linked-list base address of GPDMA channel x
pub type LBA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPDMA_C1LBAR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 16:31 - linked-list base address of GPDMA channel x
    #[inline(always)]
    pub fn lba(&self) -> LBA_R {
        LBA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 16:31 - linked-list base address of GPDMA channel x
    #[inline(always)]
    #[must_use]
    pub fn lba(&mut self) -> LBA_W<16> {
        LBA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPDMA channel 1 linked-list base address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gpdma_c1lbar](index.html) module
pub struct GPDMA_C1LBAR_SPEC;
impl crate::RegisterSpec for GPDMA_C1LBAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpdma_c1lbar::R](R) reader structure
impl crate::Readable for GPDMA_C1LBAR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gpdma_c1lbar::W](W) writer structure
impl crate::Writable for GPDMA_C1LBAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GPDMA_C1LBAR to value 0
impl crate::Resettable for GPDMA_C1LBAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
