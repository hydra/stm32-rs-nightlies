///Register `PTPTSCR` reader
pub struct R(crate::R<PTPTSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPTSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPTSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPTSCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PTPTSCR` writer
pub struct W(crate::W<PTPTSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTPTSCR_SPEC>;
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
impl From<crate::W<PTPTSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTPTSCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TSE` reader - Time stamp enable
pub type TSE_R = crate::BitReader<bool>;
///Field `TSE` writer - Time stamp enable
pub type TSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTPTSCR_SPEC, bool, O>;
///Field `TSFCU` reader - Time stamp fine or coarse update
pub type TSFCU_R = crate::BitReader<bool>;
///Field `TSFCU` writer - Time stamp fine or coarse update
pub type TSFCU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTPTSCR_SPEC, bool, O>;
///Field `TSSTI` reader - Time stamp system time initialize
pub type TSSTI_R = crate::BitReader<bool>;
///Field `TSSTI` writer - Time stamp system time initialize
pub type TSSTI_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTPTSCR_SPEC, bool, O>;
///Field `TSSTU` reader - Time stamp system time update
pub type TSSTU_R = crate::BitReader<bool>;
///Field `TSSTU` writer - Time stamp system time update
pub type TSSTU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTPTSCR_SPEC, bool, O>;
///Field `TSITE` reader - Time stamp interrupt trigger enable
pub type TSITE_R = crate::BitReader<bool>;
///Field `TSITE` writer - Time stamp interrupt trigger enable
pub type TSITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTPTSCR_SPEC, bool, O>;
///Field `TSARU` reader - Time stamp addend register update
pub type TSARU_R = crate::BitReader<bool>;
///Field `TSARU` writer - Time stamp addend register update
pub type TSARU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTPTSCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Time stamp enable
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Time stamp fine or coarse update
    #[inline(always)]
    pub fn tsfcu(&self) -> TSFCU_R {
        TSFCU_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Time stamp system time initialize
    #[inline(always)]
    pub fn tssti(&self) -> TSSTI_R {
        TSSTI_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Time stamp system time update
    #[inline(always)]
    pub fn tsstu(&self) -> TSSTU_R {
        TSSTU_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Time stamp interrupt trigger enable
    #[inline(always)]
    pub fn tsite(&self) -> TSITE_R {
        TSITE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Time stamp addend register update
    #[inline(always)]
    pub fn tsaru(&self) -> TSARU_R {
        TSARU_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Time stamp enable
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TSE_W<0> {
        TSE_W::new(self)
    }
    ///Bit 1 - Time stamp fine or coarse update
    #[inline(always)]
    #[must_use]
    pub fn tsfcu(&mut self) -> TSFCU_W<1> {
        TSFCU_W::new(self)
    }
    ///Bit 2 - Time stamp system time initialize
    #[inline(always)]
    #[must_use]
    pub fn tssti(&mut self) -> TSSTI_W<2> {
        TSSTI_W::new(self)
    }
    ///Bit 3 - Time stamp system time update
    #[inline(always)]
    #[must_use]
    pub fn tsstu(&mut self) -> TSSTU_W<3> {
        TSSTU_W::new(self)
    }
    ///Bit 4 - Time stamp interrupt trigger enable
    #[inline(always)]
    #[must_use]
    pub fn tsite(&mut self) -> TSITE_W<4> {
        TSITE_W::new(self)
    }
    ///Bit 5 - Time stamp addend register update
    #[inline(always)]
    #[must_use]
    pub fn tsaru(&mut self) -> TSARU_W<5> {
        TSARU_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet PTP time stamp control register (ETH_PTPTSCR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ptptscr](index.html) module
pub struct PTPTSCR_SPEC;
impl crate::RegisterSpec for PTPTSCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ptptscr::R](R) reader structure
impl crate::Readable for PTPTSCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ptptscr::W](W) writer structure
impl crate::Writable for PTPTSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PTPTSCR to value 0
impl crate::Resettable for PTPTSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
