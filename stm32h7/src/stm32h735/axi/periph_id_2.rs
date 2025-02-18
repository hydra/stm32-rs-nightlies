///Register `PERIPH_ID_2` reader
pub struct R(crate::R<PERIPH_ID_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIPH_ID_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIPH_ID_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIPH_ID_2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `JEP106ID` reader - JEP106 Identity bits 4 to 6
pub type JEP106ID_R = crate::FieldReader<u8, u8>;
///Field `JEDEC` reader - JEP106 code flag
pub type JEDEC_R = crate::BitReader<bool>;
///Field `REVISION` reader - Peripheral revision number
pub type REVISION_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:2 - JEP106 Identity bits 4 to 6
    #[inline(always)]
    pub fn jep106id(&self) -> JEP106ID_R {
        JEP106ID_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - JEP106 code flag
    #[inline(always)]
    pub fn jedec(&self) -> JEDEC_R {
        JEDEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - Peripheral revision number
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
///AXI interconnect - peripheral ID2 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [periph_id_2](index.html) module
pub struct PERIPH_ID_2_SPEC;
impl crate::RegisterSpec for PERIPH_ID_2_SPEC {
    type Ux = u32;
}
///`read()` method returns [periph_id_2::R](R) reader structure
impl crate::Readable for PERIPH_ID_2_SPEC {
    type Reader = R;
}
///`reset()` method sets PERIPH_ID_2 to value 0x04
impl crate::Resettable for PERIPH_ID_2_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
