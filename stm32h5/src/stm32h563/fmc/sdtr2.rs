///Register `SDTR2` reader
pub struct R(crate::R<SDTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDTR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SDTR2` writer
pub struct W(crate::W<SDTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDTR2_SPEC>;
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
impl From<crate::W<SDTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDTR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TMRD` reader - Load Mode Register to Active These bits define the delay between a Load Mode Register command and an Active or Refresh command in number of memory clock cycles. ....
pub type TMRD_R = crate::FieldReader<u8, u8>;
///Field `TMRD` writer - Load Mode Register to Active These bits define the delay between a Load Mode Register command and an Active or Refresh command in number of memory clock cycles. ....
pub type TMRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDTR2_SPEC, u8, u8, 4, O>;
///Field `TXSR` reader - Exit Self-refresh delay These bits define the delay from releasing the Self-refresh command to issuing the Activate command in number of memory clock cycles. .... Note: If two SDRAM devices are used, the FMC_SDTR1 and FMC_SDTR2 must be programmed with the same TXSR timing corresponding to the slowest SDRAM device.
pub type TXSR_R = crate::FieldReader<u8, u8>;
///Field `TXSR` writer - Exit Self-refresh delay These bits define the delay from releasing the Self-refresh command to issuing the Activate command in number of memory clock cycles. .... Note: If two SDRAM devices are used, the FMC_SDTR1 and FMC_SDTR2 must be programmed with the same TXSR timing corresponding to the slowest SDRAM device.
pub type TXSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDTR2_SPEC, u8, u8, 4, O>;
///Field `TRAS` reader - Self refresh time These bits define the minimum Self-refresh period in number of memory clock cycles. ....
pub type TRAS_R = crate::FieldReader<u8, u8>;
///Field `TRAS` writer - Self refresh time These bits define the minimum Self-refresh period in number of memory clock cycles. ....
pub type TRAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDTR2_SPEC, u8, u8, 4, O>;
///Field `TRC` reader - Row cycle delay These bits define the delay between the Refresh command and the Activate command, as well as the delay between two consecutive Refresh commands. It is expressed in number of memory clock cycles. The TRC timing is only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRC must be programmed with the timings of the slowest device. .... Note: TRC must match the TRC and TRFC (Auto Refresh period) timings defined in the SDRAM device datasheet. Note: The corresponding bits in the FMC_SDTR2 register are don’t care.
pub type TRC_R = crate::FieldReader<u8, u8>;
///Field `TRC` writer - Row cycle delay These bits define the delay between the Refresh command and the Activate command, as well as the delay between two consecutive Refresh commands. It is expressed in number of memory clock cycles. The TRC timing is only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRC must be programmed with the timings of the slowest device. .... Note: TRC must match the TRC and TRFC (Auto Refresh period) timings defined in the SDRAM device datasheet. Note: The corresponding bits in the FMC_SDTR2 register are don’t care.
pub type TRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDTR2_SPEC, u8, u8, 4, O>;
///Field `TWR` reader - Recovery delay These bits define the delay between a Write and a Precharge command in number of memory clock cycles. .... Note: TWR must be programmed to match the write recovery time (t&lt;sub>WR&lt;/sub>) defined in the SDRAM datasheet, and to guarantee that: Note: TWR ≥ TRAS - TRCD and TWR ≥TRC - TRCD - TRP Note: Example: TRAS= 4 cycles, TRCD= 2 cycles. So, TWR >= 2 cycles. TWR must be programmed to 0x1. Note: If two SDRAM devices are used, the FMC_SDTR1 and FMC_SDTR2 must be programmed with the same TWR timing corresponding to the slowest SDRAM device. Note: If only one SDRAM device is used, the TWR timing must be kept at reset value (0xF) for the not used bank.
pub type TWR_R = crate::FieldReader<u8, u8>;
///Field `TWR` writer - Recovery delay These bits define the delay between a Write and a Precharge command in number of memory clock cycles. .... Note: TWR must be programmed to match the write recovery time (t&lt;sub>WR&lt;/sub>) defined in the SDRAM datasheet, and to guarantee that: Note: TWR ≥ TRAS - TRCD and TWR ≥TRC - TRCD - TRP Note: Example: TRAS= 4 cycles, TRCD= 2 cycles. So, TWR >= 2 cycles. TWR must be programmed to 0x1. Note: If two SDRAM devices are used, the FMC_SDTR1 and FMC_SDTR2 must be programmed with the same TWR timing corresponding to the slowest SDRAM device. Note: If only one SDRAM device is used, the TWR timing must be kept at reset value (0xF) for the not used bank.
pub type TWR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDTR2_SPEC, u8, u8, 4, O>;
///Field `TRP` reader - Row precharge delay These bits define the delay between a Precharge command and another command in number of memory clock cycles. The TRP timing is only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRP must be programmed with the timing of the slowest device. .... Note: The corresponding bits in the FMC_SDTR2 register are don’t care.
pub type TRP_R = crate::FieldReader<u8, u8>;
///Field `TRP` writer - Row precharge delay These bits define the delay between a Precharge command and another command in number of memory clock cycles. The TRP timing is only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRP must be programmed with the timing of the slowest device. .... Note: The corresponding bits in the FMC_SDTR2 register are don’t care.
pub type TRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDTR2_SPEC, u8, u8, 4, O>;
///Field `TRCD` reader - Row to column delay These bits define the delay between the Activate command and a Read/Write command in number of memory clock cycles. ....
pub type TRCD_R = crate::FieldReader<u8, u8>;
///Field `TRCD` writer - Row to column delay These bits define the delay between the Activate command and a Read/Write command in number of memory clock cycles. ....
pub type TRCD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDTR2_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:3 - Load Mode Register to Active These bits define the delay between a Load Mode Register command and an Active or Refresh command in number of memory clock cycles. ....
    #[inline(always)]
    pub fn tmrd(&self) -> TMRD_R {
        TMRD_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Exit Self-refresh delay These bits define the delay from releasing the Self-refresh command to issuing the Activate command in number of memory clock cycles. .... Note: If two SDRAM devices are used, the FMC_SDTR1 and FMC_SDTR2 must be programmed with the same TXSR timing corresponding to the slowest SDRAM device.
    #[inline(always)]
    pub fn txsr(&self) -> TXSR_R {
        TXSR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Self refresh time These bits define the minimum Self-refresh period in number of memory clock cycles. ....
    #[inline(always)]
    pub fn tras(&self) -> TRAS_R {
        TRAS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Row cycle delay These bits define the delay between the Refresh command and the Activate command, as well as the delay between two consecutive Refresh commands. It is expressed in number of memory clock cycles. The TRC timing is only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRC must be programmed with the timings of the slowest device. .... Note: TRC must match the TRC and TRFC (Auto Refresh period) timings defined in the SDRAM device datasheet. Note: The corresponding bits in the FMC_SDTR2 register are don’t care.
    #[inline(always)]
    pub fn trc(&self) -> TRC_R {
        TRC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Recovery delay These bits define the delay between a Write and a Precharge command in number of memory clock cycles. .... Note: TWR must be programmed to match the write recovery time (t&lt;sub>WR&lt;/sub>) defined in the SDRAM datasheet, and to guarantee that: Note: TWR ≥ TRAS - TRCD and TWR ≥TRC - TRCD - TRP Note: Example: TRAS= 4 cycles, TRCD= 2 cycles. So, TWR >= 2 cycles. TWR must be programmed to 0x1. Note: If two SDRAM devices are used, the FMC_SDTR1 and FMC_SDTR2 must be programmed with the same TWR timing corresponding to the slowest SDRAM device. Note: If only one SDRAM device is used, the TWR timing must be kept at reset value (0xF) for the not used bank.
    #[inline(always)]
    pub fn twr(&self) -> TWR_R {
        TWR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Row precharge delay These bits define the delay between a Precharge command and another command in number of memory clock cycles. The TRP timing is only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRP must be programmed with the timing of the slowest device. .... Note: The corresponding bits in the FMC_SDTR2 register are don’t care.
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Row to column delay These bits define the delay between the Activate command and a Read/Write command in number of memory clock cycles. ....
    #[inline(always)]
    pub fn trcd(&self) -> TRCD_R {
        TRCD_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - Load Mode Register to Active These bits define the delay between a Load Mode Register command and an Active or Refresh command in number of memory clock cycles. ....
    #[inline(always)]
    #[must_use]
    pub fn tmrd(&mut self) -> TMRD_W<0> {
        TMRD_W::new(self)
    }
    ///Bits 4:7 - Exit Self-refresh delay These bits define the delay from releasing the Self-refresh command to issuing the Activate command in number of memory clock cycles. .... Note: If two SDRAM devices are used, the FMC_SDTR1 and FMC_SDTR2 must be programmed with the same TXSR timing corresponding to the slowest SDRAM device.
    #[inline(always)]
    #[must_use]
    pub fn txsr(&mut self) -> TXSR_W<4> {
        TXSR_W::new(self)
    }
    ///Bits 8:11 - Self refresh time These bits define the minimum Self-refresh period in number of memory clock cycles. ....
    #[inline(always)]
    #[must_use]
    pub fn tras(&mut self) -> TRAS_W<8> {
        TRAS_W::new(self)
    }
    ///Bits 12:15 - Row cycle delay These bits define the delay between the Refresh command and the Activate command, as well as the delay between two consecutive Refresh commands. It is expressed in number of memory clock cycles. The TRC timing is only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRC must be programmed with the timings of the slowest device. .... Note: TRC must match the TRC and TRFC (Auto Refresh period) timings defined in the SDRAM device datasheet. Note: The corresponding bits in the FMC_SDTR2 register are don’t care.
    #[inline(always)]
    #[must_use]
    pub fn trc(&mut self) -> TRC_W<12> {
        TRC_W::new(self)
    }
    ///Bits 16:19 - Recovery delay These bits define the delay between a Write and a Precharge command in number of memory clock cycles. .... Note: TWR must be programmed to match the write recovery time (t&lt;sub>WR&lt;/sub>) defined in the SDRAM datasheet, and to guarantee that: Note: TWR ≥ TRAS - TRCD and TWR ≥TRC - TRCD - TRP Note: Example: TRAS= 4 cycles, TRCD= 2 cycles. So, TWR >= 2 cycles. TWR must be programmed to 0x1. Note: If two SDRAM devices are used, the FMC_SDTR1 and FMC_SDTR2 must be programmed with the same TWR timing corresponding to the slowest SDRAM device. Note: If only one SDRAM device is used, the TWR timing must be kept at reset value (0xF) for the not used bank.
    #[inline(always)]
    #[must_use]
    pub fn twr(&mut self) -> TWR_W<16> {
        TWR_W::new(self)
    }
    ///Bits 20:23 - Row precharge delay These bits define the delay between a Precharge command and another command in number of memory clock cycles. The TRP timing is only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRP must be programmed with the timing of the slowest device. .... Note: The corresponding bits in the FMC_SDTR2 register are don’t care.
    #[inline(always)]
    #[must_use]
    pub fn trp(&mut self) -> TRP_W<20> {
        TRP_W::new(self)
    }
    ///Bits 24:27 - Row to column delay These bits define the delay between the Activate command and a Read/Write command in number of memory clock cycles. ....
    #[inline(always)]
    #[must_use]
    pub fn trcd(&mut self) -> TRCD_W<24> {
        TRCD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SDRAM timing registers 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sdtr2](index.html) module
pub struct SDTR2_SPEC;
impl crate::RegisterSpec for SDTR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [sdtr2::R](R) reader structure
impl crate::Readable for SDTR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sdtr2::W](W) writer structure
impl crate::Writable for SDTR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SDTR2 to value 0x0fff_ffff
impl crate::Resettable for SDTR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_ffff;
}
