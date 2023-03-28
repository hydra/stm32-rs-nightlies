///Register `ECCDR` reader
pub struct R(crate::R<ECCDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DATA_ECC` reader - ECC error data When an double detection ECC error occurs on special areas with 6-bit ECC on 16-bit of data (data area, read-only/OTP area), the failing data is read to this register. By checking if it is possible to determine whether the failure was on a real data, or due to access to uninitialized memory.
pub type DATA_ECC_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - ECC error data When an double detection ECC error occurs on special areas with 6-bit ECC on 16-bit of data (data area, read-only/OTP area), the failing data is read to this register. By checking if it is possible to determine whether the failure was on a real data, or due to access to uninitialized memory.
    #[inline(always)]
    pub fn data_ecc(&self) -> DATA_ECC_R {
        DATA_ECC_R::new((self.bits & 0xffff) as u16)
    }
}
///FLASH ECC data
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eccdr](index.html) module
pub struct ECCDR_SPEC;
impl crate::RegisterSpec for ECCDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eccdr::R](R) reader structure
impl crate::Readable for ECCDR_SPEC {
    type Reader = R;
}
///`reset()` method sets ECCDR to value 0
impl crate::Resettable for ECCDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
