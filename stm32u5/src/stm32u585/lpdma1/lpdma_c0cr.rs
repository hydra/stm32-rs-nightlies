///Register `LPDMA_C0CR` reader
pub struct R(crate::R<LPDMA_C0CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPDMA_C0CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPDMA_C0CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPDMA_C0CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LPDMA_C0CR` writer
pub struct W(crate::W<LPDMA_C0CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPDMA_C0CR_SPEC>;
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
impl From<crate::W<LPDMA_C0CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPDMA_C0CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EN` reader - enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is de-asserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, e.g. if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored.
pub type EN_R = crate::BitReader<bool>;
///Field `EN` writer - enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is de-asserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, e.g. if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored.
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_C0CR_SPEC, bool, O>;
///Field `RESET` writer - reset This bit is write only. Writing 0 has no impact. Writing 1 implies the reset of the following: the channel internal state, SUSP and EN bits (whatever is written receptively in bit 2 and bit 0). The reset is effective when the channel is in steady state, meaning one of the following: - active channel in suspended state (LPDMA_CxSR.SUSPF = 1 and LPDMA_CxSR.IDLEF = LPDMA_CxCR.EN = 1) - channel in disabled state (LPDMA_CxSR.IDLEF = 1 and LPDMA_CxCR.EN = 0). After writing a RESET, to continue using this channel, the user must explicitly reconfigure the channel including the hardware-modified configuration registers (LPDMA_CxBR1, LPDMA_CxSAR and LPDMA_CxDAR) before enabling again the channel (see the programming sequence in ).
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_C0CR_SPEC, bool, O>;
///Field `SUSP` reader - suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel i.e. a channel with an on-going DMA transfer over its master ports. The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in sequence.
pub type SUSP_R = crate::BitReader<bool>;
///Field `SUSP` writer - suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel i.e. a channel with an on-going DMA transfer over its master ports. The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in sequence.
pub type SUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_C0CR_SPEC, bool, O>;
///Field `TCIE` reader - transfer complete interrupt enable
pub type TCIE_R = crate::BitReader<bool>;
///Field `TCIE` writer - transfer complete interrupt enable
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_C0CR_SPEC, bool, O>;
///Field `HTIE` reader - half transfer complete interrupt enable
pub type HTIE_R = crate::BitReader<bool>;
///Field `HTIE` writer - half transfer complete interrupt enable
pub type HTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_C0CR_SPEC, bool, O>;
///Field `DTEIE` reader - data transfer error interrupt enable
pub type DTEIE_R = crate::BitReader<bool>;
///Field `DTEIE` writer - data transfer error interrupt enable
pub type DTEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_C0CR_SPEC, bool, O>;
///Field `ULEIE` reader - update link transfer error interrupt enable
pub type ULEIE_R = crate::BitReader<bool>;
///Field `ULEIE` writer - update link transfer error interrupt enable
pub type ULEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_C0CR_SPEC, bool, O>;
///Field `USEIE` reader - user setting error interrupt enable
pub type USEIE_R = crate::BitReader<bool>;
///Field `USEIE` writer - user setting error interrupt enable
pub type USEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_C0CR_SPEC, bool, O>;
///Field `SUSPIE` reader - completed suspension interrupt enable
pub type SUSPIE_R = crate::BitReader<bool>;
///Field `SUSPIE` writer - completed suspension interrupt enable
pub type SUSPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_C0CR_SPEC, bool, O>;
///Field `TOIE` reader - trigger overrun interrupt enable
pub type TOIE_R = crate::BitReader<bool>;
///Field `TOIE` writer - trigger overrun interrupt enable
pub type TOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_C0CR_SPEC, bool, O>;
///Field `LSM` reader - Link step mode First the block transfer is executed as defined by the current internal register file until LPDMA_CxBR1.BNDT\[15:0 \]
///=0). Secondly the next linked-list data structure is conditionally uploaded from memory as defined by LPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1.
pub type LSM_R = crate::BitReader<bool>;
///Field `LSM` writer - Link step mode First the block transfer is executed as defined by the current internal register file until LPDMA_CxBR1.BNDT\[15:0 \]
///=0). Secondly the next linked-list data structure is conditionally uploaded from memory as defined by LPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1.
pub type LSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_C0CR_SPEC, bool, O>;
///Field `PRIO` reader - priority level of the channel x LPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
pub type PRIO_R = crate::FieldReader<u8, u8>;
///Field `PRIO` writer - priority level of the channel x LPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
pub type PRIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPDMA_C0CR_SPEC, u8, u8, 2, O>;
impl R {
    ///Bit 0 - enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is de-asserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, e.g. if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored.
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel i.e. a channel with an on-going DMA transfer over its master ports. The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in sequence.
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - half transfer complete interrupt enable
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - data transfer error interrupt enable
    #[inline(always)]
    pub fn dteie(&self) -> DTEIE_R {
        DTEIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - update link transfer error interrupt enable
    #[inline(always)]
    pub fn uleie(&self) -> ULEIE_R {
        ULEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - user setting error interrupt enable
    #[inline(always)]
    pub fn useie(&self) -> USEIE_R {
        USEIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - completed suspension interrupt enable
    #[inline(always)]
    pub fn suspie(&self) -> SUSPIE_R {
        SUSPIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - trigger overrun interrupt enable
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Link step mode First the block transfer is executed as defined by the current internal register file until LPDMA_CxBR1.BNDT\[15:0 \]
    ///=0). Secondly the next linked-list data structure is conditionally uploaded from memory as defined by LPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1.
    #[inline(always)]
    pub fn lsm(&self) -> LSM_R {
        LSM_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 22:23 - priority level of the channel x LPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is de-asserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, e.g. if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored.
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    ///Bit 1 - reset This bit is write only. Writing 0 has no impact. Writing 1 implies the reset of the following: the channel internal state, SUSP and EN bits (whatever is written receptively in bit 2 and bit 0). The reset is effective when the channel is in steady state, meaning one of the following: - active channel in suspended state (LPDMA_CxSR.SUSPF = 1 and LPDMA_CxSR.IDLEF = LPDMA_CxCR.EN = 1) - channel in disabled state (LPDMA_CxSR.IDLEF = 1 and LPDMA_CxCR.EN = 0). After writing a RESET, to continue using this channel, the user must explicitly reconfigure the channel including the hardware-modified configuration registers (LPDMA_CxBR1, LPDMA_CxSAR and LPDMA_CxDAR) before enabling again the channel (see the programming sequence in ).
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<1> {
        RESET_W::new(self)
    }
    ///Bit 2 - suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel i.e. a channel with an on-going DMA transfer over its master ports. The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in sequence.
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<2> {
        SUSP_W::new(self)
    }
    ///Bit 8 - transfer complete interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<8> {
        TCIE_W::new(self)
    }
    ///Bit 9 - half transfer complete interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn htie(&mut self) -> HTIE_W<9> {
        HTIE_W::new(self)
    }
    ///Bit 10 - data transfer error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn dteie(&mut self) -> DTEIE_W<10> {
        DTEIE_W::new(self)
    }
    ///Bit 11 - update link transfer error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn uleie(&mut self) -> ULEIE_W<11> {
        ULEIE_W::new(self)
    }
    ///Bit 12 - user setting error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn useie(&mut self) -> USEIE_W<12> {
        USEIE_W::new(self)
    }
    ///Bit 13 - completed suspension interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn suspie(&mut self) -> SUSPIE_W<13> {
        SUSPIE_W::new(self)
    }
    ///Bit 14 - trigger overrun interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn toie(&mut self) -> TOIE_W<14> {
        TOIE_W::new(self)
    }
    ///Bit 16 - Link step mode First the block transfer is executed as defined by the current internal register file until LPDMA_CxBR1.BNDT\[15:0 \]
    ///=0). Secondly the next linked-list data structure is conditionally uploaded from memory as defined by LPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1.
    #[inline(always)]
    #[must_use]
    pub fn lsm(&mut self) -> LSM_W<16> {
        LSM_W::new(self)
    }
    ///Bits 22:23 - priority level of the channel x LPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PRIO_W<22> {
        PRIO_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LPDMA channel 0 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lpdma_c0cr](index.html) module
pub struct LPDMA_C0CR_SPEC;
impl crate::RegisterSpec for LPDMA_C0CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lpdma_c0cr::R](R) reader structure
impl crate::Readable for LPDMA_C0CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lpdma_c0cr::W](W) writer structure
impl crate::Writable for LPDMA_C0CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LPDMA_C0CR to value 0
impl crate::Resettable for LPDMA_C0CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
