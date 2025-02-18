///Register `UR7` reader
pub struct R(crate::R<UR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR7_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SA_BEG_1` reader - Secured area start address for bank 1
pub type SA_BEG_1_R = crate::FieldReader<u16, u16>;
///Field `SA_END_1` reader - Secured area end address for bank 1
pub type SA_END_1_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:11 - Secured area start address for bank 1
    #[inline(always)]
    pub fn sa_beg_1(&self) -> SA_BEG_1_R {
        SA_BEG_1_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Secured area end address for bank 1
    #[inline(always)]
    pub fn sa_end_1(&self) -> SA_END_1_R {
        SA_END_1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
///SYSCFG user register 7
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ur7](index.html) module
pub struct UR7_SPEC;
impl crate::RegisterSpec for UR7_SPEC {
    type Ux = u32;
}
///`read()` method returns [ur7::R](R) reader structure
impl crate::Readable for UR7_SPEC {
    type Reader = R;
}
///`reset()` method sets UR7 to value 0
impl crate::Resettable for UR7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
