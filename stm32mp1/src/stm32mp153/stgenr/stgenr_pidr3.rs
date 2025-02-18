///Register `STGENR_PIDR3` reader
pub struct R(crate::R<STGENR_PIDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENR_PIDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENR_PIDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENR_PIDR3_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CMOD` reader - CMOD
pub type CMOD_R = crate::FieldReader<u8, u8>;
///Field `REVAND` reader - REVAND
pub type REVAND_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - CMOD
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - REVAND
    #[inline(always)]
    pub fn revand(&self) -> REVAND_R {
        REVAND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
///STGENR peripheral ID3 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [stgenr_pidr3](index.html) module
pub struct STGENR_PIDR3_SPEC;
impl crate::RegisterSpec for STGENR_PIDR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [stgenr_pidr3::R](R) reader structure
impl crate::Readable for STGENR_PIDR3_SPEC {
    type Reader = R;
}
///`reset()` method sets STGENR_PIDR3 to value 0
impl crate::Resettable for STGENR_PIDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
