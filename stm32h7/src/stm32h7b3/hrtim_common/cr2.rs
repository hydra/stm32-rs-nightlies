///Register `CR2` reader
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR2` writer
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MSWU` reader - Master Timer Software update
pub type MSWU_R = crate::BitReader<bool>;
///Field `MSWU` writer - Master Timer Software update
pub type MSWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `TASWU` reader - Timer A Software update
pub type TASWU_R = crate::BitReader<bool>;
///Field `TASWU` writer - Timer A Software update
pub type TASWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `TBSWU` reader - Timer B Software Update
pub type TBSWU_R = crate::BitReader<bool>;
///Field `TBSWU` writer - Timer B Software Update
pub type TBSWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `TCSWU` reader - Timer C Software Update
pub type TCSWU_R = crate::BitReader<bool>;
///Field `TCSWU` writer - Timer C Software Update
pub type TCSWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `TDSWU` reader - Timer D Software Update
pub type TDSWU_R = crate::BitReader<bool>;
///Field `TDSWU` writer - Timer D Software Update
pub type TDSWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `TESWU` reader - Timer E Software Update
pub type TESWU_R = crate::BitReader<bool>;
///Field `TESWU` writer - Timer E Software Update
pub type TESWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `MRST` reader - Master Counter software reset
pub type MRST_R = crate::BitReader<bool>;
///Field `MRST` writer - Master Counter software reset
pub type MRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `TARST` reader - Timer A counter software reset
pub type TARST_R = crate::BitReader<bool>;
///Field `TARST` writer - Timer A counter software reset
pub type TARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `TBRST` reader - Timer B counter software reset
pub type TBRST_R = crate::BitReader<bool>;
///Field `TBRST` writer - Timer B counter software reset
pub type TBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `TCRST` reader - Timer C counter software reset
pub type TCRST_R = crate::BitReader<bool>;
///Field `TCRST` writer - Timer C counter software reset
pub type TCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `TDRST` reader - Timer D counter software reset
pub type TDRST_R = crate::BitReader<bool>;
///Field `TDRST` writer - Timer D counter software reset
pub type TDRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `TERST` reader - Timer E counter software reset
pub type TERST_R = crate::BitReader<bool>;
///Field `TERST` writer - Timer E counter software reset
pub type TERST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
impl R {
    ///Bit 0 - Master Timer Software update
    #[inline(always)]
    pub fn mswu(&self) -> MSWU_R {
        MSWU_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timer A Software update
    #[inline(always)]
    pub fn taswu(&self) -> TASWU_R {
        TASWU_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Timer B Software Update
    #[inline(always)]
    pub fn tbswu(&self) -> TBSWU_R {
        TBSWU_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timer C Software Update
    #[inline(always)]
    pub fn tcswu(&self) -> TCSWU_R {
        TCSWU_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timer D Software Update
    #[inline(always)]
    pub fn tdswu(&self) -> TDSWU_R {
        TDSWU_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Timer E Software Update
    #[inline(always)]
    pub fn teswu(&self) -> TESWU_R {
        TESWU_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Master Counter software reset
    #[inline(always)]
    pub fn mrst(&self) -> MRST_R {
        MRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Timer A counter software reset
    #[inline(always)]
    pub fn tarst(&self) -> TARST_R {
        TARST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Timer B counter software reset
    #[inline(always)]
    pub fn tbrst(&self) -> TBRST_R {
        TBRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Timer C counter software reset
    #[inline(always)]
    pub fn tcrst(&self) -> TCRST_R {
        TCRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Timer D counter software reset
    #[inline(always)]
    pub fn tdrst(&self) -> TDRST_R {
        TDRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Timer E counter software reset
    #[inline(always)]
    pub fn terst(&self) -> TERST_R {
        TERST_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Master Timer Software update
    #[inline(always)]
    #[must_use]
    pub fn mswu(&mut self) -> MSWU_W<0> {
        MSWU_W::new(self)
    }
    ///Bit 1 - Timer A Software update
    #[inline(always)]
    #[must_use]
    pub fn taswu(&mut self) -> TASWU_W<1> {
        TASWU_W::new(self)
    }
    ///Bit 2 - Timer B Software Update
    #[inline(always)]
    #[must_use]
    pub fn tbswu(&mut self) -> TBSWU_W<2> {
        TBSWU_W::new(self)
    }
    ///Bit 3 - Timer C Software Update
    #[inline(always)]
    #[must_use]
    pub fn tcswu(&mut self) -> TCSWU_W<3> {
        TCSWU_W::new(self)
    }
    ///Bit 4 - Timer D Software Update
    #[inline(always)]
    #[must_use]
    pub fn tdswu(&mut self) -> TDSWU_W<4> {
        TDSWU_W::new(self)
    }
    ///Bit 5 - Timer E Software Update
    #[inline(always)]
    #[must_use]
    pub fn teswu(&mut self) -> TESWU_W<5> {
        TESWU_W::new(self)
    }
    ///Bit 8 - Master Counter software reset
    #[inline(always)]
    #[must_use]
    pub fn mrst(&mut self) -> MRST_W<8> {
        MRST_W::new(self)
    }
    ///Bit 9 - Timer A counter software reset
    #[inline(always)]
    #[must_use]
    pub fn tarst(&mut self) -> TARST_W<9> {
        TARST_W::new(self)
    }
    ///Bit 10 - Timer B counter software reset
    #[inline(always)]
    #[must_use]
    pub fn tbrst(&mut self) -> TBRST_W<10> {
        TBRST_W::new(self)
    }
    ///Bit 11 - Timer C counter software reset
    #[inline(always)]
    #[must_use]
    pub fn tcrst(&mut self) -> TCRST_W<11> {
        TCRST_W::new(self)
    }
    ///Bit 12 - Timer D counter software reset
    #[inline(always)]
    #[must_use]
    pub fn tdrst(&mut self) -> TDRST_W<12> {
        TDRST_W::new(self)
    }
    ///Bit 13 - Timer E counter software reset
    #[inline(always)]
    #[must_use]
    pub fn terst(&mut self) -> TERST_W<13> {
        TERST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Control Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr2](index.html) module
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr2::R](R) reader structure
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr2::W](W) writer structure
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
