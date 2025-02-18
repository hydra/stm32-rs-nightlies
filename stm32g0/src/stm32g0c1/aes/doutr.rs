///Register `DOUTR` reader
pub struct R(crate::R<DOUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUTR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DOUT` reader - Output data word This read-only bitfield fetches a 32-bit output buffer. A four-fold sequential read of this bitfield, upon the computation completion (CCF set), virtually reads a complete 128-bit block of output data from the AES peripheral. Before reaching the output buffer, the data produced by the AES core are handled by the data swap block according to the DATATYPE\[1:0\]
///bitfield. Data weights from the first to the fourth read operation are: \[127:96\], \[95:64\], \[63:32\], and \[31:0\]. The data signification of the output data block depends on the AES operating mode: - Mode 1 (encryption): ciphertext - Mode 2 (key derivation): the bitfield is not used (AES_KEYRx registers used for output) - Mode 3 (decryption) and Mode 4 (key derivation then single decryption): plaintext The data swap operation is described in page 499.
pub type DOUT_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Output data word This read-only bitfield fetches a 32-bit output buffer. A four-fold sequential read of this bitfield, upon the computation completion (CCF set), virtually reads a complete 128-bit block of output data from the AES peripheral. Before reaching the output buffer, the data produced by the AES core are handled by the data swap block according to the DATATYPE\[1:0\]
    ///bitfield. Data weights from the first to the fourth read operation are: \[127:96\], \[95:64\], \[63:32\], and \[31:0\]. The data signification of the output data block depends on the AES operating mode: - Mode 1 (encryption): ciphertext - Mode 2 (key derivation): the bitfield is not used (AES_KEYRx registers used for output) - Mode 3 (decryption) and Mode 4 (key derivation then single decryption): plaintext The data swap operation is described in page 499.
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new(self.bits)
    }
}
///AES data output register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doutr](index.html) module
pub struct DOUTR_SPEC;
impl crate::RegisterSpec for DOUTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [doutr::R](R) reader structure
impl crate::Readable for DOUTR_SPEC {
    type Reader = R;
}
///`reset()` method sets DOUTR to value 0
impl crate::Resettable for DOUTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
