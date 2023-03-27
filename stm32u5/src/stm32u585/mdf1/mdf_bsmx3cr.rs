///Register `MDF_BSMX3CR` reader
pub struct R(crate::R<MDF_BSMX3CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDF_BSMX3CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDF_BSMX3CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDF_BSMX3CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MDF_BSMX3CR` writer
pub struct W(crate::W<MDF_BSMX3CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDF_BSMX3CR_SPEC>;
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
impl From<crate::W<MDF_BSMX3CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDF_BSMX3CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BSSEL` reader - Bitstream Selection Set and cleared by software. This field is used to select the bitstream to be processed for the digital filter x and for the SCDx. The size of this field depends on the number of DFLTx instantiated. If the BSSEL is selecting an input which is not instantiated, the MDF will select the valid stream bs\[x\]_F having the higher index number. - 00000: The bitstream bs\[0\]_R is provided to DFLTx and SCDx - 00001: The bitstream bs\[0\]_F is provided to DFLTx and SCDx - 00010: The bitstream bs\[1\]_R is provided to DFLTx and SCDx (if instantiated) - 00011: The bitstream bs\[1\]_F is provided to DFLTx and SCDx (if instantiated) ... - 11110: The bitstream bs\[15\]_R is provided to DFLTx and SCDx (if instantiated) - 11111: The bitstream bs\[15\]_F is provided to DFLTx and SCDx (if instantiated) This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type BSSEL_R = crate::FieldReader<u8, u8>;
///Field `BSSEL` writer - Bitstream Selection Set and cleared by software. This field is used to select the bitstream to be processed for the digital filter x and for the SCDx. The size of this field depends on the number of DFLTx instantiated. If the BSSEL is selecting an input which is not instantiated, the MDF will select the valid stream bs\[x\]_F having the higher index number. - 00000: The bitstream bs\[0\]_R is provided to DFLTx and SCDx - 00001: The bitstream bs\[0\]_F is provided to DFLTx and SCDx - 00010: The bitstream bs\[1\]_R is provided to DFLTx and SCDx (if instantiated) - 00011: The bitstream bs\[1\]_F is provided to DFLTx and SCDx (if instantiated) ... - 11110: The bitstream bs\[15\]_R is provided to DFLTx and SCDx (if instantiated) - 11111: The bitstream bs\[15\]_F is provided to DFLTx and SCDx (if instantiated) This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type BSSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDF_BSMX3CR_SPEC, u8, u8, 5, O>;
///Field `BSMXACTIVE` reader - BSMX Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the BSMX is effectively enabled (active) or not. BSSEL\[4:0\]
///can only be updated when the BSMXACTIVE is set to a . The BSMXACTIVE flag is a logical between OLDACTIVE, DFLTACTIVE, and SCDACTIVE flags. Both of them must be set to a in order update BSSEL\[4:0\]
///field. - 0: The BSMX is not active, and can be configured if needed - 1: The BSMX is active, and protected fields cannot be configured.
pub type BSMXACTIVE_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:4 - Bitstream Selection Set and cleared by software. This field is used to select the bitstream to be processed for the digital filter x and for the SCDx. The size of this field depends on the number of DFLTx instantiated. If the BSSEL is selecting an input which is not instantiated, the MDF will select the valid stream bs\[x\]_F having the higher index number. - 00000: The bitstream bs\[0\]_R is provided to DFLTx and SCDx - 00001: The bitstream bs\[0\]_F is provided to DFLTx and SCDx - 00010: The bitstream bs\[1\]_R is provided to DFLTx and SCDx (if instantiated) - 00011: The bitstream bs\[1\]_F is provided to DFLTx and SCDx (if instantiated) ... - 11110: The bitstream bs\[15\]_R is provided to DFLTx and SCDx (if instantiated) - 11111: The bitstream bs\[15\]_F is provided to DFLTx and SCDx (if instantiated) This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn bssel(&self) -> BSSEL_R {
        BSSEL_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 31 - BSMX Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the BSMX is effectively enabled (active) or not. BSSEL\[4:0\]
    ///can only be updated when the BSMXACTIVE is set to a . The BSMXACTIVE flag is a logical between OLDACTIVE, DFLTACTIVE, and SCDACTIVE flags. Both of them must be set to a in order update BSSEL\[4:0\]
    ///field. - 0: The BSMX is not active, and can be configured if needed - 1: The BSMX is active, and protected fields cannot be configured.
    #[inline(always)]
    pub fn bsmxactive(&self) -> BSMXACTIVE_R {
        BSMXACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:4 - Bitstream Selection Set and cleared by software. This field is used to select the bitstream to be processed for the digital filter x and for the SCDx. The size of this field depends on the number of DFLTx instantiated. If the BSSEL is selecting an input which is not instantiated, the MDF will select the valid stream bs\[x\]_F having the higher index number. - 00000: The bitstream bs\[0\]_R is provided to DFLTx and SCDx - 00001: The bitstream bs\[0\]_F is provided to DFLTx and SCDx - 00010: The bitstream bs\[1\]_R is provided to DFLTx and SCDx (if instantiated) - 00011: The bitstream bs\[1\]_F is provided to DFLTx and SCDx (if instantiated) ... - 11110: The bitstream bs\[15\]_R is provided to DFLTx and SCDx (if instantiated) - 11111: The bitstream bs\[15\]_F is provided to DFLTx and SCDx (if instantiated) This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    #[must_use]
    pub fn bssel(&mut self) -> BSSEL_W<0> {
        BSSEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdf_bsmx3cr](index.html) module
pub struct MDF_BSMX3CR_SPEC;
impl crate::RegisterSpec for MDF_BSMX3CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdf_bsmx3cr::R](R) reader structure
impl crate::Readable for MDF_BSMX3CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mdf_bsmx3cr::W](W) writer structure
impl crate::Writable for MDF_BSMX3CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MDF_BSMX3CR to value 0
impl crate::Resettable for MDF_BSMX3CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
