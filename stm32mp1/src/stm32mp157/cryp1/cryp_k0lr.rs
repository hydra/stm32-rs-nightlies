///Register `CRYP_K0LR` writer
pub struct W(crate::W<CRYP_K0LR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYP_K0LR_SPEC>;
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
impl From<crate::W<CRYP_K0LR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYP_K0LR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `K` writer - K
pub type K_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRYP_K0LR_SPEC, u32, u32, 32, O>;
impl W {
    ///Bits 0:31 - K
    #[inline(always)]
    #[must_use]
    pub fn k(&mut self) -> K_W<0> {
        K_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CRYP key registers contain the cryptographic keys. In DES/TDES mode, the keys are 64-bit binary values (number from left to right, that is the leftmost bit is bit 1) and named K1, K2 and K3 (K0 is not used). Each key consists of 56 information bits and 8 parity bits. In AES mode, the key is considered as a single 128, 192 or 256 bits long sequence K0K1K2...K127/191/255. The AES key is entered into the registers as follows: for AES-128: K0..K127 corresponds to b127..b0 (b255..b128 are not used), for AES-192: K0..K191 corresponds to b191..b0 (b255..b192 are not used), for AES-256: K0..K255 corresponds to b255..b0. In all cases key bit K0 is the leftmost bit in CRYP inner memory and register bit b0 is the rightmost bit in corresponding CRYP_KxLR key register. For more information refer to Section39.3.17: CRYP key registers. Write accesses to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register)
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cryp_k0lr](index.html) module
pub struct CRYP_K0LR_SPEC;
impl crate::RegisterSpec for CRYP_K0LR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [cryp_k0lr::W](W) writer structure
impl crate::Writable for CRYP_K0LR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CRYP_K0LR to value 0
impl crate::Resettable for CRYP_K0LR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
