///Register `LPMCSR` reader
pub struct R(crate::R<LPMCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMCSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LPMCSR` writer
pub struct W(crate::W<LPMCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMCSR_SPEC>;
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
impl From<crate::W<LPMCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMCSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LPMEN` reader - LPM support enable Device mode This bit is set by the software to enable the LPM support within the USB Device. If this bit is at 0 no LPM transactions are handled.
pub type LPMEN_R = crate::BitReader<bool>;
///Field `LPMEN` writer - LPM support enable Device mode This bit is set by the software to enable the LPM support within the USB Device. If this bit is at 0 no LPM transactions are handled.
pub type LPMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMCSR_SPEC, bool, O>;
///Field `LPMACK` reader - LPM token acknowledge enable Device mode: The NYET/ACK is returned only on a successful LPM transaction: No errors in both the EXT token and the LPM token (else ERROR) A valid bLinkState = 0001B (L1) is received (else STALL)
pub type LPMACK_R = crate::BitReader<bool>;
///Field `LPMACK` writer - LPM token acknowledge enable Device mode: The NYET/ACK is returned only on a successful LPM transaction: No errors in both the EXT token and the LPM token (else ERROR) A valid bLinkState = 0001B (L1) is received (else STALL)
pub type LPMACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMCSR_SPEC, bool, O>;
///Field `REMWAKE` reader - bRemoteWake value Device mode This bit contains the bRemoteWake value received with last ACKed LPM Token
pub type REMWAKE_R = crate::BitReader<bool>;
///Field `BESL` reader - BESL value Device mode These bits contain the BESL value received with last ACKed LPM Token
pub type BESL_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bit 0 - LPM support enable Device mode This bit is set by the software to enable the LPM support within the USB Device. If this bit is at 0 no LPM transactions are handled.
    #[inline(always)]
    pub fn lpmen(&self) -> LPMEN_R {
        LPMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LPM token acknowledge enable Device mode: The NYET/ACK is returned only on a successful LPM transaction: No errors in both the EXT token and the LPM token (else ERROR) A valid bLinkState = 0001B (L1) is received (else STALL)
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - bRemoteWake value Device mode This bit contains the bRemoteWake value received with last ACKed LPM Token
    #[inline(always)]
    pub fn remwake(&self) -> REMWAKE_R {
        REMWAKE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - BESL value Device mode These bits contain the BESL value received with last ACKed LPM Token
    #[inline(always)]
    pub fn besl(&self) -> BESL_R {
        BESL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - LPM support enable Device mode This bit is set by the software to enable the LPM support within the USB Device. If this bit is at 0 no LPM transactions are handled.
    #[inline(always)]
    #[must_use]
    pub fn lpmen(&mut self) -> LPMEN_W<0> {
        LPMEN_W::new(self)
    }
    ///Bit 1 - LPM token acknowledge enable Device mode: The NYET/ACK is returned only on a successful LPM transaction: No errors in both the EXT token and the LPM token (else ERROR) A valid bLinkState = 0001B (L1) is received (else STALL)
    #[inline(always)]
    #[must_use]
    pub fn lpmack(&mut self) -> LPMACK_W<1> {
        LPMACK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///USB_LPMCSR
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lpmcsr](index.html) module
pub struct LPMCSR_SPEC;
impl crate::RegisterSpec for LPMCSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lpmcsr::R](R) reader structure
impl crate::Readable for LPMCSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lpmcsr::W](W) writer structure
impl crate::Writable for LPMCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LPMCSR to value 0
impl crate::Resettable for LPMCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
