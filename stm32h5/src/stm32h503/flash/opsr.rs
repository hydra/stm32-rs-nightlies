///Register `OPSR` reader
pub struct R(crate::R<OPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ADDR_OP` reader - Interrupted operation address.
pub type ADDR_OP_R = crate::FieldReader<u32, u32>;
///Field `BK_OP` reader - Interrupted operation bank It indicates which bank was concerned by operation.
pub type BK_OP_R = crate::BitReader<bool>;
///Field `SYSF_OP` reader - Operation in system Flash memory interrupted Indicates that reset interrupted an ongoing operation in System Flash.
pub type SYSF_OP_R = crate::BitReader<bool>;
///Field `OTP_OP` reader - OTP operation interrupted Indicates that reset interrupted an ongoing operation in OTP area.
pub type OTP_OP_R = crate::BitReader<bool>;
///Field `CODE_OP` reader - Flash memory operation code
pub type CODE_OP_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:19 - Interrupted operation address.
    #[inline(always)]
    pub fn addr_op(&self) -> ADDR_OP_R {
        ADDR_OP_R::new(self.bits & 0x000f_ffff)
    }
    ///Bit 22 - Interrupted operation bank It indicates which bank was concerned by operation.
    #[inline(always)]
    pub fn bk_op(&self) -> BK_OP_R {
        BK_OP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Operation in system Flash memory interrupted Indicates that reset interrupted an ongoing operation in System Flash.
    #[inline(always)]
    pub fn sysf_op(&self) -> SYSF_OP_R {
        SYSF_OP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - OTP operation interrupted Indicates that reset interrupted an ongoing operation in OTP area.
    #[inline(always)]
    pub fn otp_op(&self) -> OTP_OP_R {
        OTP_OP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 29:31 - Flash memory operation code
    #[inline(always)]
    pub fn code_op(&self) -> CODE_OP_R {
        CODE_OP_R::new(((self.bits >> 29) & 7) as u8)
    }
}
///FLASH operation status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [opsr](index.html) module
pub struct OPSR_SPEC;
impl crate::RegisterSpec for OPSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [opsr::R](R) reader structure
impl crate::Readable for OPSR_SPEC {
    type Reader = R;
}
///`reset()` method sets OPSR to value 0
impl crate::Resettable for OPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
