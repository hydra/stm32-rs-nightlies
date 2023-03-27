///Register `FLASH_OPSR` reader
pub struct R(crate::R<FLASH_OPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_OPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_OPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_OPSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ADDR_OP` reader - Interrupted operation address This field indicates which address in the Flash memory was accessed when reset occurred. The address is given by bank from address 0x0 0000 to 0xF FFF0.
pub type ADDR_OP_R = crate::FieldReader<u32, u32>;
///Field `BK_OP` reader - Interrupted operation bank This bit indicates which Flash memory bank was accessed when reset occurred
pub type BK_OP_R = crate::BitReader<bool>;
///Field `SYSF_OP` reader - Operation in system Flash memory interrupted This bit indicates that the reset occurred during an operation in the system Flash memory.
pub type SYSF_OP_R = crate::BitReader<bool>;
///Field `CODE_OP` reader - Flash memory operation code This field indicates which Flash memory operation has been interrupted by a system reset:
pub type CODE_OP_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:19 - Interrupted operation address This field indicates which address in the Flash memory was accessed when reset occurred. The address is given by bank from address 0x0 0000 to 0xF FFF0.
    #[inline(always)]
    pub fn addr_op(&self) -> ADDR_OP_R {
        ADDR_OP_R::new(self.bits & 0x000f_ffff)
    }
    ///Bit 21 - Interrupted operation bank This bit indicates which Flash memory bank was accessed when reset occurred
    #[inline(always)]
    pub fn bk_op(&self) -> BK_OP_R {
        BK_OP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Operation in system Flash memory interrupted This bit indicates that the reset occurred during an operation in the system Flash memory.
    #[inline(always)]
    pub fn sysf_op(&self) -> SYSF_OP_R {
        SYSF_OP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 29:31 - Flash memory operation code This field indicates which Flash memory operation has been interrupted by a system reset:
    #[inline(always)]
    pub fn code_op(&self) -> CODE_OP_R {
        CODE_OP_R::new(((self.bits >> 29) & 7) as u8)
    }
}
///FLASH operation status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flash_opsr](index.html) module
pub struct FLASH_OPSR_SPEC;
impl crate::RegisterSpec for FLASH_OPSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [flash_opsr::R](R) reader structure
impl crate::Readable for FLASH_OPSR_SPEC {
    type Reader = R;
}
///`reset()` method sets FLASH_OPSR to value 0
impl crate::Resettable for FLASH_OPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
