///Register `SR3` reader
pub struct R(crate::R<SR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SR3` writer
pub struct W(crate::W<SR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR3_SPEC>;
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
impl From<crate::W<SR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TZSCF` reader - TZSCF
pub type TZSCF_R = crate::BitReader<bool>;
///Field `TZSCF` writer - TZSCF
pub type TZSCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR3_SPEC, bool, O>;
///Field `TZICF` reader - TZICF
pub type TZICF_R = crate::BitReader<bool>;
///Field `TZICF` writer - TZICF
pub type TZICF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR3_SPEC, bool, O>;
///Field `MPCWM1F` reader - MPCWM1F
pub type MPCWM1F_R = crate::BitReader<bool>;
///Field `MPCWM1F` writer - MPCWM1F
pub type MPCWM1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR3_SPEC, bool, O>;
///Field `MPCWM2F` reader - MPCWM2F
pub type MPCWM2F_R = crate::BitReader<bool>;
///Field `MPCWM2F` writer - MPCWM2F
pub type MPCWM2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR3_SPEC, bool, O>;
///Field `MPCBB1F` reader - MPCBB1F
pub type MPCBB1F_R = crate::BitReader<bool>;
///Field `MPCBB1F` writer - MPCBB1F
pub type MPCBB1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR3_SPEC, bool, O>;
///Field `MPCBB1_REGF` reader - MPCBB1_REGF
pub type MPCBB1_REGF_R = crate::BitReader<bool>;
///Field `MPCBB1_REGF` writer - MPCBB1_REGF
pub type MPCBB1_REGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR3_SPEC, bool, O>;
///Field `MPCBB2F` reader - MPCBB2F
pub type MPCBB2F_R = crate::BitReader<bool>;
///Field `MPCBB2F` writer - MPCBB2F
pub type MPCBB2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR3_SPEC, bool, O>;
///Field `MPCBB2_REGF` reader - MPCBB2_REGF
pub type MPCBB2_REGF_R = crate::BitReader<bool>;
///Field `MPCBB2_REGF` writer - MPCBB2_REGF
pub type MPCBB2_REGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR3_SPEC, bool, O>;
impl R {
    ///Bit 0 - TZSCF
    #[inline(always)]
    pub fn tzscf(&self) -> TZSCF_R {
        TZSCF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TZICF
    #[inline(always)]
    pub fn tzicf(&self) -> TZICF_R {
        TZICF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MPCWM1F
    #[inline(always)]
    pub fn mpcwm1f(&self) -> MPCWM1F_R {
        MPCWM1F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MPCWM2F
    #[inline(always)]
    pub fn mpcwm2f(&self) -> MPCWM2F_R {
        MPCWM2F_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MPCBB1F
    #[inline(always)]
    pub fn mpcbb1f(&self) -> MPCBB1F_R {
        MPCBB1F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MPCBB1_REGF
    #[inline(always)]
    pub fn mpcbb1_regf(&self) -> MPCBB1_REGF_R {
        MPCBB1_REGF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MPCBB2F
    #[inline(always)]
    pub fn mpcbb2f(&self) -> MPCBB2F_R {
        MPCBB2F_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - MPCBB2_REGF
    #[inline(always)]
    pub fn mpcbb2_regf(&self) -> MPCBB2_REGF_R {
        MPCBB2_REGF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TZSCF
    #[inline(always)]
    #[must_use]
    pub fn tzscf(&mut self) -> TZSCF_W<0> {
        TZSCF_W::new(self)
    }
    ///Bit 1 - TZICF
    #[inline(always)]
    #[must_use]
    pub fn tzicf(&mut self) -> TZICF_W<1> {
        TZICF_W::new(self)
    }
    ///Bit 2 - MPCWM1F
    #[inline(always)]
    #[must_use]
    pub fn mpcwm1f(&mut self) -> MPCWM1F_W<2> {
        MPCWM1F_W::new(self)
    }
    ///Bit 3 - MPCWM2F
    #[inline(always)]
    #[must_use]
    pub fn mpcwm2f(&mut self) -> MPCWM2F_W<3> {
        MPCWM2F_W::new(self)
    }
    ///Bit 4 - MPCBB1F
    #[inline(always)]
    #[must_use]
    pub fn mpcbb1f(&mut self) -> MPCBB1F_W<4> {
        MPCBB1F_W::new(self)
    }
    ///Bit 5 - MPCBB1_REGF
    #[inline(always)]
    #[must_use]
    pub fn mpcbb1_regf(&mut self) -> MPCBB1_REGF_W<5> {
        MPCBB1_REGF_W::new(self)
    }
    ///Bit 6 - MPCBB2F
    #[inline(always)]
    #[must_use]
    pub fn mpcbb2f(&mut self) -> MPCBB2F_W<6> {
        MPCBB2F_W::new(self)
    }
    ///Bit 7 - MPCBB2_REGF
    #[inline(always)]
    #[must_use]
    pub fn mpcbb2_regf(&mut self) -> MPCBB2_REGF_W<7> {
        MPCBB2_REGF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZIC interrupt status register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr3](index.html) module
pub struct SR3_SPEC;
impl crate::RegisterSpec for SR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr3::R](R) reader structure
impl crate::Readable for SR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sr3::W](W) writer structure
impl crate::Writable for SR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SR3 to value 0
impl crate::Resettable for SR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
