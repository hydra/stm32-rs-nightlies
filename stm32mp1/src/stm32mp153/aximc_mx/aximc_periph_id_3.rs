///Register `AXIMC_PERIPH_ID_3` reader
pub struct R(crate::R<AXIMC_PERIPH_ID_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AXIMC_PERIPH_ID_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AXIMC_PERIPH_ID_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AXIMC_PERIPH_ID_3_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CUST_MOD_NUM` reader - CUST_MOD_NUM
pub type CUST_MOD_NUM_R = crate::FieldReader<u8, u8>;
///Field `REV_AND` reader - REV_AND
pub type REV_AND_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - CUST_MOD_NUM
    #[inline(always)]
    pub fn cust_mod_num(&self) -> CUST_MOD_NUM_R {
        CUST_MOD_NUM_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - REV_AND
    #[inline(always)]
    pub fn rev_and(&self) -> REV_AND_R {
        REV_AND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
///AXIMC peripheral ID3 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [aximc_periph_id_3](index.html) module
pub struct AXIMC_PERIPH_ID_3_SPEC;
impl crate::RegisterSpec for AXIMC_PERIPH_ID_3_SPEC {
    type Ux = u32;
}
///`read()` method returns [aximc_periph_id_3::R](R) reader structure
impl crate::Readable for AXIMC_PERIPH_ID_3_SPEC {
    type Reader = R;
}
///`reset()` method sets AXIMC_PERIPH_ID_3 to value 0
impl crate::Resettable for AXIMC_PERIPH_ID_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
