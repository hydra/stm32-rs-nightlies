///Register `GICC_AEOIR` writer
pub struct W(crate::W<GICC_AEOIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICC_AEOIR_SPEC>;
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
impl From<crate::W<GICC_AEOIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICC_AEOIR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EOIINTID` writer - EOIINTID
pub type EOIINTID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICC_AEOIR_SPEC, u16, u16, 10, O>;
///Field `CPUID` writer - CPUID
pub type CPUID_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICC_AEOIR_SPEC, bool, O>;
impl W {
    ///Bits 0:9 - EOIINTID
    #[inline(always)]
    #[must_use]
    pub fn eoiintid(&mut self) -> EOIINTID_W<0> {
        EOIINTID_W::new(self)
    }
    ///Bit 10 - CPUID
    #[inline(always)]
    #[must_use]
    pub fn cpuid(&mut self) -> CPUID_W<10> {
        CPUID_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GICC_AEOIR is an alias of the Non-secure GICC_EOIR. A secure access to this register is similar to a non-secure access to GICC_EOIR, except that the GICC_CTLR.EOImodeS bit is used.
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gicc_aeoir](index.html) module
pub struct GICC_AEOIR_SPEC;
impl crate::RegisterSpec for GICC_AEOIR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [gicc_aeoir::W](W) writer structure
impl crate::Writable for GICC_AEOIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GICC_AEOIR to value 0
impl crate::Resettable for GICC_AEOIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
