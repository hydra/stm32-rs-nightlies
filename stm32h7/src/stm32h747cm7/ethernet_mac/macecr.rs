///Register `MACECR` reader
pub struct R(crate::R<MACECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACECR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACECR` writer
pub struct W(crate::W<MACECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACECR_SPEC>;
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
impl From<crate::W<MACECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACECR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GPSL` reader - Giant Packet Size Limit
pub type GPSL_R = crate::FieldReader<u16, u16>;
///Field `GPSL` writer - Giant Packet Size Limit
pub type GPSL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACECR_SPEC, u16, u16, 14, O>;
///Field `DCRCC` reader - Disable CRC Checking for Received Packets
pub type DCRCC_R = crate::BitReader<bool>;
///Field `DCRCC` writer - Disable CRC Checking for Received Packets
pub type DCRCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACECR_SPEC, bool, O>;
///Field `SPEN` reader - Slow Protocol Detection Enable
pub type SPEN_R = crate::BitReader<bool>;
///Field `SPEN` writer - Slow Protocol Detection Enable
pub type SPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACECR_SPEC, bool, O>;
///Field `USP` reader - Unicast Slow Protocol Packet Detect
pub type USP_R = crate::BitReader<bool>;
///Field `USP` writer - Unicast Slow Protocol Packet Detect
pub type USP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACECR_SPEC, bool, O>;
///Field `EIPGEN` reader - Extended Inter-Packet Gap Enable
pub type EIPGEN_R = crate::BitReader<bool>;
///Field `EIPGEN` writer - Extended Inter-Packet Gap Enable
pub type EIPGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACECR_SPEC, bool, O>;
///Field `EIPG` reader - Extended Inter-Packet Gap
pub type EIPG_R = crate::FieldReader<u8, u8>;
///Field `EIPG` writer - Extended Inter-Packet Gap
pub type EIPG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACECR_SPEC, u8, u8, 5, O>;
impl R {
    ///Bits 0:13 - Giant Packet Size Limit
    #[inline(always)]
    pub fn gpsl(&self) -> GPSL_R {
        GPSL_R::new((self.bits & 0x3fff) as u16)
    }
    ///Bit 16 - Disable CRC Checking for Received Packets
    #[inline(always)]
    pub fn dcrcc(&self) -> DCRCC_R {
        DCRCC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Slow Protocol Detection Enable
    #[inline(always)]
    pub fn spen(&self) -> SPEN_R {
        SPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Unicast Slow Protocol Packet Detect
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 24 - Extended Inter-Packet Gap Enable
    #[inline(always)]
    pub fn eipgen(&self) -> EIPGEN_R {
        EIPGEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:29 - Extended Inter-Packet Gap
    #[inline(always)]
    pub fn eipg(&self) -> EIPG_R {
        EIPG_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:13 - Giant Packet Size Limit
    #[inline(always)]
    #[must_use]
    pub fn gpsl(&mut self) -> GPSL_W<0> {
        GPSL_W::new(self)
    }
    ///Bit 16 - Disable CRC Checking for Received Packets
    #[inline(always)]
    #[must_use]
    pub fn dcrcc(&mut self) -> DCRCC_W<16> {
        DCRCC_W::new(self)
    }
    ///Bit 17 - Slow Protocol Detection Enable
    #[inline(always)]
    #[must_use]
    pub fn spen(&mut self) -> SPEN_W<17> {
        SPEN_W::new(self)
    }
    ///Bit 18 - Unicast Slow Protocol Packet Detect
    #[inline(always)]
    #[must_use]
    pub fn usp(&mut self) -> USP_W<18> {
        USP_W::new(self)
    }
    ///Bit 24 - Extended Inter-Packet Gap Enable
    #[inline(always)]
    #[must_use]
    pub fn eipgen(&mut self) -> EIPGEN_W<24> {
        EIPGEN_W::new(self)
    }
    ///Bits 25:29 - Extended Inter-Packet Gap
    #[inline(always)]
    #[must_use]
    pub fn eipg(&mut self) -> EIPG_W<25> {
        EIPG_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Extended operating mode configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macecr](index.html) module
pub struct MACECR_SPEC;
impl crate::RegisterSpec for MACECR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macecr::R](R) reader structure
impl crate::Readable for MACECR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macecr::W](W) writer structure
impl crate::Writable for MACECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACECR to value 0
impl crate::Resettable for MACECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
