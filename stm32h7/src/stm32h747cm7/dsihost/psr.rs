///Register `PSR` reader
pub struct R(crate::R<PSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PSR` writer
pub struct W(crate::W<PSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSR_SPEC>;
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
impl From<crate::W<PSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PD` reader - PHY direction
pub type PD_R = crate::BitReader<bool>;
///Field `PD` writer - PHY direction
pub type PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
///Field `PSSC` reader - PHY stop state clock lane
pub type PSSC_R = crate::BitReader<bool>;
///Field `PSSC` writer - PHY stop state clock lane
pub type PSSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
///Field `UANC` reader - ULPS active not clock lane
pub type UANC_R = crate::BitReader<bool>;
///Field `UANC` writer - ULPS active not clock lane
pub type UANC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
///Field `PSS0` reader - PHY stop state lane 0
pub type PSS0_R = crate::BitReader<bool>;
///Field `PSS0` writer - PHY stop state lane 0
pub type PSS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
///Field `UAN0` reader - ULPS active not lane 1
pub type UAN0_R = crate::BitReader<bool>;
///Field `UAN0` writer - ULPS active not lane 1
pub type UAN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
///Field `RUE0` reader - RX ULPS escape lane 0
pub type RUE0_R = crate::BitReader<bool>;
///Field `RUE0` writer - RX ULPS escape lane 0
pub type RUE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
///Field `PSS1` reader - PHY stop state lane 1
pub type PSS1_R = crate::BitReader<bool>;
///Field `PSS1` writer - PHY stop state lane 1
pub type PSS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
///Field `UAN1` reader - ULPS active not lane 1
pub type UAN1_R = crate::BitReader<bool>;
///Field `UAN1` writer - ULPS active not lane 1
pub type UAN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
impl R {
    ///Bit 1 - PHY direction
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PHY stop state clock lane
    #[inline(always)]
    pub fn pssc(&self) -> PSSC_R {
        PSSC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ULPS active not clock lane
    #[inline(always)]
    pub fn uanc(&self) -> UANC_R {
        UANC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PHY stop state lane 0
    #[inline(always)]
    pub fn pss0(&self) -> PSS0_R {
        PSS0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ULPS active not lane 1
    #[inline(always)]
    pub fn uan0(&self) -> UAN0_R {
        UAN0_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RX ULPS escape lane 0
    #[inline(always)]
    pub fn rue0(&self) -> RUE0_R {
        RUE0_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PHY stop state lane 1
    #[inline(always)]
    pub fn pss1(&self) -> PSS1_R {
        PSS1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ULPS active not lane 1
    #[inline(always)]
    pub fn uan1(&self) -> UAN1_R {
        UAN1_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - PHY direction
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<1> {
        PD_W::new(self)
    }
    ///Bit 2 - PHY stop state clock lane
    #[inline(always)]
    #[must_use]
    pub fn pssc(&mut self) -> PSSC_W<2> {
        PSSC_W::new(self)
    }
    ///Bit 3 - ULPS active not clock lane
    #[inline(always)]
    #[must_use]
    pub fn uanc(&mut self) -> UANC_W<3> {
        UANC_W::new(self)
    }
    ///Bit 4 - PHY stop state lane 0
    #[inline(always)]
    #[must_use]
    pub fn pss0(&mut self) -> PSS0_W<4> {
        PSS0_W::new(self)
    }
    ///Bit 5 - ULPS active not lane 1
    #[inline(always)]
    #[must_use]
    pub fn uan0(&mut self) -> UAN0_W<5> {
        UAN0_W::new(self)
    }
    ///Bit 6 - RX ULPS escape lane 0
    #[inline(always)]
    #[must_use]
    pub fn rue0(&mut self) -> RUE0_W<6> {
        RUE0_W::new(self)
    }
    ///Bit 7 - PHY stop state lane 1
    #[inline(always)]
    #[must_use]
    pub fn pss1(&mut self) -> PSS1_W<7> {
        PSS1_W::new(self)
    }
    ///Bit 8 - ULPS active not lane 1
    #[inline(always)]
    #[must_use]
    pub fn uan1(&mut self) -> UAN1_W<8> {
        UAN1_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host PHY status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [psr](index.html) module
pub struct PSR_SPEC;
impl crate::RegisterSpec for PSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [psr::R](R) reader structure
impl crate::Readable for PSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [psr::W](W) writer structure
impl crate::Writable for PSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PSR to value 0x1528
impl crate::Resettable for PSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1528;
}
