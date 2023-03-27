///Register `MACRXTXSR` reader
pub struct R(crate::R<MACRXTXSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACRXTXSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACRXTXSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACRXTXSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACRXTXSR` writer
pub struct W(crate::W<MACRXTXSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACRXTXSR_SPEC>;
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
impl From<crate::W<MACRXTXSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACRXTXSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TJT` reader - Transmit Jabber Timeout This bit indicates that the Transmit Jabber Timer expired which happens when the packet size exceeds 2,048 bytes (10,240 bytes when the Jumbo packet is enabled) and JD bit is reset in the Operating mode configuration register (ETH_MACCR). This bit is set when the packet size exceeds 16,383 bytes and the JD bit is set in the Operating mode configuration register (ETH_MACCR). Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
///
///The field is **cleared** (set to zero) following a read operation.
pub type TJT_R = crate::BitReader<bool>;
///Field `TJT` writer - Transmit Jabber Timeout This bit indicates that the Transmit Jabber Timer expired which happens when the packet size exceeds 2,048 bytes (10,240 bytes when the Jumbo packet is enabled) and JD bit is reset in the Operating mode configuration register (ETH_MACCR). This bit is set when the packet size exceeds 16,383 bytes and the JD bit is set in the Operating mode configuration register (ETH_MACCR). Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
pub type TJT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACRXTXSR_SPEC, bool, O>;
///Field `NCARR` reader - No Carrier When the DTXSTS bit is set in the Operating mode Register (ETH_MTLOMR), this bit indicates that the carrier signal from the PHY is not present at the end of preamble transmission. Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
///
///The field is **cleared** (set to zero) following a read operation.
pub type NCARR_R = crate::BitReader<bool>;
///Field `NCARR` writer - No Carrier When the DTXSTS bit is set in the Operating mode Register (ETH_MTLOMR), this bit indicates that the carrier signal from the PHY is not present at the end of preamble transmission. Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
pub type NCARR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACRXTXSR_SPEC, bool, O>;
///Field `LCARR` reader - Loss of Carrier When the DTXSTS bit is set in the Operating mode Register (ETH_MTLOMR), this bit indicates that the loss of carrier occurred during packet transmission, that is, the ETH_CRS signal was inactive for one or more transmission clock periods during packet transmission. This bit is valid only for packets transmitted without collision. Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
///
///The field is **cleared** (set to zero) following a read operation.
pub type LCARR_R = crate::BitReader<bool>;
///Field `LCARR` writer - Loss of Carrier When the DTXSTS bit is set in the Operating mode Register (ETH_MTLOMR), this bit indicates that the loss of carrier occurred during packet transmission, that is, the ETH_CRS signal was inactive for one or more transmission clock periods during packet transmission. This bit is valid only for packets transmitted without collision. Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
pub type LCARR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACRXTXSR_SPEC, bool, O>;
///Field `EXDEF` reader - Excessive Deferral When the DTXSTS bit is set in the Operating mode Register (ETH_MTLOMR) and the DC bit is set in the Operating mode configuration register (ETH_MACCR), this bit indicates that the transmission ended because of excessive deferral of over 24,288 bit times (155,680 when Jumbo packet is enabled). Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
///
///The field is **cleared** (set to zero) following a read operation.
pub type EXDEF_R = crate::BitReader<bool>;
///Field `EXDEF` writer - Excessive Deferral When the DTXSTS bit is set in the Operating mode Register (ETH_MTLOMR) and the DC bit is set in the Operating mode configuration register (ETH_MACCR), this bit indicates that the transmission ended because of excessive deferral of over 24,288 bit times (155,680 when Jumbo packet is enabled). Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
pub type EXDEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACRXTXSR_SPEC, bool, O>;
///Field `LCOL` reader - Late Collision When the DTXSTS bit is set in the Operating mode Register (ETH_MTLOMR), this bit indicates that the packet transmission aborted because a collision occurred after the collision window (64 bytes including Preamble in MII mode. This bit is not valid if the Underflow error occurs. Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
///
///The field is **cleared** (set to zero) following a read operation.
pub type LCOL_R = crate::BitReader<bool>;
///Field `LCOL` writer - Late Collision When the DTXSTS bit is set in the Operating mode Register (ETH_MTLOMR), this bit indicates that the packet transmission aborted because a collision occurred after the collision window (64 bytes including Preamble in MII mode. This bit is not valid if the Underflow error occurs. Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
pub type LCOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACRXTXSR_SPEC, bool, O>;
///Field `EXCOL` reader - Excessive Collisions When the DTXSTS bit is set in the Operating mode Register (ETH_MTLOMR), this bit indicates that the transmission aborted after 16 successive collisions while attempting to transmit the current packet. If the DR bit is set in the Operating mode configuration register (ETH_MACCR), this bit is set after the first collision and the packet transmission is aborted. Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
///
///The field is **cleared** (set to zero) following a read operation.
pub type EXCOL_R = crate::BitReader<bool>;
///Field `EXCOL` writer - Excessive Collisions When the DTXSTS bit is set in the Operating mode Register (ETH_MTLOMR), this bit indicates that the transmission aborted after 16 successive collisions while attempting to transmit the current packet. If the DR bit is set in the Operating mode configuration register (ETH_MACCR), this bit is set after the first collision and the packet transmission is aborted. Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
pub type EXCOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACRXTXSR_SPEC, bool, O>;
///Field `RWT` reader - Receive Watchdog Timeout This bit is set when a packet with length greater than 2,048 bytes is received (10, 240 bytes when Jumbo Packet mode is enabled) and the WD bit is reset in the Operating mode configuration register (ETH_MACCR). This bit is set when a packet with length greater than 16,383 bytes is received and the WD bit is set in the Operating mode configuration register (ETH_MACCR). Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
///
///The field is **cleared** (set to zero) following a read operation.
pub type RWT_R = crate::BitReader<bool>;
///Field `RWT` writer - Receive Watchdog Timeout This bit is set when a packet with length greater than 2,048 bytes is received (10, 240 bytes when Jumbo Packet mode is enabled) and the WD bit is reset in the Operating mode configuration register (ETH_MACCR). This bit is set when a packet with length greater than 16,383 bytes is received and the WD bit is set in the Operating mode configuration register (ETH_MACCR). Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
pub type RWT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACRXTXSR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Transmit Jabber Timeout This bit indicates that the Transmit Jabber Timer expired which happens when the packet size exceeds 2,048 bytes (10,240 bytes when the Jumbo packet is enabled) and JD bit is reset in the Operating mode configuration register (ETH_MACCR). This bit is set when the packet size exceeds 16,383 bytes and the JD bit is set in the Operating mode configuration register (ETH_MACCR). Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
    #[inline(always)]
    pub fn tjt(&self) -> TJT_R {
        TJT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - No Carrier When the DTXSTS bit is set in the Operating mode Register (ETH_MTLOMR), this bit indicates that the carrier signal from the PHY is not present at the end of preamble transmission. Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
    #[inline(always)]
    pub fn ncarr(&self) -> NCARR_R {
        NCARR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Loss of Carrier When the DTXSTS bit is set in the Operating mode Register (ETH_MTLOMR), this bit indicates that the loss of carrier occurred during packet transmission, that is, the ETH_CRS signal was inactive for one or more transmission clock periods during packet transmission. This bit is valid only for packets transmitted without collision. Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
    #[inline(always)]
    pub fn lcarr(&self) -> LCARR_R {
        LCARR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Excessive Deferral When the DTXSTS bit is set in the Operating mode Register (ETH_MTLOMR) and the DC bit is set in the Operating mode configuration register (ETH_MACCR), this bit indicates that the transmission ended because of excessive deferral of over 24,288 bit times (155,680 when Jumbo packet is enabled). Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
    #[inline(always)]
    pub fn exdef(&self) -> EXDEF_R {
        EXDEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Late Collision When the DTXSTS bit is set in the Operating mode Register (ETH_MTLOMR), this bit indicates that the packet transmission aborted because a collision occurred after the collision window (64 bytes including Preamble in MII mode. This bit is not valid if the Underflow error occurs. Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Excessive Collisions When the DTXSTS bit is set in the Operating mode Register (ETH_MTLOMR), this bit indicates that the transmission aborted after 16 successive collisions while attempting to transmit the current packet. If the DR bit is set in the Operating mode configuration register (ETH_MACCR), this bit is set after the first collision and the packet transmission is aborted. Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
    #[inline(always)]
    pub fn excol(&self) -> EXCOL_R {
        EXCOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Receive Watchdog Timeout This bit is set when a packet with length greater than 2,048 bytes is received (10, 240 bytes when Jumbo Packet mode is enabled) and the WD bit is reset in the Operating mode configuration register (ETH_MACCR). This bit is set when a packet with length greater than 16,383 bytes is received and the WD bit is set in the Operating mode configuration register (ETH_MACCR). Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Transmit Jabber Timeout This bit indicates that the Transmit Jabber Timer expired which happens when the packet size exceeds 2,048 bytes (10,240 bytes when the Jumbo packet is enabled) and JD bit is reset in the Operating mode configuration register (ETH_MACCR). This bit is set when the packet size exceeds 16,383 bytes and the JD bit is set in the Operating mode configuration register (ETH_MACCR). Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
    #[inline(always)]
    #[must_use]
    pub fn tjt(&mut self) -> TJT_W<0> {
        TJT_W::new(self)
    }
    ///Bit 1 - No Carrier When the DTXSTS bit is set in the Operating mode Register (ETH_MTLOMR), this bit indicates that the carrier signal from the PHY is not present at the end of preamble transmission. Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
    #[inline(always)]
    #[must_use]
    pub fn ncarr(&mut self) -> NCARR_W<1> {
        NCARR_W::new(self)
    }
    ///Bit 2 - Loss of Carrier When the DTXSTS bit is set in the Operating mode Register (ETH_MTLOMR), this bit indicates that the loss of carrier occurred during packet transmission, that is, the ETH_CRS signal was inactive for one or more transmission clock periods during packet transmission. This bit is valid only for packets transmitted without collision. Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
    #[inline(always)]
    #[must_use]
    pub fn lcarr(&mut self) -> LCARR_W<2> {
        LCARR_W::new(self)
    }
    ///Bit 3 - Excessive Deferral When the DTXSTS bit is set in the Operating mode Register (ETH_MTLOMR) and the DC bit is set in the Operating mode configuration register (ETH_MACCR), this bit indicates that the transmission ended because of excessive deferral of over 24,288 bit times (155,680 when Jumbo packet is enabled). Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
    #[inline(always)]
    #[must_use]
    pub fn exdef(&mut self) -> EXDEF_W<3> {
        EXDEF_W::new(self)
    }
    ///Bit 4 - Late Collision When the DTXSTS bit is set in the Operating mode Register (ETH_MTLOMR), this bit indicates that the packet transmission aborted because a collision occurred after the collision window (64 bytes including Preamble in MII mode. This bit is not valid if the Underflow error occurs. Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
    #[inline(always)]
    #[must_use]
    pub fn lcol(&mut self) -> LCOL_W<4> {
        LCOL_W::new(self)
    }
    ///Bit 5 - Excessive Collisions When the DTXSTS bit is set in the Operating mode Register (ETH_MTLOMR), this bit indicates that the transmission aborted after 16 successive collisions while attempting to transmit the current packet. If the DR bit is set in the Operating mode configuration register (ETH_MACCR), this bit is set after the first collision and the packet transmission is aborted. Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
    #[inline(always)]
    #[must_use]
    pub fn excol(&mut self) -> EXCOL_W<5> {
        EXCOL_W::new(self)
    }
    ///Bit 8 - Receive Watchdog Timeout This bit is set when a packet with length greater than 2,048 bytes is received (10, 240 bytes when Jumbo Packet mode is enabled) and the WD bit is reset in the Operating mode configuration register (ETH_MACCR). This bit is set when a packet with length greater than 16,383 bytes is received and the WD bit is set in the Operating mode configuration register (ETH_MACCR). Cleared on read (or write of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
    #[inline(always)]
    #[must_use]
    pub fn rwt(&mut self) -> RWT_W<8> {
        RWT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Rx Tx status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macrxtxsr](index.html) module
pub struct MACRXTXSR_SPEC;
impl crate::RegisterSpec for MACRXTXSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macrxtxsr::R](R) reader structure
impl crate::Readable for MACRXTXSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macrxtxsr::W](W) writer structure
impl crate::Writable for MACRXTXSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACRXTXSR to value 0
impl crate::Resettable for MACRXTXSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
