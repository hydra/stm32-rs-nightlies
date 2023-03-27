///Register `RDR` reader
pub struct R(crate::R<RDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RDB0` reader - 8-bit received data on I3C bus.
pub type RDB0_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - 8-bit received data on I3C bus.
    #[inline(always)]
    pub fn rdb0(&self) -> RDB0_R {
        RDB0_R::new((self.bits & 0xff) as u8)
    }
}
///I3C receive data byte register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rdr](index.html) module
pub struct RDR_SPEC;
impl crate::RegisterSpec for RDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rdr::R](R) reader structure
impl crate::Readable for RDR_SPEC {
    type Reader = R;
}
///`reset()` method sets RDR to value 0
impl crate::Resettable for RDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
