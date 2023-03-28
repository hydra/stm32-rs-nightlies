///Register `ITLINE3` reader
pub struct R(crate::R<ITLINE3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE3_SPEC>) -> Self {
        R(reader)
    }
}
///Field `FLASH_ITF` reader - FLASH_ITF
pub type FLASH_ITF_R = crate::BitReader<bool>;
///Field `FLASH_ECC` reader - FLASH_ECC
pub type FLASH_ECC_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - FLASH_ITF
    #[inline(always)]
    pub fn flash_itf(&self) -> FLASH_ITF_R {
        FLASH_ITF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FLASH_ECC
    #[inline(always)]
    pub fn flash_ecc(&self) -> FLASH_ECC_R {
        FLASH_ECC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
///interrupt line 3 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itline3](index.html) module
pub struct ITLINE3_SPEC;
impl crate::RegisterSpec for ITLINE3_SPEC {
    type Ux = u32;
}
///`read()` method returns [itline3::R](R) reader structure
impl crate::Readable for ITLINE3_SPEC {
    type Reader = R;
}
///`reset()` method sets ITLINE3 to value 0
impl crate::Resettable for ITLINE3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
