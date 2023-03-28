///Register `BDMADR` reader
pub struct R(crate::R<BDMADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDMADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDMADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDMADR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BDMADR` writer
pub struct W(crate::W<BDMADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDMADR_SPEC>;
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
impl From<crate::W<BDMADR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDMADR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BDMADR` reader - Burst DMA Data register
pub type BDMADR_R = crate::FieldReader<u32, u32>;
///Field `BDMADR` writer - Burst DMA Data register
pub type BDMADR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BDMADR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Burst DMA Data register
    #[inline(always)]
    pub fn bdmadr(&self) -> BDMADR_R {
        BDMADR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Burst DMA Data register
    #[inline(always)]
    #[must_use]
    pub fn bdmadr(&mut self) -> BDMADR_W<0> {
        BDMADR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
///Burst DMA Data Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bdmadr](index.html) module
pub struct BDMADR_SPEC;
impl crate::RegisterSpec for BDMADR_SPEC {
    type Ux = u32;
}
///`read()` method returns [bdmadr::R](R) reader structure
impl crate::Readable for BDMADR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bdmadr::W](W) writer structure
impl crate::Writable for BDMADR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BDMADR to value 0
impl crate::Resettable for BDMADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
