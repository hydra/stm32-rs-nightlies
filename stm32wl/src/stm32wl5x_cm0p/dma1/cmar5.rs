///Register `CMAR5` reader
pub struct R(crate::R<CMAR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMAR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMAR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMAR5_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CMAR5` writer
pub struct W(crate::W<CMAR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMAR5_SPEC>;
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
impl From<crate::W<CMAR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMAR5_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MA` reader - peripheral address
pub type MA_R = crate::FieldReader<u32, u32>;
///Field `MA` writer - peripheral address
pub type MA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CMAR5_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - peripheral address
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - peripheral address
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MA_W<0> {
        MA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
///channel x memory address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmar5](index.html) module
pub struct CMAR5_SPEC;
impl crate::RegisterSpec for CMAR5_SPEC {
    type Ux = u32;
}
///`read()` method returns [cmar5::R](R) reader structure
impl crate::Readable for CMAR5_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cmar5::W](W) writer structure
impl crate::Writable for CMAR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CMAR5 to value 0
impl crate::Resettable for CMAR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
