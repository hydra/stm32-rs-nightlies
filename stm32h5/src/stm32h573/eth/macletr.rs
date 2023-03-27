///Register `MACLETR` reader
pub struct R(crate::R<MACLETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACLETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACLETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACLETR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACLETR` writer
pub struct W(crate::W<MACLETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACLETR_SPEC>;
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
impl From<crate::W<MACLETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACLETR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LPIET` reader - LPI Entry Timer This field specifies the time in microseconds the MAC waits to enter LPI mode, after it has transmitted all the frames. This field is valid and used only when LPITE and LPITXA are set to 1. Bits \[2:0\]
///are read-only so that the granularity of this timer is in steps of 8 micro-seconds.
pub type LPIET_R = crate::FieldReader<u32, u32>;
///Field `LPIET` writer - LPI Entry Timer This field specifies the time in microseconds the MAC waits to enter LPI mode, after it has transmitted all the frames. This field is valid and used only when LPITE and LPITXA are set to 1. Bits \[2:0\]
///are read-only so that the granularity of this timer is in steps of 8 micro-seconds.
pub type LPIET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACLETR_SPEC, u32, u32, 20, O>;
impl R {
    ///Bits 0:19 - LPI Entry Timer This field specifies the time in microseconds the MAC waits to enter LPI mode, after it has transmitted all the frames. This field is valid and used only when LPITE and LPITXA are set to 1. Bits \[2:0\]
    ///are read-only so that the granularity of this timer is in steps of 8 micro-seconds.
    #[inline(always)]
    pub fn lpiet(&self) -> LPIET_R {
        LPIET_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    ///Bits 0:19 - LPI Entry Timer This field specifies the time in microseconds the MAC waits to enter LPI mode, after it has transmitted all the frames. This field is valid and used only when LPITE and LPITXA are set to 1. Bits \[2:0\]
    ///are read-only so that the granularity of this timer is in steps of 8 micro-seconds.
    #[inline(always)]
    #[must_use]
    pub fn lpiet(&mut self) -> LPIET_W<0> {
        LPIET_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LPI entry timer register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macletr](index.html) module
pub struct MACLETR_SPEC;
impl crate::RegisterSpec for MACLETR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macletr::R](R) reader structure
impl crate::Readable for MACLETR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macletr::W](W) writer structure
impl crate::Writable for MACLETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACLETR to value 0
impl crate::Resettable for MACLETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
