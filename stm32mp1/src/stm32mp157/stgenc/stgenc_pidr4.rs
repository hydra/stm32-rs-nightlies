///Register `STGENC_PIDR4` reader
pub struct R(crate::R<STGENC_PIDR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENC_PIDR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENC_PIDR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENC_PIDR4_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DES_2` reader - DES_2
pub type DES_2_R = crate::FieldReader<u8, u8>;
///Field `SIZE` reader - SIZE
pub type SIZE_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - DES_2
    #[inline(always)]
    pub fn des_2(&self) -> DES_2_R {
        DES_2_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - SIZE
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
///STGENC peripheral ID4 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [stgenc_pidr4](index.html) module
pub struct STGENC_PIDR4_SPEC;
impl crate::RegisterSpec for STGENC_PIDR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [stgenc_pidr4::R](R) reader structure
impl crate::Readable for STGENC_PIDR4_SPEC {
    type Reader = R;
}
///`reset()` method sets STGENC_PIDR4 to value 0x04
impl crate::Resettable for STGENC_PIDR4_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
