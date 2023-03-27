///Register `FDCAN_TTOCN` reader
pub struct R(crate::R<FDCAN_TTOCN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTOCN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTOCN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTOCN_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FDCAN_TTOCN` writer
pub struct W(crate::W<FDCAN_TTOCN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TTOCN_SPEC>;
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
impl From<crate::W<FDCAN_TTOCN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TTOCN_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SGT` reader - SGT
pub type SGT_R = crate::BitReader<bool>;
///Field `SGT` writer - SGT
pub type SGT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCN_SPEC, bool, O>;
///Field `ECS` reader - ECS
pub type ECS_R = crate::BitReader<bool>;
///Field `ECS` writer - ECS
pub type ECS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCN_SPEC, bool, O>;
///Field `SWP` reader - SWP
pub type SWP_R = crate::BitReader<bool>;
///Field `SWP` writer - SWP
pub type SWP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCN_SPEC, bool, O>;
///Field `SWS` reader - SWS
pub type SWS_R = crate::FieldReader<u8, u8>;
///Field `SWS` writer - SWS
pub type SWS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTOCN_SPEC, u8, u8, 2, O>;
///Field `RTIE` reader - RTIE
pub type RTIE_R = crate::BitReader<bool>;
///Field `RTIE` writer - RTIE
pub type RTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCN_SPEC, bool, O>;
///Field `TMC` reader - TMC
pub type TMC_R = crate::FieldReader<u8, u8>;
///Field `TMC` writer - TMC
pub type TMC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTOCN_SPEC, u8, u8, 2, O>;
///Field `TTIE` reader - TTIE
pub type TTIE_R = crate::BitReader<bool>;
///Field `TTIE` writer - TTIE
pub type TTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCN_SPEC, bool, O>;
///Field `GCS` reader - GCS
pub type GCS_R = crate::BitReader<bool>;
///Field `GCS` writer - GCS
pub type GCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCN_SPEC, bool, O>;
///Field `FGP` reader - FGP
pub type FGP_R = crate::BitReader<bool>;
///Field `FGP` writer - FGP
pub type FGP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCN_SPEC, bool, O>;
///Field `TMG` reader - TMG
pub type TMG_R = crate::BitReader<bool>;
///Field `TMG` writer - TMG
pub type TMG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCN_SPEC, bool, O>;
///Field `NIG` reader - NIG
pub type NIG_R = crate::BitReader<bool>;
///Field `NIG` writer - NIG
pub type NIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCN_SPEC, bool, O>;
///Field `ESCN` reader - ESCN
pub type ESCN_R = crate::BitReader<bool>;
///Field `ESCN` writer - ESCN
pub type ESCN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCN_SPEC, bool, O>;
///Field `LCKC` reader - LCKC
pub type LCKC_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - SGT
    #[inline(always)]
    pub fn sgt(&self) -> SGT_R {
        SGT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ECS
    #[inline(always)]
    pub fn ecs(&self) -> ECS_R {
        ECS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SWP
    #[inline(always)]
    pub fn swp(&self) -> SWP_R {
        SWP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - SWS
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - RTIE
    #[inline(always)]
    pub fn rtie(&self) -> RTIE_R {
        RTIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - TMC
    #[inline(always)]
    pub fn tmc(&self) -> TMC_R {
        TMC_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - TTIE
    #[inline(always)]
    pub fn ttie(&self) -> TTIE_R {
        TTIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GCS
    #[inline(always)]
    pub fn gcs(&self) -> GCS_R {
        GCS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - FGP
    #[inline(always)]
    pub fn fgp(&self) -> FGP_R {
        FGP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TMG
    #[inline(always)]
    pub fn tmg(&self) -> TMG_R {
        TMG_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - NIG
    #[inline(always)]
    pub fn nig(&self) -> NIG_R {
        NIG_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ESCN
    #[inline(always)]
    pub fn escn(&self) -> ESCN_R {
        ESCN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - LCKC
    #[inline(always)]
    pub fn lckc(&self) -> LCKC_R {
        LCKC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SGT
    #[inline(always)]
    #[must_use]
    pub fn sgt(&mut self) -> SGT_W<0> {
        SGT_W::new(self)
    }
    ///Bit 1 - ECS
    #[inline(always)]
    #[must_use]
    pub fn ecs(&mut self) -> ECS_W<1> {
        ECS_W::new(self)
    }
    ///Bit 2 - SWP
    #[inline(always)]
    #[must_use]
    pub fn swp(&mut self) -> SWP_W<2> {
        SWP_W::new(self)
    }
    ///Bits 3:4 - SWS
    #[inline(always)]
    #[must_use]
    pub fn sws(&mut self) -> SWS_W<3> {
        SWS_W::new(self)
    }
    ///Bit 5 - RTIE
    #[inline(always)]
    #[must_use]
    pub fn rtie(&mut self) -> RTIE_W<5> {
        RTIE_W::new(self)
    }
    ///Bits 6:7 - TMC
    #[inline(always)]
    #[must_use]
    pub fn tmc(&mut self) -> TMC_W<6> {
        TMC_W::new(self)
    }
    ///Bit 8 - TTIE
    #[inline(always)]
    #[must_use]
    pub fn ttie(&mut self) -> TTIE_W<8> {
        TTIE_W::new(self)
    }
    ///Bit 9 - GCS
    #[inline(always)]
    #[must_use]
    pub fn gcs(&mut self) -> GCS_W<9> {
        GCS_W::new(self)
    }
    ///Bit 10 - FGP
    #[inline(always)]
    #[must_use]
    pub fn fgp(&mut self) -> FGP_W<10> {
        FGP_W::new(self)
    }
    ///Bit 11 - TMG
    #[inline(always)]
    #[must_use]
    pub fn tmg(&mut self) -> TMG_W<11> {
        TMG_W::new(self)
    }
    ///Bit 12 - NIG
    #[inline(always)]
    #[must_use]
    pub fn nig(&mut self) -> NIG_W<12> {
        NIG_W::new(self)
    }
    ///Bit 13 - ESCN
    #[inline(always)]
    #[must_use]
    pub fn escn(&mut self) -> ESCN_W<13> {
        ESCN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN TT operation control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_ttocn](index.html) module
pub struct FDCAN_TTOCN_SPEC;
impl crate::RegisterSpec for FDCAN_TTOCN_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_ttocn::R](R) reader structure
impl crate::Readable for FDCAN_TTOCN_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fdcan_ttocn::W](W) writer structure
impl crate::Writable for FDCAN_TTOCN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FDCAN_TTOCN to value 0
impl crate::Resettable for FDCAN_TTOCN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
