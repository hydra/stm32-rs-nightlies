///Register `PERIPH_ID_3` reader
pub struct R(crate::R<PERIPH_ID_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIPH_ID_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIPH_ID_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIPH_ID_3_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CUST_MOD_NUM` reader - Customer modification
pub type CUST_MOD_NUM_R = crate::FieldReader<u8, u8>;
///Field `REV_AND` reader - Customer version
pub type REV_AND_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - Customer modification
    #[inline(always)]
    pub fn cust_mod_num(&self) -> CUST_MOD_NUM_R {
        CUST_MOD_NUM_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Customer version
    #[inline(always)]
    pub fn rev_and(&self) -> REV_AND_R {
        REV_AND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
///AXI interconnect - peripheral ID3 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [periph_id_3](index.html) module
pub struct PERIPH_ID_3_SPEC;
impl crate::RegisterSpec for PERIPH_ID_3_SPEC {
    type Ux = u32;
}
///`read()` method returns [periph_id_3::R](R) reader structure
impl crate::Readable for PERIPH_ID_3_SPEC {
    type Reader = R;
}
///`reset()` method sets PERIPH_ID_3 to value 0x04
impl crate::Resettable for PERIPH_ID_3_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
