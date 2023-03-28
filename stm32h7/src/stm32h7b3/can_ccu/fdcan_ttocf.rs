///Register `FDCAN_TTOCF` reader
pub struct R(crate::R<FDCAN_TTOCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTOCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTOCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTOCF_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FDCAN_TTOCF` writer
pub struct W(crate::W<FDCAN_TTOCF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TTOCF_SPEC>;
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
impl From<crate::W<FDCAN_TTOCF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TTOCF_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OM` reader - Operation Mode
pub type OM_R = crate::FieldReader<u8, u8>;
///Field `OM` writer - Operation Mode
pub type OM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTOCF_SPEC, u8, u8, 2, O>;
///Field `GEN` reader - Gap Enable
pub type GEN_R = crate::BitReader<bool>;
///Field `GEN` writer - Gap Enable
pub type GEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCF_SPEC, bool, O>;
///Field `TM` reader - Time Master
pub type TM_R = crate::BitReader<bool>;
///Field `TM` writer - Time Master
pub type TM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCF_SPEC, bool, O>;
///Field `LDSDL` reader - LD of Synchronization Deviation Limit
pub type LDSDL_R = crate::FieldReader<u8, u8>;
///Field `LDSDL` writer - LD of Synchronization Deviation Limit
pub type LDSDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTOCF_SPEC, u8, u8, 3, O>;
///Field `IRTO` reader - Initial Reference Trigger Offset
pub type IRTO_R = crate::FieldReader<u8, u8>;
///Field `IRTO` writer - Initial Reference Trigger Offset
pub type IRTO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTOCF_SPEC, u8, u8, 7, O>;
///Field `EECS` reader - Enable External Clock Synchronization
pub type EECS_R = crate::BitReader<bool>;
///Field `EECS` writer - Enable External Clock Synchronization
pub type EECS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCF_SPEC, bool, O>;
///Field `AWL` reader - Application Watchdog Limit
pub type AWL_R = crate::FieldReader<u8, u8>;
///Field `AWL` writer - Application Watchdog Limit
pub type AWL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTOCF_SPEC, u8, u8, 8, O>;
///Field `EGTF` reader - Enable Global Time Filtering
pub type EGTF_R = crate::BitReader<bool>;
///Field `EGTF` writer - Enable Global Time Filtering
pub type EGTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCF_SPEC, bool, O>;
///Field `ECC` reader - Enable Clock Calibration
pub type ECC_R = crate::BitReader<bool>;
///Field `ECC` writer - Enable Clock Calibration
pub type ECC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCF_SPEC, bool, O>;
///Field `EVTP` reader - Event Trigger Polarity
pub type EVTP_R = crate::BitReader<bool>;
///Field `EVTP` writer - Event Trigger Polarity
pub type EVTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCF_SPEC, bool, O>;
impl R {
    ///Bits 0:1 - Operation Mode
    #[inline(always)]
    pub fn om(&self) -> OM_R {
        OM_R::new((self.bits & 3) as u8)
    }
    ///Bit 3 - Gap Enable
    #[inline(always)]
    pub fn gen(&self) -> GEN_R {
        GEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Time Master
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - LD of Synchronization Deviation Limit
    #[inline(always)]
    pub fn ldsdl(&self) -> LDSDL_R {
        LDSDL_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bits 8:14 - Initial Reference Trigger Offset
    #[inline(always)]
    pub fn irto(&self) -> IRTO_R {
        IRTO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 15 - Enable External Clock Synchronization
    #[inline(always)]
    pub fn eecs(&self) -> EECS_R {
        EECS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - Application Watchdog Limit
    #[inline(always)]
    pub fn awl(&self) -> AWL_R {
        AWL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - Enable Global Time Filtering
    #[inline(always)]
    pub fn egtf(&self) -> EGTF_R {
        EGTF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Enable Clock Calibration
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Event Trigger Polarity
    #[inline(always)]
    pub fn evtp(&self) -> EVTP_R {
        EVTP_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Operation Mode
    #[inline(always)]
    #[must_use]
    pub fn om(&mut self) -> OM_W<0> {
        OM_W::new(self)
    }
    ///Bit 3 - Gap Enable
    #[inline(always)]
    #[must_use]
    pub fn gen(&mut self) -> GEN_W<3> {
        GEN_W::new(self)
    }
    ///Bit 4 - Time Master
    #[inline(always)]
    #[must_use]
    pub fn tm(&mut self) -> TM_W<4> {
        TM_W::new(self)
    }
    ///Bits 5:7 - LD of Synchronization Deviation Limit
    #[inline(always)]
    #[must_use]
    pub fn ldsdl(&mut self) -> LDSDL_W<5> {
        LDSDL_W::new(self)
    }
    ///Bits 8:14 - Initial Reference Trigger Offset
    #[inline(always)]
    #[must_use]
    pub fn irto(&mut self) -> IRTO_W<8> {
        IRTO_W::new(self)
    }
    ///Bit 15 - Enable External Clock Synchronization
    #[inline(always)]
    #[must_use]
    pub fn eecs(&mut self) -> EECS_W<15> {
        EECS_W::new(self)
    }
    ///Bits 16:23 - Application Watchdog Limit
    #[inline(always)]
    #[must_use]
    pub fn awl(&mut self) -> AWL_W<16> {
        AWL_W::new(self)
    }
    ///Bit 24 - Enable Global Time Filtering
    #[inline(always)]
    #[must_use]
    pub fn egtf(&mut self) -> EGTF_W<24> {
        EGTF_W::new(self)
    }
    ///Bit 25 - Enable Clock Calibration
    #[inline(always)]
    #[must_use]
    pub fn ecc(&mut self) -> ECC_W<25> {
        ECC_W::new(self)
    }
    ///Bit 26 - Event Trigger Polarity
    #[inline(always)]
    #[must_use]
    pub fn evtp(&mut self) -> EVTP_W<26> {
        EVTP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN TT Operation Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_ttocf](index.html) module
pub struct FDCAN_TTOCF_SPEC;
impl crate::RegisterSpec for FDCAN_TTOCF_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_ttocf::R](R) reader structure
impl crate::Readable for FDCAN_TTOCF_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fdcan_ttocf::W](W) writer structure
impl crate::Writable for FDCAN_TTOCF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FDCAN_TTOCF to value 0x0001_0000
impl crate::Resettable for FDCAN_TTOCF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
