///Register `R4KEYR0` writer
pub struct W(crate::W<R4KEYR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R4KEYR0_SPEC>;
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
impl From<crate::W<R4KEYR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R4KEYR0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `REGx_KEY` writer - Region key, bits \[31:0\]
///This register must be written before the region corresponding REG_EN bit in OTFDEC_RxCFGR is set. Reading this register returns a zero value. Writing to this register is discarded if performed while the region CONFIGLOCK or KEYLOCK bit is set in the OTFDEC_RxCFGR. Note: When application successfully changes MODE bits in OTFDEC_RxCFGR and OTFDEC_RxKEYR, and associated KEYCRC are erased.
pub type REGX_KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, R4KEYR0_SPEC, u32, u32, 32, O>;
impl W {
    ///Bits 0:31 - Region key, bits \[31:0\]
    ///This register must be written before the region corresponding REG_EN bit in OTFDEC_RxCFGR is set. Reading this register returns a zero value. Writing to this register is discarded if performed while the region CONFIGLOCK or KEYLOCK bit is set in the OTFDEC_RxCFGR. Note: When application successfully changes MODE bits in OTFDEC_RxCFGR and OTFDEC_RxKEYR, and associated KEYCRC are erased.
    #[inline(always)]
    #[must_use]
    pub fn regx_key(&mut self) -> REGX_KEY_W<0> {
        REGX_KEY_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTFDEC region 4 key register 0
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [r4keyr0](index.html) module
pub struct R4KEYR0_SPEC;
impl crate::RegisterSpec for R4KEYR0_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [r4keyr0::W](W) writer structure
impl crate::Writable for R4KEYR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets R4KEYR0 to value 0
impl crate::Resettable for R4KEYR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
