///Register `SECR` reader
pub struct R(crate::R<SECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SECR` writer
pub struct W(crate::W<SECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECR_SPEC>;
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
impl From<crate::W<SECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SEC_SIZE` reader - Securable memory area size
pub type SEC_SIZE_R = crate::FieldReader<u8, u8>;
///Field `SEC_SIZE` writer - Securable memory area size
pub type SEC_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SECR_SPEC, u8, u8, 8, O>;
///Field `BOOT_LOCK` reader - used to force boot from user area
pub type BOOT_LOCK_R = crate::BitReader<bool>;
///Field `BOOT_LOCK` writer - used to force boot from user area
pub type BOOT_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECR_SPEC, bool, O>;
///Field `SEC_SIZE2` reader - Securable memory area size
pub type SEC_SIZE2_R = crate::FieldReader<u8, u8>;
///Field `SEC_SIZE2` writer - Securable memory area size
pub type SEC_SIZE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SECR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - Securable memory area size
    #[inline(always)]
    pub fn sec_size(&self) -> SEC_SIZE_R {
        SEC_SIZE_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 16 - used to force boot from user area
    #[inline(always)]
    pub fn boot_lock(&self) -> BOOT_LOCK_R {
        BOOT_LOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 20:27 - Securable memory area size
    #[inline(always)]
    pub fn sec_size2(&self) -> SEC_SIZE2_R {
        SEC_SIZE2_R::new(((self.bits >> 20) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Securable memory area size
    #[inline(always)]
    #[must_use]
    pub fn sec_size(&mut self) -> SEC_SIZE_W<0> {
        SEC_SIZE_W::new(self)
    }
    ///Bit 16 - used to force boot from user area
    #[inline(always)]
    #[must_use]
    pub fn boot_lock(&mut self) -> BOOT_LOCK_W<16> {
        BOOT_LOCK_W::new(self)
    }
    ///Bits 20:27 - Securable memory area size
    #[inline(always)]
    #[must_use]
    pub fn sec_size2(&mut self) -> SEC_SIZE2_W<20> {
        SEC_SIZE2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flash Security register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [secr](index.html) module
pub struct SECR_SPEC;
impl crate::RegisterSpec for SECR_SPEC {
    type Ux = u32;
}
///`read()` method returns [secr::R](R) reader structure
impl crate::Readable for SECR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [secr::W](W) writer structure
impl crate::Writable for SECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SECR to value 0
impl crate::Resettable for SECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
