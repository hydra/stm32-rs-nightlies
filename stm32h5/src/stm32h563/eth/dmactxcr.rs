///Register `DMACTXCR` reader
pub struct R(crate::R<DMACTXCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACTXCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACTXCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACTXCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMACTXCR` writer
pub struct W(crate::W<DMACTXCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACTXCR_SPEC>;
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
impl From<crate::W<DMACTXCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACTXCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ST` reader - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state. The DMA checks the Transmit list at the current position for a packet to be transmitted. The DMA tries to acquire descriptor from either of the following positions: The current position in the list: this is the base address of the Transmit list set by the ETH_DMACTXDLAR register. The position at which the transmission was previously stopped If the DMA does not own the current descriptor, the transmission enters the Suspended state and the TBU bit of the ETH_DMACSR is set. The Start Transmission command is effective only when the transmission is stopped. If the command is issued before setting the ETH_DMACTXDLAR register, the DMA behavior is unpredictable. When this bit is reset, the transmission process is placed in the Stopped state after completing the transmission of the current packet. The Next Descriptor position in the Transmit list is saved, and it becomes the current position when the transmission is restarted. To change the list address, you need to program ETH_DMACTXDLAR register with a new value when this bit is reset. The new value is considered when this bit is set again. The stop transmission command is effective only when the transmission of the current packet is complete or the transmission is in the Suspended state.
pub type ST_R = crate::BitReader<bool>;
///Field `ST` writer - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state. The DMA checks the Transmit list at the current position for a packet to be transmitted. The DMA tries to acquire descriptor from either of the following positions: The current position in the list: this is the base address of the Transmit list set by the ETH_DMACTXDLAR register. The position at which the transmission was previously stopped If the DMA does not own the current descriptor, the transmission enters the Suspended state and the TBU bit of the ETH_DMACSR is set. The Start Transmission command is effective only when the transmission is stopped. If the command is issued before setting the ETH_DMACTXDLAR register, the DMA behavior is unpredictable. When this bit is reset, the transmission process is placed in the Stopped state after completing the transmission of the current packet. The Next Descriptor position in the Transmit list is saved, and it becomes the current position when the transmission is restarted. To change the list address, you need to program ETH_DMACTXDLAR register with a new value when this bit is reset. The new value is considered when this bit is set again. The stop transmission command is effective only when the transmission of the current packet is complete or the transmission is in the Suspended state.
pub type ST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACTXCR_SPEC, bool, O>;
///Field `OSF` reader - Operate on Second Packet When this bit is set, it instructs the DMA to process the second packet of the Transmit data even before the status for the first packet is obtained.
pub type OSF_R = crate::BitReader<bool>;
///Field `OSF` writer - Operate on Second Packet When this bit is set, it instructs the DMA to process the second packet of the Transmit data even before the status for the first packet is obtained.
pub type OSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACTXCR_SPEC, bool, O>;
///Field `TSE` reader - TCP Segmentation Enabled When this bit is set, the DMA performs the TCP segmentation for packets in Channel x. The TCP segmentation is done only for those packets for which the TSE bit (TDES0\[19\]) is set in the Tx Normal descriptor. When this bit is set, the TxPBL value must be greater than or equal to 4.
pub type TSE_R = crate::BitReader<bool>;
///Field `TSE` writer - TCP Segmentation Enabled When this bit is set, the DMA performs the TCP segmentation for packets in Channel x. The TCP segmentation is done only for those packets for which the TSE bit (TDES0\[19\]) is set in the Tx Normal descriptor. When this bit is set, the TxPBL value must be greater than or equal to 4.
pub type TSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACTXCR_SPEC, bool, O>;
///Field `TXPBL` reader - Transmit Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer. This is the maximum value that is used in a single block Read or Write. The DMA always attempts to burst as specified in PBL each time it starts a burst transfer on the application bus. You can program PBL with any of the following values: 1, 2, 4, 8, 16, or 32. Any other value results in undefined behavior. To transfer more than 32 beats, perform the following steps: Set the PBLx8 mode in ETH_DMACCR. Set the TXPBL\[5:0\]. Note: The maximum value of TXPBL must be less than or equal to half the Tx Queue size (TQS field of Tx queue operating mode Register (ETH_MTLTXQOMR)) in terms of beats. This is required so that the Tx Queue has space to store at least another Tx PBL worth of data while the MTL Tx Queue Controller is transferring data to MAC. The total locations in Tx Queue of size 2048 bytes is 512, TXPBL and 8xPBL needs to be programmed to less than or equal to 512/2.
pub type TXPBL_R = crate::FieldReader<u8, u8>;
///Field `TXPBL` writer - Transmit Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer. This is the maximum value that is used in a single block Read or Write. The DMA always attempts to burst as specified in PBL each time it starts a burst transfer on the application bus. You can program PBL with any of the following values: 1, 2, 4, 8, 16, or 32. Any other value results in undefined behavior. To transfer more than 32 beats, perform the following steps: Set the PBLx8 mode in ETH_DMACCR. Set the TXPBL\[5:0\]. Note: The maximum value of TXPBL must be less than or equal to half the Tx Queue size (TQS field of Tx queue operating mode Register (ETH_MTLTXQOMR)) in terms of beats. This is required so that the Tx Queue has space to store at least another Tx PBL worth of data while the MTL Tx Queue Controller is transferring data to MAC. The total locations in Tx Queue of size 2048 bytes is 512, TXPBL and 8xPBL needs to be programmed to less than or equal to 512/2.
pub type TXPBL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACTXCR_SPEC, u8, u8, 6, O>;
impl R {
    ///Bit 0 - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state. The DMA checks the Transmit list at the current position for a packet to be transmitted. The DMA tries to acquire descriptor from either of the following positions: The current position in the list: this is the base address of the Transmit list set by the ETH_DMACTXDLAR register. The position at which the transmission was previously stopped If the DMA does not own the current descriptor, the transmission enters the Suspended state and the TBU bit of the ETH_DMACSR is set. The Start Transmission command is effective only when the transmission is stopped. If the command is issued before setting the ETH_DMACTXDLAR register, the DMA behavior is unpredictable. When this bit is reset, the transmission process is placed in the Stopped state after completing the transmission of the current packet. The Next Descriptor position in the Transmit list is saved, and it becomes the current position when the transmission is restarted. To change the list address, you need to program ETH_DMACTXDLAR register with a new value when this bit is reset. The new value is considered when this bit is set again. The stop transmission command is effective only when the transmission of the current packet is complete or the transmission is in the Suspended state.
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Operate on Second Packet When this bit is set, it instructs the DMA to process the second packet of the Transmit data even before the status for the first packet is obtained.
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 12 - TCP Segmentation Enabled When this bit is set, the DMA performs the TCP segmentation for packets in Channel x. The TCP segmentation is done only for those packets for which the TSE bit (TDES0\[19\]) is set in the Tx Normal descriptor. When this bit is set, the TxPBL value must be greater than or equal to 4.
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 16:21 - Transmit Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer. This is the maximum value that is used in a single block Read or Write. The DMA always attempts to burst as specified in PBL each time it starts a burst transfer on the application bus. You can program PBL with any of the following values: 1, 2, 4, 8, 16, or 32. Any other value results in undefined behavior. To transfer more than 32 beats, perform the following steps: Set the PBLx8 mode in ETH_DMACCR. Set the TXPBL\[5:0\]. Note: The maximum value of TXPBL must be less than or equal to half the Tx Queue size (TQS field of Tx queue operating mode Register (ETH_MTLTXQOMR)) in terms of beats. This is required so that the Tx Queue has space to store at least another Tx PBL worth of data while the MTL Tx Queue Controller is transferring data to MAC. The total locations in Tx Queue of size 2048 bytes is 512, TXPBL and 8xPBL needs to be programmed to less than or equal to 512/2.
    #[inline(always)]
    pub fn txpbl(&self) -> TXPBL_R {
        TXPBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    ///Bit 0 - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state. The DMA checks the Transmit list at the current position for a packet to be transmitted. The DMA tries to acquire descriptor from either of the following positions: The current position in the list: this is the base address of the Transmit list set by the ETH_DMACTXDLAR register. The position at which the transmission was previously stopped If the DMA does not own the current descriptor, the transmission enters the Suspended state and the TBU bit of the ETH_DMACSR is set. The Start Transmission command is effective only when the transmission is stopped. If the command is issued before setting the ETH_DMACTXDLAR register, the DMA behavior is unpredictable. When this bit is reset, the transmission process is placed in the Stopped state after completing the transmission of the current packet. The Next Descriptor position in the Transmit list is saved, and it becomes the current position when the transmission is restarted. To change the list address, you need to program ETH_DMACTXDLAR register with a new value when this bit is reset. The new value is considered when this bit is set again. The stop transmission command is effective only when the transmission of the current packet is complete or the transmission is in the Suspended state.
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<0> {
        ST_W::new(self)
    }
    ///Bit 4 - Operate on Second Packet When this bit is set, it instructs the DMA to process the second packet of the Transmit data even before the status for the first packet is obtained.
    #[inline(always)]
    #[must_use]
    pub fn osf(&mut self) -> OSF_W<4> {
        OSF_W::new(self)
    }
    ///Bit 12 - TCP Segmentation Enabled When this bit is set, the DMA performs the TCP segmentation for packets in Channel x. The TCP segmentation is done only for those packets for which the TSE bit (TDES0\[19\]) is set in the Tx Normal descriptor. When this bit is set, the TxPBL value must be greater than or equal to 4.
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TSE_W<12> {
        TSE_W::new(self)
    }
    ///Bits 16:21 - Transmit Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer. This is the maximum value that is used in a single block Read or Write. The DMA always attempts to burst as specified in PBL each time it starts a burst transfer on the application bus. You can program PBL with any of the following values: 1, 2, 4, 8, 16, or 32. Any other value results in undefined behavior. To transfer more than 32 beats, perform the following steps: Set the PBLx8 mode in ETH_DMACCR. Set the TXPBL\[5:0\]. Note: The maximum value of TXPBL must be less than or equal to half the Tx Queue size (TQS field of Tx queue operating mode Register (ETH_MTLTXQOMR)) in terms of beats. This is required so that the Tx Queue has space to store at least another Tx PBL worth of data while the MTL Tx Queue Controller is transferring data to MAC. The total locations in Tx Queue of size 2048 bytes is 512, TXPBL and 8xPBL needs to be programmed to less than or equal to 512/2.
    #[inline(always)]
    #[must_use]
    pub fn txpbl(&mut self) -> TXPBL_W<16> {
        TXPBL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Channel transmit control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmactxcr](index.html) module
pub struct DMACTXCR_SPEC;
impl crate::RegisterSpec for DMACTXCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmactxcr::R](R) reader structure
impl crate::Readable for DMACTXCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dmactxcr::W](W) writer structure
impl crate::Writable for DMACTXCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMACTXCR to value 0
impl crate::Resettable for DMACTXCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
