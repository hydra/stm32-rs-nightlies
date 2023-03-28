///Register `PSMAR` reader
pub struct R(crate::R<PSMAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSMAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSMAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSMAR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PSMAR` writer
pub struct W(crate::W<PSMAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSMAR_SPEC>;
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
impl From<crate::W<PSMAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSMAR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `INTERVAL` reader - Polling interval
pub type INTERVAL_R = crate::FieldReader<u16, u16>;
///Field `INTERVAL` writer - Polling interval
pub type INTERVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSMAR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Polling interval
    #[inline(always)]
    pub fn interval(&self) -> INTERVAL_R {
        INTERVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Polling interval
    #[inline(always)]
    #[must_use]
    pub fn interval(&mut self) -> INTERVAL_W<0> {
        INTERVAL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///polling status match register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [psmar](index.html) module
pub struct PSMAR_SPEC;
impl crate::RegisterSpec for PSMAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [psmar::R](R) reader structure
impl crate::Readable for PSMAR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [psmar::W](W) writer structure
impl crate::Writable for PSMAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PSMAR to value 0
impl crate::Resettable for PSMAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
