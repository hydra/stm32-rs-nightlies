///Register `MAR` reader
pub struct R(crate::R<MAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MAR` writer
pub struct W(crate::W<MAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAR_SPEC>;
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
impl From<crate::W<MAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MA` reader - Memory address
pub type MA_R = crate::FieldReader<u32, u32>;
///Field `MA` writer - Memory address
pub type MA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Memory address
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Memory address
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
///DMA channel 1 memory address
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mar](index.html) module
pub struct MAR_SPEC;
impl crate::RegisterSpec for MAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mar::R](R) reader structure
impl crate::Readable for MAR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mar::W](W) writer structure
impl crate::Writable for MAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MAR to value 0
impl crate::Resettable for MAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
