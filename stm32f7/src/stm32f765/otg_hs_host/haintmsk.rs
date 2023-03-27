///Register `HAINTMSK` reader
pub struct R(crate::R<HAINTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HAINTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HAINTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HAINTMSK_SPEC>) -> Self {
        R(reader)
    }
}
///Register `HAINTMSK` writer
pub struct W(crate::W<HAINTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HAINTMSK_SPEC>;
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
impl From<crate::W<HAINTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HAINTMSK_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HAINTM` reader - Channel interrupt mask
pub type HAINTM_R = crate::FieldReader<u16, u16>;
///Field `HAINTM` writer - Channel interrupt mask
pub type HAINTM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HAINTMSK_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Channel interrupt mask
    #[inline(always)]
    pub fn haintm(&self) -> HAINTM_R {
        HAINTM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Channel interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn haintm(&mut self) -> HAINTM_W<0> {
        HAINTM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTG_HS host all channels interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [haintmsk](index.html) module
pub struct HAINTMSK_SPEC;
impl crate::RegisterSpec for HAINTMSK_SPEC {
    type Ux = u32;
}
///`read()` method returns [haintmsk::R](R) reader structure
impl crate::Readable for HAINTMSK_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [haintmsk::W](W) writer structure
impl crate::Writable for HAINTMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets HAINTMSK to value 0
impl crate::Resettable for HAINTMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
