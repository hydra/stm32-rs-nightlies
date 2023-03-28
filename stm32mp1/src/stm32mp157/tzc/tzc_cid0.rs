///Register `TZC_CID0` reader
pub struct R(crate::R<TZC_CID0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_CID0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_CID0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_CID0_SPEC>) -> Self {
        R(reader)
    }
}
///Field `COMP_ID_0` reader - COMP_ID_0
pub type COMP_ID_0_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - COMP_ID_0
    #[inline(always)]
    pub fn comp_id_0(&self) -> COMP_ID_0_R {
        COMP_ID_0_R::new((self.bits & 0xff) as u8)
    }
}
///Component ID 0.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzc_cid0](index.html) module
pub struct TZC_CID0_SPEC;
impl crate::RegisterSpec for TZC_CID0_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzc_cid0::R](R) reader structure
impl crate::Readable for TZC_CID0_SPEC {
    type Reader = R;
}
///`reset()` method sets TZC_CID0 to value 0x0d
impl crate::Resettable for TZC_CID0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0d;
}
