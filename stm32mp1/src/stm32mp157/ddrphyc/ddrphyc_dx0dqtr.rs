///Register `DDRPHYC_DX0DQTR` reader
pub struct R(crate::R<DDRPHYC_DX0DQTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DX0DQTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DX0DQTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DX0DQTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DDRPHYC_DX0DQTR` writer
pub struct W(crate::W<DDRPHYC_DX0DQTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DX0DQTR_SPEC>;
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
impl From<crate::W<DDRPHYC_DX0DQTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DX0DQTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DQDLY0` reader - DQDLY0
pub type DQDLY0_R = crate::FieldReader<u8, u8>;
///Field `DQDLY0` writer - DQDLY0
pub type DQDLY0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DX0DQTR_SPEC, u8, u8, 4, O>;
///Field `DQDLY1` reader - DQDLY1
pub type DQDLY1_R = crate::FieldReader<u8, u8>;
///Field `DQDLY1` writer - DQDLY1
pub type DQDLY1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DX0DQTR_SPEC, u8, u8, 4, O>;
///Field `DQDLY2` reader - DQDLY2
pub type DQDLY2_R = crate::FieldReader<u8, u8>;
///Field `DQDLY2` writer - DQDLY2
pub type DQDLY2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DX0DQTR_SPEC, u8, u8, 4, O>;
///Field `DQDLY3` reader - DQDLY3
pub type DQDLY3_R = crate::FieldReader<u8, u8>;
///Field `DQDLY3` writer - DQDLY3
pub type DQDLY3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DX0DQTR_SPEC, u8, u8, 4, O>;
///Field `DQDLY4` reader - DQDLY4
pub type DQDLY4_R = crate::FieldReader<u8, u8>;
///Field `DQDLY4` writer - DQDLY4
pub type DQDLY4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DX0DQTR_SPEC, u8, u8, 4, O>;
///Field `DQDLY5` reader - DQDLY5
pub type DQDLY5_R = crate::FieldReader<u8, u8>;
///Field `DQDLY5` writer - DQDLY5
pub type DQDLY5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DX0DQTR_SPEC, u8, u8, 4, O>;
///Field `DQDLY6` reader - DQDLY6
pub type DQDLY6_R = crate::FieldReader<u8, u8>;
///Field `DQDLY6` writer - DQDLY6
pub type DQDLY6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DX0DQTR_SPEC, u8, u8, 4, O>;
///Field `DQDLY7` reader - DQDLY7
pub type DQDLY7_R = crate::FieldReader<u8, u8>;
///Field `DQDLY7` writer - DQDLY7
pub type DQDLY7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DX0DQTR_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:3 - DQDLY0
    #[inline(always)]
    pub fn dqdly0(&self) -> DQDLY0_R {
        DQDLY0_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - DQDLY1
    #[inline(always)]
    pub fn dqdly1(&self) -> DQDLY1_R {
        DQDLY1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - DQDLY2
    #[inline(always)]
    pub fn dqdly2(&self) -> DQDLY2_R {
        DQDLY2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - DQDLY3
    #[inline(always)]
    pub fn dqdly3(&self) -> DQDLY3_R {
        DQDLY3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - DQDLY4
    #[inline(always)]
    pub fn dqdly4(&self) -> DQDLY4_R {
        DQDLY4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - DQDLY5
    #[inline(always)]
    pub fn dqdly5(&self) -> DQDLY5_R {
        DQDLY5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - DQDLY6
    #[inline(always)]
    pub fn dqdly6(&self) -> DQDLY6_R {
        DQDLY6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - DQDLY7
    #[inline(always)]
    pub fn dqdly7(&self) -> DQDLY7_R {
        DQDLY7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - DQDLY0
    #[inline(always)]
    #[must_use]
    pub fn dqdly0(&mut self) -> DQDLY0_W<0> {
        DQDLY0_W::new(self)
    }
    ///Bits 4:7 - DQDLY1
    #[inline(always)]
    #[must_use]
    pub fn dqdly1(&mut self) -> DQDLY1_W<4> {
        DQDLY1_W::new(self)
    }
    ///Bits 8:11 - DQDLY2
    #[inline(always)]
    #[must_use]
    pub fn dqdly2(&mut self) -> DQDLY2_W<8> {
        DQDLY2_W::new(self)
    }
    ///Bits 12:15 - DQDLY3
    #[inline(always)]
    #[must_use]
    pub fn dqdly3(&mut self) -> DQDLY3_W<12> {
        DQDLY3_W::new(self)
    }
    ///Bits 16:19 - DQDLY4
    #[inline(always)]
    #[must_use]
    pub fn dqdly4(&mut self) -> DQDLY4_W<16> {
        DQDLY4_W::new(self)
    }
    ///Bits 20:23 - DQDLY5
    #[inline(always)]
    #[must_use]
    pub fn dqdly5(&mut self) -> DQDLY5_W<20> {
        DQDLY5_W::new(self)
    }
    ///Bits 24:27 - DQDLY6
    #[inline(always)]
    #[must_use]
    pub fn dqdly6(&mut self) -> DQDLY6_W<24> {
        DQDLY6_W::new(self)
    }
    ///Bits 28:31 - DQDLY7
    #[inline(always)]
    #[must_use]
    pub fn dqdly7(&mut self) -> DQDLY7_W<28> {
        DQDLY7_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DDRPHYC byte lane 0 DQT register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrphyc_dx0dqtr](index.html) module
pub struct DDRPHYC_DX0DQTR_SPEC;
impl crate::RegisterSpec for DDRPHYC_DX0DQTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrphyc_dx0dqtr::R](R) reader structure
impl crate::Readable for DDRPHYC_DX0DQTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ddrphyc_dx0dqtr::W](W) writer structure
impl crate::Writable for DDRPHYC_DX0DQTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DDRPHYC_DX0DQTR to value 0xffff_ffff
impl crate::Resettable for DDRPHYC_DX0DQTR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
