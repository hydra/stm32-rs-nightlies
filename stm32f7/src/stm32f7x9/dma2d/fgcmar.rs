///Register `FGCMAR` reader
pub struct R(crate::R<FGCMAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGCMAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGCMAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGCMAR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FGCMAR` writer
pub struct W(crate::W<FGCMAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGCMAR_SPEC>;
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
impl From<crate::W<FGCMAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGCMAR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MA` reader - Memory Address
pub type MA_R = crate::FieldReader<u32, u32>;
///Field `MA` writer - Memory Address
pub type MA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGCMAR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Memory Address
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Memory Address
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MA_W<0> {
        MA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///foreground CLUT memory address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fgcmar](index.html) module
pub struct FGCMAR_SPEC;
impl crate::RegisterSpec for FGCMAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [fgcmar::R](R) reader structure
impl crate::Readable for FGCMAR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fgcmar::W](W) writer structure
impl crate::Writable for FGCMAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FGCMAR to value 0
impl crate::Resettable for FGCMAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
