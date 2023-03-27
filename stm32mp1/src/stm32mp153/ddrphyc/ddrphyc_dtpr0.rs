///Register `DDRPHYC_DTPR0` reader
pub struct R(crate::R<DDRPHYC_DTPR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DTPR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DTPR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DTPR0_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DDRPHYC_DTPR0` writer
pub struct W(crate::W<DDRPHYC_DTPR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DTPR0_SPEC>;
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
impl From<crate::W<DDRPHYC_DTPR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DTPR0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TMRD` reader - TMRD
pub type TMRD_R = crate::FieldReader<u8, u8>;
///Field `TMRD` writer - TMRD
pub type TMRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTPR0_SPEC, u8, u8, 2, O>;
///Field `TRTP` reader - TRTP
pub type TRTP_R = crate::FieldReader<u8, u8>;
///Field `TRTP` writer - TRTP
pub type TRTP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTPR0_SPEC, u8, u8, 3, O>;
///Field `TWTR` reader - TWTR
pub type TWTR_R = crate::FieldReader<u8, u8>;
///Field `TWTR` writer - TWTR
pub type TWTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTPR0_SPEC, u8, u8, 3, O>;
///Field `TRP` reader - TRP
pub type TRP_R = crate::FieldReader<u8, u8>;
///Field `TRP` writer - TRP
pub type TRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTPR0_SPEC, u8, u8, 4, O>;
///Field `TRCD` reader - TRCD
pub type TRCD_R = crate::FieldReader<u8, u8>;
///Field `TRCD` writer - TRCD
pub type TRCD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTPR0_SPEC, u8, u8, 4, O>;
///Field `TRAS` reader - TRAS
pub type TRAS_R = crate::FieldReader<u8, u8>;
///Field `TRAS` writer - TRAS
pub type TRAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTPR0_SPEC, u8, u8, 5, O>;
///Field `TRRD` reader - TRRD
pub type TRRD_R = crate::FieldReader<u8, u8>;
///Field `TRRD` writer - TRRD
pub type TRRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTPR0_SPEC, u8, u8, 4, O>;
///Field `TRC` reader - TRC
pub type TRC_R = crate::FieldReader<u8, u8>;
///Field `TRC` writer - TRC
pub type TRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTPR0_SPEC, u8, u8, 6, O>;
///Field `TCCD` reader - TCCD
pub type TCCD_R = crate::BitReader<bool>;
///Field `TCCD` writer - TCCD
pub type TCCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DTPR0_SPEC, bool, O>;
impl R {
    ///Bits 0:1 - TMRD
    #[inline(always)]
    pub fn tmrd(&self) -> TMRD_R {
        TMRD_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:4 - TRTP
    #[inline(always)]
    pub fn trtp(&self) -> TRTP_R {
        TRTP_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bits 5:7 - TWTR
    #[inline(always)]
    pub fn twtr(&self) -> TWTR_R {
        TWTR_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bits 8:11 - TRP
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - TRCD
    #[inline(always)]
    pub fn trcd(&self) -> TRCD_R {
        TRCD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:20 - TRAS
    #[inline(always)]
    pub fn tras(&self) -> TRAS_R {
        TRAS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 21:24 - TRRD
    #[inline(always)]
    pub fn trrd(&self) -> TRRD_R {
        TRRD_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    ///Bits 25:30 - TRC
    #[inline(always)]
    pub fn trc(&self) -> TRC_R {
        TRC_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
    ///Bit 31 - TCCD
    #[inline(always)]
    pub fn tccd(&self) -> TCCD_R {
        TCCD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - TMRD
    #[inline(always)]
    #[must_use]
    pub fn tmrd(&mut self) -> TMRD_W<0> {
        TMRD_W::new(self)
    }
    ///Bits 2:4 - TRTP
    #[inline(always)]
    #[must_use]
    pub fn trtp(&mut self) -> TRTP_W<2> {
        TRTP_W::new(self)
    }
    ///Bits 5:7 - TWTR
    #[inline(always)]
    #[must_use]
    pub fn twtr(&mut self) -> TWTR_W<5> {
        TWTR_W::new(self)
    }
    ///Bits 8:11 - TRP
    #[inline(always)]
    #[must_use]
    pub fn trp(&mut self) -> TRP_W<8> {
        TRP_W::new(self)
    }
    ///Bits 12:15 - TRCD
    #[inline(always)]
    #[must_use]
    pub fn trcd(&mut self) -> TRCD_W<12> {
        TRCD_W::new(self)
    }
    ///Bits 16:20 - TRAS
    #[inline(always)]
    #[must_use]
    pub fn tras(&mut self) -> TRAS_W<16> {
        TRAS_W::new(self)
    }
    ///Bits 21:24 - TRRD
    #[inline(always)]
    #[must_use]
    pub fn trrd(&mut self) -> TRRD_W<21> {
        TRRD_W::new(self)
    }
    ///Bits 25:30 - TRC
    #[inline(always)]
    #[must_use]
    pub fn trc(&mut self) -> TRC_W<25> {
        TRC_W::new(self)
    }
    ///Bit 31 - TCCD
    #[inline(always)]
    #[must_use]
    pub fn tccd(&mut self) -> TCCD_W<31> {
        TCCD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DDRPHYC DTP register 0
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrphyc_dtpr0](index.html) module
pub struct DDRPHYC_DTPR0_SPEC;
impl crate::RegisterSpec for DDRPHYC_DTPR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrphyc_dtpr0::R](R) reader structure
impl crate::Readable for DDRPHYC_DTPR0_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ddrphyc_dtpr0::W](W) writer structure
impl crate::Writable for DDRPHYC_DTPR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DDRPHYC_DTPR0 to value 0x3012_666e
impl crate::Resettable for DDRPHYC_DTPR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x3012_666e;
}
