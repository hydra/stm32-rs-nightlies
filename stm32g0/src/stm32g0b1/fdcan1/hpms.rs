///Register `HPMS` reader
pub struct R(crate::R<HPMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPMS_SPEC>) -> Self {
        R(reader)
    }
}
///Field `BIDX` reader - Buffer index Index of Rx FIFO element to which the message was stored. Only valid when MSI\[1\]
///= 1.
pub type BIDX_R = crate::FieldReader<u8, u8>;
///Field `MSI` reader - Message storage indicator
pub type MSI_R = crate::FieldReader<u8, u8>;
///Field `FIDX` reader - Filter index Index of matching filter element. Range is 0 to RXGFC\[LSS\]
///- 1 or RXGFC\[LSE\]
///- 1.
pub type FIDX_R = crate::FieldReader<u8, u8>;
///Field `FLST` reader - Filter list Indicates the filter list of the matching filter element.
pub type FLST_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:2 - Buffer index Index of Rx FIFO element to which the message was stored. Only valid when MSI\[1\]
    ///= 1.
    #[inline(always)]
    pub fn bidx(&self) -> BIDX_R {
        BIDX_R::new((self.bits & 7) as u8)
    }
    ///Bits 6:7 - Message storage indicator
    #[inline(always)]
    pub fn msi(&self) -> MSI_R {
        MSI_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:12 - Filter index Index of matching filter element. Range is 0 to RXGFC\[LSS\]
    ///- 1 or RXGFC\[LSE\]
    ///- 1.
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bit 15 - Filter list Indicates the filter list of the matching filter element.
    #[inline(always)]
    pub fn flst(&self) -> FLST_R {
        FLST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
///FDCAN high-priority message status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hpms](index.html) module
pub struct HPMS_SPEC;
impl crate::RegisterSpec for HPMS_SPEC {
    type Ux = u32;
}
///`read()` method returns [hpms::R](R) reader structure
impl crate::Readable for HPMS_SPEC {
    type Reader = R;
}
///`reset()` method sets HPMS to value 0
impl crate::Resettable for HPMS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
