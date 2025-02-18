///Register `PERIPH_ID_1` reader
pub struct R(crate::R<PERIPH_ID_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIPH_ID_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIPH_ID_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIPH_ID_1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PARTNUM` reader - Peripheral part number bits 8 to 11
pub type PARTNUM_R = crate::FieldReader<u8, u8>;
///Field `JEP106I` reader - JEP106 identity bits 0 to 3
pub type JEP106I_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - Peripheral part number bits 8 to 11
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - JEP106 identity bits 0 to 3
    #[inline(always)]
    pub fn jep106i(&self) -> JEP106I_R {
        JEP106I_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
///AXI interconnect - peripheral ID1 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [periph_id_1](index.html) module
pub struct PERIPH_ID_1_SPEC;
impl crate::RegisterSpec for PERIPH_ID_1_SPEC {
    type Ux = u32;
}
///`read()` method returns [periph_id_1::R](R) reader structure
impl crate::Readable for PERIPH_ID_1_SPEC {
    type Reader = R;
}
///`reset()` method sets PERIPH_ID_1 to value 0x04
impl crate::Resettable for PERIPH_ID_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
