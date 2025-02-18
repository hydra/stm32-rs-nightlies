///Register `PERIPH_ID_4` reader
pub struct R(crate::R<PERIPH_ID_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIPH_ID_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIPH_ID_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIPH_ID_4_SPEC>) -> Self {
        R(reader)
    }
}
///Field `JEP106CON` reader - JEP106 continuation code
pub type JEP106CON_R = crate::FieldReader<u8, u8>;
///Field `KCOUNT4` reader - Register file size
pub type KCOUNT4_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - JEP106 continuation code
    #[inline(always)]
    pub fn jep106con(&self) -> JEP106CON_R {
        JEP106CON_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Register file size
    #[inline(always)]
    pub fn kcount4(&self) -> KCOUNT4_R {
        KCOUNT4_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
///AXI interconnect - peripheral ID4 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [periph_id_4](index.html) module
pub struct PERIPH_ID_4_SPEC;
impl crate::RegisterSpec for PERIPH_ID_4_SPEC {
    type Ux = u32;
}
///`read()` method returns [periph_id_4::R](R) reader structure
impl crate::Readable for PERIPH_ID_4_SPEC {
    type Reader = R;
}
///`reset()` method sets PERIPH_ID_4 to value 0x04
impl crate::Resettable for PERIPH_ID_4_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
