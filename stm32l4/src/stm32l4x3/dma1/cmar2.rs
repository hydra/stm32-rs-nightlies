///Register `CMAR2` reader
pub struct R(crate::R<CMAR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMAR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMAR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMAR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CMAR2` writer
pub struct W(crate::W<CMAR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMAR2_SPEC>;
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
impl From<crate::W<CMAR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMAR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MA` reader - Memory address
pub type MA_R = crate::FieldReader<u32, u32>;
///Field `MA` writer - Memory address
pub type MA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMAR2_SPEC, u32, u32, 32, O>;
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
///channel x memory address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmar2](index.html) module
pub struct CMAR2_SPEC;
impl crate::RegisterSpec for CMAR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cmar2::R](R) reader structure
impl crate::Readable for CMAR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cmar2::W](W) writer structure
impl crate::Writable for CMAR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CMAR2 to value 0
impl crate::Resettable for CMAR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
