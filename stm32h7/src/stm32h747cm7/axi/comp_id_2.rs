///Register `COMP_ID_2` reader
pub struct R(crate::R<COMP_ID_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP_ID_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP_ID_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP_ID_2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PREAMBLE` reader - Preamble bits 12 to 19
pub type PREAMBLE_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - Preamble bits 12 to 19
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0xff) as u8)
    }
}
///AXI interconnect - component ID2 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [comp_id_2](index.html) module
pub struct COMP_ID_2_SPEC;
impl crate::RegisterSpec for COMP_ID_2_SPEC {
    type Ux = u32;
}
///`read()` method returns [comp_id_2::R](R) reader structure
impl crate::Readable for COMP_ID_2_SPEC {
    type Reader = R;
}
///`reset()` method sets COMP_ID_2 to value 0x04
impl crate::Resettable for COMP_ID_2_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
