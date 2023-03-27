///Register `WRPSGN1R_CUR` reader
pub struct R(crate::R<WRPSGN1R_CUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRPSGN1R_CUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRPSGN1R_CUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRPSGN1R_CUR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `WRPSG1` reader - Bank1 sector protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding sectors in bank 1 (0: write protected; 1: not write protected)
pub type WRPSG1_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - Bank1 sector protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding sectors in bank 1 (0: write protected; 1: not write protected)
    #[inline(always)]
    pub fn wrpsg1(&self) -> WRPSG1_R {
        WRPSG1_R::new((self.bits & 0xff) as u8)
    }
}
///FLASH write sector protection for Bank1
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wrpsgn1r_cur](index.html) module
pub struct WRPSGN1R_CUR_SPEC;
impl crate::RegisterSpec for WRPSGN1R_CUR_SPEC {
    type Ux = u32;
}
///`read()` method returns [wrpsgn1r_cur::R](R) reader structure
impl crate::Readable for WRPSGN1R_CUR_SPEC {
    type Reader = R;
}
///`reset()` method sets WRPSGN1R_CUR to value 0
impl crate::Resettable for WRPSGN1R_CUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
