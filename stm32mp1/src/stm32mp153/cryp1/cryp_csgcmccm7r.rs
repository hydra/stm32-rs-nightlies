///Register `CRYP_CSGCMCCM7R` reader
pub struct R(crate::R<CRYP_CSGCMCCM7R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYP_CSGCMCCM7R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYP_CSGCMCCM7R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYP_CSGCMCCM7R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CRYP_CSGCMCCM7R` writer
pub struct W(crate::W<CRYP_CSGCMCCM7R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYP_CSGCMCCM7R_SPEC>;
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
impl From<crate::W<CRYP_CSGCMCCM7R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYP_CSGCMCCM7R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CSGCMCCM7` reader - CSGCMCCM7
pub type CSGCMCCM7_R = crate::FieldReader<u32, u32>;
///Field `CSGCMCCM7` writer - CSGCMCCM7
pub type CSGCMCCM7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CRYP_CSGCMCCM7R_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - CSGCMCCM7
    #[inline(always)]
    pub fn csgcmccm7(&self) -> CSGCMCCM7_R {
        CSGCMCCM7_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - CSGCMCCM7
    #[inline(always)]
    #[must_use]
    pub fn csgcmccm7(&mut self) -> CSGCMCCM7_W<0> {
        CSGCMCCM7_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cryp_csgcmccm7r](index.html) module
pub struct CRYP_CSGCMCCM7R_SPEC;
impl crate::RegisterSpec for CRYP_CSGCMCCM7R_SPEC {
    type Ux = u32;
}
///`read()` method returns [cryp_csgcmccm7r::R](R) reader structure
impl crate::Readable for CRYP_CSGCMCCM7R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cryp_csgcmccm7r::W](W) writer structure
impl crate::Writable for CRYP_CSGCMCCM7R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CRYP_CSGCMCCM7R to value 0
impl crate::Resettable for CRYP_CSGCMCCM7R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
