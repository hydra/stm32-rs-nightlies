///Register `LPDMA_C2TR2` reader
pub struct R(crate::R<LPDMA_C2TR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPDMA_C2TR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPDMA_C2TR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPDMA_C2TR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LPDMA_C2TR2` writer
pub struct W(crate::W<LPDMA_C2TR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPDMA_C2TR2_SPEC>;
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
impl From<crate::W<LPDMA_C2TR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPDMA_C2TR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `REQSEL` reader - DMA hardware request selection These bits are ignored if channel x is activated (LPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per request. Note: The user must not assign a same input hardware request (same REQSEL\[4:0\]
///value) to different active DMA channels (LPDMA_CxCR.EN = 1 and LPDMA_CxTR2.SWREQ = 0 for these channels). DMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting.
pub type REQSEL_R = crate::FieldReader<u8, u8>;
///Field `REQSEL` writer - DMA hardware request selection These bits are ignored if channel x is activated (LPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per request. Note: The user must not assign a same input hardware request (same REQSEL\[4:0\]
///value) to different active DMA channels (LPDMA_CxCR.EN = 1 and LPDMA_CxTR2.SWREQ = 0 for these channels). DMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting.
pub type REQSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPDMA_C2TR2_SPEC, u8, u8, 5, O>;
///Field `SWREQ` reader - software request This bit is internally taken into account when LPDMA_CxCR.EN is asserted.
pub type SWREQ_R = crate::BitReader<bool>;
///Field `SWREQ` writer - software request This bit is internally taken into account when LPDMA_CxCR.EN is asserted.
pub type SWREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_C2TR2_SPEC, bool, O>;
///Field `BREQ` reader - block hardware request If the channel x is activated (LPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:
pub type BREQ_R = crate::BitReader<bool>;
///Field `BREQ` writer - block hardware request If the channel x is activated (LPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:
pub type BREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_C2TR2_SPEC, bool, O>;
///Field `TRIGM` reader - trigger mode These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (LPDMA_CxCR.EN asserted) with TRIGPOL\[1:0\]
///= 0b00 or 0b11, these TRIGM\[1:0\]
///bits are ignored. Else, a DMA transfer is conditioned by at least one trigger hit: The LPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL\[1:0\]
///= 0b01 or respectively TRIGPOL\[1:0\]
///= 0b10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL\[4:0\]
///is not modified, and the channel is enabled. Transferring a next LLIn+1 that updates the LPDMA_CxTR2 with a new value for any of TRIGSEL\[4:0\]
///or TRIGPOL\[1:0\], resets the monitoring, trashing the memorized hit of the formerly defined LLIn trigger. After a first new trigger hitn+1 is memorized, if another second trigger hitn+2 is detected and if the hitn triggered transfer is still not completed, hitn+2 is lost and not memorized, and a trigger overrun flag is reported (LPDMA_CxSR.TOF = 1), an interrupt is generated if enabled (LPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun.
pub type TRIGM_R = crate::FieldReader<u8, u8>;
///Field `TRIGM` writer - trigger mode These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (LPDMA_CxCR.EN asserted) with TRIGPOL\[1:0\]
///= 0b00 or 0b11, these TRIGM\[1:0\]
///bits are ignored. Else, a DMA transfer is conditioned by at least one trigger hit: The LPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL\[1:0\]
///= 0b01 or respectively TRIGPOL\[1:0\]
///= 0b10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL\[4:0\]
///is not modified, and the channel is enabled. Transferring a next LLIn+1 that updates the LPDMA_CxTR2 with a new value for any of TRIGSEL\[4:0\]
///or TRIGPOL\[1:0\], resets the monitoring, trashing the memorized hit of the formerly defined LLIn trigger. After a first new trigger hitn+1 is memorized, if another second trigger hitn+2 is detected and if the hitn triggered transfer is still not completed, hitn+2 is lost and not memorized, and a trigger overrun flag is reported (LPDMA_CxSR.TOF = 1), an interrupt is generated if enabled (LPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun.
pub type TRIGM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPDMA_C2TR2_SPEC, u8, u8, 2, O>;
///Field `TRIGSEL` reader - trigger event input selection These bits select the trigger event input of the LPDMA transfer (as per Programmed LPDMA1 trigger), with an active trigger event if TRIGPOL\[1:0\]
///= 00.
pub type TRIGSEL_R = crate::FieldReader<u8, u8>;
///Field `TRIGSEL` writer - trigger event input selection These bits select the trigger event input of the LPDMA transfer (as per Programmed LPDMA1 trigger), with an active trigger event if TRIGPOL\[1:0\]
///= 00.
pub type TRIGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPDMA_C2TR2_SPEC, u8, u8, 5, O>;
///Field `TRIGPOL` reader - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\[4:0\].
pub type TRIGPOL_R = crate::FieldReader<u8, u8>;
///Field `TRIGPOL` writer - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\[4:0\].
pub type TRIGPOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPDMA_C2TR2_SPEC, u8, u8, 2, O>;
///Field `TCEM` reader - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\]
///= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\]
///=0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1.
pub type TCEM_R = crate::FieldReader<u8, u8>;
///Field `TCEM` writer - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\]
///= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\]
///=0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1.
pub type TCEM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPDMA_C2TR2_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:4 - DMA hardware request selection These bits are ignored if channel x is activated (LPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per request. Note: The user must not assign a same input hardware request (same REQSEL\[4:0\]
    ///value) to different active DMA channels (LPDMA_CxCR.EN = 1 and LPDMA_CxTR2.SWREQ = 0 for these channels). DMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting.
    #[inline(always)]
    pub fn reqsel(&self) -> REQSEL_R {
        REQSEL_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 9 - software request This bit is internally taken into account when LPDMA_CxCR.EN is asserted.
    #[inline(always)]
    pub fn swreq(&self) -> SWREQ_R {
        SWREQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - block hardware request If the channel x is activated (LPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:
    #[inline(always)]
    pub fn breq(&self) -> BREQ_R {
        BREQ_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 14:15 - trigger mode These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (LPDMA_CxCR.EN asserted) with TRIGPOL\[1:0\]
    ///= 0b00 or 0b11, these TRIGM\[1:0\]
    ///bits are ignored. Else, a DMA transfer is conditioned by at least one trigger hit: The LPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL\[1:0\]
    ///= 0b01 or respectively TRIGPOL\[1:0\]
    ///= 0b10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL\[4:0\]
    ///is not modified, and the channel is enabled. Transferring a next LLIn+1 that updates the LPDMA_CxTR2 with a new value for any of TRIGSEL\[4:0\]
    ///or TRIGPOL\[1:0\], resets the monitoring, trashing the memorized hit of the formerly defined LLIn trigger. After a first new trigger hitn+1 is memorized, if another second trigger hitn+2 is detected and if the hitn triggered transfer is still not completed, hitn+2 is lost and not memorized, and a trigger overrun flag is reported (LPDMA_CxSR.TOF = 1), an interrupt is generated if enabled (LPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun.
    #[inline(always)]
    pub fn trigm(&self) -> TRIGM_R {
        TRIGM_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:20 - trigger event input selection These bits select the trigger event input of the LPDMA transfer (as per Programmed LPDMA1 trigger), with an active trigger event if TRIGPOL\[1:0\]
    ///= 00.
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:25 - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\[4:0\].
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 30:31 - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\]
    ///= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\]
    ///=0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1.
    #[inline(always)]
    pub fn tcem(&self) -> TCEM_R {
        TCEM_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    ///Bits 0:4 - DMA hardware request selection These bits are ignored if channel x is activated (LPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per request. Note: The user must not assign a same input hardware request (same REQSEL\[4:0\]
    ///value) to different active DMA channels (LPDMA_CxCR.EN = 1 and LPDMA_CxTR2.SWREQ = 0 for these channels). DMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting.
    #[inline(always)]
    #[must_use]
    pub fn reqsel(&mut self) -> REQSEL_W<0> {
        REQSEL_W::new(self)
    }
    ///Bit 9 - software request This bit is internally taken into account when LPDMA_CxCR.EN is asserted.
    #[inline(always)]
    #[must_use]
    pub fn swreq(&mut self) -> SWREQ_W<9> {
        SWREQ_W::new(self)
    }
    ///Bit 11 - block hardware request If the channel x is activated (LPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:
    #[inline(always)]
    #[must_use]
    pub fn breq(&mut self) -> BREQ_W<11> {
        BREQ_W::new(self)
    }
    ///Bits 14:15 - trigger mode These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (LPDMA_CxCR.EN asserted) with TRIGPOL\[1:0\]
    ///= 0b00 or 0b11, these TRIGM\[1:0\]
    ///bits are ignored. Else, a DMA transfer is conditioned by at least one trigger hit: The LPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL\[1:0\]
    ///= 0b01 or respectively TRIGPOL\[1:0\]
    ///= 0b10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL\[4:0\]
    ///is not modified, and the channel is enabled. Transferring a next LLIn+1 that updates the LPDMA_CxTR2 with a new value for any of TRIGSEL\[4:0\]
    ///or TRIGPOL\[1:0\], resets the monitoring, trashing the memorized hit of the formerly defined LLIn trigger. After a first new trigger hitn+1 is memorized, if another second trigger hitn+2 is detected and if the hitn triggered transfer is still not completed, hitn+2 is lost and not memorized, and a trigger overrun flag is reported (LPDMA_CxSR.TOF = 1), an interrupt is generated if enabled (LPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun.
    #[inline(always)]
    #[must_use]
    pub fn trigm(&mut self) -> TRIGM_W<14> {
        TRIGM_W::new(self)
    }
    ///Bits 16:20 - trigger event input selection These bits select the trigger event input of the LPDMA transfer (as per Programmed LPDMA1 trigger), with an active trigger event if TRIGPOL\[1:0\]
    ///= 00.
    #[inline(always)]
    #[must_use]
    pub fn trigsel(&mut self) -> TRIGSEL_W<16> {
        TRIGSEL_W::new(self)
    }
    ///Bits 24:25 - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\[4:0\].
    #[inline(always)]
    #[must_use]
    pub fn trigpol(&mut self) -> TRIGPOL_W<24> {
        TRIGPOL_W::new(self)
    }
    ///Bits 30:31 - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\]
    ///= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\]
    ///=0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1.
    #[inline(always)]
    #[must_use]
    pub fn tcem(&mut self) -> TCEM_W<30> {
        TCEM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LPDMA channel 2 transfer register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lpdma_c2tr2](index.html) module
pub struct LPDMA_C2TR2_SPEC;
impl crate::RegisterSpec for LPDMA_C2TR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [lpdma_c2tr2::R](R) reader structure
impl crate::Readable for LPDMA_C2TR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lpdma_c2tr2::W](W) writer structure
impl crate::Writable for LPDMA_C2TR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LPDMA_C2TR2 to value 0
impl crate::Resettable for LPDMA_C2TR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
