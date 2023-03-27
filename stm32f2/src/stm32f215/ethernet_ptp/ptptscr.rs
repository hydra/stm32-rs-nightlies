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
///Field `TTSARU` reader - Time stamp addend register update
pub type TTSARU_R = crate::BitReader<bool>;
///Field `TTSARU` writer - Time stamp addend register update
pub type TTSARU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTPTSCR_SPEC, bool, O>;
///Field `TSSARFE` reader - Time stamp snapshot for all received frames enable
pub type TSSARFE_R = crate::BitReader<bool>;
///Field `TSSARFE` writer - Time stamp snapshot for all received frames enable
pub type TSSARFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTPTSCR_SPEC, bool, O>;
///Field `TSSSR` reader - Time stamp subsecond rollover: digital or binary rollover control
pub type TSSSR_R = crate::BitReader<bool>;
///Field `TSSSR` writer - Time stamp subsecond rollover: digital or binary rollover control
pub type TSSSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTPTSCR_SPEC, bool, O>;
///Field `TSPTPPSV2E` reader - Time stamp PTP packet snooping for version2 format enable
pub type TSPTPPSV2E_R = crate::BitReader<bool>;
///Field `TSPTPPSV2E` writer - Time stamp PTP packet snooping for version2 format enable
pub type TSPTPPSV2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTPTSCR_SPEC, bool, O>;
///Field `TSSPTPOEFE` reader - Time stamp snapshot for PTP over ethernet frames enable
pub type TSSPTPOEFE_R = crate::BitReader<bool>;
///Field `TSSPTPOEFE` writer - Time stamp snapshot for PTP over ethernet frames enable
pub type TSSPTPOEFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTPTSCR_SPEC, bool, O>;
///Field `TSSIPV6FE` reader - Time stamp snapshot for IPv6 frames enable
pub type TSSIPV6FE_R = crate::BitReader<bool>;
///Field `TSSIPV6FE` writer - Time stamp snapshot for IPv6 frames enable
pub type TSSIPV6FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTPTSCR_SPEC, bool, O>;
///Field `TSSIPV4FE` reader - Time stamp snapshot for IPv4 frames enable
pub type TSSIPV4FE_R = crate::BitReader<bool>;
///Field `TSSIPV4FE` writer - Time stamp snapshot for IPv4 frames enable
pub type TSSIPV4FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTPTSCR_SPEC, bool, O>;
///Field `TSSEME` reader - Time stamp snapshot for event message enable
pub type TSSEME_R = crate::BitReader<bool>;
///Field `TSSEME` writer - Time stamp snapshot for event message enable
pub type TSSEME_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTPTSCR_SPEC, bool, O>;
///Field `TSSMRME` reader - Time stamp snapshot for message relevant to master enable
pub type TSSMRME_R = crate::BitReader<bool>;
///Field `TSSMRME` writer - Time stamp snapshot for message relevant to master enable
pub type TSSMRME_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTPTSCR_SPEC, bool, O>;
///Field `TSCNT` reader - Time stamp clock node type
pub type TSCNT_R = crate::FieldReader<u8, u8>;
///Field `TSCNT` writer - Time stamp clock node type
pub type TSCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTPTSCR_SPEC, u8, u8, 2, O>;
///Field `TSPFFMAE` reader - Time stamp PTP frame filtering MAC address enable
pub type TSPFFMAE_R = crate::BitReader<bool>;
///Field `TSPFFMAE` writer - Time stamp PTP frame filtering MAC address enable
pub type TSPFFMAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTPTSCR_SPEC, bool, O>;
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
    pub fn ttsaru(&self) -> TTSARU_R {
        TTSARU_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Time stamp snapshot for all received frames enable
    #[inline(always)]
    pub fn tssarfe(&self) -> TSSARFE_R {
        TSSARFE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Time stamp subsecond rollover: digital or binary rollover control
    #[inline(always)]
    pub fn tsssr(&self) -> TSSSR_R {
        TSSSR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Time stamp PTP packet snooping for version2 format enable
    #[inline(always)]
    pub fn tsptppsv2e(&self) -> TSPTPPSV2E_R {
        TSPTPPSV2E_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Time stamp snapshot for PTP over ethernet frames enable
    #[inline(always)]
    pub fn tssptpoefe(&self) -> TSSPTPOEFE_R {
        TSSPTPOEFE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Time stamp snapshot for IPv6 frames enable
    #[inline(always)]
    pub fn tssipv6fe(&self) -> TSSIPV6FE_R {
        TSSIPV6FE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Time stamp snapshot for IPv4 frames enable
    #[inline(always)]
    pub fn tssipv4fe(&self) -> TSSIPV4FE_R {
        TSSIPV4FE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Time stamp snapshot for event message enable
    #[inline(always)]
    pub fn tsseme(&self) -> TSSEME_R {
        TSSEME_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Time stamp snapshot for message relevant to master enable
    #[inline(always)]
    pub fn tssmrme(&self) -> TSSMRME_R {
        TSSMRME_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Time stamp clock node type
    #[inline(always)]
    pub fn tscnt(&self) -> TSCNT_R {
        TSCNT_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - Time stamp PTP frame filtering MAC address enable
    #[inline(always)]
    pub fn tspffmae(&self) -> TSPFFMAE_R {
        TSPFFMAE_R::new(((self.bits >> 18) & 1) != 0)
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
    pub fn ttsaru(&mut self) -> TTSARU_W<5> {
        TTSARU_W::new(self)
    }
    ///Bit 8 - Time stamp snapshot for all received frames enable
    #[inline(always)]
    #[must_use]
    pub fn tssarfe(&mut self) -> TSSARFE_W<8> {
        TSSARFE_W::new(self)
    }
    ///Bit 9 - Time stamp subsecond rollover: digital or binary rollover control
    #[inline(always)]
    #[must_use]
    pub fn tsssr(&mut self) -> TSSSR_W<9> {
        TSSSR_W::new(self)
    }
    ///Bit 10 - Time stamp PTP packet snooping for version2 format enable
    #[inline(always)]
    #[must_use]
    pub fn tsptppsv2e(&mut self) -> TSPTPPSV2E_W<10> {
        TSPTPPSV2E_W::new(self)
    }
    ///Bit 11 - Time stamp snapshot for PTP over ethernet frames enable
    #[inline(always)]
    #[must_use]
    pub fn tssptpoefe(&mut self) -> TSSPTPOEFE_W<11> {
        TSSPTPOEFE_W::new(self)
    }
    ///Bit 12 - Time stamp snapshot for IPv6 frames enable
    #[inline(always)]
    #[must_use]
    pub fn tssipv6fe(&mut self) -> TSSIPV6FE_W<12> {
        TSSIPV6FE_W::new(self)
    }
    ///Bit 13 - Time stamp snapshot for IPv4 frames enable
    #[inline(always)]
    #[must_use]
    pub fn tssipv4fe(&mut self) -> TSSIPV4FE_W<13> {
        TSSIPV4FE_W::new(self)
    }
    ///Bit 14 - Time stamp snapshot for event message enable
    #[inline(always)]
    #[must_use]
    pub fn tsseme(&mut self) -> TSSEME_W<14> {
        TSSEME_W::new(self)
    }
    ///Bit 15 - Time stamp snapshot for message relevant to master enable
    #[inline(always)]
    #[must_use]
    pub fn tssmrme(&mut self) -> TSSMRME_W<15> {
        TSSMRME_W::new(self)
    }
    ///Bits 16:17 - Time stamp clock node type
    #[inline(always)]
    #[must_use]
    pub fn tscnt(&mut self) -> TSCNT_W<16> {
        TSCNT_W::new(self)
    }
    ///Bit 18 - Time stamp PTP frame filtering MAC address enable
    #[inline(always)]
    #[must_use]
    pub fn tspffmae(&mut self) -> TSPFFMAE_W<18> {
        TSPFFMAE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet PTP time stamp control register
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
///`reset()` method sets PTPTSCR to value 0x2000
impl crate::Resettable for PTPTSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000;
}
