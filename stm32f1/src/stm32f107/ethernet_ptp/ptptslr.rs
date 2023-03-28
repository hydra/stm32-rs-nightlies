///Register `PTPTSLR` reader
pub struct R(crate::R<PTPTSLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPTSLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPTSLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPTSLR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `STSS` reader - System time subseconds
pub type STSS_R = crate::FieldReader<u32, u32>;
///Field `STPNS` reader - System time positive or negative sign
pub type STPNS_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:30 - System time subseconds
    #[inline(always)]
    pub fn stss(&self) -> STSS_R {
        STSS_R::new(self.bits & 0x7fff_ffff)
    }
    ///Bit 31 - System time positive or negative sign
    #[inline(always)]
    pub fn stpns(&self) -> STPNS_R {
        STPNS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
///Ethernet PTP time stamp low register (ETH_PTPTSLR)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ptptslr](index.html) module
pub struct PTPTSLR_SPEC;
impl crate::RegisterSpec for PTPTSLR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ptptslr::R](R) reader structure
impl crate::Readable for PTPTSLR_SPEC {
    type Reader = R;
}
///`reset()` method sets PTPTSLR to value 0
impl crate::Resettable for PTPTSLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
