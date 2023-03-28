///Register `TSR` reader
pub struct R(crate::R<TSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TSR` writer
pub struct W(crate::W<TSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSR_SPEC>;
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
impl From<crate::W<TSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RQCP0` reader - RQCP0
pub type RQCP0_R = crate::BitReader<bool>;
///Field `RQCP0` writer - RQCP0
pub type RQCP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
///Field `TXOK0` reader - TXOK0
pub type TXOK0_R = crate::BitReader<bool>;
///Field `TXOK0` writer - TXOK0
pub type TXOK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
///Field `ALST0` reader - ALST0
pub type ALST0_R = crate::BitReader<bool>;
///Field `ALST0` writer - ALST0
pub type ALST0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
///Field `TERR0` reader - TERR0
pub type TERR0_R = crate::BitReader<bool>;
///Field `TERR0` writer - TERR0
pub type TERR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
///Field `ABRQ0` reader - ABRQ0
pub type ABRQ0_R = crate::BitReader<bool>;
///Field `ABRQ0` writer - ABRQ0
pub type ABRQ0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
///Field `RQCP1` reader - RQCP1
pub type RQCP1_R = crate::BitReader<bool>;
///Field `RQCP1` writer - RQCP1
pub type RQCP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
///Field `TXOK1` reader - TXOK1
pub type TXOK1_R = crate::BitReader<bool>;
///Field `TXOK1` writer - TXOK1
pub type TXOK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
///Field `ALST1` reader - ALST1
pub type ALST1_R = crate::BitReader<bool>;
///Field `ALST1` writer - ALST1
pub type ALST1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
///Field `TERR1` reader - TERR1
pub type TERR1_R = crate::BitReader<bool>;
///Field `TERR1` writer - TERR1
pub type TERR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
///Field `ABRQ1` reader - ABRQ1
pub type ABRQ1_R = crate::BitReader<bool>;
///Field `ABRQ1` writer - ABRQ1
pub type ABRQ1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
///Field `RQCP2` reader - RQCP2
pub type RQCP2_R = crate::BitReader<bool>;
///Field `RQCP2` writer - RQCP2
pub type RQCP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
///Field `TXOK2` reader - TXOK2
pub type TXOK2_R = crate::BitReader<bool>;
///Field `TXOK2` writer - TXOK2
pub type TXOK2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
///Field `ALST2` reader - ALST2
pub type ALST2_R = crate::BitReader<bool>;
///Field `ALST2` writer - ALST2
pub type ALST2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
///Field `TERR2` reader - TERR2
pub type TERR2_R = crate::BitReader<bool>;
///Field `TERR2` writer - TERR2
pub type TERR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
///Field `ABRQ2` reader - ABRQ2
pub type ABRQ2_R = crate::BitReader<bool>;
///Field `ABRQ2` writer - ABRQ2
pub type ABRQ2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
///Field `CODE` reader - CODE
pub type CODE_R = crate::FieldReader<u8, u8>;
///Field `TME` reader - Lowest priority flag for mailbox 2
pub type TME_R = crate::FieldReader<u8, u8>;
///Field `TME` writer - Lowest priority flag for mailbox 2
pub type TME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSR_SPEC, u8, u8, 3, O>;
///Field `LOW` reader - Lowest priority flag for mailbox 2
pub type LOW_R = crate::FieldReader<u8, u8>;
///Field `LOW` writer - Lowest priority flag for mailbox 2
pub type LOW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSR_SPEC, u8, u8, 3, O>;
impl R {
    ///Bit 0 - RQCP0
    #[inline(always)]
    pub fn rqcp0(&self) -> RQCP0_R {
        RQCP0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TXOK0
    #[inline(always)]
    pub fn txok0(&self) -> TXOK0_R {
        TXOK0_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ALST0
    #[inline(always)]
    pub fn alst0(&self) -> ALST0_R {
        ALST0_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TERR0
    #[inline(always)]
    pub fn terr0(&self) -> TERR0_R {
        TERR0_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 7 - ABRQ0
    #[inline(always)]
    pub fn abrq0(&self) -> ABRQ0_R {
        ABRQ0_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - RQCP1
    #[inline(always)]
    pub fn rqcp1(&self) -> RQCP1_R {
        RQCP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TXOK1
    #[inline(always)]
    pub fn txok1(&self) -> TXOK1_R {
        TXOK1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ALST1
    #[inline(always)]
    pub fn alst1(&self) -> ALST1_R {
        ALST1_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TERR1
    #[inline(always)]
    pub fn terr1(&self) -> TERR1_R {
        TERR1_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - ABRQ1
    #[inline(always)]
    pub fn abrq1(&self) -> ABRQ1_R {
        ABRQ1_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - RQCP2
    #[inline(always)]
    pub fn rqcp2(&self) -> RQCP2_R {
        RQCP2_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TXOK2
    #[inline(always)]
    pub fn txok2(&self) -> TXOK2_R {
        TXOK2_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ALST2
    #[inline(always)]
    pub fn alst2(&self) -> ALST2_R {
        ALST2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - TERR2
    #[inline(always)]
    pub fn terr2(&self) -> TERR2_R {
        TERR2_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 23 - ABRQ2
    #[inline(always)]
    pub fn abrq2(&self) -> ABRQ2_R {
        ABRQ2_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:25 - CODE
    #[inline(always)]
    pub fn code(&self) -> CODE_R {
        CODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:28 - Lowest priority flag for mailbox 2
    #[inline(always)]
    pub fn tme(&self) -> TME_R {
        TME_R::new(((self.bits >> 26) & 7) as u8)
    }
    ///Bits 29:31 - Lowest priority flag for mailbox 2
    #[inline(always)]
    pub fn low(&self) -> LOW_R {
        LOW_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    ///Bit 0 - RQCP0
    #[inline(always)]
    #[must_use]
    pub fn rqcp0(&mut self) -> RQCP0_W<0> {
        RQCP0_W::new(self)
    }
    ///Bit 1 - TXOK0
    #[inline(always)]
    #[must_use]
    pub fn txok0(&mut self) -> TXOK0_W<1> {
        TXOK0_W::new(self)
    }
    ///Bit 2 - ALST0
    #[inline(always)]
    #[must_use]
    pub fn alst0(&mut self) -> ALST0_W<2> {
        ALST0_W::new(self)
    }
    ///Bit 3 - TERR0
    #[inline(always)]
    #[must_use]
    pub fn terr0(&mut self) -> TERR0_W<3> {
        TERR0_W::new(self)
    }
    ///Bit 7 - ABRQ0
    #[inline(always)]
    #[must_use]
    pub fn abrq0(&mut self) -> ABRQ0_W<7> {
        ABRQ0_W::new(self)
    }
    ///Bit 8 - RQCP1
    #[inline(always)]
    #[must_use]
    pub fn rqcp1(&mut self) -> RQCP1_W<8> {
        RQCP1_W::new(self)
    }
    ///Bit 9 - TXOK1
    #[inline(always)]
    #[must_use]
    pub fn txok1(&mut self) -> TXOK1_W<9> {
        TXOK1_W::new(self)
    }
    ///Bit 10 - ALST1
    #[inline(always)]
    #[must_use]
    pub fn alst1(&mut self) -> ALST1_W<10> {
        ALST1_W::new(self)
    }
    ///Bit 11 - TERR1
    #[inline(always)]
    #[must_use]
    pub fn terr1(&mut self) -> TERR1_W<11> {
        TERR1_W::new(self)
    }
    ///Bit 15 - ABRQ1
    #[inline(always)]
    #[must_use]
    pub fn abrq1(&mut self) -> ABRQ1_W<15> {
        ABRQ1_W::new(self)
    }
    ///Bit 16 - RQCP2
    #[inline(always)]
    #[must_use]
    pub fn rqcp2(&mut self) -> RQCP2_W<16> {
        RQCP2_W::new(self)
    }
    ///Bit 17 - TXOK2
    #[inline(always)]
    #[must_use]
    pub fn txok2(&mut self) -> TXOK2_W<17> {
        TXOK2_W::new(self)
    }
    ///Bit 18 - ALST2
    #[inline(always)]
    #[must_use]
    pub fn alst2(&mut self) -> ALST2_W<18> {
        ALST2_W::new(self)
    }
    ///Bit 19 - TERR2
    #[inline(always)]
    #[must_use]
    pub fn terr2(&mut self) -> TERR2_W<19> {
        TERR2_W::new(self)
    }
    ///Bit 23 - ABRQ2
    #[inline(always)]
    #[must_use]
    pub fn abrq2(&mut self) -> ABRQ2_W<23> {
        ABRQ2_W::new(self)
    }
    ///Bits 26:28 - Lowest priority flag for mailbox 2
    #[inline(always)]
    #[must_use]
    pub fn tme(&mut self) -> TME_W<26> {
        TME_W::new(self)
    }
    ///Bits 29:31 - Lowest priority flag for mailbox 2
    #[inline(always)]
    #[must_use]
    pub fn low(&mut self) -> LOW_W<29> {
        LOW_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///transmit status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tsr](index.html) module
pub struct TSR_SPEC;
impl crate::RegisterSpec for TSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tsr::R](R) reader structure
impl crate::Readable for TSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tsr::W](W) writer structure
impl crate::Writable for TSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TSR to value 0x1c00_0000
impl crate::Resettable for TSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1c00_0000;
}
