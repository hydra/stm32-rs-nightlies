///Register `SECBOOTR_CUR` reader
pub struct R(crate::R<SECBOOTR_CUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECBOOTR_CUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECBOOTR_CUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECBOOTR_CUR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SECBOOT_LOCK` reader - A field locking the values of UBE, SWAP_BANK, and SECBOOTADD settings.
pub type SECBOOT_LOCK_R = crate::FieldReader<u8, u8>;
///Field `SECBOOTADD` reader - Unique boot entry secure address These bits reflect the Secure UBE address
pub type SECBOOTADD_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:7 - A field locking the values of UBE, SWAP_BANK, and SECBOOTADD settings.
    #[inline(always)]
    pub fn secboot_lock(&self) -> SECBOOT_LOCK_R {
        SECBOOT_LOCK_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:31 - Unique boot entry secure address These bits reflect the Secure UBE address
    #[inline(always)]
    pub fn secbootadd(&self) -> SECBOOTADD_R {
        SECBOOTADD_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
///FLASH secure boot register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [secbootr_cur](index.html) module
pub struct SECBOOTR_CUR_SPEC;
impl crate::RegisterSpec for SECBOOTR_CUR_SPEC {
    type Ux = u32;
}
///`read()` method returns [secbootr_cur::R](R) reader structure
impl crate::Readable for SECBOOTR_CUR_SPEC {
    type Reader = R;
}
///`reset()` method sets SECBOOTR_CUR to value 0
impl crate::Resettable for SECBOOTR_CUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
