///Register `GPSR` reader
pub struct R(crate::R<GPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GPSR` writer
pub struct W(crate::W<GPSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPSR_SPEC>;
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
impl From<crate::W<GPSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CMDFE` reader - Command FIFO empty
pub type CMDFE_R = crate::BitReader<bool>;
///Field `CMDFE` writer - Command FIFO empty
pub type CMDFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPSR_SPEC, bool, O>;
///Field `CMDFF` reader - Command FIFO full
pub type CMDFF_R = crate::BitReader<bool>;
///Field `CMDFF` writer - Command FIFO full
pub type CMDFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPSR_SPEC, bool, O>;
///Field `PWRFE` reader - Payload write FIFO empty
pub type PWRFE_R = crate::BitReader<bool>;
///Field `PWRFE` writer - Payload write FIFO empty
pub type PWRFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPSR_SPEC, bool, O>;
///Field `PWRFF` reader - Payload write FIFO full
pub type PWRFF_R = crate::BitReader<bool>;
///Field `PWRFF` writer - Payload write FIFO full
pub type PWRFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPSR_SPEC, bool, O>;
///Field `PRDFE` reader - Payload read FIFO empty
pub type PRDFE_R = crate::BitReader<bool>;
///Field `PRDFE` writer - Payload read FIFO empty
pub type PRDFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPSR_SPEC, bool, O>;
///Field `PRDFF` reader - Payload read FIFO full
pub type PRDFF_R = crate::BitReader<bool>;
///Field `PRDFF` writer - Payload read FIFO full
pub type PRDFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPSR_SPEC, bool, O>;
///Field `RCB` reader - Read command busy
pub type RCB_R = crate::BitReader<bool>;
///Field `RCB` writer - Read command busy
pub type RCB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPSR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Command FIFO empty
    #[inline(always)]
    pub fn cmdfe(&self) -> CMDFE_R {
        CMDFE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Command FIFO full
    #[inline(always)]
    pub fn cmdff(&self) -> CMDFF_R {
        CMDFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Payload write FIFO empty
    #[inline(always)]
    pub fn pwrfe(&self) -> PWRFE_R {
        PWRFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Payload write FIFO full
    #[inline(always)]
    pub fn pwrff(&self) -> PWRFF_R {
        PWRFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Payload read FIFO empty
    #[inline(always)]
    pub fn prdfe(&self) -> PRDFE_R {
        PRDFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Payload read FIFO full
    #[inline(always)]
    pub fn prdff(&self) -> PRDFF_R {
        PRDFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Read command busy
    #[inline(always)]
    pub fn rcb(&self) -> RCB_R {
        RCB_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Command FIFO empty
    #[inline(always)]
    #[must_use]
    pub fn cmdfe(&mut self) -> CMDFE_W<0> {
        CMDFE_W::new(self)
    }
    ///Bit 1 - Command FIFO full
    #[inline(always)]
    #[must_use]
    pub fn cmdff(&mut self) -> CMDFF_W<1> {
        CMDFF_W::new(self)
    }
    ///Bit 2 - Payload write FIFO empty
    #[inline(always)]
    #[must_use]
    pub fn pwrfe(&mut self) -> PWRFE_W<2> {
        PWRFE_W::new(self)
    }
    ///Bit 3 - Payload write FIFO full
    #[inline(always)]
    #[must_use]
    pub fn pwrff(&mut self) -> PWRFF_W<3> {
        PWRFF_W::new(self)
    }
    ///Bit 4 - Payload read FIFO empty
    #[inline(always)]
    #[must_use]
    pub fn prdfe(&mut self) -> PRDFE_W<4> {
        PRDFE_W::new(self)
    }
    ///Bit 5 - Payload read FIFO full
    #[inline(always)]
    #[must_use]
    pub fn prdff(&mut self) -> PRDFF_W<5> {
        PRDFF_W::new(self)
    }
    ///Bit 6 - Read command busy
    #[inline(always)]
    #[must_use]
    pub fn rcb(&mut self) -> RCB_W<6> {
        RCB_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host generic packet status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gpsr](index.html) module
pub struct GPSR_SPEC;
impl crate::RegisterSpec for GPSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpsr::R](R) reader structure
impl crate::Readable for GPSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gpsr::W](W) writer structure
impl crate::Writable for GPSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GPSR to value 0x15
impl crate::Resettable for GPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x15;
}
