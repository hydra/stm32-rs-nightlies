///Register `CRYP_DOUT` reader
pub struct R(crate::R<CRYP_DOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYP_DOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYP_DOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYP_DOUT_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DATAOUT` reader - DATAOUT
pub type DATAOUT_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - DATAOUT
    #[inline(always)]
    pub fn dataout(&self) -> DATAOUT_R {
        DATAOUT_R::new(self.bits)
    }
}
///The CRYP_DOUT register is the data output register. It is read-only and 32-bit wide. It is used to retrieve from the output FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DOUT register is read, the last data entered into the output FIFO (pointed to by the read pointer) is returned.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cryp_dout](index.html) module
pub struct CRYP_DOUT_SPEC;
impl crate::RegisterSpec for CRYP_DOUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [cryp_dout::R](R) reader structure
impl crate::Readable for CRYP_DOUT_SPEC {
    type Reader = R;
}
///`reset()` method sets CRYP_DOUT to value 0
impl crate::Resettable for CRYP_DOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
