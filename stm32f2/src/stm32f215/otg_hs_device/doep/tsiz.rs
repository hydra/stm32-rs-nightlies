///Register `TSIZ` reader
pub struct R(crate::R<TSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSIZ_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TSIZ` writer
pub struct W(crate::W<TSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSIZ_SPEC>;
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
impl From<crate::W<TSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSIZ_SPEC>) -> Self {
        W(writer)
    }
}
///Field `XFRSIZ` reader - Transfer size
pub type XFRSIZ_R = crate::FieldReader<u32, u32>;
///Field `XFRSIZ` writer - Transfer size
pub type XFRSIZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSIZ_SPEC, u32, u32, 19, O>;
///Field `PKTCNT` reader - Packet count
pub type PKTCNT_R = crate::FieldReader<u16, u16>;
///Field `PKTCNT` writer - Packet count
pub type PKTCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSIZ_SPEC, u16, u16, 10, O>;
///Field `RXDPID_STUPCNT` reader - Received data PID/SETUP packet count
pub type RXDPID_STUPCNT_R = crate::FieldReader<u8, u8>;
///Field `RXDPID_STUPCNT` writer - Received data PID/SETUP packet count
pub type RXDPID_STUPCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSIZ_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:18 - Transfer size
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new(self.bits & 0x0007_ffff)
    }
    ///Bits 19:28 - Packet count
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    ///Bits 29:30 - Received data PID/SETUP packet count
    #[inline(always)]
    pub fn rxdpid_stupcnt(&self) -> RXDPID_STUPCNT_R {
        RXDPID_STUPCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    ///Bits 0:18 - Transfer size
    #[inline(always)]
    #[must_use]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<0> {
        XFRSIZ_W::new(self)
    }
    ///Bits 19:28 - Packet count
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<19> {
        PKTCNT_W::new(self)
    }
    ///Bits 29:30 - Received data PID/SETUP packet count
    #[inline(always)]
    #[must_use]
    pub fn rxdpid_stupcnt(&mut self) -> RXDPID_STUPCNT_W<29> {
        RXDPID_STUPCNT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTG_HS device endpoint-2 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tsiz](index.html) module
pub struct TSIZ_SPEC;
impl crate::RegisterSpec for TSIZ_SPEC {
    type Ux = u32;
}
///`read()` method returns [tsiz::R](R) reader structure
impl crate::Readable for TSIZ_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tsiz::W](W) writer structure
impl crate::Writable for TSIZ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TSIZ to value 0
impl crate::Resettable for TSIZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
