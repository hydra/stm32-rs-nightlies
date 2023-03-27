///Register `R4NONCER0` reader
pub struct R(crate::R<R4NONCER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R4NONCER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R4NONCER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R4NONCER0_SPEC>) -> Self {
        R(reader)
    }
}
///Register `R4NONCER0` writer
pub struct W(crate::W<R4NONCER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R4NONCER0_SPEC>;
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
impl From<crate::W<R4NONCER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R4NONCER0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `REGx_NONCE` reader - Region nonce, bits \[31:0\]
///This register must be written before the region corresponding REG_EN bit in OTFDEC_RxCFGR is set. Writing is discarded in this register if performed while the region CONFIGLOCK bit in the OTFDEC_RxCFGR is set.
pub type REGX_NONCE_R = crate::FieldReader<u32, u32>;
///Field `REGx_NONCE` writer - Region nonce, bits \[31:0\]
///This register must be written before the region corresponding REG_EN bit in OTFDEC_RxCFGR is set. Writing is discarded in this register if performed while the region CONFIGLOCK bit in the OTFDEC_RxCFGR is set.
pub type REGX_NONCE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R4NONCER0_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Region nonce, bits \[31:0\]
    ///This register must be written before the region corresponding REG_EN bit in OTFDEC_RxCFGR is set. Writing is discarded in this register if performed while the region CONFIGLOCK bit in the OTFDEC_RxCFGR is set.
    #[inline(always)]
    pub fn regx_nonce(&self) -> REGX_NONCE_R {
        REGX_NONCE_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Region nonce, bits \[31:0\]
    ///This register must be written before the region corresponding REG_EN bit in OTFDEC_RxCFGR is set. Writing is discarded in this register if performed while the region CONFIGLOCK bit in the OTFDEC_RxCFGR is set.
    #[inline(always)]
    #[must_use]
    pub fn regx_nonce(&mut self) -> REGX_NONCE_W<0> {
        REGX_NONCE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTFDEC region 4 nonce register 0
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [r4noncer0](index.html) module
pub struct R4NONCER0_SPEC;
impl crate::RegisterSpec for R4NONCER0_SPEC {
    type Ux = u32;
}
///`read()` method returns [r4noncer0::R](R) reader structure
impl crate::Readable for R4NONCER0_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [r4noncer0::W](W) writer structure
impl crate::Writable for R4NONCER0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets R4NONCER0 to value 0
impl crate::Resettable for R4NONCER0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
