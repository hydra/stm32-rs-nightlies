///Register `FCCAN_CCU_CSTAT` reader
pub struct R(crate::R<FCCAN_CCU_CSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCCAN_CCU_CSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCCAN_CCU_CSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCCAN_CCU_CSTAT_SPEC>) -> Self {
        R(reader)
    }
}
///Field `OCPC` reader - OCPC
pub type OCPC_R = crate::FieldReader<u32, u32>;
///Field `TQC` reader - TQC
pub type TQC_R = crate::FieldReader<u16, u16>;
///Field `CALS` reader - CALS
pub type CALS_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:17 - OCPC
    #[inline(always)]
    pub fn ocpc(&self) -> OCPC_R {
        OCPC_R::new(self.bits & 0x0003_ffff)
    }
    ///Bits 18:28 - TQC
    #[inline(always)]
    pub fn tqc(&self) -> TQC_R {
        TQC_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    ///Bits 30:31 - CALS
    #[inline(always)]
    pub fn cals(&self) -> CALS_R {
        CALS_R::new(((self.bits >> 30) & 3) as u8)
    }
}
///Calibration status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fccan_ccu_cstat](index.html) module
pub struct FCCAN_CCU_CSTAT_SPEC;
impl crate::RegisterSpec for FCCAN_CCU_CSTAT_SPEC {
    type Ux = u32;
}
///`read()` method returns [fccan_ccu_cstat::R](R) reader structure
impl crate::Readable for FCCAN_CCU_CSTAT_SPEC {
    type Reader = R;
}
///`reset()` method sets FCCAN_CCU_CSTAT to value 0x0203_ffff
impl crate::Resettable for FCCAN_CCU_CSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0203_ffff;
}
