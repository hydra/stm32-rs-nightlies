///Register `PDCRG` reader
pub struct R(crate::R<PDCRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDCRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDCRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDCRG_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PDCRG` writer
pub struct W(crate::W<PDCRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDCRG_SPEC>;
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
impl From<crate::W<PDCRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDCRG_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PD0` reader - Port G pull-down bit y (y=0..15)
pub type PD0_R = crate::BitReader<bool>;
///Field `PD0` writer - Port G pull-down bit y (y=0..15)
pub type PD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRG_SPEC, bool, O>;
///Field `PD1` reader - Port G pull-down bit y (y=0..15)
pub type PD1_R = crate::BitReader<bool>;
///Field `PD1` writer - Port G pull-down bit y (y=0..15)
pub type PD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRG_SPEC, bool, O>;
///Field `PD2` reader - Port G pull-down bit y (y=0..15)
pub type PD2_R = crate::BitReader<bool>;
///Field `PD2` writer - Port G pull-down bit y (y=0..15)
pub type PD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRG_SPEC, bool, O>;
///Field `PD3` reader - Port G pull-down bit y (y=0..15)
pub type PD3_R = crate::BitReader<bool>;
///Field `PD3` writer - Port G pull-down bit y (y=0..15)
pub type PD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRG_SPEC, bool, O>;
///Field `PD4` reader - Port G pull-down bit y (y=0..15)
pub type PD4_R = crate::BitReader<bool>;
///Field `PD4` writer - Port G pull-down bit y (y=0..15)
pub type PD4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRG_SPEC, bool, O>;
///Field `PD5` reader - Port G pull-down bit y (y=0..15)
pub type PD5_R = crate::BitReader<bool>;
///Field `PD5` writer - Port G pull-down bit y (y=0..15)
pub type PD5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRG_SPEC, bool, O>;
///Field `PD6` reader - Port G pull-down bit y (y=0..15)
pub type PD6_R = crate::BitReader<bool>;
///Field `PD6` writer - Port G pull-down bit y (y=0..15)
pub type PD6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRG_SPEC, bool, O>;
///Field `PD7` reader - Port G pull-down bit y (y=0..15)
pub type PD7_R = crate::BitReader<bool>;
///Field `PD7` writer - Port G pull-down bit y (y=0..15)
pub type PD7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRG_SPEC, bool, O>;
///Field `PD8` reader - Port G pull-down bit y (y=0..15)
pub type PD8_R = crate::BitReader<bool>;
///Field `PD8` writer - Port G pull-down bit y (y=0..15)
pub type PD8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRG_SPEC, bool, O>;
///Field `PD9` reader - Port G pull-down bit y (y=0..15)
pub type PD9_R = crate::BitReader<bool>;
///Field `PD9` writer - Port G pull-down bit y (y=0..15)
pub type PD9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRG_SPEC, bool, O>;
///Field `PD10` reader - Port G pull-down bit y (y=0..15)
pub type PD10_R = crate::BitReader<bool>;
///Field `PD10` writer - Port G pull-down bit y (y=0..15)
pub type PD10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRG_SPEC, bool, O>;
impl R {
    ///Bit 0 - Port G pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port G pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port G pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port G pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port G pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port G pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port G pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port G pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port G pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd8(&self) -> PD8_R {
        PD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port G pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd9(&self) -> PD9_R {
        PD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port G pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd10(&self) -> PD10_R {
        PD10_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Port G pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> PD0_W<0> {
        PD0_W::new(self)
    }
    ///Bit 1 - Port G pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> PD1_W<1> {
        PD1_W::new(self)
    }
    ///Bit 2 - Port G pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> PD2_W<2> {
        PD2_W::new(self)
    }
    ///Bit 3 - Port G pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> PD3_W<3> {
        PD3_W::new(self)
    }
    ///Bit 4 - Port G pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd4(&mut self) -> PD4_W<4> {
        PD4_W::new(self)
    }
    ///Bit 5 - Port G pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd5(&mut self) -> PD5_W<5> {
        PD5_W::new(self)
    }
    ///Bit 6 - Port G pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd6(&mut self) -> PD6_W<6> {
        PD6_W::new(self)
    }
    ///Bit 7 - Port G pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd7(&mut self) -> PD7_W<7> {
        PD7_W::new(self)
    }
    ///Bit 8 - Port G pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd8(&mut self) -> PD8_W<8> {
        PD8_W::new(self)
    }
    ///Bit 9 - Port G pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd9(&mut self) -> PD9_W<9> {
        PD9_W::new(self)
    }
    ///Bit 10 - Port G pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd10(&mut self) -> PD10_W<10> {
        PD10_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Power Port G pull-down control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pdcrg](index.html) module
pub struct PDCRG_SPEC;
impl crate::RegisterSpec for PDCRG_SPEC {
    type Ux = u32;
}
///`read()` method returns [pdcrg::R](R) reader structure
impl crate::Readable for PDCRG_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pdcrg::W](W) writer structure
impl crate::Writable for PDCRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PDCRG to value 0
impl crate::Resettable for PDCRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
