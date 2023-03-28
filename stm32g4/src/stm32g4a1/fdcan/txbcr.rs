///Register `TXBCR` reader
pub struct R(crate::R<TXBCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TXBCR` writer
pub struct W(crate::W<TXBCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBCR_SPEC>;
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
impl From<crate::W<TXBCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CR` reader - CR
pub type CR_R = crate::FieldReader<u8, u8>;
///Field `CR` writer - CR
pub type CR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXBCR_SPEC, u8, u8, 3, O>;
impl R {
    ///Bits 0:2 - CR
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - CR
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<0> {
        CR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Tx Buffer Cancellation Request Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [txbcr](index.html) module
pub struct TXBCR_SPEC;
impl crate::RegisterSpec for TXBCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [txbcr::R](R) reader structure
impl crate::Readable for TXBCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [txbcr::W](W) writer structure
impl crate::Writable for TXBCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TXBCR to value 0
impl crate::Resettable for TXBCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
