///Register `MDF_OLD0CR` reader
pub struct R(crate::R<MDF_OLD0CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDF_OLD0CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDF_OLD0CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDF_OLD0CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MDF_OLD0CR` writer
pub struct W(crate::W<MDF_OLD0CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDF_OLD0CR_SPEC>;
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
impl From<crate::W<MDF_OLD0CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDF_OLD0CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OLDEN` reader - Over-Current Detector Enable Set and cleared by software. - 0: The OLD is disabled (Default value) - 1: The OLD is enabled, including the ACIC filter working in continuous mode.
pub type OLDEN_R = crate::BitReader<bool>;
///Field `OLDEN` writer - Over-Current Detector Enable Set and cleared by software. - 0: The OLD is disabled (Default value) - 1: The OLD is enabled, including the ACIC filter working in continuous mode.
pub type OLDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_OLD0CR_SPEC, bool, O>;
///Field `THINB` reader - Threshold In band Set and cleared by software. - 0: The OLD generates an event if the signal is lower than OLDTHL OR higher than OLDTHH (Default value) - 1: The OLD generates an event if the signal is lower than OLDTHH AND higher than OLDTHL This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type THINB_R = crate::BitReader<bool>;
///Field `THINB` writer - Threshold In band Set and cleared by software. - 0: The OLD generates an event if the signal is lower than OLDTHL OR higher than OLDTHH (Default value) - 1: The OLD generates an event if the signal is lower than OLDTHH AND higher than OLDTHL This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type THINB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_OLD0CR_SPEC, bool, O>;
///Field `BKOLD` reader - Break signal assignment for out-of limit detector Set and cleared by software. BKOLD\[i\]
///= 0: Break signal (mdf_break\[i\]) is not assigned to threshold event BKOLD\[i\]
///= 1: Break signal (mdf_break\[i\]) is assigned to threshold event This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type BKOLD_R = crate::FieldReader<u8, u8>;
///Field `BKOLD` writer - Break signal assignment for out-of limit detector Set and cleared by software. BKOLD\[i\]
///= 0: Break signal (mdf_break\[i\]) is not assigned to threshold event BKOLD\[i\]
///= 1: Break signal (mdf_break\[i\]) is assigned to threshold event This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type BKOLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDF_OLD0CR_SPEC, u8, u8, 4, O>;
///Field `ACICN` reader - OLD CIC order selection Set and cleared by software. This field allows the application to select the type, and the order of the ACIC. This field is only taken into account by the MDF when CICMOD\[2:0\]
///= 0xx . - 00: FastSinc filter type - 01: Sinc1 filter type - 10: Sinc2 filter type - 11: Sinc3 filter type This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type ACICN_R = crate::FieldReader<u8, u8>;
///Field `ACICN` writer - OLD CIC order selection Set and cleared by software. This field allows the application to select the type, and the order of the ACIC. This field is only taken into account by the MDF when CICMOD\[2:0\]
///= 0xx . - 00: FastSinc filter type - 01: Sinc1 filter type - 10: Sinc2 filter type - 11: Sinc3 filter type This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type ACICN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDF_OLD0CR_SPEC, u8, u8, 2, O>;
///Field `ACICD` reader - OLD CIC decimation ratio selection Set and cleared by software. This field is used to allow the application to select the decimation ratio of the ACIC. This field is only taken into account by the MDF when CICMOD\[2:0\]
///= 0xx . The decimation ratio is given by (ACICD+1). - 0: Decimation ratio is 1 - 1: Decimation ratio is 2 - 2: Decimation ratio is 3 - 3: Decimation ratio is 4 ... - 31: Decimation ratio is 32 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type ACICD_R = crate::FieldReader<u8, u8>;
///Field `ACICD` writer - OLD CIC decimation ratio selection Set and cleared by software. This field is used to allow the application to select the decimation ratio of the ACIC. This field is only taken into account by the MDF when CICMOD\[2:0\]
///= 0xx . The decimation ratio is given by (ACICD+1). - 0: Decimation ratio is 1 - 1: Decimation ratio is 2 - 2: Decimation ratio is 3 - 3: Decimation ratio is 4 ... - 31: Decimation ratio is 32 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type ACICD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDF_OLD0CR_SPEC, u8, u8, 5, O>;
///Field `OLDACTIVE` reader - OLD Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the OLD is effectively enabled (active) or not. The protected fields and registers of this function can only be updated when the OLDACTIVE is set to , please refer to Section 1.4.15: Register protection for details. The delay between a transition on OLDEN and a transition on OLDACTIVE is 2 periods of AHB clock and 2 periods of mdf_proc_ck. - 0: The OLD is not active, and can be configured if needed - 1: The OLD is active, and protected fields cannot be configured.
pub type OLDACTIVE_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Over-Current Detector Enable Set and cleared by software. - 0: The OLD is disabled (Default value) - 1: The OLD is enabled, including the ACIC filter working in continuous mode.
    #[inline(always)]
    pub fn olden(&self) -> OLDEN_R {
        OLDEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Threshold In band Set and cleared by software. - 0: The OLD generates an event if the signal is lower than OLDTHL OR higher than OLDTHH (Default value) - 1: The OLD generates an event if the signal is lower than OLDTHH AND higher than OLDTHL This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn thinb(&self) -> THINB_R {
        THINB_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 4:7 - Break signal assignment for out-of limit detector Set and cleared by software. BKOLD\[i\]
    ///= 0: Break signal (mdf_break\[i\]) is not assigned to threshold event BKOLD\[i\]
    ///= 1: Break signal (mdf_break\[i\]) is assigned to threshold event This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn bkold(&self) -> BKOLD_R {
        BKOLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 12:13 - OLD CIC order selection Set and cleared by software. This field allows the application to select the type, and the order of the ACIC. This field is only taken into account by the MDF when CICMOD\[2:0\]
    ///= 0xx . - 00: FastSinc filter type - 01: Sinc1 filter type - 10: Sinc2 filter type - 11: Sinc3 filter type This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn acicn(&self) -> ACICN_R {
        ACICN_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 17:21 - OLD CIC decimation ratio selection Set and cleared by software. This field is used to allow the application to select the decimation ratio of the ACIC. This field is only taken into account by the MDF when CICMOD\[2:0\]
    ///= 0xx . The decimation ratio is given by (ACICD+1). - 0: Decimation ratio is 1 - 1: Decimation ratio is 2 - 2: Decimation ratio is 3 - 3: Decimation ratio is 4 ... - 31: Decimation ratio is 32 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn acicd(&self) -> ACICD_R {
        ACICD_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    ///Bit 31 - OLD Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the OLD is effectively enabled (active) or not. The protected fields and registers of this function can only be updated when the OLDACTIVE is set to , please refer to Section 1.4.15: Register protection for details. The delay between a transition on OLDEN and a transition on OLDACTIVE is 2 periods of AHB clock and 2 periods of mdf_proc_ck. - 0: The OLD is not active, and can be configured if needed - 1: The OLD is active, and protected fields cannot be configured.
    #[inline(always)]
    pub fn oldactive(&self) -> OLDACTIVE_R {
        OLDACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Over-Current Detector Enable Set and cleared by software. - 0: The OLD is disabled (Default value) - 1: The OLD is enabled, including the ACIC filter working in continuous mode.
    #[inline(always)]
    #[must_use]
    pub fn olden(&mut self) -> OLDEN_W<0> {
        OLDEN_W::new(self)
    }
    ///Bit 1 - Threshold In band Set and cleared by software. - 0: The OLD generates an event if the signal is lower than OLDTHL OR higher than OLDTHH (Default value) - 1: The OLD generates an event if the signal is lower than OLDTHH AND higher than OLDTHL This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    #[must_use]
    pub fn thinb(&mut self) -> THINB_W<1> {
        THINB_W::new(self)
    }
    ///Bits 4:7 - Break signal assignment for out-of limit detector Set and cleared by software. BKOLD\[i\]
    ///= 0: Break signal (mdf_break\[i\]) is not assigned to threshold event BKOLD\[i\]
    ///= 1: Break signal (mdf_break\[i\]) is assigned to threshold event This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    #[must_use]
    pub fn bkold(&mut self) -> BKOLD_W<4> {
        BKOLD_W::new(self)
    }
    ///Bits 12:13 - OLD CIC order selection Set and cleared by software. This field allows the application to select the type, and the order of the ACIC. This field is only taken into account by the MDF when CICMOD\[2:0\]
    ///= 0xx . - 00: FastSinc filter type - 01: Sinc1 filter type - 10: Sinc2 filter type - 11: Sinc3 filter type This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    #[must_use]
    pub fn acicn(&mut self) -> ACICN_W<12> {
        ACICN_W::new(self)
    }
    ///Bits 17:21 - OLD CIC decimation ratio selection Set and cleared by software. This field is used to allow the application to select the decimation ratio of the ACIC. This field is only taken into account by the MDF when CICMOD\[2:0\]
    ///= 0xx . The decimation ratio is given by (ACICD+1). - 0: Decimation ratio is 1 - 1: Decimation ratio is 2 - 2: Decimation ratio is 3 - 3: Decimation ratio is 4 ... - 31: Decimation ratio is 32 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    #[must_use]
    pub fn acicd(&mut self) -> ACICD_W<17> {
        ACICD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to configure the Out-of Limit Detector function.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdf_old0cr](index.html) module
pub struct MDF_OLD0CR_SPEC;
impl crate::RegisterSpec for MDF_OLD0CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdf_old0cr::R](R) reader structure
impl crate::Readable for MDF_OLD0CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mdf_old0cr::W](W) writer structure
impl crate::Writable for MDF_OLD0CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MDF_OLD0CR to value 0
impl crate::Resettable for MDF_OLD0CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
