///Register `CH0WDATR` reader
pub struct R(crate::R<CH0WDATR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH0WDATR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH0WDATR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH0WDATR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CH0WDATR` writer
pub struct W(crate::W<CH0WDATR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH0WDATR_SPEC>;
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
impl From<crate::W<CH0WDATR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH0WDATR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WDATA` reader - WDATA
pub type WDATA_R = crate::FieldReader<u16, u16>;
///Field `WDATA` writer - WDATA
pub type WDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH0WDATR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - WDATA
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - WDATA
    #[inline(always)]
    #[must_use]
    pub fn wdata(&mut self) -> WDATA_W<0> {
        WDATA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///channel watchdog filter data register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch0wdatr](index.html) module
pub struct CH0WDATR_SPEC;
impl crate::RegisterSpec for CH0WDATR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ch0wdatr::R](R) reader structure
impl crate::Readable for CH0WDATR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ch0wdatr::W](W) writer structure
impl crate::Writable for CH0WDATR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CH0WDATR to value 0
impl crate::Resettable for CH0WDATR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
