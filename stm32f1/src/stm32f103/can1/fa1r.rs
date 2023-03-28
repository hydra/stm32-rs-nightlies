///Register `FA1R` reader
pub struct R(crate::R<FA1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FA1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FA1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FA1R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FA1R` writer
pub struct W(crate::W<FA1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FA1R_SPEC>;
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
impl From<crate::W<FA1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FA1R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FACT0` reader - Filter active
pub type FACT0_R = crate::BitReader<bool>;
///Field `FACT0` writer - Filter active
pub type FACT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
///Field `FACT1` reader - Filter active
pub type FACT1_R = crate::BitReader<bool>;
///Field `FACT1` writer - Filter active
pub type FACT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
///Field `FACT2` reader - Filter active
pub type FACT2_R = crate::BitReader<bool>;
///Field `FACT2` writer - Filter active
pub type FACT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
///Field `FACT3` reader - Filter active
pub type FACT3_R = crate::BitReader<bool>;
///Field `FACT3` writer - Filter active
pub type FACT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
///Field `FACT4` reader - Filter active
pub type FACT4_R = crate::BitReader<bool>;
///Field `FACT4` writer - Filter active
pub type FACT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
///Field `FACT5` reader - Filter active
pub type FACT5_R = crate::BitReader<bool>;
///Field `FACT5` writer - Filter active
pub type FACT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
///Field `FACT6` reader - Filter active
pub type FACT6_R = crate::BitReader<bool>;
///Field `FACT6` writer - Filter active
pub type FACT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
///Field `FACT7` reader - Filter active
pub type FACT7_R = crate::BitReader<bool>;
///Field `FACT7` writer - Filter active
pub type FACT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
///Field `FACT8` reader - Filter active
pub type FACT8_R = crate::BitReader<bool>;
///Field `FACT8` writer - Filter active
pub type FACT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
///Field `FACT9` reader - Filter active
pub type FACT9_R = crate::BitReader<bool>;
///Field `FACT9` writer - Filter active
pub type FACT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
///Field `FACT10` reader - Filter active
pub type FACT10_R = crate::BitReader<bool>;
///Field `FACT10` writer - Filter active
pub type FACT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
///Field `FACT11` reader - Filter active
pub type FACT11_R = crate::BitReader<bool>;
///Field `FACT11` writer - Filter active
pub type FACT11_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
///Field `FACT12` reader - Filter active
pub type FACT12_R = crate::BitReader<bool>;
///Field `FACT12` writer - Filter active
pub type FACT12_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
///Field `FACT13` reader - Filter active
pub type FACT13_R = crate::BitReader<bool>;
///Field `FACT13` writer - Filter active
pub type FACT13_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
impl R {
    ///Bit 0 - Filter active
    #[inline(always)]
    pub fn fact0(&self) -> FACT0_R {
        FACT0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Filter active
    #[inline(always)]
    pub fn fact1(&self) -> FACT1_R {
        FACT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Filter active
    #[inline(always)]
    pub fn fact2(&self) -> FACT2_R {
        FACT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Filter active
    #[inline(always)]
    pub fn fact3(&self) -> FACT3_R {
        FACT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Filter active
    #[inline(always)]
    pub fn fact4(&self) -> FACT4_R {
        FACT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Filter active
    #[inline(always)]
    pub fn fact5(&self) -> FACT5_R {
        FACT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Filter active
    #[inline(always)]
    pub fn fact6(&self) -> FACT6_R {
        FACT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Filter active
    #[inline(always)]
    pub fn fact7(&self) -> FACT7_R {
        FACT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Filter active
    #[inline(always)]
    pub fn fact8(&self) -> FACT8_R {
        FACT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Filter active
    #[inline(always)]
    pub fn fact9(&self) -> FACT9_R {
        FACT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Filter active
    #[inline(always)]
    pub fn fact10(&self) -> FACT10_R {
        FACT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Filter active
    #[inline(always)]
    pub fn fact11(&self) -> FACT11_R {
        FACT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Filter active
    #[inline(always)]
    pub fn fact12(&self) -> FACT12_R {
        FACT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Filter active
    #[inline(always)]
    pub fn fact13(&self) -> FACT13_R {
        FACT13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Filter active
    #[inline(always)]
    #[must_use]
    pub fn fact0(&mut self) -> FACT0_W<0> {
        FACT0_W::new(self)
    }
    ///Bit 1 - Filter active
    #[inline(always)]
    #[must_use]
    pub fn fact1(&mut self) -> FACT1_W<1> {
        FACT1_W::new(self)
    }
    ///Bit 2 - Filter active
    #[inline(always)]
    #[must_use]
    pub fn fact2(&mut self) -> FACT2_W<2> {
        FACT2_W::new(self)
    }
    ///Bit 3 - Filter active
    #[inline(always)]
    #[must_use]
    pub fn fact3(&mut self) -> FACT3_W<3> {
        FACT3_W::new(self)
    }
    ///Bit 4 - Filter active
    #[inline(always)]
    #[must_use]
    pub fn fact4(&mut self) -> FACT4_W<4> {
        FACT4_W::new(self)
    }
    ///Bit 5 - Filter active
    #[inline(always)]
    #[must_use]
    pub fn fact5(&mut self) -> FACT5_W<5> {
        FACT5_W::new(self)
    }
    ///Bit 6 - Filter active
    #[inline(always)]
    #[must_use]
    pub fn fact6(&mut self) -> FACT6_W<6> {
        FACT6_W::new(self)
    }
    ///Bit 7 - Filter active
    #[inline(always)]
    #[must_use]
    pub fn fact7(&mut self) -> FACT7_W<7> {
        FACT7_W::new(self)
    }
    ///Bit 8 - Filter active
    #[inline(always)]
    #[must_use]
    pub fn fact8(&mut self) -> FACT8_W<8> {
        FACT8_W::new(self)
    }
    ///Bit 9 - Filter active
    #[inline(always)]
    #[must_use]
    pub fn fact9(&mut self) -> FACT9_W<9> {
        FACT9_W::new(self)
    }
    ///Bit 10 - Filter active
    #[inline(always)]
    #[must_use]
    pub fn fact10(&mut self) -> FACT10_W<10> {
        FACT10_W::new(self)
    }
    ///Bit 11 - Filter active
    #[inline(always)]
    #[must_use]
    pub fn fact11(&mut self) -> FACT11_W<11> {
        FACT11_W::new(self)
    }
    ///Bit 12 - Filter active
    #[inline(always)]
    #[must_use]
    pub fn fact12(&mut self) -> FACT12_W<12> {
        FACT12_W::new(self)
    }
    ///Bit 13 - Filter active
    #[inline(always)]
    #[must_use]
    pub fn fact13(&mut self) -> FACT13_W<13> {
        FACT13_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CAN_FA1R
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fa1r](index.html) module
pub struct FA1R_SPEC;
impl crate::RegisterSpec for FA1R_SPEC {
    type Ux = u32;
}
///`read()` method returns [fa1r::R](R) reader structure
impl crate::Readable for FA1R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fa1r::W](W) writer structure
impl crate::Writable for FA1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FA1R to value 0
impl crate::Resettable for FA1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
