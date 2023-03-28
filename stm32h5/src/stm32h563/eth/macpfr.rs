///Register `MACPFR` reader
pub struct R(crate::R<MACPFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACPFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACPFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACPFR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACPFR` writer
pub struct W(crate::W<MACPFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACPFR_SPEC>;
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
impl From<crate::W<MACPFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACPFR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PR` reader - Promiscuous Mode When this bit is set, the Address Filtering module passes all incoming packets irrespective of the destination or source address. The SA or DA Filter Fails status bits of the Rx Status Word are always cleared when PR is set.
pub type PR_R = crate::BitReader<bool>;
///Field `PR` writer - Promiscuous Mode When this bit is set, the Address Filtering module passes all incoming packets irrespective of the destination or source address. The SA or DA Filter Fails status bits of the Rx Status Word are always cleared when PR is set.
pub type PR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
///Field `HUC` reader - Hash Unicast When this bit is set, the MAC performs the destination address filtering of unicast packets according to the Hash table. When this bit is reset, the MAC performs a perfect destination address filtering for unicast packets, that is, it compares the DA field with the values programmed in DA registers.
pub type HUC_R = crate::BitReader<bool>;
///Field `HUC` writer - Hash Unicast When this bit is set, the MAC performs the destination address filtering of unicast packets according to the Hash table. When this bit is reset, the MAC performs a perfect destination address filtering for unicast packets, that is, it compares the DA field with the values programmed in DA registers.
pub type HUC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
///Field `HMC` reader - Hash Multicast When this bit is set, the MAC performs the destination address filtering of received multicast packets according to the Hash table. When this bit is reset, the MAC performs the perfect destination address filtering for multicast packets, that is, it compares the DA field with the values programmed in DA registers.
pub type HMC_R = crate::BitReader<bool>;
///Field `HMC` writer - Hash Multicast When this bit is set, the MAC performs the destination address filtering of received multicast packets according to the Hash table. When this bit is reset, the MAC performs the perfect destination address filtering for multicast packets, that is, it compares the DA field with the values programmed in DA registers.
pub type HMC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
///Field `DAIF` reader - DA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast packets. When this bit is reset, normal filtering of packets is performed.
pub type DAIF_R = crate::BitReader<bool>;
///Field `DAIF` writer - DA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast packets. When this bit is reset, normal filtering of packets is performed.
pub type DAIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
///Field `PM` reader - Pass All Multicast When this bit is set, it indicates that all received packets with a multicast destination address (first bit in the destination address field is '1') are passed. When this bit is reset, filtering of multicast packet depends on HMC bit.
pub type PM_R = crate::BitReader<bool>;
///Field `PM` writer - Pass All Multicast When this bit is set, it indicates that all received packets with a multicast destination address (first bit in the destination address field is '1') are passed. When this bit is reset, filtering of multicast packet depends on HMC bit.
pub type PM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
///Field `DBF` reader - Disable Broadcast Packets When this bit is set, the AFM module blocks all incoming broadcast packets. In addition, it overrides all other filter settings. When this bit is reset, the AFM module passes all received broadcast packets.
pub type DBF_R = crate::BitReader<bool>;
///Field `DBF` writer - Disable Broadcast Packets When this bit is set, the AFM module blocks all incoming broadcast packets. In addition, it overrides all other filter settings. When this bit is reset, the AFM module passes all received broadcast packets.
pub type DBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
///Field `PCF` reader - Pass Control Packets These bits control the forwarding of all control packets (including unicast and multicast Pause packets).
pub type PCF_R = crate::FieldReader<u8, u8>;
///Field `PCF` writer - Pass Control Packets These bits control the forwarding of all control packets (including unicast and multicast Pause packets).
pub type PCF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACPFR_SPEC, u8, u8, 2, O>;
///Field `SAIF` reader - SA Inverse Filtering When this bit is set, the Address Check block operates in the inverse filtering mode for SA address comparison. If the SA of a packet matches the values programmed in the SA registers, it is marked as failing the SA Address filter. When this bit is reset, if the SA of a packet does not match the values programmed in the SA registers, it is marked as failing the SA Address filter.
pub type SAIF_R = crate::BitReader<bool>;
///Field `SAIF` writer - SA Inverse Filtering When this bit is set, the Address Check block operates in the inverse filtering mode for SA address comparison. If the SA of a packet matches the values programmed in the SA registers, it is marked as failing the SA Address filter. When this bit is reset, if the SA of a packet does not match the values programmed in the SA registers, it is marked as failing the SA Address filter.
pub type SAIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
///Field `SAF` reader - Source Address Filter Enable When this bit is set, the MAC compares the SA field of the received packets with the values programmed in the enabled SA registers. If the comparison fails, the MAC drops the packet. When this bit is reset, the MAC forwards the received packet to the application with updated SAF bit of the Rx Status depending on the SA address comparison. Note: According to the IEEE specification, Bit 47 of the SA is reserved. However, the MAC compares all 48 bits. The software driver should take this into consideration while programming the MAC address registers for SA.
pub type SAF_R = crate::BitReader<bool>;
///Field `SAF` writer - Source Address Filter Enable When this bit is set, the MAC compares the SA field of the received packets with the values programmed in the enabled SA registers. If the comparison fails, the MAC drops the packet. When this bit is reset, the MAC forwards the received packet to the application with updated SAF bit of the Rx Status depending on the SA address comparison. Note: According to the IEEE specification, Bit 47 of the SA is reserved. However, the MAC compares all 48 bits. The software driver should take this into consideration while programming the MAC address registers for SA.
pub type SAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
///Field `HPF` reader - Hash or Perfect Filter When this bit is set, the address filter passes a packet if it matches either the perfect filtering or Hash filtering as set by the HMC or HUC bit. When this bit is reset and the HUC or HMC bit is set, the packet is passed only if it matches the Hash filter.
pub type HPF_R = crate::BitReader<bool>;
///Field `HPF` writer - Hash or Perfect Filter When this bit is set, the address filter passes a packet if it matches either the perfect filtering or Hash filtering as set by the HMC or HUC bit. When this bit is reset and the HUC or HMC bit is set, the packet is passed only if it matches the Hash filter.
pub type HPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
///Field `VTFE` reader - VLAN Tag Filter Enable When this bit is set, the MAC drops the VLAN tagged packets that do not match the VLAN Tag. When this bit is reset, the MAC forwards all packets irrespective of the match status of the VLAN Tag.
pub type VTFE_R = crate::BitReader<bool>;
///Field `VTFE` writer - VLAN Tag Filter Enable When this bit is set, the MAC drops the VLAN tagged packets that do not match the VLAN Tag. When this bit is reset, the MAC forwards all packets irrespective of the match status of the VLAN Tag.
pub type VTFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
///Field `IPFE` reader - Layer 3 and Layer 4 Filter Enable When this bit is set, the MAC drops packets that do not match the enabled Layer 3 and Layer 4 filters. If Layer 3 or Layer 4 filters are not enabled for matching, this bit does not have any effect. When this bit is reset, the MAC forwards all packets irrespective of the match status of the Layer 3 and Layer 4 fields.
pub type IPFE_R = crate::BitReader<bool>;
///Field `IPFE` writer - Layer 3 and Layer 4 Filter Enable When this bit is set, the MAC drops packets that do not match the enabled Layer 3 and Layer 4 filters. If Layer 3 or Layer 4 filters are not enabled for matching, this bit does not have any effect. When this bit is reset, the MAC forwards all packets irrespective of the match status of the Layer 3 and Layer 4 fields.
pub type IPFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
///Field `DNTU` reader - Drop Non-TCP/UDP over IP Packets When this bit is set, the MAC drops the non-TCP or UDP over IP packets. The MAC forward only those packets that are processed by the Layer 4 filter. When this bit is reset, the MAC forwards all non-TCP or UDP over IP packets.
pub type DNTU_R = crate::BitReader<bool>;
///Field `DNTU` writer - Drop Non-TCP/UDP over IP Packets When this bit is set, the MAC drops the non-TCP or UDP over IP packets. The MAC forward only those packets that are processed by the Layer 4 filter. When this bit is reset, the MAC forwards all non-TCP or UDP over IP packets.
pub type DNTU_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
///Field `RA` reader - Receive All When this bit is set, the MAC Receiver module passes all received packets to the application, irrespective of whether they pass the address filter or not. The result of the SA or DA filtering is updated (pass or fail) in the corresponding bit in the Rx Status Word. When this bit is reset, the Receiver module passes only those packets to the application that pass the SA or DA address filter.
pub type RA_R = crate::BitReader<bool>;
///Field `RA` writer - Receive All When this bit is set, the MAC Receiver module passes all received packets to the application, irrespective of whether they pass the address filter or not. The result of the SA or DA filtering is updated (pass or fail) in the corresponding bit in the Rx Status Word. When this bit is reset, the Receiver module passes only those packets to the application that pass the SA or DA address filter.
pub type RA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Promiscuous Mode When this bit is set, the Address Filtering module passes all incoming packets irrespective of the destination or source address. The SA or DA Filter Fails status bits of the Rx Status Word are always cleared when PR is set.
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Hash Unicast When this bit is set, the MAC performs the destination address filtering of unicast packets according to the Hash table. When this bit is reset, the MAC performs a perfect destination address filtering for unicast packets, that is, it compares the DA field with the values programmed in DA registers.
    #[inline(always)]
    pub fn huc(&self) -> HUC_R {
        HUC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Hash Multicast When this bit is set, the MAC performs the destination address filtering of received multicast packets according to the Hash table. When this bit is reset, the MAC performs the perfect destination address filtering for multicast packets, that is, it compares the DA field with the values programmed in DA registers.
    #[inline(always)]
    pub fn hmc(&self) -> HMC_R {
        HMC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast packets. When this bit is reset, normal filtering of packets is performed.
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Pass All Multicast When this bit is set, it indicates that all received packets with a multicast destination address (first bit in the destination address field is '1') are passed. When this bit is reset, filtering of multicast packet depends on HMC bit.
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Disable Broadcast Packets When this bit is set, the AFM module blocks all incoming broadcast packets. In addition, it overrides all other filter settings. When this bit is reset, the AFM module passes all received broadcast packets.
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - Pass Control Packets These bits control the forwarding of all control packets (including unicast and multicast Pause packets).
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - SA Inverse Filtering When this bit is set, the Address Check block operates in the inverse filtering mode for SA address comparison. If the SA of a packet matches the values programmed in the SA registers, it is marked as failing the SA Address filter. When this bit is reset, if the SA of a packet does not match the values programmed in the SA registers, it is marked as failing the SA Address filter.
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Source Address Filter Enable When this bit is set, the MAC compares the SA field of the received packets with the values programmed in the enabled SA registers. If the comparison fails, the MAC drops the packet. When this bit is reset, the MAC forwards the received packet to the application with updated SAF bit of the Rx Status depending on the SA address comparison. Note: According to the IEEE specification, Bit 47 of the SA is reserved. However, the MAC compares all 48 bits. The software driver should take this into consideration while programming the MAC address registers for SA.
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Hash or Perfect Filter When this bit is set, the address filter passes a packet if it matches either the perfect filtering or Hash filtering as set by the HMC or HUC bit. When this bit is reset and the HUC or HMC bit is set, the packet is passed only if it matches the Hash filter.
    #[inline(always)]
    pub fn hpf(&self) -> HPF_R {
        HPF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 16 - VLAN Tag Filter Enable When this bit is set, the MAC drops the VLAN tagged packets that do not match the VLAN Tag. When this bit is reset, the MAC forwards all packets irrespective of the match status of the VLAN Tag.
    #[inline(always)]
    pub fn vtfe(&self) -> VTFE_R {
        VTFE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - Layer 3 and Layer 4 Filter Enable When this bit is set, the MAC drops packets that do not match the enabled Layer 3 and Layer 4 filters. If Layer 3 or Layer 4 filters are not enabled for matching, this bit does not have any effect. When this bit is reset, the MAC forwards all packets irrespective of the match status of the Layer 3 and Layer 4 fields.
    #[inline(always)]
    pub fn ipfe(&self) -> IPFE_R {
        IPFE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Drop Non-TCP/UDP over IP Packets When this bit is set, the MAC drops the non-TCP or UDP over IP packets. The MAC forward only those packets that are processed by the Layer 4 filter. When this bit is reset, the MAC forwards all non-TCP or UDP over IP packets.
    #[inline(always)]
    pub fn dntu(&self) -> DNTU_R {
        DNTU_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 31 - Receive All When this bit is set, the MAC Receiver module passes all received packets to the application, irrespective of whether they pass the address filter or not. The result of the SA or DA filtering is updated (pass or fail) in the corresponding bit in the Rx Status Word. When this bit is reset, the Receiver module passes only those packets to the application that pass the SA or DA address filter.
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Promiscuous Mode When this bit is set, the Address Filtering module passes all incoming packets irrespective of the destination or source address. The SA or DA Filter Fails status bits of the Rx Status Word are always cleared when PR is set.
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<0> {
        PR_W::new(self)
    }
    ///Bit 1 - Hash Unicast When this bit is set, the MAC performs the destination address filtering of unicast packets according to the Hash table. When this bit is reset, the MAC performs a perfect destination address filtering for unicast packets, that is, it compares the DA field with the values programmed in DA registers.
    #[inline(always)]
    #[must_use]
    pub fn huc(&mut self) -> HUC_W<1> {
        HUC_W::new(self)
    }
    ///Bit 2 - Hash Multicast When this bit is set, the MAC performs the destination address filtering of received multicast packets according to the Hash table. When this bit is reset, the MAC performs the perfect destination address filtering for multicast packets, that is, it compares the DA field with the values programmed in DA registers.
    #[inline(always)]
    #[must_use]
    pub fn hmc(&mut self) -> HMC_W<2> {
        HMC_W::new(self)
    }
    ///Bit 3 - DA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast packets. When this bit is reset, normal filtering of packets is performed.
    #[inline(always)]
    #[must_use]
    pub fn daif(&mut self) -> DAIF_W<3> {
        DAIF_W::new(self)
    }
    ///Bit 4 - Pass All Multicast When this bit is set, it indicates that all received packets with a multicast destination address (first bit in the destination address field is '1') are passed. When this bit is reset, filtering of multicast packet depends on HMC bit.
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<4> {
        PM_W::new(self)
    }
    ///Bit 5 - Disable Broadcast Packets When this bit is set, the AFM module blocks all incoming broadcast packets. In addition, it overrides all other filter settings. When this bit is reset, the AFM module passes all received broadcast packets.
    #[inline(always)]
    #[must_use]
    pub fn dbf(&mut self) -> DBF_W<5> {
        DBF_W::new(self)
    }
    ///Bits 6:7 - Pass Control Packets These bits control the forwarding of all control packets (including unicast and multicast Pause packets).
    #[inline(always)]
    #[must_use]
    pub fn pcf(&mut self) -> PCF_W<6> {
        PCF_W::new(self)
    }
    ///Bit 8 - SA Inverse Filtering When this bit is set, the Address Check block operates in the inverse filtering mode for SA address comparison. If the SA of a packet matches the values programmed in the SA registers, it is marked as failing the SA Address filter. When this bit is reset, if the SA of a packet does not match the values programmed in the SA registers, it is marked as failing the SA Address filter.
    #[inline(always)]
    #[must_use]
    pub fn saif(&mut self) -> SAIF_W<8> {
        SAIF_W::new(self)
    }
    ///Bit 9 - Source Address Filter Enable When this bit is set, the MAC compares the SA field of the received packets with the values programmed in the enabled SA registers. If the comparison fails, the MAC drops the packet. When this bit is reset, the MAC forwards the received packet to the application with updated SAF bit of the Rx Status depending on the SA address comparison. Note: According to the IEEE specification, Bit 47 of the SA is reserved. However, the MAC compares all 48 bits. The software driver should take this into consideration while programming the MAC address registers for SA.
    #[inline(always)]
    #[must_use]
    pub fn saf(&mut self) -> SAF_W<9> {
        SAF_W::new(self)
    }
    ///Bit 10 - Hash or Perfect Filter When this bit is set, the address filter passes a packet if it matches either the perfect filtering or Hash filtering as set by the HMC or HUC bit. When this bit is reset and the HUC or HMC bit is set, the packet is passed only if it matches the Hash filter.
    #[inline(always)]
    #[must_use]
    pub fn hpf(&mut self) -> HPF_W<10> {
        HPF_W::new(self)
    }
    ///Bit 16 - VLAN Tag Filter Enable When this bit is set, the MAC drops the VLAN tagged packets that do not match the VLAN Tag. When this bit is reset, the MAC forwards all packets irrespective of the match status of the VLAN Tag.
    #[inline(always)]
    #[must_use]
    pub fn vtfe(&mut self) -> VTFE_W<16> {
        VTFE_W::new(self)
    }
    ///Bit 20 - Layer 3 and Layer 4 Filter Enable When this bit is set, the MAC drops packets that do not match the enabled Layer 3 and Layer 4 filters. If Layer 3 or Layer 4 filters are not enabled for matching, this bit does not have any effect. When this bit is reset, the MAC forwards all packets irrespective of the match status of the Layer 3 and Layer 4 fields.
    #[inline(always)]
    #[must_use]
    pub fn ipfe(&mut self) -> IPFE_W<20> {
        IPFE_W::new(self)
    }
    ///Bit 21 - Drop Non-TCP/UDP over IP Packets When this bit is set, the MAC drops the non-TCP or UDP over IP packets. The MAC forward only those packets that are processed by the Layer 4 filter. When this bit is reset, the MAC forwards all non-TCP or UDP over IP packets.
    #[inline(always)]
    #[must_use]
    pub fn dntu(&mut self) -> DNTU_W<21> {
        DNTU_W::new(self)
    }
    ///Bit 31 - Receive All When this bit is set, the MAC Receiver module passes all received packets to the application, irrespective of whether they pass the address filter or not. The result of the SA or DA filtering is updated (pass or fail) in the corresponding bit in the Rx Status Word. When this bit is reset, the Receiver module passes only those packets to the application that pass the SA or DA address filter.
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RA_W<31> {
        RA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Packet filtering control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macpfr](index.html) module
pub struct MACPFR_SPEC;
impl crate::RegisterSpec for MACPFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macpfr::R](R) reader structure
impl crate::Readable for MACPFR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macpfr::W](W) writer structure
impl crate::Writable for MACPFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACPFR to value 0
impl crate::Resettable for MACPFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
