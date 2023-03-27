///Register `MDF_SITF5CR` reader
pub struct R(crate::R<MDF_SITF5CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDF_SITF5CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDF_SITF5CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDF_SITF5CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MDF_SITF5CR` writer
pub struct W(crate::W<MDF_SITF5CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDF_SITF5CR_SPEC>;
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
impl From<crate::W<MDF_SITF5CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDF_SITF5CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SITFEN` reader - Serial interface enable Set and cleared by software. This bit is used to enable/disable the serial interface. - 0: Serial interface disabled - 1: Serial interface enabled
pub type SITFEN_R = crate::BitReader<bool>;
///Field `SITFEN` writer - Serial interface enable Set and cleared by software. This bit is used to enable/disable the serial interface. - 0: Serial interface disabled - 1: Serial interface enabled
pub type SITFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_SITF5CR_SPEC, bool, O>;
///Field `SCKSRC` reader - Serial clock source Set and cleared by software. This bit is used to select the clock source of the serial interface. - 00: Serial clock source is MDF_CCK0 - 01: Serial clock source is MDF_CCK1 1x: Serial clock source is MDF_CKIx, not allowed in LF_MASTER SPI mode This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type SCKSRC_R = crate::FieldReader<u8, u8>;
///Field `SCKSRC` writer - Serial clock source Set and cleared by software. This bit is used to select the clock source of the serial interface. - 00: Serial clock source is MDF_CCK0 - 01: Serial clock source is MDF_CCK1 1x: Serial clock source is MDF_CKIx, not allowed in LF_MASTER SPI mode This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type SCKSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDF_SITF5CR_SPEC, u8, u8, 2, O>;
///Field `SITFMOD` reader - Serial interface type Set and cleared by software. This field is used to defined the serial interface type. - 00: LF_MASTER (Low-Frequency MASTER) SPI mode - 01: Normal SPI mode - 10: Manchester mode: rising edge = logic 0, falling edge = logic 1 - 11: Manchester mode: rising edge = logic 1, falling edge = logic 0 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type SITFMOD_R = crate::FieldReader<u8, u8>;
///Field `SITFMOD` writer - Serial interface type Set and cleared by software. This field is used to defined the serial interface type. - 00: LF_MASTER (Low-Frequency MASTER) SPI mode - 01: Normal SPI mode - 10: Manchester mode: rising edge = logic 0, falling edge = logic 1 - 11: Manchester mode: rising edge = logic 1, falling edge = logic 0 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type SITFMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDF_SITF5CR_SPEC, u8, u8, 2, O>;
///Field `STH` reader - Manchester Symbol threshold / SPI threshold Set and cleared by software. This field is used for Manchester mode, in order to define the expected symbol threshold levels. Please refer to Section : Manchester mode for details on computation. In addition this field is used to define the timeout value for the clock absence detection in Normal SPI mode. Values of STH\[4:0\]
///lower than 4 are invalid. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type STH_R = crate::FieldReader<u8, u8>;
///Field `STH` writer - Manchester Symbol threshold / SPI threshold Set and cleared by software. This field is used for Manchester mode, in order to define the expected symbol threshold levels. Please refer to Section : Manchester mode for details on computation. In addition this field is used to define the timeout value for the clock absence detection in Normal SPI mode. Values of STH\[4:0\]
///lower than 4 are invalid. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type STH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDF_SITF5CR_SPEC, u8, u8, 5, O>;
///Field `SITFACTIVE` reader - Serial interface Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the serial interface is effectively enabled (active) or not. The protected fields of this function can only be updated when the SITFACTIVE is set , please refer to Section 1.4.15: Register protection for details. The delay between a transition on SITFEN and a transition on SITFACTIVE is 2 periods of AHB clock and 2 periods of mdf_proc_ck. - 0: The serial interface is not active, and can be configured if needed - 1: The serial interface is active, and protected fields cannot be configured.
pub type SITFACTIVE_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Serial interface enable Set and cleared by software. This bit is used to enable/disable the serial interface. - 0: Serial interface disabled - 1: Serial interface enabled
    #[inline(always)]
    pub fn sitfen(&self) -> SITFEN_R {
        SITFEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Serial clock source Set and cleared by software. This bit is used to select the clock source of the serial interface. - 00: Serial clock source is MDF_CCK0 - 01: Serial clock source is MDF_CCK1 1x: Serial clock source is MDF_CKIx, not allowed in LF_MASTER SPI mode This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn scksrc(&self) -> SCKSRC_R {
        SCKSRC_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 4:5 - Serial interface type Set and cleared by software. This field is used to defined the serial interface type. - 00: LF_MASTER (Low-Frequency MASTER) SPI mode - 01: Normal SPI mode - 10: Manchester mode: rising edge = logic 0, falling edge = logic 1 - 11: Manchester mode: rising edge = logic 1, falling edge = logic 0 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn sitfmod(&self) -> SITFMOD_R {
        SITFMOD_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:12 - Manchester Symbol threshold / SPI threshold Set and cleared by software. This field is used for Manchester mode, in order to define the expected symbol threshold levels. Please refer to Section : Manchester mode for details on computation. In addition this field is used to define the timeout value for the clock absence detection in Normal SPI mode. Values of STH\[4:0\]
    ///lower than 4 are invalid. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn sth(&self) -> STH_R {
        STH_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bit 31 - Serial interface Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the serial interface is effectively enabled (active) or not. The protected fields of this function can only be updated when the SITFACTIVE is set , please refer to Section 1.4.15: Register protection for details. The delay between a transition on SITFEN and a transition on SITFACTIVE is 2 periods of AHB clock and 2 periods of mdf_proc_ck. - 0: The serial interface is not active, and can be configured if needed - 1: The serial interface is active, and protected fields cannot be configured.
    #[inline(always)]
    pub fn sitfactive(&self) -> SITFACTIVE_R {
        SITFACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Serial interface enable Set and cleared by software. This bit is used to enable/disable the serial interface. - 0: Serial interface disabled - 1: Serial interface enabled
    #[inline(always)]
    #[must_use]
    pub fn sitfen(&mut self) -> SITFEN_W<0> {
        SITFEN_W::new(self)
    }
    ///Bits 1:2 - Serial clock source Set and cleared by software. This bit is used to select the clock source of the serial interface. - 00: Serial clock source is MDF_CCK0 - 01: Serial clock source is MDF_CCK1 1x: Serial clock source is MDF_CKIx, not allowed in LF_MASTER SPI mode This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    #[must_use]
    pub fn scksrc(&mut self) -> SCKSRC_W<1> {
        SCKSRC_W::new(self)
    }
    ///Bits 4:5 - Serial interface type Set and cleared by software. This field is used to defined the serial interface type. - 00: LF_MASTER (Low-Frequency MASTER) SPI mode - 01: Normal SPI mode - 10: Manchester mode: rising edge = logic 0, falling edge = logic 1 - 11: Manchester mode: rising edge = logic 1, falling edge = logic 0 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    #[must_use]
    pub fn sitfmod(&mut self) -> SITFMOD_W<4> {
        SITFMOD_W::new(self)
    }
    ///Bits 8:12 - Manchester Symbol threshold / SPI threshold Set and cleared by software. This field is used for Manchester mode, in order to define the expected symbol threshold levels. Please refer to Section : Manchester mode for details on computation. In addition this field is used to define the timeout value for the clock absence detection in Normal SPI mode. Values of STH\[4:0\]
    ///lower than 4 are invalid. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    #[must_use]
    pub fn sth(&mut self) -> STH_W<8> {
        STH_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to control the serial interfaces (SITFx).
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdf_sitf5cr](index.html) module
pub struct MDF_SITF5CR_SPEC;
impl crate::RegisterSpec for MDF_SITF5CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdf_sitf5cr::R](R) reader structure
impl crate::Readable for MDF_SITF5CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mdf_sitf5cr::W](W) writer structure
impl crate::Writable for MDF_SITF5CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MDF_SITF5CR to value 0x1f00
impl crate::Resettable for MDF_SITF5CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f00;
}
