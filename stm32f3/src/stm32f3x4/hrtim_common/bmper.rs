///Register `BMPER` reader
pub struct R(crate::R<BMPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMPER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BMPER` writer
pub struct W(crate::W<BMPER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMPER_SPEC>;
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
impl From<crate::W<BMPER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMPER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BMPER` reader - Burst mode Period
pub type BMPER_R = crate::FieldReader<u16, u16>;
///Field `BMPER` writer - Burst mode Period
pub type BMPER_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BMPER_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Burst mode Period
    #[inline(always)]
    pub fn bmper(&self) -> BMPER_R {
        BMPER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Burst mode Period
    #[inline(always)]
    #[must_use]
    pub fn bmper(&mut self) -> BMPER_W<0> {
        BMPER_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Burst Mode Period Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bmper](index.html) module
pub struct BMPER_SPEC;
impl crate::RegisterSpec for BMPER_SPEC {
    type Ux = u32;
}
///`read()` method returns [bmper::R](R) reader structure
impl crate::Readable for BMPER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bmper::W](W) writer structure
impl crate::Writable for BMPER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BMPER to value 0
impl crate::Resettable for BMPER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
