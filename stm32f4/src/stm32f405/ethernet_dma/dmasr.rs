///Register `DMASR` reader
pub struct R(crate::R<DMASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMASR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMASR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMASR` writer
pub struct W(crate::W<DMASR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMASR_SPEC>;
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
impl From<crate::W<DMASR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMASR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TS` reader - TS
pub type TS_R = crate::BitReader<bool>;
///Field `TS` writer - TS
pub type TS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `TPSS` reader - TPSS
pub type TPSS_R = crate::BitReader<bool>;
///Field `TPSS` writer - TPSS
pub type TPSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `TBUS` reader - TBUS
pub type TBUS_R = crate::BitReader<bool>;
///Field `TBUS` writer - TBUS
pub type TBUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `TJTS` reader - TJTS
pub type TJTS_R = crate::BitReader<bool>;
///Field `TJTS` writer - TJTS
pub type TJTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `ROS` reader - ROS
pub type ROS_R = crate::BitReader<bool>;
///Field `ROS` writer - ROS
pub type ROS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `TUS` reader - TUS
pub type TUS_R = crate::BitReader<bool>;
///Field `TUS` writer - TUS
pub type TUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `RS` reader - RS
pub type RS_R = crate::BitReader<bool>;
///Field `RS` writer - RS
pub type RS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `RBUS` reader - RBUS
pub type RBUS_R = crate::BitReader<bool>;
///Field `RBUS` writer - RBUS
pub type RBUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `RPSS` reader - RPSS
pub type RPSS_R = crate::BitReader<bool>;
///Field `RPSS` writer - RPSS
pub type RPSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `PWTS` reader - PWTS
pub type PWTS_R = crate::BitReader<bool>;
///Field `PWTS` writer - PWTS
pub type PWTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `ETS` reader - ETS
pub type ETS_R = crate::BitReader<bool>;
///Field `ETS` writer - ETS
pub type ETS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `FBES` reader - FBES
pub type FBES_R = crate::BitReader<bool>;
///Field `FBES` writer - FBES
pub type FBES_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `ERS` reader - ERS
pub type ERS_R = crate::BitReader<bool>;
///Field `ERS` writer - ERS
pub type ERS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `AIS` reader - AIS
pub type AIS_R = crate::BitReader<bool>;
///Field `AIS` writer - AIS
pub type AIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `NIS` reader - NIS
pub type NIS_R = crate::BitReader<bool>;
///Field `NIS` writer - NIS
pub type NIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `RPS` reader - RPS
pub type RPS_R = crate::FieldReader<u8, u8>;
///Field `TPS` reader - TPS
pub type TPS_R = crate::FieldReader<u8, u8>;
///Field `EBS` reader - EBS
pub type EBS_R = crate::FieldReader<u8, u8>;
///Field `MMCS` reader - MMCS
pub type MMCS_R = crate::BitReader<bool>;
///Field `PMTS` reader - PMTS
pub type PMTS_R = crate::BitReader<bool>;
///Field `TSTS` reader - TSTS
pub type TSTS_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - TS
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TPSS
    #[inline(always)]
    pub fn tpss(&self) -> TPSS_R {
        TPSS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TBUS
    #[inline(always)]
    pub fn tbus(&self) -> TBUS_R {
        TBUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TJTS
    #[inline(always)]
    pub fn tjts(&self) -> TJTS_R {
        TJTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ROS
    #[inline(always)]
    pub fn ros(&self) -> ROS_R {
        ROS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TUS
    #[inline(always)]
    pub fn tus(&self) -> TUS_R {
        TUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RS
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - RBUS
    #[inline(always)]
    pub fn rbus(&self) -> RBUS_R {
        RBUS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - RPSS
    #[inline(always)]
    pub fn rpss(&self) -> RPSS_R {
        RPSS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PWTS
    #[inline(always)]
    pub fn pwts(&self) -> PWTS_R {
        PWTS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ETS
    #[inline(always)]
    pub fn ets(&self) -> ETS_R {
        ETS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - FBES
    #[inline(always)]
    pub fn fbes(&self) -> FBES_R {
        FBES_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ERS
    #[inline(always)]
    pub fn ers(&self) -> ERS_R {
        ERS_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - AIS
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - NIS
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:19 - RPS
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bits 20:22 - TPS
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 23:25 - EBS
    #[inline(always)]
    pub fn ebs(&self) -> EBS_R {
        EBS_R::new(((self.bits >> 23) & 7) as u8)
    }
    ///Bit 27 - MMCS
    #[inline(always)]
    pub fn mmcs(&self) -> MMCS_R {
        MMCS_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - PMTS
    #[inline(always)]
    pub fn pmts(&self) -> PMTS_R {
        PMTS_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - TSTS
    #[inline(always)]
    pub fn tsts(&self) -> TSTS_R {
        TSTS_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TS
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<0> {
        TS_W::new(self)
    }
    ///Bit 1 - TPSS
    #[inline(always)]
    #[must_use]
    pub fn tpss(&mut self) -> TPSS_W<1> {
        TPSS_W::new(self)
    }
    ///Bit 2 - TBUS
    #[inline(always)]
    #[must_use]
    pub fn tbus(&mut self) -> TBUS_W<2> {
        TBUS_W::new(self)
    }
    ///Bit 3 - TJTS
    #[inline(always)]
    #[must_use]
    pub fn tjts(&mut self) -> TJTS_W<3> {
        TJTS_W::new(self)
    }
    ///Bit 4 - ROS
    #[inline(always)]
    #[must_use]
    pub fn ros(&mut self) -> ROS_W<4> {
        ROS_W::new(self)
    }
    ///Bit 5 - TUS
    #[inline(always)]
    #[must_use]
    pub fn tus(&mut self) -> TUS_W<5> {
        TUS_W::new(self)
    }
    ///Bit 6 - RS
    #[inline(always)]
    #[must_use]
    pub fn rs(&mut self) -> RS_W<6> {
        RS_W::new(self)
    }
    ///Bit 7 - RBUS
    #[inline(always)]
    #[must_use]
    pub fn rbus(&mut self) -> RBUS_W<7> {
        RBUS_W::new(self)
    }
    ///Bit 8 - RPSS
    #[inline(always)]
    #[must_use]
    pub fn rpss(&mut self) -> RPSS_W<8> {
        RPSS_W::new(self)
    }
    ///Bit 9 - PWTS
    #[inline(always)]
    #[must_use]
    pub fn pwts(&mut self) -> PWTS_W<9> {
        PWTS_W::new(self)
    }
    ///Bit 10 - ETS
    #[inline(always)]
    #[must_use]
    pub fn ets(&mut self) -> ETS_W<10> {
        ETS_W::new(self)
    }
    ///Bit 13 - FBES
    #[inline(always)]
    #[must_use]
    pub fn fbes(&mut self) -> FBES_W<13> {
        FBES_W::new(self)
    }
    ///Bit 14 - ERS
    #[inline(always)]
    #[must_use]
    pub fn ers(&mut self) -> ERS_W<14> {
        ERS_W::new(self)
    }
    ///Bit 15 - AIS
    #[inline(always)]
    #[must_use]
    pub fn ais(&mut self) -> AIS_W<15> {
        AIS_W::new(self)
    }
    ///Bit 16 - NIS
    #[inline(always)]
    #[must_use]
    pub fn nis(&mut self) -> NIS_W<16> {
        NIS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet DMA status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmasr](index.html) module
pub struct DMASR_SPEC;
impl crate::RegisterSpec for DMASR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmasr::R](R) reader structure
impl crate::Readable for DMASR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dmasr::W](W) writer structure
impl crate::Writable for DMASR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMASR to value 0
impl crate::Resettable for DMASR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
