///Register `LPDMA_C0TR1` reader
pub struct R(crate::R<LPDMA_C0TR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPDMA_C0TR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPDMA_C0TR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPDMA_C0TR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LPDMA_C0TR1` writer
pub struct W(crate::W<LPDMA_C0TR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPDMA_C0TR1_SPEC>;
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
impl From<crate::W<LPDMA_C0TR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPDMA_C0TR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SDW_LOG2` reader - binary logarithm of the source data width of a single in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. a source block size must be a multiple of the source data width (LPDMA_CxBR1.BNDT\[2:0\]
///versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and no transfer is issued. A source single transfer must have an aligned address with its data width (start address LPDMA_CxSAR\[2:0\]
///versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and none transfer is issued.
pub type SDW_LOG2_R = crate::FieldReader<u8, u8>;
///Field `SDW_LOG2` writer - binary logarithm of the source data width of a single in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. a source block size must be a multiple of the source data width (LPDMA_CxBR1.BNDT\[2:0\]
///versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and no transfer is issued. A source single transfer must have an aligned address with its data width (start address LPDMA_CxSAR\[2:0\]
///versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and none transfer is issued.
pub type SDW_LOG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPDMA_C0TR1_SPEC, u8, u8, 2, O>;
///Field `SINC` reader - source incrementing single The source address, pointed by LPDMA_CxSAR, is kept constant after a single transfer or is incremented by the offset value corresponding to a contiguous data after a single transfer.
pub type SINC_R = crate::BitReader<bool>;
///Field `SINC` writer - source incrementing single The source address, pointed by LPDMA_CxSAR, is kept constant after a single transfer or is incremented by the offset value corresponding to a contiguous data after a single transfer.
pub type SINC_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_C0TR1_SPEC, bool, O>;
///Field `PAM` reader - padding/alignment mode If DDW_LOG2\[1:0\]=SDW_LOG2\[1:0\]: if the data width of a single destination transfer is equal to the data width of a single source transfer, this bit is ignored. Else: Case 1: If destination data width > source data width Case 2: If destination data width &lt; source data width
pub type PAM_R = crate::BitReader<bool>;
///Field `PAM` writer - padding/alignment mode If DDW_LOG2\[1:0\]=SDW_LOG2\[1:0\]: if the data width of a single destination transfer is equal to the data width of a single source transfer, this bit is ignored. Else: Case 1: If destination data width > source data width Case 2: If destination data width &lt; source data width
pub type PAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_C0TR1_SPEC, bool, O>;
///Field `SSEC` reader - security attribute of the LPDMA transfer from the source If LPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when LPDMA_SECCFGR.SECx =1 . A secure write is ignored when LPDMA_SECCFGR.SECx = 0. When LPDMA_SECCFGR.SECx is de-asserted, this SSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the LPDMA transfer from the source is non-secure.
pub type SSEC_R = crate::BitReader<bool>;
///Field `SSEC` writer - security attribute of the LPDMA transfer from the source If LPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when LPDMA_SECCFGR.SECx =1 . A secure write is ignored when LPDMA_SECCFGR.SECx = 0. When LPDMA_SECCFGR.SECx is de-asserted, this SSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the LPDMA transfer from the source is non-secure.
pub type SSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_C0TR1_SPEC, bool, O>;
///Field `DDW_LOG2` reader - binary logarithm of the destination data width of a single in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. A destination single transfer must have an aligned address with its data width (start address LPDMA_CxDAR\[2:0\]
///versus DDW_LOG2\[1:0\]). Otherwise a user setting error is reported and none transfer is issued.
pub type DDW_LOG2_R = crate::FieldReader<u8, u8>;
///Field `DDW_LOG2` writer - binary logarithm of the destination data width of a single in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. A destination single transfer must have an aligned address with its data width (start address LPDMA_CxDAR\[2:0\]
///versus DDW_LOG2\[1:0\]). Otherwise a user setting error is reported and none transfer is issued.
pub type DDW_LOG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPDMA_C0TR1_SPEC, u8, u8, 2, O>;
///Field `DINC` reader - destination incrementing single The destination address, pointed by LPDMA_CxDAR, is kept constant after a single transfer, or is incremented by the offset value corresponding to a contiguous data after a single transfer.
pub type DINC_R = crate::BitReader<bool>;
///Field `DINC` writer - destination incrementing single The destination address, pointed by LPDMA_CxDAR, is kept constant after a single transfer, or is incremented by the offset value corresponding to a contiguous data after a single transfer.
pub type DINC_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_C0TR1_SPEC, bool, O>;
///Field `DSEC` reader - security attribute of the LPDMA transfer to the destination If LPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when LPDMA_SECCFGR.SECx = 1. A secure write is ignored when LPDMA_SECCFGR.SECx = 0. When LPDMA_SECCFGR.SECx is de-asserted, this DSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the LPDMA transfer to the destination is non-secure.
pub type DSEC_R = crate::BitReader<bool>;
///Field `DSEC` writer - security attribute of the LPDMA transfer to the destination If LPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when LPDMA_SECCFGR.SECx = 1. A secure write is ignored when LPDMA_SECCFGR.SECx = 0. When LPDMA_SECCFGR.SECx is de-asserted, this DSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the LPDMA transfer to the destination is non-secure.
pub type DSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_C0TR1_SPEC, bool, O>;
impl R {
    ///Bits 0:1 - binary logarithm of the source data width of a single in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. a source block size must be a multiple of the source data width (LPDMA_CxBR1.BNDT\[2:0\]
    ///versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and no transfer is issued. A source single transfer must have an aligned address with its data width (start address LPDMA_CxSAR\[2:0\]
    ///versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and none transfer is issued.
    #[inline(always)]
    pub fn sdw_log2(&self) -> SDW_LOG2_R {
        SDW_LOG2_R::new((self.bits & 3) as u8)
    }
    ///Bit 3 - source incrementing single The source address, pointed by LPDMA_CxSAR, is kept constant after a single transfer or is incremented by the offset value corresponding to a contiguous data after a single transfer.
    #[inline(always)]
    pub fn sinc(&self) -> SINC_R {
        SINC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 11 - padding/alignment mode If DDW_LOG2\[1:0\]=SDW_LOG2\[1:0\]: if the data width of a single destination transfer is equal to the data width of a single source transfer, this bit is ignored. Else: Case 1: If destination data width > source data width Case 2: If destination data width &lt; source data width
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - security attribute of the LPDMA transfer from the source If LPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when LPDMA_SECCFGR.SECx =1 . A secure write is ignored when LPDMA_SECCFGR.SECx = 0. When LPDMA_SECCFGR.SECx is de-asserted, this SSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the LPDMA transfer from the source is non-secure.
    #[inline(always)]
    pub fn ssec(&self) -> SSEC_R {
        SSEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - binary logarithm of the destination data width of a single in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. A destination single transfer must have an aligned address with its data width (start address LPDMA_CxDAR\[2:0\]
    ///versus DDW_LOG2\[1:0\]). Otherwise a user setting error is reported and none transfer is issued.
    #[inline(always)]
    pub fn ddw_log2(&self) -> DDW_LOG2_R {
        DDW_LOG2_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 19 - destination incrementing single The destination address, pointed by LPDMA_CxDAR, is kept constant after a single transfer, or is incremented by the offset value corresponding to a contiguous data after a single transfer.
    #[inline(always)]
    pub fn dinc(&self) -> DINC_R {
        DINC_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 31 - security attribute of the LPDMA transfer to the destination If LPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when LPDMA_SECCFGR.SECx = 1. A secure write is ignored when LPDMA_SECCFGR.SECx = 0. When LPDMA_SECCFGR.SECx is de-asserted, this DSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the LPDMA transfer to the destination is non-secure.
    #[inline(always)]
    pub fn dsec(&self) -> DSEC_R {
        DSEC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - binary logarithm of the source data width of a single in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. a source block size must be a multiple of the source data width (LPDMA_CxBR1.BNDT\[2:0\]
    ///versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and no transfer is issued. A source single transfer must have an aligned address with its data width (start address LPDMA_CxSAR\[2:0\]
    ///versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and none transfer is issued.
    #[inline(always)]
    #[must_use]
    pub fn sdw_log2(&mut self) -> SDW_LOG2_W<0> {
        SDW_LOG2_W::new(self)
    }
    ///Bit 3 - source incrementing single The source address, pointed by LPDMA_CxSAR, is kept constant after a single transfer or is incremented by the offset value corresponding to a contiguous data after a single transfer.
    #[inline(always)]
    #[must_use]
    pub fn sinc(&mut self) -> SINC_W<3> {
        SINC_W::new(self)
    }
    ///Bit 11 - padding/alignment mode If DDW_LOG2\[1:0\]=SDW_LOG2\[1:0\]: if the data width of a single destination transfer is equal to the data width of a single source transfer, this bit is ignored. Else: Case 1: If destination data width > source data width Case 2: If destination data width &lt; source data width
    #[inline(always)]
    #[must_use]
    pub fn pam(&mut self) -> PAM_W<11> {
        PAM_W::new(self)
    }
    ///Bit 15 - security attribute of the LPDMA transfer from the source If LPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when LPDMA_SECCFGR.SECx =1 . A secure write is ignored when LPDMA_SECCFGR.SECx = 0. When LPDMA_SECCFGR.SECx is de-asserted, this SSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the LPDMA transfer from the source is non-secure.
    #[inline(always)]
    #[must_use]
    pub fn ssec(&mut self) -> SSEC_W<15> {
        SSEC_W::new(self)
    }
    ///Bits 16:17 - binary logarithm of the destination data width of a single in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. A destination single transfer must have an aligned address with its data width (start address LPDMA_CxDAR\[2:0\]
    ///versus DDW_LOG2\[1:0\]). Otherwise a user setting error is reported and none transfer is issued.
    #[inline(always)]
    #[must_use]
    pub fn ddw_log2(&mut self) -> DDW_LOG2_W<16> {
        DDW_LOG2_W::new(self)
    }
    ///Bit 19 - destination incrementing single The destination address, pointed by LPDMA_CxDAR, is kept constant after a single transfer, or is incremented by the offset value corresponding to a contiguous data after a single transfer.
    #[inline(always)]
    #[must_use]
    pub fn dinc(&mut self) -> DINC_W<19> {
        DINC_W::new(self)
    }
    ///Bit 31 - security attribute of the LPDMA transfer to the destination If LPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when LPDMA_SECCFGR.SECx = 1. A secure write is ignored when LPDMA_SECCFGR.SECx = 0. When LPDMA_SECCFGR.SECx is de-asserted, this DSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the LPDMA transfer to the destination is non-secure.
    #[inline(always)]
    #[must_use]
    pub fn dsec(&mut self) -> DSEC_W<31> {
        DSEC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LPDMA channel 0 transfer register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lpdma_c0tr1](index.html) module
pub struct LPDMA_C0TR1_SPEC;
impl crate::RegisterSpec for LPDMA_C0TR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [lpdma_c0tr1::R](R) reader structure
impl crate::Readable for LPDMA_C0TR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lpdma_c0tr1::W](W) writer structure
impl crate::Writable for LPDMA_C0TR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LPDMA_C0TR1 to value 0
impl crate::Resettable for LPDMA_C0TR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
