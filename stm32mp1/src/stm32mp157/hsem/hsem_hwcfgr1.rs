///Register `HSEM_HWCFGR1` reader
pub struct R(crate::R<HSEM_HWCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSEM_HWCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSEM_HWCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSEM_HWCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `NBSEM` reader - NBSEM
pub type NBSEM_R = crate::FieldReader<u8, u8>;
///Field `NBINT` reader - NBINT
pub type NBINT_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - NBSEM
    #[inline(always)]
    pub fn nbsem(&self) -> NBSEM_R {
        NBSEM_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - NBINT
    #[inline(always)]
    pub fn nbint(&self) -> NBINT_R {
        NBINT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
///HSEM hardware configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hsem_hwcfgr1](index.html) module
pub struct HSEM_HWCFGR1_SPEC;
impl crate::RegisterSpec for HSEM_HWCFGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [hsem_hwcfgr1::R](R) reader structure
impl crate::Readable for HSEM_HWCFGR1_SPEC {
    type Reader = R;
}
///`reset()` method sets HSEM_HWCFGR1 to value 0x0220
impl crate::Resettable for HSEM_HWCFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0220;
}
