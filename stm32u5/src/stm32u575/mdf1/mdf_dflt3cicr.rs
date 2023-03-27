///Register `MDF_DFLT3CICR` reader
pub struct R(crate::R<MDF_DFLT3CICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDF_DFLT3CICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDF_DFLT3CICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDF_DFLT3CICR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MDF_DFLT3CICR` writer
pub struct W(crate::W<MDF_DFLT3CICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDF_DFLT3CICR_SPEC>;
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
impl From<crate::W<MDF_DFLT3CICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDF_DFLT3CICR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DATSRC` reader - Source data for the digital filter Set and cleared by software. 0x: Select the stream coming from the BSMX - 10: Select the stream coming from the ADCITF1 - 11: Select the stream coming from the ADCITF2 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type DATSRC_R = crate::FieldReader<u8, u8>;
///Field `DATSRC` writer - Source data for the digital filter Set and cleared by software. 0x: Select the stream coming from the BSMX - 10: Select the stream coming from the ADCITF1 - 11: Select the stream coming from the ADCITF2 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type DATSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDF_DFLT3CICR_SPEC, u8, u8, 2, O>;
///Field `CICMOD` reader - Select the CIC mode Set and cleared by software. This field allows the application to select the configuration and the order of the MCIC. When CICMOD\[2:0\]
///is equal to 0xx , the CIC is split into two filters: - The main CIC (MCIC) - The auxiliary CIC (ACIC), used for the out-off limit detector - 000: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in FastSinc filter - 001: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc1 filter - 010: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc2 filter - 011: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc3 filter - 100: The CIC is configured in single sinc4 filter others: The CIC is configured in single sinc5 filter This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type CICMOD_R = crate::FieldReader<u8, u8>;
///Field `CICMOD` writer - Select the CIC mode Set and cleared by software. This field allows the application to select the configuration and the order of the MCIC. When CICMOD\[2:0\]
///is equal to 0xx , the CIC is split into two filters: - The main CIC (MCIC) - The auxiliary CIC (ACIC), used for the out-off limit detector - 000: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in FastSinc filter - 001: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc1 filter - 010: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc2 filter - 011: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc3 filter - 100: The CIC is configured in single sinc4 filter others: The CIC is configured in single sinc5 filter This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type CICMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDF_DFLT3CICR_SPEC, u8, u8, 3, O>;
///Field `MCICD` reader - CIC decimation ratio selection Set and cleared by software. This bit is used to allow the application to select the decimation ratio of the CIC. Decimation ratio smaller than 2 is not allowed. The decimation ratio is given by (CICDEC+1). - 0: Decimation ratio is 2 - 1: Decimation ratio is 2 - 2: Decimation ratio is 3 - 3: Decimation ratio is 4 ... - 511: Decimation ratio is 512 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type MCICD_R = crate::FieldReader<u16, u16>;
///Field `MCICD` writer - CIC decimation ratio selection Set and cleared by software. This bit is used to allow the application to select the decimation ratio of the CIC. Decimation ratio smaller than 2 is not allowed. The decimation ratio is given by (CICDEC+1). - 0: Decimation ratio is 2 - 1: Decimation ratio is 2 - 2: Decimation ratio is 3 - 3: Decimation ratio is 4 ... - 511: Decimation ratio is 512 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type MCICD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDF_DFLT3CICR_SPEC, u16, u16, 9, O>;
///Field `SCALE` reader - Scaling factor selection Set and cleared by software. This field is used to allow the application to select the gain to be applied at CIC output. Please refer to Table 13: Possible gain values for details. If the application attempts to write a new gain value while the previous one is not yet applied, this new gain value is ignored. Reading back the SCALE\[5:0\]
///field will inform the application on the current gain value. - 100000: - 48.2 dB, or shift right by 8 bits (default value) - 100001: - 44.6 dB, - 100010: - 42.1 dB, or shift right by 7 bits - 100011: - 38.6 dB, ... - 101110: -6 dB, or shift right by 1 bit - 101111: -2.5 dB, - 000000: 0 dB - 000001: + 3.5 dB, - 000010: + 6 dB, or shift left by 1 bit ... - 011000: + 72 dB, or shift left by 12 bits
pub type SCALE_R = crate::FieldReader<u8, u8>;
///Field `SCALE` writer - Scaling factor selection Set and cleared by software. This field is used to allow the application to select the gain to be applied at CIC output. Please refer to Table 13: Possible gain values for details. If the application attempts to write a new gain value while the previous one is not yet applied, this new gain value is ignored. Reading back the SCALE\[5:0\]
///field will inform the application on the current gain value. - 100000: - 48.2 dB, or shift right by 8 bits (default value) - 100001: - 44.6 dB, - 100010: - 42.1 dB, or shift right by 7 bits - 100011: - 38.6 dB, ... - 101110: -6 dB, or shift right by 1 bit - 101111: -2.5 dB, - 000000: 0 dB - 000001: + 3.5 dB, - 000010: + 6 dB, or shift left by 1 bit ... - 011000: + 72 dB, or shift left by 12 bits
pub type SCALE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDF_DFLT3CICR_SPEC, u8, u8, 6, O>;
impl R {
    ///Bits 0:1 - Source data for the digital filter Set and cleared by software. 0x: Select the stream coming from the BSMX - 10: Select the stream coming from the ADCITF1 - 11: Select the stream coming from the ADCITF2 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn datsrc(&self) -> DATSRC_R {
        DATSRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:6 - Select the CIC mode Set and cleared by software. This field allows the application to select the configuration and the order of the MCIC. When CICMOD\[2:0\]
    ///is equal to 0xx , the CIC is split into two filters: - The main CIC (MCIC) - The auxiliary CIC (ACIC), used for the out-off limit detector - 000: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in FastSinc filter - 001: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc1 filter - 010: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc2 filter - 011: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc3 filter - 100: The CIC is configured in single sinc4 filter others: The CIC is configured in single sinc5 filter This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn cicmod(&self) -> CICMOD_R {
        CICMOD_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:16 - CIC decimation ratio selection Set and cleared by software. This bit is used to allow the application to select the decimation ratio of the CIC. Decimation ratio smaller than 2 is not allowed. The decimation ratio is given by (CICDEC+1). - 0: Decimation ratio is 2 - 1: Decimation ratio is 2 - 2: Decimation ratio is 3 - 3: Decimation ratio is 4 ... - 511: Decimation ratio is 512 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn mcicd(&self) -> MCICD_R {
        MCICD_R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    ///Bits 20:25 - Scaling factor selection Set and cleared by software. This field is used to allow the application to select the gain to be applied at CIC output. Please refer to Table 13: Possible gain values for details. If the application attempts to write a new gain value while the previous one is not yet applied, this new gain value is ignored. Reading back the SCALE\[5:0\]
    ///field will inform the application on the current gain value. - 100000: - 48.2 dB, or shift right by 8 bits (default value) - 100001: - 44.6 dB, - 100010: - 42.1 dB, or shift right by 7 bits - 100011: - 38.6 dB, ... - 101110: -6 dB, or shift right by 1 bit - 101111: -2.5 dB, - 000000: 0 dB - 000001: + 3.5 dB, - 000010: + 6 dB, or shift left by 1 bit ... - 011000: + 72 dB, or shift left by 12 bits
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
impl W {
    ///Bits 0:1 - Source data for the digital filter Set and cleared by software. 0x: Select the stream coming from the BSMX - 10: Select the stream coming from the ADCITF1 - 11: Select the stream coming from the ADCITF2 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    #[must_use]
    pub fn datsrc(&mut self) -> DATSRC_W<0> {
        DATSRC_W::new(self)
    }
    ///Bits 4:6 - Select the CIC mode Set and cleared by software. This field allows the application to select the configuration and the order of the MCIC. When CICMOD\[2:0\]
    ///is equal to 0xx , the CIC is split into two filters: - The main CIC (MCIC) - The auxiliary CIC (ACIC), used for the out-off limit detector - 000: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in FastSinc filter - 001: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc1 filter - 010: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc2 filter - 011: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc3 filter - 100: The CIC is configured in single sinc4 filter others: The CIC is configured in single sinc5 filter This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    #[must_use]
    pub fn cicmod(&mut self) -> CICMOD_W<4> {
        CICMOD_W::new(self)
    }
    ///Bits 8:16 - CIC decimation ratio selection Set and cleared by software. This bit is used to allow the application to select the decimation ratio of the CIC. Decimation ratio smaller than 2 is not allowed. The decimation ratio is given by (CICDEC+1). - 0: Decimation ratio is 2 - 1: Decimation ratio is 2 - 2: Decimation ratio is 3 - 3: Decimation ratio is 4 ... - 511: Decimation ratio is 512 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    #[must_use]
    pub fn mcicd(&mut self) -> MCICD_W<8> {
        MCICD_W::new(self)
    }
    ///Bits 20:25 - Scaling factor selection Set and cleared by software. This field is used to allow the application to select the gain to be applied at CIC output. Please refer to Table 13: Possible gain values for details. If the application attempts to write a new gain value while the previous one is not yet applied, this new gain value is ignored. Reading back the SCALE\[5:0\]
    ///field will inform the application on the current gain value. - 100000: - 48.2 dB, or shift right by 8 bits (default value) - 100001: - 44.6 dB, - 100010: - 42.1 dB, or shift right by 7 bits - 100011: - 38.6 dB, ... - 101110: -6 dB, or shift right by 1 bit - 101111: -2.5 dB, - 000000: 0 dB - 000001: + 3.5 dB, - 000010: + 6 dB, or shift left by 1 bit ... - 011000: + 72 dB, or shift left by 12 bits
    #[inline(always)]
    #[must_use]
    pub fn scale(&mut self) -> SCALE_W<20> {
        SCALE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to control the main CIC filter.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdf_dflt3cicr](index.html) module
pub struct MDF_DFLT3CICR_SPEC;
impl crate::RegisterSpec for MDF_DFLT3CICR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdf_dflt3cicr::R](R) reader structure
impl crate::Readable for MDF_DFLT3CICR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mdf_dflt3cicr::W](W) writer structure
impl crate::Writable for MDF_DFLT3CICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MDF_DFLT3CICR to value 0
impl crate::Resettable for MDF_DFLT3CICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
