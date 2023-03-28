///Register `DTIMER` reader
pub struct R(crate::R<DTIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTIMER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DTIMER` writer
pub struct W(crate::W<DTIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTIMER_SPEC>;
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
impl From<crate::W<DTIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTIMER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DATATIME` reader - Data timeout period
pub type DATATIME_R = crate::FieldReader<u32, u32>;
///Field `DATATIME` writer - Data timeout period
pub type DATATIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DTIMER_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Data timeout period
    #[inline(always)]
    pub fn datatime(&self) -> DATATIME_R {
        DATATIME_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Data timeout period
    #[inline(always)]
    #[must_use]
    pub fn datatime(&mut self) -> DATATIME_W<0> {
        DATATIME_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Bits 31:0 = DATATIME: Data timeout period
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dtimer](index.html) module
pub struct DTIMER_SPEC;
impl crate::RegisterSpec for DTIMER_SPEC {
    type Ux = u32;
}
///`read()` method returns [dtimer::R](R) reader structure
impl crate::Readable for DTIMER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dtimer::W](W) writer structure
impl crate::Writable for DTIMER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DTIMER to value 0
impl crate::Resettable for DTIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
