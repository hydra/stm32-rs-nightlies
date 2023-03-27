///Register `FFA1R` reader
pub struct R(crate::R<FFA1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFA1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFA1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFA1R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FFA1R` writer
pub struct W(crate::W<FFA1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFA1R_SPEC>;
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
impl From<crate::W<FFA1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFA1R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FFA0` reader - Filter FIFO assignment for filter 0
pub type FFA0_R = crate::BitReader<bool>;
///Field `FFA0` writer - Filter FIFO assignment for filter 0
pub type FFA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
///Field `FFA1` reader - Filter FIFO assignment for filter 1
pub type FFA1_R = crate::BitReader<bool>;
///Field `FFA1` writer - Filter FIFO assignment for filter 1
pub type FFA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
///Field `FFA2` reader - Filter FIFO assignment for filter 2
pub type FFA2_R = crate::BitReader<bool>;
///Field `FFA2` writer - Filter FIFO assignment for filter 2
pub type FFA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
///Field `FFA3` reader - Filter FIFO assignment for filter 3
pub type FFA3_R = crate::BitReader<bool>;
///Field `FFA3` writer - Filter FIFO assignment for filter 3
pub type FFA3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
///Field `FFA4` reader - Filter FIFO assignment for filter 4
pub type FFA4_R = crate::BitReader<bool>;
///Field `FFA4` writer - Filter FIFO assignment for filter 4
pub type FFA4_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
///Field `FFA5` reader - Filter FIFO assignment for filter 5
pub type FFA5_R = crate::BitReader<bool>;
///Field `FFA5` writer - Filter FIFO assignment for filter 5
pub type FFA5_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
///Field `FFA6` reader - Filter FIFO assignment for filter 6
pub type FFA6_R = crate::BitReader<bool>;
///Field `FFA6` writer - Filter FIFO assignment for filter 6
pub type FFA6_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
///Field `FFA7` reader - Filter FIFO assignment for filter 7
pub type FFA7_R = crate::BitReader<bool>;
///Field `FFA7` writer - Filter FIFO assignment for filter 7
pub type FFA7_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
///Field `FFA8` reader - Filter FIFO assignment for filter 8
pub type FFA8_R = crate::BitReader<bool>;
///Field `FFA8` writer - Filter FIFO assignment for filter 8
pub type FFA8_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
///Field `FFA9` reader - Filter FIFO assignment for filter 9
pub type FFA9_R = crate::BitReader<bool>;
///Field `FFA9` writer - Filter FIFO assignment for filter 9
pub type FFA9_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
///Field `FFA10` reader - Filter FIFO assignment for filter 10
pub type FFA10_R = crate::BitReader<bool>;
///Field `FFA10` writer - Filter FIFO assignment for filter 10
pub type FFA10_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
///Field `FFA11` reader - Filter FIFO assignment for filter 11
pub type FFA11_R = crate::BitReader<bool>;
///Field `FFA11` writer - Filter FIFO assignment for filter 11
pub type FFA11_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
///Field `FFA12` reader - Filter FIFO assignment for filter 12
pub type FFA12_R = crate::BitReader<bool>;
///Field `FFA12` writer - Filter FIFO assignment for filter 12
pub type FFA12_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
///Field `FFA13` reader - Filter FIFO assignment for filter 13
pub type FFA13_R = crate::BitReader<bool>;
///Field `FFA13` writer - Filter FIFO assignment for filter 13
pub type FFA13_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
impl R {
    ///Bit 0 - Filter FIFO assignment for filter 0
    #[inline(always)]
    pub fn ffa0(&self) -> FFA0_R {
        FFA0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Filter FIFO assignment for filter 1
    #[inline(always)]
    pub fn ffa1(&self) -> FFA1_R {
        FFA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Filter FIFO assignment for filter 2
    #[inline(always)]
    pub fn ffa2(&self) -> FFA2_R {
        FFA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Filter FIFO assignment for filter 3
    #[inline(always)]
    pub fn ffa3(&self) -> FFA3_R {
        FFA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Filter FIFO assignment for filter 4
    #[inline(always)]
    pub fn ffa4(&self) -> FFA4_R {
        FFA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Filter FIFO assignment for filter 5
    #[inline(always)]
    pub fn ffa5(&self) -> FFA5_R {
        FFA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Filter FIFO assignment for filter 6
    #[inline(always)]
    pub fn ffa6(&self) -> FFA6_R {
        FFA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Filter FIFO assignment for filter 7
    #[inline(always)]
    pub fn ffa7(&self) -> FFA7_R {
        FFA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Filter FIFO assignment for filter 8
    #[inline(always)]
    pub fn ffa8(&self) -> FFA8_R {
        FFA8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Filter FIFO assignment for filter 9
    #[inline(always)]
    pub fn ffa9(&self) -> FFA9_R {
        FFA9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Filter FIFO assignment for filter 10
    #[inline(always)]
    pub fn ffa10(&self) -> FFA10_R {
        FFA10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Filter FIFO assignment for filter 11
    #[inline(always)]
    pub fn ffa11(&self) -> FFA11_R {
        FFA11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Filter FIFO assignment for filter 12
    #[inline(always)]
    pub fn ffa12(&self) -> FFA12_R {
        FFA12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Filter FIFO assignment for filter 13
    #[inline(always)]
    pub fn ffa13(&self) -> FFA13_R {
        FFA13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Filter FIFO assignment for filter 0
    #[inline(always)]
    #[must_use]
    pub fn ffa0(&mut self) -> FFA0_W<0> {
        FFA0_W::new(self)
    }
    ///Bit 1 - Filter FIFO assignment for filter 1
    #[inline(always)]
    #[must_use]
    pub fn ffa1(&mut self) -> FFA1_W<1> {
        FFA1_W::new(self)
    }
    ///Bit 2 - Filter FIFO assignment for filter 2
    #[inline(always)]
    #[must_use]
    pub fn ffa2(&mut self) -> FFA2_W<2> {
        FFA2_W::new(self)
    }
    ///Bit 3 - Filter FIFO assignment for filter 3
    #[inline(always)]
    #[must_use]
    pub fn ffa3(&mut self) -> FFA3_W<3> {
        FFA3_W::new(self)
    }
    ///Bit 4 - Filter FIFO assignment for filter 4
    #[inline(always)]
    #[must_use]
    pub fn ffa4(&mut self) -> FFA4_W<4> {
        FFA4_W::new(self)
    }
    ///Bit 5 - Filter FIFO assignment for filter 5
    #[inline(always)]
    #[must_use]
    pub fn ffa5(&mut self) -> FFA5_W<5> {
        FFA5_W::new(self)
    }
    ///Bit 6 - Filter FIFO assignment for filter 6
    #[inline(always)]
    #[must_use]
    pub fn ffa6(&mut self) -> FFA6_W<6> {
        FFA6_W::new(self)
    }
    ///Bit 7 - Filter FIFO assignment for filter 7
    #[inline(always)]
    #[must_use]
    pub fn ffa7(&mut self) -> FFA7_W<7> {
        FFA7_W::new(self)
    }
    ///Bit 8 - Filter FIFO assignment for filter 8
    #[inline(always)]
    #[must_use]
    pub fn ffa8(&mut self) -> FFA8_W<8> {
        FFA8_W::new(self)
    }
    ///Bit 9 - Filter FIFO assignment for filter 9
    #[inline(always)]
    #[must_use]
    pub fn ffa9(&mut self) -> FFA9_W<9> {
        FFA9_W::new(self)
    }
    ///Bit 10 - Filter FIFO assignment for filter 10
    #[inline(always)]
    #[must_use]
    pub fn ffa10(&mut self) -> FFA10_W<10> {
        FFA10_W::new(self)
    }
    ///Bit 11 - Filter FIFO assignment for filter 11
    #[inline(always)]
    #[must_use]
    pub fn ffa11(&mut self) -> FFA11_W<11> {
        FFA11_W::new(self)
    }
    ///Bit 12 - Filter FIFO assignment for filter 12
    #[inline(always)]
    #[must_use]
    pub fn ffa12(&mut self) -> FFA12_W<12> {
        FFA12_W::new(self)
    }
    ///Bit 13 - Filter FIFO assignment for filter 13
    #[inline(always)]
    #[must_use]
    pub fn ffa13(&mut self) -> FFA13_W<13> {
        FFA13_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///filter FIFO assignment register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ffa1r](index.html) module
pub struct FFA1R_SPEC;
impl crate::RegisterSpec for FFA1R_SPEC {
    type Ux = u32;
}
///`read()` method returns [ffa1r::R](R) reader structure
impl crate::Readable for FFA1R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ffa1r::W](W) writer structure
impl crate::Writable for FFA1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FFA1R to value 0
impl crate::Resettable for FFA1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
