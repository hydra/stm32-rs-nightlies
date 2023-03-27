///Register `SMCR` reader
pub struct R(crate::R<SMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SMCR` writer
pub struct W(crate::W<SMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMCR_SPEC>;
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
impl From<crate::W<SMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BKPRWDPROT` reader - BKPRWDPROT
pub type BKPRWDPROT_R = crate::FieldReader<u8, u8>;
///Field `BKPRWDPROT` writer - BKPRWDPROT
pub type BKPRWDPROT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMCR_SPEC, u8, u8, 8, O>;
///Field `BKPWDPROT` reader - BKPWDPROT
pub type BKPWDPROT_R = crate::FieldReader<u8, u8>;
///Field `BKPWDPROT` writer - BKPWDPROT
pub type BKPWDPROT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMCR_SPEC, u8, u8, 8, O>;
///Field `TAMPDPROT` reader - TAMPDPROT
pub type TAMPDPROT_R = crate::BitReader<bool>;
///Field `TAMPDPROT` writer - TAMPDPROT
pub type TAMPDPROT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMCR_SPEC, bool, O>;
impl R {
    ///Bits 0:7 - BKPRWDPROT
    #[inline(always)]
    pub fn bkprwdprot(&self) -> BKPRWDPROT_R {
        BKPRWDPROT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - BKPWDPROT
    #[inline(always)]
    pub fn bkpwdprot(&self) -> BKPWDPROT_R {
        BKPWDPROT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 31 - TAMPDPROT
    #[inline(always)]
    pub fn tampdprot(&self) -> TAMPDPROT_R {
        TAMPDPROT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - BKPRWDPROT
    #[inline(always)]
    #[must_use]
    pub fn bkprwdprot(&mut self) -> BKPRWDPROT_W<0> {
        BKPRWDPROT_W::new(self)
    }
    ///Bits 16:23 - BKPWDPROT
    #[inline(always)]
    #[must_use]
    pub fn bkpwdprot(&mut self) -> BKPWDPROT_W<16> {
        BKPWDPROT_W::new(self)
    }
    ///Bit 31 - TAMPDPROT
    #[inline(always)]
    #[must_use]
    pub fn tampdprot(&mut self) -> TAMPDPROT_W<31> {
        TAMPDPROT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register can be written only when the APB access is secure.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [smcr](index.html) module
pub struct SMCR_SPEC;
impl crate::RegisterSpec for SMCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [smcr::R](R) reader structure
impl crate::Readable for SMCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [smcr::W](W) writer structure
impl crate::Writable for SMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SMCR to value 0x8000_0000
impl crate::Resettable for SMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
