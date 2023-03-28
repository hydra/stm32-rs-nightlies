///Register `DAC_CR` reader
pub struct R(crate::R<DAC_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DAC_CR` writer
pub struct W(crate::W<DAC_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_CR_SPEC>;
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
impl From<crate::W<DAC_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EN1` reader - EN1
pub type EN1_R = crate::BitReader<bool>;
///Field `EN1` writer - EN1
pub type EN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CR_SPEC, bool, O>;
///Field `TEN1` reader - TEN1
pub type TEN1_R = crate::BitReader<bool>;
///Field `TEN1` writer - TEN1
pub type TEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CR_SPEC, bool, O>;
///Field `TSEL10` reader - TSEL10
pub type TSEL10_R = crate::BitReader<bool>;
///Field `TSEL10` writer - TSEL10
pub type TSEL10_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CR_SPEC, bool, O>;
///Field `TSEL11` reader - TSEL11
pub type TSEL11_R = crate::BitReader<bool>;
///Field `TSEL11` writer - TSEL11
pub type TSEL11_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CR_SPEC, bool, O>;
///Field `TSEL12` reader - TSEL12
pub type TSEL12_R = crate::BitReader<bool>;
///Field `TSEL12` writer - TSEL12
pub type TSEL12_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CR_SPEC, bool, O>;
///Field `TSEL13` reader - TSEL13
pub type TSEL13_R = crate::BitReader<bool>;
///Field `TSEL13` writer - TSEL13
pub type TSEL13_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CR_SPEC, bool, O>;
///Field `WAVE1` reader - WAVE1
pub type WAVE1_R = crate::FieldReader<u8, u8>;
///Field `WAVE1` writer - WAVE1
pub type WAVE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC_CR_SPEC, u8, u8, 2, O>;
///Field `MAMP1` reader - MAMP1
pub type MAMP1_R = crate::FieldReader<u8, u8>;
///Field `MAMP1` writer - MAMP1
pub type MAMP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC_CR_SPEC, u8, u8, 4, O>;
///Field `DMAEN1` reader - DMAEN1
pub type DMAEN1_R = crate::BitReader<bool>;
///Field `DMAEN1` writer - DMAEN1
pub type DMAEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CR_SPEC, bool, O>;
///Field `DMAUDRIE1` reader - DMAUDRIE1
pub type DMAUDRIE1_R = crate::BitReader<bool>;
///Field `DMAUDRIE1` writer - DMAUDRIE1
pub type DMAUDRIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CR_SPEC, bool, O>;
///Field `CEN1` reader - CEN1
pub type CEN1_R = crate::BitReader<bool>;
///Field `CEN1` writer - CEN1
pub type CEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CR_SPEC, bool, O>;
///Field `HFSEL` reader - HFSEL
pub type HFSEL_R = crate::BitReader<bool>;
///Field `HFSEL` writer - HFSEL
pub type HFSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CR_SPEC, bool, O>;
///Field `EN2` reader - EN2
pub type EN2_R = crate::BitReader<bool>;
///Field `EN2` writer - EN2
pub type EN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CR_SPEC, bool, O>;
///Field `TEN2` reader - TEN2
pub type TEN2_R = crate::BitReader<bool>;
///Field `TEN2` writer - TEN2
pub type TEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CR_SPEC, bool, O>;
///Field `TSEL20` reader - TSEL20
pub type TSEL20_R = crate::BitReader<bool>;
///Field `TSEL20` writer - TSEL20
pub type TSEL20_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CR_SPEC, bool, O>;
///Field `TSEL21` reader - TSEL21
pub type TSEL21_R = crate::BitReader<bool>;
///Field `TSEL21` writer - TSEL21
pub type TSEL21_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CR_SPEC, bool, O>;
///Field `TSEL22` reader - TSEL22
pub type TSEL22_R = crate::BitReader<bool>;
///Field `TSEL22` writer - TSEL22
pub type TSEL22_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CR_SPEC, bool, O>;
///Field `TSEL23` reader - TSEL23
pub type TSEL23_R = crate::BitReader<bool>;
///Field `TSEL23` writer - TSEL23
pub type TSEL23_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CR_SPEC, bool, O>;
///Field `WAVE2` reader - WAVE2
pub type WAVE2_R = crate::FieldReader<u8, u8>;
///Field `WAVE2` writer - WAVE2
pub type WAVE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC_CR_SPEC, u8, u8, 2, O>;
///Field `MAMP2` reader - MAMP2
pub type MAMP2_R = crate::FieldReader<u8, u8>;
///Field `MAMP2` writer - MAMP2
pub type MAMP2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC_CR_SPEC, u8, u8, 4, O>;
///Field `DMAEN2` reader - DMAEN2
pub type DMAEN2_R = crate::BitReader<bool>;
///Field `DMAEN2` writer - DMAEN2
pub type DMAEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CR_SPEC, bool, O>;
///Field `DMAUDRIE2` reader - DMAUDRIE2
pub type DMAUDRIE2_R = crate::BitReader<bool>;
///Field `DMAUDRIE2` writer - DMAUDRIE2
pub type DMAUDRIE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CR_SPEC, bool, O>;
///Field `CEN2` reader - CEN2
pub type CEN2_R = crate::BitReader<bool>;
///Field `CEN2` writer - CEN2
pub type CEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CR_SPEC, bool, O>;
impl R {
    ///Bit 0 - EN1
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TEN1
    #[inline(always)]
    pub fn ten1(&self) -> TEN1_R {
        TEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TSEL10
    #[inline(always)]
    pub fn tsel10(&self) -> TSEL10_R {
        TSEL10_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TSEL11
    #[inline(always)]
    pub fn tsel11(&self) -> TSEL11_R {
        TSEL11_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TSEL12
    #[inline(always)]
    pub fn tsel12(&self) -> TSEL12_R {
        TSEL12_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TSEL13
    #[inline(always)]
    pub fn tsel13(&self) -> TSEL13_R {
        TSEL13_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - WAVE1
    #[inline(always)]
    pub fn wave1(&self) -> WAVE1_R {
        WAVE1_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:11 - MAMP1
    #[inline(always)]
    pub fn mamp1(&self) -> MAMP1_R {
        MAMP1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - DMAEN1
    #[inline(always)]
    pub fn dmaen1(&self) -> DMAEN1_R {
        DMAEN1_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - DMAUDRIE1
    #[inline(always)]
    pub fn dmaudrie1(&self) -> DMAUDRIE1_R {
        DMAUDRIE1_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CEN1
    #[inline(always)]
    pub fn cen1(&self) -> CEN1_R {
        CEN1_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - HFSEL
    #[inline(always)]
    pub fn hfsel(&self) -> HFSEL_R {
        HFSEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - EN2
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TEN2
    #[inline(always)]
    pub fn ten2(&self) -> TEN2_R {
        TEN2_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TSEL20
    #[inline(always)]
    pub fn tsel20(&self) -> TSEL20_R {
        TSEL20_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - TSEL21
    #[inline(always)]
    pub fn tsel21(&self) -> TSEL21_R {
        TSEL21_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - TSEL22
    #[inline(always)]
    pub fn tsel22(&self) -> TSEL22_R {
        TSEL22_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - TSEL23
    #[inline(always)]
    pub fn tsel23(&self) -> TSEL23_R {
        TSEL23_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:23 - WAVE2
    #[inline(always)]
    pub fn wave2(&self) -> WAVE2_R {
        WAVE2_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:27 - MAMP2
    #[inline(always)]
    pub fn mamp2(&self) -> MAMP2_R {
        MAMP2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bit 28 - DMAEN2
    #[inline(always)]
    pub fn dmaen2(&self) -> DMAEN2_R {
        DMAEN2_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DMAUDRIE2
    #[inline(always)]
    pub fn dmaudrie2(&self) -> DMAUDRIE2_R {
        DMAUDRIE2_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - CEN2
    #[inline(always)]
    pub fn cen2(&self) -> CEN2_R {
        CEN2_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - EN1
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> EN1_W<0> {
        EN1_W::new(self)
    }
    ///Bit 1 - TEN1
    #[inline(always)]
    #[must_use]
    pub fn ten1(&mut self) -> TEN1_W<1> {
        TEN1_W::new(self)
    }
    ///Bit 2 - TSEL10
    #[inline(always)]
    #[must_use]
    pub fn tsel10(&mut self) -> TSEL10_W<2> {
        TSEL10_W::new(self)
    }
    ///Bit 3 - TSEL11
    #[inline(always)]
    #[must_use]
    pub fn tsel11(&mut self) -> TSEL11_W<3> {
        TSEL11_W::new(self)
    }
    ///Bit 4 - TSEL12
    #[inline(always)]
    #[must_use]
    pub fn tsel12(&mut self) -> TSEL12_W<4> {
        TSEL12_W::new(self)
    }
    ///Bit 5 - TSEL13
    #[inline(always)]
    #[must_use]
    pub fn tsel13(&mut self) -> TSEL13_W<5> {
        TSEL13_W::new(self)
    }
    ///Bits 6:7 - WAVE1
    #[inline(always)]
    #[must_use]
    pub fn wave1(&mut self) -> WAVE1_W<6> {
        WAVE1_W::new(self)
    }
    ///Bits 8:11 - MAMP1
    #[inline(always)]
    #[must_use]
    pub fn mamp1(&mut self) -> MAMP1_W<8> {
        MAMP1_W::new(self)
    }
    ///Bit 12 - DMAEN1
    #[inline(always)]
    #[must_use]
    pub fn dmaen1(&mut self) -> DMAEN1_W<12> {
        DMAEN1_W::new(self)
    }
    ///Bit 13 - DMAUDRIE1
    #[inline(always)]
    #[must_use]
    pub fn dmaudrie1(&mut self) -> DMAUDRIE1_W<13> {
        DMAUDRIE1_W::new(self)
    }
    ///Bit 14 - CEN1
    #[inline(always)]
    #[must_use]
    pub fn cen1(&mut self) -> CEN1_W<14> {
        CEN1_W::new(self)
    }
    ///Bit 15 - HFSEL
    #[inline(always)]
    #[must_use]
    pub fn hfsel(&mut self) -> HFSEL_W<15> {
        HFSEL_W::new(self)
    }
    ///Bit 16 - EN2
    #[inline(always)]
    #[must_use]
    pub fn en2(&mut self) -> EN2_W<16> {
        EN2_W::new(self)
    }
    ///Bit 17 - TEN2
    #[inline(always)]
    #[must_use]
    pub fn ten2(&mut self) -> TEN2_W<17> {
        TEN2_W::new(self)
    }
    ///Bit 18 - TSEL20
    #[inline(always)]
    #[must_use]
    pub fn tsel20(&mut self) -> TSEL20_W<18> {
        TSEL20_W::new(self)
    }
    ///Bit 19 - TSEL21
    #[inline(always)]
    #[must_use]
    pub fn tsel21(&mut self) -> TSEL21_W<19> {
        TSEL21_W::new(self)
    }
    ///Bit 20 - TSEL22
    #[inline(always)]
    #[must_use]
    pub fn tsel22(&mut self) -> TSEL22_W<20> {
        TSEL22_W::new(self)
    }
    ///Bit 21 - TSEL23
    #[inline(always)]
    #[must_use]
    pub fn tsel23(&mut self) -> TSEL23_W<21> {
        TSEL23_W::new(self)
    }
    ///Bits 22:23 - WAVE2
    #[inline(always)]
    #[must_use]
    pub fn wave2(&mut self) -> WAVE2_W<22> {
        WAVE2_W::new(self)
    }
    ///Bits 24:27 - MAMP2
    #[inline(always)]
    #[must_use]
    pub fn mamp2(&mut self) -> MAMP2_W<24> {
        MAMP2_W::new(self)
    }
    ///Bit 28 - DMAEN2
    #[inline(always)]
    #[must_use]
    pub fn dmaen2(&mut self) -> DMAEN2_W<28> {
        DMAEN2_W::new(self)
    }
    ///Bit 29 - DMAUDRIE2
    #[inline(always)]
    #[must_use]
    pub fn dmaudrie2(&mut self) -> DMAUDRIE2_W<29> {
        DMAUDRIE2_W::new(self)
    }
    ///Bit 30 - CEN2
    #[inline(always)]
    #[must_use]
    pub fn cen2(&mut self) -> CEN2_W<30> {
        CEN2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DAC control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dac_cr](index.html) module
pub struct DAC_CR_SPEC;
impl crate::RegisterSpec for DAC_CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dac_cr::R](R) reader structure
impl crate::Readable for DAC_CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dac_cr::W](W) writer structure
impl crate::Writable for DAC_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DAC_CR to value 0
impl crate::Resettable for DAC_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
