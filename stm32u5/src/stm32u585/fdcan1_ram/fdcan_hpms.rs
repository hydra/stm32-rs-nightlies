///Register `FDCAN_HPMS` reader
pub struct R(crate::R<FDCAN_HPMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_HPMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_HPMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_HPMS_SPEC>) -> Self {
        R(reader)
    }
}
///Field `BIDX` reader - Buffer Index
pub type BIDX_R = crate::FieldReader<u8, u8>;
///Field `MSI` reader - Message Storage Indicator
pub type MSI_R = crate::FieldReader<u8, u8>;
///Field `FIDX` reader - Filter Index
pub type FIDX_R = crate::FieldReader<u8, u8>;
///Field `FLST` reader - Filter List
pub type FLST_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:2 - Buffer Index
    #[inline(always)]
    pub fn bidx(&self) -> BIDX_R {
        BIDX_R::new((self.bits & 7) as u8)
    }
    ///Bits 6:7 - Message Storage Indicator
    #[inline(always)]
    pub fn msi(&self) -> MSI_R {
        MSI_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:12 - Filter Index
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bit 15 - Filter List
    #[inline(always)]
    pub fn flst(&self) -> FLST_R {
        FLST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
///FDCAN High Priority Message Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_hpms](index.html) module
pub struct FDCAN_HPMS_SPEC;
impl crate::RegisterSpec for FDCAN_HPMS_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_hpms::R](R) reader structure
impl crate::Readable for FDCAN_HPMS_SPEC {
    type Reader = R;
}
///`reset()` method sets FDCAN_HPMS to value 0
impl crate::Resettable for FDCAN_HPMS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
