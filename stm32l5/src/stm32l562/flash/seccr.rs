///Register `SECCR` reader
pub struct R(crate::R<SECCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SECCR` writer
pub struct W(crate::W<SECCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECCR_SPEC>;
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
impl From<crate::W<SECCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SECPG` reader - SECPG
pub type SECPG_R = crate::BitReader<bool>;
///Field `SECPG` writer - SECPG
pub type SECPG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCR_SPEC, bool, O>;
///Field `SECPER` reader - SECPER
pub type SECPER_R = crate::BitReader<bool>;
///Field `SECPER` writer - SECPER
pub type SECPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCR_SPEC, bool, O>;
///Field `SECMER1` reader - SECMER1
pub type SECMER1_R = crate::BitReader<bool>;
///Field `SECMER1` writer - SECMER1
pub type SECMER1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCR_SPEC, bool, O>;
///Field `SECPNB` reader - SECPNB
pub type SECPNB_R = crate::FieldReader<u8, u8>;
///Field `SECPNB` writer - SECPNB
pub type SECPNB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SECCR_SPEC, u8, u8, 7, O>;
///Field `SECBKER` reader - SECBKER
pub type SECBKER_R = crate::BitReader<bool>;
///Field `SECBKER` writer - SECBKER
pub type SECBKER_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCR_SPEC, bool, O>;
///Field `SECMER2` reader - SECMER2
pub type SECMER2_R = crate::BitReader<bool>;
///Field `SECMER2` writer - SECMER2
pub type SECMER2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCR_SPEC, bool, O>;
///Field `SECSTRT` reader - SECSTRT
pub type SECSTRT_R = crate::BitReader<bool>;
///Field `SECSTRT` writer - SECSTRT
pub type SECSTRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCR_SPEC, bool, O>;
///Field `SECEOPIE` reader - SECEOPIE
pub type SECEOPIE_R = crate::BitReader<bool>;
///Field `SECEOPIE` writer - SECEOPIE
pub type SECEOPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCR_SPEC, bool, O>;
///Field `SECERRIE` reader - SECERRIE
pub type SECERRIE_R = crate::BitReader<bool>;
///Field `SECERRIE` writer - SECERRIE
pub type SECERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCR_SPEC, bool, O>;
///Field `SECRDERRIE` reader - SECRDERRIE
pub type SECRDERRIE_R = crate::BitReader<bool>;
///Field `SECRDERRIE` writer - SECRDERRIE
pub type SECRDERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCR_SPEC, bool, O>;
///Field `SECINV` reader - SECINV
pub type SECINV_R = crate::BitReader<bool>;
///Field `SECINV` writer - SECINV
pub type SECINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCR_SPEC, bool, O>;
///Field `SECLOCK` reader - SECLOCK
pub type SECLOCK_R = crate::BitReader<bool>;
///Field `SECLOCK` writer - SECLOCK
pub type SECLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - SECPG
    #[inline(always)]
    pub fn secpg(&self) -> SECPG_R {
        SECPG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SECPER
    #[inline(always)]
    pub fn secper(&self) -> SECPER_R {
        SECPER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SECMER1
    #[inline(always)]
    pub fn secmer1(&self) -> SECMER1_R {
        SECMER1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:9 - SECPNB
    #[inline(always)]
    pub fn secpnb(&self) -> SECPNB_R {
        SECPNB_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    ///Bit 11 - SECBKER
    #[inline(always)]
    pub fn secbker(&self) -> SECBKER_R {
        SECBKER_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - SECMER2
    #[inline(always)]
    pub fn secmer2(&self) -> SECMER2_R {
        SECMER2_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SECSTRT
    #[inline(always)]
    pub fn secstrt(&self) -> SECSTRT_R {
        SECSTRT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - SECEOPIE
    #[inline(always)]
    pub fn seceopie(&self) -> SECEOPIE_R {
        SECEOPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SECERRIE
    #[inline(always)]
    pub fn secerrie(&self) -> SECERRIE_R {
        SECERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SECRDERRIE
    #[inline(always)]
    pub fn secrderrie(&self) -> SECRDERRIE_R {
        SECRDERRIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 29 - SECINV
    #[inline(always)]
    pub fn secinv(&self) -> SECINV_R {
        SECINV_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - SECLOCK
    #[inline(always)]
    pub fn seclock(&self) -> SECLOCK_R {
        SECLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SECPG
    #[inline(always)]
    #[must_use]
    pub fn secpg(&mut self) -> SECPG_W<0> {
        SECPG_W::new(self)
    }
    ///Bit 1 - SECPER
    #[inline(always)]
    #[must_use]
    pub fn secper(&mut self) -> SECPER_W<1> {
        SECPER_W::new(self)
    }
    ///Bit 2 - SECMER1
    #[inline(always)]
    #[must_use]
    pub fn secmer1(&mut self) -> SECMER1_W<2> {
        SECMER1_W::new(self)
    }
    ///Bits 3:9 - SECPNB
    #[inline(always)]
    #[must_use]
    pub fn secpnb(&mut self) -> SECPNB_W<3> {
        SECPNB_W::new(self)
    }
    ///Bit 11 - SECBKER
    #[inline(always)]
    #[must_use]
    pub fn secbker(&mut self) -> SECBKER_W<11> {
        SECBKER_W::new(self)
    }
    ///Bit 15 - SECMER2
    #[inline(always)]
    #[must_use]
    pub fn secmer2(&mut self) -> SECMER2_W<15> {
        SECMER2_W::new(self)
    }
    ///Bit 16 - SECSTRT
    #[inline(always)]
    #[must_use]
    pub fn secstrt(&mut self) -> SECSTRT_W<16> {
        SECSTRT_W::new(self)
    }
    ///Bit 24 - SECEOPIE
    #[inline(always)]
    #[must_use]
    pub fn seceopie(&mut self) -> SECEOPIE_W<24> {
        SECEOPIE_W::new(self)
    }
    ///Bit 25 - SECERRIE
    #[inline(always)]
    #[must_use]
    pub fn secerrie(&mut self) -> SECERRIE_W<25> {
        SECERRIE_W::new(self)
    }
    ///Bit 26 - SECRDERRIE
    #[inline(always)]
    #[must_use]
    pub fn secrderrie(&mut self) -> SECRDERRIE_W<26> {
        SECRDERRIE_W::new(self)
    }
    ///Bit 29 - SECINV
    #[inline(always)]
    #[must_use]
    pub fn secinv(&mut self) -> SECINV_W<29> {
        SECINV_W::new(self)
    }
    ///Bit 31 - SECLOCK
    #[inline(always)]
    #[must_use]
    pub fn seclock(&mut self) -> SECLOCK_W<31> {
        SECLOCK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flash secure control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [seccr](index.html) module
pub struct SECCR_SPEC;
impl crate::RegisterSpec for SECCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [seccr::R](R) reader structure
impl crate::Readable for SECCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [seccr::W](W) writer structure
impl crate::Writable for SECCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SECCR to value 0x8000_0000
impl crate::Resettable for SECCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
