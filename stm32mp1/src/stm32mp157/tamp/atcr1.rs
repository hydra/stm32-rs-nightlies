///Register `ATCR1` reader
pub struct R(crate::R<ATCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATCR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ATCR1` writer
pub struct W(crate::W<ATCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATCR1_SPEC>;
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
impl From<crate::W<ATCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATCR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TAMP1AM` reader - TAMP1AM
pub type TAMP1AM_R = crate::BitReader<bool>;
///Field `TAMP1AM` writer - TAMP1AM
pub type TAMP1AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR1_SPEC, bool, O>;
///Field `TAMP2AM` reader - TAMP2AM
pub type TAMP2AM_R = crate::BitReader<bool>;
///Field `TAMP2AM` writer - TAMP2AM
pub type TAMP2AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR1_SPEC, bool, O>;
///Field `TAMP3AM` reader - TAMP3AM
pub type TAMP3AM_R = crate::BitReader<bool>;
///Field `TAMP3AM` writer - TAMP3AM
pub type TAMP3AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR1_SPEC, bool, O>;
///Field `ATOSEL1` reader - ATOSEL1
pub type ATOSEL1_R = crate::FieldReader<u8, u8>;
///Field `ATOSEL1` writer - ATOSEL1
pub type ATOSEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCR1_SPEC, u8, u8, 2, O>;
///Field `ATOSEL2` reader - ATOSEL2
pub type ATOSEL2_R = crate::FieldReader<u8, u8>;
///Field `ATOSEL2` writer - ATOSEL2
pub type ATOSEL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCR1_SPEC, u8, u8, 2, O>;
///Field `ATOSEL3` reader - ATOSEL3
pub type ATOSEL3_R = crate::FieldReader<u8, u8>;
///Field `ATOSEL3` writer - ATOSEL3
pub type ATOSEL3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCR1_SPEC, u8, u8, 2, O>;
///Field `ATCKSEL` reader - ATCKSEL
pub type ATCKSEL_R = crate::FieldReader<u8, u8>;
///Field `ATCKSEL` writer - ATCKSEL
pub type ATCKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCR1_SPEC, u8, u8, 3, O>;
///Field `ATPER` reader - ATPER
pub type ATPER_R = crate::FieldReader<u8, u8>;
///Field `ATPER` writer - ATPER
pub type ATPER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCR1_SPEC, u8, u8, 3, O>;
///Field `ATOSHARE` reader - ATOSHARE
pub type ATOSHARE_R = crate::BitReader<bool>;
///Field `ATOSHARE` writer - ATOSHARE
pub type ATOSHARE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR1_SPEC, bool, O>;
///Field `FLTEN` reader - FLTEN
pub type FLTEN_R = crate::BitReader<bool>;
///Field `FLTEN` writer - FLTEN
pub type FLTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - TAMP1AM
    #[inline(always)]
    pub fn tamp1am(&self) -> TAMP1AM_R {
        TAMP1AM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TAMP2AM
    #[inline(always)]
    pub fn tamp2am(&self) -> TAMP2AM_R {
        TAMP2AM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TAMP3AM
    #[inline(always)]
    pub fn tamp3am(&self) -> TAMP3AM_R {
        TAMP3AM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 8:9 - ATOSEL1
    #[inline(always)]
    pub fn atosel1(&self) -> ATOSEL1_R {
        ATOSEL1_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - ATOSEL2
    #[inline(always)]
    pub fn atosel2(&self) -> ATOSEL2_R {
        ATOSEL2_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - ATOSEL3
    #[inline(always)]
    pub fn atosel3(&self) -> ATOSEL3_R {
        ATOSEL3_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 16:18 - ATCKSEL
    #[inline(always)]
    pub fn atcksel(&self) -> ATCKSEL_R {
        ATCKSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 24:26 - ATPER
    #[inline(always)]
    pub fn atper(&self) -> ATPER_R {
        ATPER_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bit 30 - ATOSHARE
    #[inline(always)]
    pub fn atoshare(&self) -> ATOSHARE_R {
        ATOSHARE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - FLTEN
    #[inline(always)]
    pub fn flten(&self) -> FLTEN_R {
        FLTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TAMP1AM
    #[inline(always)]
    #[must_use]
    pub fn tamp1am(&mut self) -> TAMP1AM_W<0> {
        TAMP1AM_W::new(self)
    }
    ///Bit 1 - TAMP2AM
    #[inline(always)]
    #[must_use]
    pub fn tamp2am(&mut self) -> TAMP2AM_W<1> {
        TAMP2AM_W::new(self)
    }
    ///Bit 2 - TAMP3AM
    #[inline(always)]
    #[must_use]
    pub fn tamp3am(&mut self) -> TAMP3AM_W<2> {
        TAMP3AM_W::new(self)
    }
    ///Bits 8:9 - ATOSEL1
    #[inline(always)]
    #[must_use]
    pub fn atosel1(&mut self) -> ATOSEL1_W<8> {
        ATOSEL1_W::new(self)
    }
    ///Bits 10:11 - ATOSEL2
    #[inline(always)]
    #[must_use]
    pub fn atosel2(&mut self) -> ATOSEL2_W<10> {
        ATOSEL2_W::new(self)
    }
    ///Bits 12:13 - ATOSEL3
    #[inline(always)]
    #[must_use]
    pub fn atosel3(&mut self) -> ATOSEL3_W<12> {
        ATOSEL3_W::new(self)
    }
    ///Bits 16:18 - ATCKSEL
    #[inline(always)]
    #[must_use]
    pub fn atcksel(&mut self) -> ATCKSEL_W<16> {
        ATCKSEL_W::new(self)
    }
    ///Bits 24:26 - ATPER
    #[inline(always)]
    #[must_use]
    pub fn atper(&mut self) -> ATPER_W<24> {
        ATPER_W::new(self)
    }
    ///Bit 30 - ATOSHARE
    #[inline(always)]
    #[must_use]
    pub fn atoshare(&mut self) -> ATOSHARE_W<30> {
        ATOSHARE_W::new(self)
    }
    ///Bit 31 - FLTEN
    #[inline(always)]
    #[must_use]
    pub fn flten(&mut self) -> FLTEN_W<31> {
        FLTEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [atcr1](index.html) module
pub struct ATCR1_SPEC;
impl crate::RegisterSpec for ATCR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [atcr1::R](R) reader structure
impl crate::Readable for ATCR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [atcr1::W](W) writer structure
impl crate::Writable for ATCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ATCR1 to value 0x0007_0000
impl crate::Resettable for ATCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0000;
}
