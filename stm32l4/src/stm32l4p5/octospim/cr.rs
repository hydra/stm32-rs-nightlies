///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MUXEN` reader - Multiplexed mode enable
pub type MUXEN_R = crate::BitReader<bool>;
///Field `MUXEN` writer - Multiplexed mode enable
pub type MUXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `REQ2ACK_TIME` reader - REQ to ACK time
pub type REQ2ACK_TIME_R = crate::FieldReader<u8, u8>;
///Field `REQ2ACK_TIME` writer - REQ to ACK time
pub type REQ2ACK_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bit 0 - Multiplexed mode enable
    #[inline(always)]
    pub fn muxen(&self) -> MUXEN_R {
        MUXEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 16:23 - REQ to ACK time
    #[inline(always)]
    pub fn req2ack_time(&self) -> REQ2ACK_TIME_R {
        REQ2ACK_TIME_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bit 0 - Multiplexed mode enable
    #[inline(always)]
    #[must_use]
    pub fn muxen(&mut self) -> MUXEN_W<0> {
        MUXEN_W::new(self)
    }
    ///Bits 16:23 - REQ to ACK time
    #[inline(always)]
    #[must_use]
    pub fn req2ack_time(&mut self) -> REQ2ACK_TIME_W<16> {
        REQ2ACK_TIME_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
