///Register `JOFR4` reader
pub struct R(crate::R<JOFR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JOFR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JOFR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JOFR4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `JOFR4` writer
pub struct W(crate::W<JOFR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JOFR4_SPEC>;
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
impl From<crate::W<JOFR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JOFR4_SPEC>) -> Self {
        W(writer)
    }
}
///Field `JOFFSET4` reader - Data offset for injected channel 4
pub type JOFFSET4_R = crate::FieldReader<u16, u16>;
///Field `JOFFSET4` writer - Data offset for injected channel 4
pub type JOFFSET4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JOFR4_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 0:11 - Data offset for injected channel 4
    #[inline(always)]
    pub fn joffset4(&self) -> JOFFSET4_R {
        JOFFSET4_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - Data offset for injected channel 4
    #[inline(always)]
    #[must_use]
    pub fn joffset4(&mut self) -> JOFFSET4_W<0> {
        JOFFSET4_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///injected channel data offset register x
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [jofr4](index.html) module
pub struct JOFR4_SPEC;
impl crate::RegisterSpec for JOFR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [jofr4::R](R) reader structure
impl crate::Readable for JOFR4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [jofr4::W](W) writer structure
impl crate::Writable for JOFR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets JOFR4 to value 0
impl crate::Resettable for JOFR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
