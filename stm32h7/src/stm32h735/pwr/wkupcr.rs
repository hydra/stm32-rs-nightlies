///Register `WKUPCR` reader
pub struct R(crate::R<WKUPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKUPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKUPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKUPCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `WKUPCR` writer
pub struct W(crate::W<WKUPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WKUPCR_SPEC>;
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
impl From<crate::W<WKUPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WKUPCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WKUPC1` reader - Clear Wakeup pin flag for WKUPC1
pub type WKUPC1_R = crate::BitReader<bool>;
///Field `WKUPC1` writer - Clear Wakeup pin flag for WKUPC1
pub type WKUPC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKUPCR_SPEC, bool, O>;
///Field `WKUPC2` reader - Clear Wakeup pin flag for WKUPC2
pub type WKUPC2_R = crate::BitReader<bool>;
///Field `WKUPC2` writer - Clear Wakeup pin flag for WKUPC2
pub type WKUPC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKUPCR_SPEC, bool, O>;
///Field `WKUPC4` reader - Clear Wakeup pin flag for WKUPC4
pub type WKUPC4_R = crate::BitReader<bool>;
///Field `WKUPC4` writer - Clear Wakeup pin flag for WKUPC4
pub type WKUPC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKUPCR_SPEC, bool, O>;
///Field `WKUPC6` reader - Clear Wakeup pin flag for WKUPC6
pub type WKUPC6_R = crate::BitReader<bool>;
///Field `WKUPC6` writer - Clear Wakeup pin flag for WKUPC6
pub type WKUPC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKUPCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Clear Wakeup pin flag for WKUPC1
    #[inline(always)]
    pub fn wkupc1(&self) -> WKUPC1_R {
        WKUPC1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clear Wakeup pin flag for WKUPC2
    #[inline(always)]
    pub fn wkupc2(&self) -> WKUPC2_R {
        WKUPC2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Clear Wakeup pin flag for WKUPC4
    #[inline(always)]
    pub fn wkupc4(&self) -> WKUPC4_R {
        WKUPC4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - Clear Wakeup pin flag for WKUPC6
    #[inline(always)]
    pub fn wkupc6(&self) -> WKUPC6_R {
        WKUPC6_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Clear Wakeup pin flag for WKUPC1
    #[inline(always)]
    #[must_use]
    pub fn wkupc1(&mut self) -> WKUPC1_W<0> {
        WKUPC1_W::new(self)
    }
    ///Bit 1 - Clear Wakeup pin flag for WKUPC2
    #[inline(always)]
    #[must_use]
    pub fn wkupc2(&mut self) -> WKUPC2_W<1> {
        WKUPC2_W::new(self)
    }
    ///Bit 3 - Clear Wakeup pin flag for WKUPC4
    #[inline(always)]
    #[must_use]
    pub fn wkupc4(&mut self) -> WKUPC4_W<3> {
        WKUPC4_W::new(self)
    }
    ///Bit 5 - Clear Wakeup pin flag for WKUPC6
    #[inline(always)]
    #[must_use]
    pub fn wkupc6(&mut self) -> WKUPC6_W<5> {
        WKUPC6_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared).
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wkupcr](index.html) module
pub struct WKUPCR_SPEC;
impl crate::RegisterSpec for WKUPCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [wkupcr::R](R) reader structure
impl crate::Readable for WKUPCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [wkupcr::W](W) writer structure
impl crate::Writable for WKUPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets WKUPCR to value 0
impl crate::Resettable for WKUPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
