///Register `CEC_TXDR` reader
pub struct R(crate::R<CEC_TXDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEC_TXDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEC_TXDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEC_TXDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CEC_TXDR` writer
pub struct W(crate::W<CEC_TXDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEC_TXDR_SPEC>;
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
impl From<crate::W<CEC_TXDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEC_TXDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TXD` writer - TXD
pub type TXD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CEC_TXDR_SPEC, u8, u8, 8, O>;
impl W {
    ///Bits 0:7 - TXD
    #[inline(always)]
    #[must_use]
    pub fn txd(&mut self) -> TXD_W<0> {
        TXD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CEC Tx data register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cec_txdr](index.html) module
pub struct CEC_TXDR_SPEC;
impl crate::RegisterSpec for CEC_TXDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cec_txdr::R](R) reader structure
impl crate::Readable for CEC_TXDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cec_txdr::W](W) writer structure
impl crate::Writable for CEC_TXDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CEC_TXDR to value 0
impl crate::Resettable for CEC_TXDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
