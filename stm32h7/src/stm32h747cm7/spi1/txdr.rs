///Register `TXDR` writer
pub struct W(crate::W<TXDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXDR_SPEC>;
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
impl From<crate::W<TXDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TXDR` writer - Transmit data register
pub type TXDR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TXDR_SPEC, u32, u32, 32, O>;
impl W {
    ///Bits 0:31 - Transmit data register
    #[inline(always)]
    #[must_use]
    pub fn txdr(&mut self) -> TXDR_W<0> {
        TXDR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
///Transmit Data Register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [txdr](index.html) module
pub struct TXDR_SPEC;
impl crate::RegisterSpec for TXDR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [txdr::W](W) writer structure
impl crate::Writable for TXDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TXDR to value 0
impl crate::Resettable for TXDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
