///Register `DMAR` reader
pub struct R(crate::R<DMAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMAR` writer
pub struct W(crate::W<DMAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAR_SPEC>;
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
impl From<crate::W<DMAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DMAB` reader - DMA register for burst accesses
pub type DMAB_R = crate::FieldReader<u16, u16>;
///Field `DMAB` writer - DMA register for burst accesses
pub type DMAB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMAR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - DMA register for burst accesses
    #[inline(always)]
    pub fn dmab(&self) -> DMAB_R {
        DMAB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - DMA register for burst accesses
    #[inline(always)]
    #[must_use]
    pub fn dmab(&mut self) -> DMAB_W<0> {
        DMAB_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DMA address for full transfer
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmar](index.html) module
pub struct DMAR_SPEC;
impl crate::RegisterSpec for DMAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmar::R](R) reader structure
impl crate::Readable for DMAR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dmar::W](W) writer structure
impl crate::Writable for DMAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMAR to value 0
impl crate::Resettable for DMAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
