///Register `FDCAN_DBTP` reader
pub struct R(crate::R<FDCAN_DBTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_DBTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_DBTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_DBTP_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DSJW` reader - Synchronization Jump Width
pub type DSJW_R = crate::FieldReader<u8, u8>;
///Field `DTSEG2` reader - Data time segment after sample point
pub type DTSEG2_R = crate::FieldReader<u8, u8>;
///Field `DTSEG1` reader - Data time segment after sample point
pub type DTSEG1_R = crate::FieldReader<u8, u8>;
///Field `DBRP` reader - Data BIt Rate Prescaler
pub type DBRP_R = crate::FieldReader<u8, u8>;
///Field `TDC` reader - Transceiver Delay Compensation
pub type TDC_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:3 - Synchronization Jump Width
    #[inline(always)]
    pub fn dsjw(&self) -> DSJW_R {
        DSJW_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Data time segment after sample point
    #[inline(always)]
    pub fn dtseg2(&self) -> DTSEG2_R {
        DTSEG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:12 - Data time segment after sample point
    #[inline(always)]
    pub fn dtseg1(&self) -> DTSEG1_R {
        DTSEG1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:20 - Data BIt Rate Prescaler
    #[inline(always)]
    pub fn dbrp(&self) -> DBRP_R {
        DBRP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bit 23 - Transceiver Delay Compensation
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 23) & 1) != 0)
    }
}
///FDCAN Data Bit Timing and Prescaler Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_dbtp](index.html) module
pub struct FDCAN_DBTP_SPEC;
impl crate::RegisterSpec for FDCAN_DBTP_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_dbtp::R](R) reader structure
impl crate::Readable for FDCAN_DBTP_SPEC {
    type Reader = R;
}
///`reset()` method sets FDCAN_DBTP to value 0x0a33
impl crate::Resettable for FDCAN_DBTP_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a33;
}
