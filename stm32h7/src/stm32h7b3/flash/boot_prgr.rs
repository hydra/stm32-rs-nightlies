///Register `BOOT_PRGR` reader
pub struct R(crate::R<BOOT_PRGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOT_PRGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOT_PRGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOT_PRGR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `BOOT_ADD0` reader - Boot address 0
pub type BOOT_ADD0_R = crate::FieldReader<u16, u16>;
///Field `BOOT_ADD1` reader - Boot address 1
pub type BOOT_ADD1_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Boot address 0
    #[inline(always)]
    pub fn boot_add0(&self) -> BOOT_ADD0_R {
        BOOT_ADD0_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Boot address 1
    #[inline(always)]
    pub fn boot_add1(&self) -> BOOT_ADD1_R {
        BOOT_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
///FLASH register with boot address
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [boot_prgr](index.html) module
pub struct BOOT_PRGR_SPEC;
impl crate::RegisterSpec for BOOT_PRGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [boot_prgr::R](R) reader structure
impl crate::Readable for BOOT_PRGR_SPEC {
    type Reader = R;
}
///`reset()` method sets BOOT_PRGR to value 0
impl crate::Resettable for BOOT_PRGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
