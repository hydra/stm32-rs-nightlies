///Register `STGENC_PIDR0` reader
pub struct R(crate::R<STGENC_PIDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENC_PIDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENC_PIDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENC_PIDR0_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PART_0` reader - PART_0
pub type PART_0_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - PART_0
    #[inline(always)]
    pub fn part_0(&self) -> PART_0_R {
        PART_0_R::new((self.bits & 0xff) as u8)
    }
}
///STGENC peripheral ID0 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [stgenc_pidr0](index.html) module
pub struct STGENC_PIDR0_SPEC;
impl crate::RegisterSpec for STGENC_PIDR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [stgenc_pidr0::R](R) reader structure
impl crate::Readable for STGENC_PIDR0_SPEC {
    type Reader = R;
}
///`reset()` method sets STGENC_PIDR0 to value 0x01
impl crate::Resettable for STGENC_PIDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
