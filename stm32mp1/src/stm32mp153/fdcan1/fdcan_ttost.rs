///Register `FDCAN_TTOST` reader
pub struct R(crate::R<FDCAN_TTOST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTOST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTOST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTOST_SPEC>) -> Self {
        R(reader)
    }
}
///Field `EL` reader - EL
pub type EL_R = crate::FieldReader<u8, u8>;
///Field `MS` reader - MS
pub type MS_R = crate::FieldReader<u8, u8>;
///Field `SYS` reader - SYS
pub type SYS_R = crate::FieldReader<u8, u8>;
///Field `QGTP` reader - QGTP
pub type QGTP_R = crate::BitReader<bool>;
///Field `QCS` reader - QCS
pub type QCS_R = crate::BitReader<bool>;
///Field `RTO` reader - RTO
pub type RTO_R = crate::FieldReader<u8, u8>;
///Field `WGTD` reader - WGTD
pub type WGTD_R = crate::BitReader<bool>;
///Field `GFI` reader - GFI
pub type GFI_R = crate::BitReader<bool>;
///Field `TMP` reader - TMP
pub type TMP_R = crate::FieldReader<u8, u8>;
///Field `GSI` reader - GSI
pub type GSI_R = crate::BitReader<bool>;
///Field `WFE` reader - WFE
pub type WFE_R = crate::BitReader<bool>;
///Field `AWE` reader - AWE
pub type AWE_R = crate::BitReader<bool>;
///Field `WECS` reader - WECS
pub type WECS_R = crate::BitReader<bool>;
///Field `SPL` reader - SPL
pub type SPL_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:1 - EL
    #[inline(always)]
    pub fn el(&self) -> EL_R {
        EL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - MS
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - SYS
    #[inline(always)]
    pub fn sys(&self) -> SYS_R {
        SYS_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - QGTP
    #[inline(always)]
    pub fn qgtp(&self) -> QGTP_R {
        QGTP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - QCS
    #[inline(always)]
    pub fn qcs(&self) -> QCS_R {
        QCS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - RTO
    #[inline(always)]
    pub fn rto(&self) -> RTO_R {
        RTO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 22 - WGTD
    #[inline(always)]
    pub fn wgtd(&self) -> WGTD_R {
        WGTD_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - GFI
    #[inline(always)]
    pub fn gfi(&self) -> GFI_R {
        GFI_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:26 - TMP
    #[inline(always)]
    pub fn tmp(&self) -> TMP_R {
        TMP_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bit 27 - GSI
    #[inline(always)]
    pub fn gsi(&self) -> GSI_R {
        GSI_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - WFE
    #[inline(always)]
    pub fn wfe(&self) -> WFE_R {
        WFE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - AWE
    #[inline(always)]
    pub fn awe(&self) -> AWE_R {
        AWE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - WECS
    #[inline(always)]
    pub fn wecs(&self) -> WECS_R {
        WECS_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SPL
    #[inline(always)]
    pub fn spl(&self) -> SPL_R {
        SPL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
///FDCAN TT operation status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_ttost](index.html) module
pub struct FDCAN_TTOST_SPEC;
impl crate::RegisterSpec for FDCAN_TTOST_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_ttost::R](R) reader structure
impl crate::Readable for FDCAN_TTOST_SPEC {
    type Reader = R;
}
///`reset()` method sets FDCAN_TTOST to value 0x80
impl crate::Resettable for FDCAN_TTOST_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
