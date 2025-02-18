///Register `RTOR` reader
pub struct R(crate::R<RTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTOR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RTOR` writer
pub struct W(crate::W<RTOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTOR_SPEC>;
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
impl From<crate::W<RTOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTOR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RTO` reader - Receiver timeout value
pub type RTO_R = crate::FieldReader<u32, u32>;
///Field `RTO` writer - Receiver timeout value
pub type RTO_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, RTOR_SPEC, u32, u32, 24, O>;
///Field `BLEN` reader - Block Length
pub type BLEN_R = crate::FieldReader<u8, u8>;
///Field `BLEN` writer - Block Length
pub type BLEN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, RTOR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:23 - Receiver timeout value
    #[inline(always)]
    pub fn rto(&self) -> RTO_R {
        RTO_R::new(self.bits & 0x00ff_ffff)
    }
    ///Bits 24:31 - Block Length
    #[inline(always)]
    pub fn blen(&self) -> BLEN_R {
        BLEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:23 - Receiver timeout value
    #[inline(always)]
    #[must_use]
    pub fn rto(&mut self) -> RTO_W<0> {
        RTO_W::new(self)
    }
    ///Bits 24:31 - Block Length
    #[inline(always)]
    #[must_use]
    pub fn blen(&mut self) -> BLEN_W<24> {
        BLEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Receiver timeout register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rtor](index.html) module
pub struct RTOR_SPEC;
impl crate::RegisterSpec for RTOR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rtor::R](R) reader structure
impl crate::Readable for RTOR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rtor::W](W) writer structure
impl crate::Writable for RTOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RTOR to value 0
impl crate::Resettable for RTOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
