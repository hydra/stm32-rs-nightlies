///Register `HSEM_HWCFGR2` reader
pub struct R(crate::R<HSEM_HWCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSEM_HWCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSEM_HWCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSEM_HWCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `MASTERID1` reader - MASTERID1
pub type MASTERID1_R = crate::FieldReader<u8, u8>;
///Field `MASTERID2` reader - MASTERID2
pub type MASTERID2_R = crate::FieldReader<u8, u8>;
///Field `MASTERID3` reader - MASTERID3
pub type MASTERID3_R = crate::FieldReader<u8, u8>;
///Field `MASTERID4` reader - MASTERID4
pub type MASTERID4_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - MASTERID1
    #[inline(always)]
    pub fn masterid1(&self) -> MASTERID1_R {
        MASTERID1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - MASTERID2
    #[inline(always)]
    pub fn masterid2(&self) -> MASTERID2_R {
        MASTERID2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - MASTERID3
    #[inline(always)]
    pub fn masterid3(&self) -> MASTERID3_R {
        MASTERID3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - MASTERID4
    #[inline(always)]
    pub fn masterid4(&self) -> MASTERID4_R {
        MASTERID4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
///HSEM hardware configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hsem_hwcfgr2](index.html) module
pub struct HSEM_HWCFGR2_SPEC;
impl crate::RegisterSpec for HSEM_HWCFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [hsem_hwcfgr2::R](R) reader structure
impl crate::Readable for HSEM_HWCFGR2_SPEC {
    type Reader = R;
}
///`reset()` method sets HSEM_HWCFGR2 to value 0x21
impl crate::Resettable for HSEM_HWCFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x21;
}
