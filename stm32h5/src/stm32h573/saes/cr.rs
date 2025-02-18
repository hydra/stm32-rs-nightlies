///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EN` reader - SAES enable This bit enables/disables the SAES peripheral: At any moment, clearing then setting the bit re-initializes the SAES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase. The bit cannot be set as long as KEYVALID = 0 nor along with the following settings: KMOD = 01 + CHMOD = 011 and KMOD = 01 + CHMOD = 010 + MODE = 00. Note: With KMOD\[1:0\]
///other than 00, use the IPRST bit rather than the bit EN.
pub type EN_R = crate::BitReader<bool>;
///Field `EN` writer - SAES enable This bit enables/disables the SAES peripheral: At any moment, clearing then setting the bit re-initializes the SAES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase. The bit cannot be set as long as KEYVALID = 0 nor along with the following settings: KMOD = 01 + CHMOD = 011 and KMOD = 01 + CHMOD = 010 + MODE = 00. Note: With KMOD\[1:0\]
///other than 00, use the IPRST bit rather than the bit EN.
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `DATATYPE` reader - Data type selection This bitfield defines the format of data written in the SAES_DINR register or read from the SAES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type DATATYPE_R = crate::FieldReader<u8, u8>;
///Field `DATATYPE` writer - Data type selection This bitfield defines the format of data written in the SAES_DINR register or read from the SAES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type DATATYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
///Field `MODE` reader - SAES operating mode This bitfield selects the SAES operating mode: Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type MODE_R = crate::FieldReader<u8, u8>;
///Field `MODE` writer - SAES operating mode This bitfield selects the SAES operating mode: Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
///Field `CHMOD1` reader - Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type CHMOD1_R = crate::FieldReader<u8, u8>;
///Field `CHMOD1` writer - Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type CHMOD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
///Field `DMAINEN` reader - DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by SAES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\]
///bitfield. It is not effective for Mode 2 (key derivation).
pub type DMAINEN_R = crate::BitReader<bool>;
///Field `DMAINEN` writer - DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by SAES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\]
///bitfield. It is not effective for Mode 2 (key derivation).
pub type DMAINEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `DMAOUTEN` reader - DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by SAES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\]
///bitfield. It is not effective for Mode 2 (key derivation).
pub type DMAOUTEN_R = crate::BitReader<bool>;
///Field `DMAOUTEN` writer - DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by SAES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\]
///bitfield. It is not effective for Mode 2 (key derivation).
pub type DMAOUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `GCMPH` reader - GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield).
pub type GCMPH_R = crate::FieldReader<u8, u8>;
///Field `GCMPH` writer - GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield).
pub type GCMPH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
///Field `CHMOD2` reader - Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type CHMOD2_R = crate::BitReader<bool>;
///Field `CHMOD2` writer - Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type CHMOD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `KEYSIZE` reader - Key size selection This bitfield defines the length of the key used in the SAES cryptographic core, in bits: When KMOD\[1:0\]=01 or 10 KEYSIZE also defines the length of the key to encrypt or decrypt. Attempts to write the bit are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type KEYSIZE_R = crate::BitReader<bool>;
///Field `KEYSIZE` writer - Key size selection This bitfield defines the length of the key used in the SAES cryptographic core, in bits: When KMOD\[1:0\]=01 or 10 KEYSIZE also defines the length of the key to encrypt or decrypt. Attempts to write the bit are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type KEYSIZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `KEYPROT` reader - Key protection When set, hardware-based key protection is enabled. Attempts to write the bit are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type KEYPROT_R = crate::BitReader<bool>;
///Field `KEYPROT` writer - Key protection When set, hardware-based key protection is enabled. Attempts to write the bit are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type KEYPROT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `NPBLB` reader - Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ...
pub type NPBLB_R = crate::FieldReader<u8, u8>;
///Field `NPBLB` writer - Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ...
pub type NPBLB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 4, O>;
///Field `KMOD` reader - Key mode selection The bitfield defines how the SAES key can be used by the application: Others: Reserved With normal key selection, the key registers are freely usable, no specific usage or protection applies to SAES_DIN and SAES_DOUT registers. With wrapped key selection, the key loaded in key registers can only be used to encrypt or decrypt AES keys. Hence, when a decryption is selected in Wrapped-key mode read-as-zero SAES_DOUT register is automatically loaded into SAES key registers after a successful decryption process. With shared key selection, after a successful decryption process, SAES key registers are shared with the peripheral described in KSHAREID(1:0\]
///bitfield. This sharing is valid only while KMOD\[1:0\]=10 and KEYVALID = 1. When a decryption is selected, read-as-zero SAES_DOUT register is automatically loaded into SAES key registers after a successful decryption process. With KMOD\[1:0\]
///other than zero, any attempt to configure the SAES peripheral for use by an application belonging to a different security domain (secure or non-secure) results in automatic key erasure and setting of the KEIF flag.
///Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type KMOD_R = crate::FieldReader<u8, u8>;
///Field `KMOD` writer - Key mode selection The bitfield defines how the SAES key can be used by the application: Others: Reserved With normal key selection, the key registers are freely usable, no specific usage or protection applies to SAES_DIN and SAES_DOUT registers. With wrapped key selection, the key loaded in key registers can only be used to encrypt or decrypt AES keys. Hence, when a decryption is selected in Wrapped-key mode read-as-zero SAES_DOUT register is automatically loaded into SAES key registers after a successful decryption process. With shared key selection, after a successful decryption process, SAES key registers are shared with the peripheral described in KSHAREID(1:0\]
///bitfield. This sharing is valid only while KMOD\[1:0\]=10 and KEYVALID = 1. When a decryption is selected, read-as-zero SAES_DOUT register is automatically loaded into SAES key registers after a successful decryption process. With KMOD\[1:0\]
///other than zero, any attempt to configure the SAES peripheral for use by an application belonging to a different security domain (secure or non-secure) results in automatic key erasure and setting of the KEIF flag.
///Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type KMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
///Field `KSHAREID` reader - Key share identification This bitfield defines, at the end of a decryption process with KMOD\[1:0\]=10 (shared key), which target can read the SAES key registers using a dedicated hardware bus. Others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type KSHAREID_R = crate::FieldReader<u8, u8>;
///Field `KSHAREID` writer - Key share identification This bitfield defines, at the end of a decryption process with KMOD\[1:0\]=10 (shared key), which target can read the SAES key registers using a dedicated hardware bus. Others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type KSHAREID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
///Field `KEYSEL` reader - Key selection The bitfield defines the source of the key information to use in the AES cryptographic core. Others: Reserved (if used, unfreeze SAES with IPRST) When KEYSEL is different from zero, selected key value is available in key registers when BUSY bit is cleared and KEYVALID is set in the SAES_SR register. Otherwise, the key error flag KEIF is set. Repeated writing of KEYSEL\[2:0\]
///with the same non-zero value only triggers the loading of DHUK or BHK if KEYVALID = 0. When the application software changes the key selection by writing the KEYSEL\[2:0\]
///bitfield, the key registers are immediately erased and the KEYVALID flag cleared. At the end of the decryption process, if KMOD\[1:0\]
///is other than zero, KEYSEL\[2:0\]
///is cleared. With the bitfield value other than zero and KEYVALID set, the application cannot transfer the ownership of SAES with a loaded key to an application running in another security context (such as secure, non-secure). More specifically, when security of an access to any register does not match the information recorded by SAES, the KEIF flag is set. Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type KEYSEL_R = crate::FieldReader<u8, u8>;
///Field `KEYSEL` writer - Key selection The bitfield defines the source of the key information to use in the AES cryptographic core. Others: Reserved (if used, unfreeze SAES with IPRST) When KEYSEL is different from zero, selected key value is available in key registers when BUSY bit is cleared and KEYVALID is set in the SAES_SR register. Otherwise, the key error flag KEIF is set. Repeated writing of KEYSEL\[2:0\]
///with the same non-zero value only triggers the loading of DHUK or BHK if KEYVALID = 0. When the application software changes the key selection by writing the KEYSEL\[2:0\]
///bitfield, the key registers are immediately erased and the KEYVALID flag cleared. At the end of the decryption process, if KMOD\[1:0\]
///is other than zero, KEYSEL\[2:0\]
///is cleared. With the bitfield value other than zero and KEYVALID set, the application cannot transfer the ownership of SAES with a loaded key to an application running in another security context (such as secure, non-secure). More specifically, when security of an access to any register does not match the information recorded by SAES, the KEIF flag is set. Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type KEYSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
///Field `IPRST` reader - SAES peripheral software reset Setting the bit resets the SAES peripheral, putting all registers to their default values, except the IPRST bit itself and the SAES_DPACFG register. Hence, any key-relative data is lost. For this reason, it is recommended to set the bit before handing over the SAES to a less secure application. The bit must be low while writing any configuration registers.
pub type IPRST_R = crate::BitReader<bool>;
///Field `IPRST` writer - SAES peripheral software reset Setting the bit resets the SAES peripheral, putting all registers to their default values, except the IPRST bit itself and the SAES_DPACFG register. Hence, any key-relative data is lost. For this reason, it is recommended to set the bit before handing over the SAES to a less secure application. The bit must be low while writing any configuration registers.
pub type IPRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    ///Bit 0 - SAES enable This bit enables/disables the SAES peripheral: At any moment, clearing then setting the bit re-initializes the SAES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase. The bit cannot be set as long as KEYVALID = 0 nor along with the following settings: KMOD = 01 + CHMOD = 011 and KMOD = 01 + CHMOD = 010 + MODE = 00. Note: With KMOD\[1:0\]
    ///other than 00, use the IPRST bit rather than the bit EN.
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Data type selection This bitfield defines the format of data written in the SAES_DINR register or read from the SAES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - SAES operating mode This bitfield selects the SAES operating mode: Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6 - Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn chmod1(&self) -> CHMOD1_R {
        CHMOD1_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 11 - DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by SAES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\]
    ///bitfield. It is not effective for Mode 2 (key derivation).
    #[inline(always)]
    pub fn dmainen(&self) -> DMAINEN_R {
        DMAINEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by SAES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\]
    ///bitfield. It is not effective for Mode 2 (key derivation).
    #[inline(always)]
    pub fn dmaouten(&self) -> DMAOUTEN_R {
        DMAOUTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield).
    #[inline(always)]
    pub fn gcmph(&self) -> GCMPH_R {
        GCMPH_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 16 - Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn chmod2(&self) -> CHMOD2_R {
        CHMOD2_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Key size selection This bitfield defines the length of the key used in the SAES cryptographic core, in bits: When KMOD\[1:0\]=01 or 10 KEYSIZE also defines the length of the key to encrypt or decrypt. Attempts to write the bit are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Key protection When set, hardware-based key protection is enabled. Attempts to write the bit are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn keyprot(&self) -> KEYPROT_R {
        KEYPROT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:23 - Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ...
    #[inline(always)]
    pub fn npblb(&self) -> NPBLB_R {
        NPBLB_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:25 - Key mode selection The bitfield defines how the SAES key can be used by the application: Others: Reserved With normal key selection, the key registers are freely usable, no specific usage or protection applies to SAES_DIN and SAES_DOUT registers. With wrapped key selection, the key loaded in key registers can only be used to encrypt or decrypt AES keys. Hence, when a decryption is selected in Wrapped-key mode read-as-zero SAES_DOUT register is automatically loaded into SAES key registers after a successful decryption process. With shared key selection, after a successful decryption process, SAES key registers are shared with the peripheral described in KSHAREID(1:0\]
    ///bitfield. This sharing is valid only while KMOD\[1:0\]=10 and KEYVALID = 1. When a decryption is selected, read-as-zero SAES_DOUT register is automatically loaded into SAES key registers after a successful decryption process. With KMOD\[1:0\]
    ///other than zero, any attempt to configure the SAES peripheral for use by an application belonging to a different security domain (secure or non-secure) results in automatic key erasure and setting of the KEIF flag.
    ///Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn kmod(&self) -> KMOD_R {
        KMOD_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Key share identification This bitfield defines, at the end of a decryption process with KMOD\[1:0\]=10 (shared key), which target can read the SAES key registers using a dedicated hardware bus. Others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn kshareid(&self) -> KSHAREID_R {
        KSHAREID_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:30 - Key selection The bitfield defines the source of the key information to use in the AES cryptographic core. Others: Reserved (if used, unfreeze SAES with IPRST) When KEYSEL is different from zero, selected key value is available in key registers when BUSY bit is cleared and KEYVALID is set in the SAES_SR register. Otherwise, the key error flag KEIF is set. Repeated writing of KEYSEL\[2:0\]
    ///with the same non-zero value only triggers the loading of DHUK or BHK if KEYVALID = 0. When the application software changes the key selection by writing the KEYSEL\[2:0\]
    ///bitfield, the key registers are immediately erased and the KEYVALID flag cleared. At the end of the decryption process, if KMOD\[1:0\]
    ///is other than zero, KEYSEL\[2:0\]
    ///is cleared. With the bitfield value other than zero and KEYVALID set, the application cannot transfer the ownership of SAES with a loaded key to an application running in another security context (such as secure, non-secure). More specifically, when security of an access to any register does not match the information recorded by SAES, the KEIF flag is set. Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn keysel(&self) -> KEYSEL_R {
        KEYSEL_R::new(((self.bits >> 28) & 7) as u8)
    }
    ///Bit 31 - SAES peripheral software reset Setting the bit resets the SAES peripheral, putting all registers to their default values, except the IPRST bit itself and the SAES_DPACFG register. Hence, any key-relative data is lost. For this reason, it is recommended to set the bit before handing over the SAES to a less secure application. The bit must be low while writing any configuration registers.
    #[inline(always)]
    pub fn iprst(&self) -> IPRST_R {
        IPRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SAES enable This bit enables/disables the SAES peripheral: At any moment, clearing then setting the bit re-initializes the SAES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase. The bit cannot be set as long as KEYVALID = 0 nor along with the following settings: KMOD = 01 + CHMOD = 011 and KMOD = 01 + CHMOD = 010 + MODE = 00. Note: With KMOD\[1:0\]
    ///other than 00, use the IPRST bit rather than the bit EN.
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    ///Bits 1:2 - Data type selection This bitfield defines the format of data written in the SAES_DINR register or read from the SAES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    #[must_use]
    pub fn datatype(&mut self) -> DATATYPE_W<1> {
        DATATYPE_W::new(self)
    }
    ///Bits 3:4 - SAES operating mode This bitfield selects the SAES operating mode: Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<3> {
        MODE_W::new(self)
    }
    ///Bits 5:6 - Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    #[must_use]
    pub fn chmod1(&mut self) -> CHMOD1_W<5> {
        CHMOD1_W::new(self)
    }
    ///Bit 11 - DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by SAES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\]
    ///bitfield. It is not effective for Mode 2 (key derivation).
    #[inline(always)]
    #[must_use]
    pub fn dmainen(&mut self) -> DMAINEN_W<11> {
        DMAINEN_W::new(self)
    }
    ///Bit 12 - DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by SAES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\]
    ///bitfield. It is not effective for Mode 2 (key derivation).
    #[inline(always)]
    #[must_use]
    pub fn dmaouten(&mut self) -> DMAOUTEN_W<12> {
        DMAOUTEN_W::new(self)
    }
    ///Bits 13:14 - GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield).
    #[inline(always)]
    #[must_use]
    pub fn gcmph(&mut self) -> GCMPH_W<13> {
        GCMPH_W::new(self)
    }
    ///Bit 16 - Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    #[must_use]
    pub fn chmod2(&mut self) -> CHMOD2_W<16> {
        CHMOD2_W::new(self)
    }
    ///Bit 18 - Key size selection This bitfield defines the length of the key used in the SAES cryptographic core, in bits: When KMOD\[1:0\]=01 or 10 KEYSIZE also defines the length of the key to encrypt or decrypt. Attempts to write the bit are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    #[must_use]
    pub fn keysize(&mut self) -> KEYSIZE_W<18> {
        KEYSIZE_W::new(self)
    }
    ///Bit 19 - Key protection When set, hardware-based key protection is enabled. Attempts to write the bit are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    #[must_use]
    pub fn keyprot(&mut self) -> KEYPROT_W<19> {
        KEYPROT_W::new(self)
    }
    ///Bits 20:23 - Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ...
    #[inline(always)]
    #[must_use]
    pub fn npblb(&mut self) -> NPBLB_W<20> {
        NPBLB_W::new(self)
    }
    ///Bits 24:25 - Key mode selection The bitfield defines how the SAES key can be used by the application: Others: Reserved With normal key selection, the key registers are freely usable, no specific usage or protection applies to SAES_DIN and SAES_DOUT registers. With wrapped key selection, the key loaded in key registers can only be used to encrypt or decrypt AES keys. Hence, when a decryption is selected in Wrapped-key mode read-as-zero SAES_DOUT register is automatically loaded into SAES key registers after a successful decryption process. With shared key selection, after a successful decryption process, SAES key registers are shared with the peripheral described in KSHAREID(1:0\]
    ///bitfield. This sharing is valid only while KMOD\[1:0\]=10 and KEYVALID = 1. When a decryption is selected, read-as-zero SAES_DOUT register is automatically loaded into SAES key registers after a successful decryption process. With KMOD\[1:0\]
    ///other than zero, any attempt to configure the SAES peripheral for use by an application belonging to a different security domain (secure or non-secure) results in automatic key erasure and setting of the KEIF flag.
    ///Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    #[must_use]
    pub fn kmod(&mut self) -> KMOD_W<24> {
        KMOD_W::new(self)
    }
    ///Bits 26:27 - Key share identification This bitfield defines, at the end of a decryption process with KMOD\[1:0\]=10 (shared key), which target can read the SAES key registers using a dedicated hardware bus. Others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    #[must_use]
    pub fn kshareid(&mut self) -> KSHAREID_W<26> {
        KSHAREID_W::new(self)
    }
    ///Bits 28:30 - Key selection The bitfield defines the source of the key information to use in the AES cryptographic core. Others: Reserved (if used, unfreeze SAES with IPRST) When KEYSEL is different from zero, selected key value is available in key registers when BUSY bit is cleared and KEYVALID is set in the SAES_SR register. Otherwise, the key error flag KEIF is set. Repeated writing of KEYSEL\[2:0\]
    ///with the same non-zero value only triggers the loading of DHUK or BHK if KEYVALID = 0. When the application software changes the key selection by writing the KEYSEL\[2:0\]
    ///bitfield, the key registers are immediately erased and the KEYVALID flag cleared. At the end of the decryption process, if KMOD\[1:0\]
    ///is other than zero, KEYSEL\[2:0\]
    ///is cleared. With the bitfield value other than zero and KEYVALID set, the application cannot transfer the ownership of SAES with a loaded key to an application running in another security context (such as secure, non-secure). More specifically, when security of an access to any register does not match the information recorded by SAES, the KEIF flag is set. Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    #[must_use]
    pub fn keysel(&mut self) -> KEYSEL_W<28> {
        KEYSEL_W::new(self)
    }
    ///Bit 31 - SAES peripheral software reset Setting the bit resets the SAES peripheral, putting all registers to their default values, except the IPRST bit itself and the SAES_DPACFG register. Hence, any key-relative data is lost. For this reason, it is recommended to set the bit before handing over the SAES to a less secure application. The bit must be low while writing any configuration registers.
    #[inline(always)]
    #[must_use]
    pub fn iprst(&mut self) -> IPRST_W<31> {
        IPRST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SAES control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
