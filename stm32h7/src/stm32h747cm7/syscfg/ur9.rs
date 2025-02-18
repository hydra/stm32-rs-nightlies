///Register `UR9` reader
pub struct R(crate::R<UR9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR9_SPEC>) -> Self {
        R(reader)
    }
}
///Field `WRPS_2` reader - Write protection for flash bank 2
pub type WRPS_2_R = crate::FieldReader<u8, u8>;
///Field `PA_BEG_2` reader - Protected area start address for bank 2
pub type PA_BEG_2_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:7 - Write protection for flash bank 2
    #[inline(always)]
    pub fn wrps_2(&self) -> WRPS_2_R {
        WRPS_2_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:27 - Protected area start address for bank 2
    #[inline(always)]
    pub fn pa_beg_2(&self) -> PA_BEG_2_R {
        PA_BEG_2_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
///SYSCFG user register 9
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ur9](index.html) module
pub struct UR9_SPEC;
impl crate::RegisterSpec for UR9_SPEC {
    type Ux = u32;
}
///`read()` method returns [ur9::R](R) reader structure
impl crate::Readable for UR9_SPEC {
    type Reader = R;
}
///`reset()` method sets UR9 to value 0
impl crate::Resettable for UR9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
