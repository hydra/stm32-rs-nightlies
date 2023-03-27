///Register `R3KEYR2` writer
pub struct W(crate::W<R3KEYR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R3KEYR2_SPEC>;
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
impl From<crate::W<R3KEYR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R3KEYR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `REGx_KEY` writer - Region key, bits \[95:64\]
///Refer to the OTFDEC_RxKEYR0 register for description of the KEY\[127:0\]
///bitfield.
pub type REGX_KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, R3KEYR2_SPEC, u32, u32, 32, O>;
impl W {
    ///Bits 0:31 - Region key, bits \[95:64\]
    ///Refer to the OTFDEC_RxKEYR0 register for description of the KEY\[127:0\]
    ///bitfield.
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
///OTFDEC region 3 key register 2
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [r3keyr2](index.html) module
pub struct R3KEYR2_SPEC;
impl crate::RegisterSpec for R3KEYR2_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [r3keyr2::W](W) writer structure
impl crate::Writable for R3KEYR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets R3KEYR2 to value 0
impl crate::Resettable for R3KEYR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
