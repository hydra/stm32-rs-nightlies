///Register `FLASH_PRIVCFGR` reader
pub struct R(crate::R<FLASH_PRIVCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_PRIVCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_PRIVCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_PRIVCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLASH_PRIVCFGR` writer
pub struct W(crate::W<FLASH_PRIVCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_PRIVCFGR_SPEC>;
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
impl From<crate::W<FLASH_PRIVCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_PRIVCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SPRIV` reader - Privileged protection for secure registers This bit can be accessed only when TrustZone is enabled (TZEN = 1). This bit can be read by both privileged or unprivileged, secure and non-secure access. The SPRIV bit can be written only by a secure privileged access. A non-secure write access on SPRIV bit is ignored. A secure unprivileged write access on SPRIV bit is ignored.
pub type SPRIV_R = crate::BitReader<bool>;
///Field `SPRIV` writer - Privileged protection for secure registers This bit can be accessed only when TrustZone is enabled (TZEN = 1). This bit can be read by both privileged or unprivileged, secure and non-secure access. The SPRIV bit can be written only by a secure privileged access. A non-secure write access on SPRIV bit is ignored. A secure unprivileged write access on SPRIV bit is ignored.
pub type SPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIVCFGR_SPEC, bool, O>;
///Field `NSPRIV` reader - Privileged protection for non-secure registers This bit can be read by both privileged or unprivileged, secure and non-secure access. The NSPRIV bit can be written by a secure or non-secure privileged access. A secure or non-secure unprivileged write access on NSPRIV bit is ignored.
pub type NSPRIV_R = crate::BitReader<bool>;
///Field `NSPRIV` writer - Privileged protection for non-secure registers This bit can be read by both privileged or unprivileged, secure and non-secure access. The NSPRIV bit can be written by a secure or non-secure privileged access. A secure or non-secure unprivileged write access on NSPRIV bit is ignored.
pub type NSPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIVCFGR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Privileged protection for secure registers This bit can be accessed only when TrustZone is enabled (TZEN = 1). This bit can be read by both privileged or unprivileged, secure and non-secure access. The SPRIV bit can be written only by a secure privileged access. A non-secure write access on SPRIV bit is ignored. A secure unprivileged write access on SPRIV bit is ignored.
    #[inline(always)]
    pub fn spriv(&self) -> SPRIV_R {
        SPRIV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Privileged protection for non-secure registers This bit can be read by both privileged or unprivileged, secure and non-secure access. The NSPRIV bit can be written by a secure or non-secure privileged access. A secure or non-secure unprivileged write access on NSPRIV bit is ignored.
    #[inline(always)]
    pub fn nspriv(&self) -> NSPRIV_R {
        NSPRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Privileged protection for secure registers This bit can be accessed only when TrustZone is enabled (TZEN = 1). This bit can be read by both privileged or unprivileged, secure and non-secure access. The SPRIV bit can be written only by a secure privileged access. A non-secure write access on SPRIV bit is ignored. A secure unprivileged write access on SPRIV bit is ignored.
    #[inline(always)]
    #[must_use]
    pub fn spriv(&mut self) -> SPRIV_W<0> {
        SPRIV_W::new(self)
    }
    ///Bit 1 - Privileged protection for non-secure registers This bit can be read by both privileged or unprivileged, secure and non-secure access. The NSPRIV bit can be written by a secure or non-secure privileged access. A secure or non-secure unprivileged write access on NSPRIV bit is ignored.
    #[inline(always)]
    #[must_use]
    pub fn nspriv(&mut self) -> NSPRIV_W<1> {
        NSPRIV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH privilege configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flash_privcfgr](index.html) module
pub struct FLASH_PRIVCFGR_SPEC;
impl crate::RegisterSpec for FLASH_PRIVCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [flash_privcfgr::R](R) reader structure
impl crate::Readable for FLASH_PRIVCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [flash_privcfgr::W](W) writer structure
impl crate::Writable for FLASH_PRIVCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLASH_PRIVCFGR to value 0
impl crate::Resettable for FLASH_PRIVCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
