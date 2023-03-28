///Register `GOTGCTL` reader
pub struct R(crate::R<GOTGCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GOTGCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GOTGCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GOTGCTL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GOTGCTL` writer
pub struct W(crate::W<GOTGCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GOTGCTL_SPEC>;
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
impl From<crate::W<GOTGCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GOTGCTL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SRQSCS` reader - SRQSCS
pub type SRQSCS_R = crate::BitReader<bool>;
///Field `SRQ` reader - SRQ
pub type SRQ_R = crate::BitReader<bool>;
///Field `SRQ` writer - SRQ
pub type SRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
///Field `VBVALOEN` reader - VBVALOEN
pub type VBVALOEN_R = crate::BitReader<bool>;
///Field `VBVALOEN` writer - VBVALOEN
pub type VBVALOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
///Field `VBVALOVAL` reader - VBVALOVAL
pub type VBVALOVAL_R = crate::BitReader<bool>;
///Field `VBVALOVAL` writer - VBVALOVAL
pub type VBVALOVAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
///Field `AVALOEN` reader - AVALOEN
pub type AVALOEN_R = crate::BitReader<bool>;
///Field `AVALOEN` writer - AVALOEN
pub type AVALOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
///Field `AVALOVAL` reader - AVALOVAL
pub type AVALOVAL_R = crate::BitReader<bool>;
///Field `AVALOVAL` writer - AVALOVAL
pub type AVALOVAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
///Field `BVALOEN` reader - BVALOEN
pub type BVALOEN_R = crate::BitReader<bool>;
///Field `BVALOEN` writer - BVALOEN
pub type BVALOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
///Field `BVALOVAL` reader - BVALOVAL
pub type BVALOVAL_R = crate::BitReader<bool>;
///Field `BVALOVAL` writer - BVALOVAL
pub type BVALOVAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
///Field `HNGSCS` reader - HNGSCS
pub type HNGSCS_R = crate::BitReader<bool>;
///Field `HNPRQ` reader - HNPRQ
pub type HNPRQ_R = crate::BitReader<bool>;
///Field `HNPRQ` writer - HNPRQ
pub type HNPRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
///Field `HSHNPEN` reader - HSHNPEN
pub type HSHNPEN_R = crate::BitReader<bool>;
///Field `HSHNPEN` writer - HSHNPEN
pub type HSHNPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
///Field `DHNPEN` reader - DHNPEN
pub type DHNPEN_R = crate::BitReader<bool>;
///Field `DHNPEN` writer - DHNPEN
pub type DHNPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
///Field `EHEN` reader - EHEN
pub type EHEN_R = crate::BitReader<bool>;
///Field `EHEN` writer - EHEN
pub type EHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
///Field `CIDSTS` reader - CIDSTS
pub type CIDSTS_R = crate::BitReader<bool>;
///Field `DBCT` reader - DBCT
pub type DBCT_R = crate::BitReader<bool>;
///Field `ASVLD` reader - ASVLD
pub type ASVLD_R = crate::BitReader<bool>;
///Field `BSVLD` reader - BSVLD
pub type BSVLD_R = crate::BitReader<bool>;
///Field `OTGVER` reader - OTGVER
pub type OTGVER_R = crate::BitReader<bool>;
///Field `OTGVER` writer - OTGVER
pub type OTGVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
///Field `CURMOD` reader - CURMOD
pub type CURMOD_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - SRQSCS
    #[inline(always)]
    pub fn srqscs(&self) -> SRQSCS_R {
        SRQSCS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SRQ
    #[inline(always)]
    pub fn srq(&self) -> SRQ_R {
        SRQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - VBVALOEN
    #[inline(always)]
    pub fn vbvaloen(&self) -> VBVALOEN_R {
        VBVALOEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - VBVALOVAL
    #[inline(always)]
    pub fn vbvaloval(&self) -> VBVALOVAL_R {
        VBVALOVAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AVALOEN
    #[inline(always)]
    pub fn avaloen(&self) -> AVALOEN_R {
        AVALOEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AVALOVAL
    #[inline(always)]
    pub fn avaloval(&self) -> AVALOVAL_R {
        AVALOVAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - BVALOEN
    #[inline(always)]
    pub fn bvaloen(&self) -> BVALOEN_R {
        BVALOEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - BVALOVAL
    #[inline(always)]
    pub fn bvaloval(&self) -> BVALOVAL_R {
        BVALOVAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - HNGSCS
    #[inline(always)]
    pub fn hngscs(&self) -> HNGSCS_R {
        HNGSCS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HNPRQ
    #[inline(always)]
    pub fn hnprq(&self) -> HNPRQ_R {
        HNPRQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSHNPEN
    #[inline(always)]
    pub fn hshnpen(&self) -> HSHNPEN_R {
        HSHNPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DHNPEN
    #[inline(always)]
    pub fn dhnpen(&self) -> DHNPEN_R {
        DHNPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - EHEN
    #[inline(always)]
    pub fn ehen(&self) -> EHEN_R {
        EHEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - CIDSTS
    #[inline(always)]
    pub fn cidsts(&self) -> CIDSTS_R {
        CIDSTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DBCT
    #[inline(always)]
    pub fn dbct(&self) -> DBCT_R {
        DBCT_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ASVLD
    #[inline(always)]
    pub fn asvld(&self) -> ASVLD_R {
        ASVLD_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - BSVLD
    #[inline(always)]
    pub fn bsvld(&self) -> BSVLD_R {
        BSVLD_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - OTGVER
    #[inline(always)]
    pub fn otgver(&self) -> OTGVER_R {
        OTGVER_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - CURMOD
    #[inline(always)]
    pub fn curmod(&self) -> CURMOD_R {
        CURMOD_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - SRQ
    #[inline(always)]
    #[must_use]
    pub fn srq(&mut self) -> SRQ_W<1> {
        SRQ_W::new(self)
    }
    ///Bit 2 - VBVALOEN
    #[inline(always)]
    #[must_use]
    pub fn vbvaloen(&mut self) -> VBVALOEN_W<2> {
        VBVALOEN_W::new(self)
    }
    ///Bit 3 - VBVALOVAL
    #[inline(always)]
    #[must_use]
    pub fn vbvaloval(&mut self) -> VBVALOVAL_W<3> {
        VBVALOVAL_W::new(self)
    }
    ///Bit 4 - AVALOEN
    #[inline(always)]
    #[must_use]
    pub fn avaloen(&mut self) -> AVALOEN_W<4> {
        AVALOEN_W::new(self)
    }
    ///Bit 5 - AVALOVAL
    #[inline(always)]
    #[must_use]
    pub fn avaloval(&mut self) -> AVALOVAL_W<5> {
        AVALOVAL_W::new(self)
    }
    ///Bit 6 - BVALOEN
    #[inline(always)]
    #[must_use]
    pub fn bvaloen(&mut self) -> BVALOEN_W<6> {
        BVALOEN_W::new(self)
    }
    ///Bit 7 - BVALOVAL
    #[inline(always)]
    #[must_use]
    pub fn bvaloval(&mut self) -> BVALOVAL_W<7> {
        BVALOVAL_W::new(self)
    }
    ///Bit 9 - HNPRQ
    #[inline(always)]
    #[must_use]
    pub fn hnprq(&mut self) -> HNPRQ_W<9> {
        HNPRQ_W::new(self)
    }
    ///Bit 10 - HSHNPEN
    #[inline(always)]
    #[must_use]
    pub fn hshnpen(&mut self) -> HSHNPEN_W<10> {
        HSHNPEN_W::new(self)
    }
    ///Bit 11 - DHNPEN
    #[inline(always)]
    #[must_use]
    pub fn dhnpen(&mut self) -> DHNPEN_W<11> {
        DHNPEN_W::new(self)
    }
    ///Bit 12 - EHEN
    #[inline(always)]
    #[must_use]
    pub fn ehen(&mut self) -> EHEN_W<12> {
        EHEN_W::new(self)
    }
    ///Bit 20 - OTGVER
    #[inline(always)]
    #[must_use]
    pub fn otgver(&mut self) -> OTGVER_W<20> {
        OTGVER_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///The GOTGCTL register controls the behavior and reflects the status of the OTG function of the core.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gotgctl](index.html) module
pub struct GOTGCTL_SPEC;
impl crate::RegisterSpec for GOTGCTL_SPEC {
    type Ux = u32;
}
///`read()` method returns [gotgctl::R](R) reader structure
impl crate::Readable for GOTGCTL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gotgctl::W](W) writer structure
impl crate::Writable for GOTGCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GOTGCTL to value 0x0001_0000
impl crate::Resettable for GOTGCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
