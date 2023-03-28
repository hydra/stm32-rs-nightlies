///Register `LWR` reader
pub struct R(crate::R<LWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LWR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LWR` writer
pub struct W(crate::W<LWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LWR_SPEC>;
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
impl From<crate::W<LWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LWR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LW` reader - Line watermark These bits allow to configure the line watermark for interrupt generation. An interrupt is raised when the last pixel of the watermarked line has been transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.
pub type LW_R = crate::FieldReader<u16, u16>;
///Field `LW` writer - Line watermark These bits allow to configure the line watermark for interrupt generation. An interrupt is raised when the last pixel of the watermarked line has been transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.
pub type LW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LWR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Line watermark These bits allow to configure the line watermark for interrupt generation. An interrupt is raised when the last pixel of the watermarked line has been transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.
    #[inline(always)]
    pub fn lw(&self) -> LW_R {
        LW_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Line watermark These bits allow to configure the line watermark for interrupt generation. An interrupt is raised when the last pixel of the watermarked line has been transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.
    #[inline(always)]
    #[must_use]
    pub fn lw(&mut self) -> LW_W<0> {
        LW_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DMA2D line watermark register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lwr](index.html) module
pub struct LWR_SPEC;
impl crate::RegisterSpec for LWR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lwr::R](R) reader structure
impl crate::Readable for LWR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lwr::W](W) writer structure
impl crate::Writable for LWR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LWR to value 0
impl crate::Resettable for LWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
