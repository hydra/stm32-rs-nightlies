///Register `CM1AR6` reader
pub struct R(crate::R<CM1AR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM1AR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM1AR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM1AR6_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CM1AR6` writer
pub struct W(crate::W<CM1AR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM1AR6_SPEC>;
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
impl From<crate::W<CM1AR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM1AR6_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MA` reader - peripheral address
pub type MA_R = crate::FieldReader<u32, u32>;
///Field `MA` writer - peripheral address
pub type MA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CM1AR6_SPEC, u32, u32, 32, O>;
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///channel x memory address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cm1ar6](index.html) module
pub struct CM1AR6_SPEC;
impl crate::RegisterSpec for CM1AR6_SPEC {
    type Ux = u32;
}
///`read()` method returns [cm1ar6::R](R) reader structure
impl crate::Readable for CM1AR6_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cm1ar6::W](W) writer structure
impl crate::Writable for CM1AR6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CM1AR6 to value 0
impl crate::Resettable for CM1AR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
