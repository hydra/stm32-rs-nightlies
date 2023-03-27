///Register `FCR3` reader
pub struct R(crate::R<FCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FCR3` writer
pub struct W(crate::W<FCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR3_SPEC>;
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
impl From<crate::W<FCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TZSCFC` reader - TZSCFC
pub type TZSCFC_R = crate::BitReader<bool>;
///Field `TZSCFC` writer - TZSCFC
pub type TZSCFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `TZICFC` reader - TZICFC
pub type TZICFC_R = crate::BitReader<bool>;
///Field `TZICFC` writer - TZICFC
pub type TZICFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `MPCWM1FC` reader - MPCWM1FC
pub type MPCWM1FC_R = crate::BitReader<bool>;
///Field `MPCWM1FC` writer - MPCWM1FC
pub type MPCWM1FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `MPCWM2FC` reader - MPCWM2FC
pub type MPCWM2FC_R = crate::BitReader<bool>;
///Field `MPCWM2FC` writer - MPCWM2FC
pub type MPCWM2FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `MPCBB1FC` reader - MPCBB1FC
pub type MPCBB1FC_R = crate::BitReader<bool>;
///Field `MPCBB1FC` writer - MPCBB1FC
pub type MPCBB1FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `MPCBB1_REGFC` reader - MPCBB1_REGFC
pub type MPCBB1_REGFC_R = crate::BitReader<bool>;
///Field `MPCBB1_REGFC` writer - MPCBB1_REGFC
pub type MPCBB1_REGFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `MPCBB2FC` reader - MPCBB2FC
pub type MPCBB2FC_R = crate::BitReader<bool>;
///Field `MPCBB2FC` writer - MPCBB2FC
pub type MPCBB2FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `MPCBB2_REGFC` reader - MPCBB2_REGFC
pub type MPCBB2_REGFC_R = crate::BitReader<bool>;
///Field `MPCBB2_REGFC` writer - MPCBB2_REGFC
pub type MPCBB2_REGFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
impl R {
    ///Bit 0 - TZSCFC
    #[inline(always)]
    pub fn tzscfc(&self) -> TZSCFC_R {
        TZSCFC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TZICFC
    #[inline(always)]
    pub fn tzicfc(&self) -> TZICFC_R {
        TZICFC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MPCWM1FC
    #[inline(always)]
    pub fn mpcwm1fc(&self) -> MPCWM1FC_R {
        MPCWM1FC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MPCWM2FC
    #[inline(always)]
    pub fn mpcwm2fc(&self) -> MPCWM2FC_R {
        MPCWM2FC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MPCBB1FC
    #[inline(always)]
    pub fn mpcbb1fc(&self) -> MPCBB1FC_R {
        MPCBB1FC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MPCBB1_REGFC
    #[inline(always)]
    pub fn mpcbb1_regfc(&self) -> MPCBB1_REGFC_R {
        MPCBB1_REGFC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MPCBB2FC
    #[inline(always)]
    pub fn mpcbb2fc(&self) -> MPCBB2FC_R {
        MPCBB2FC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - MPCBB2_REGFC
    #[inline(always)]
    pub fn mpcbb2_regfc(&self) -> MPCBB2_REGFC_R {
        MPCBB2_REGFC_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TZSCFC
    #[inline(always)]
    #[must_use]
    pub fn tzscfc(&mut self) -> TZSCFC_W<0> {
        TZSCFC_W::new(self)
    }
    ///Bit 1 - TZICFC
    #[inline(always)]
    #[must_use]
    pub fn tzicfc(&mut self) -> TZICFC_W<1> {
        TZICFC_W::new(self)
    }
    ///Bit 2 - MPCWM1FC
    #[inline(always)]
    #[must_use]
    pub fn mpcwm1fc(&mut self) -> MPCWM1FC_W<2> {
        MPCWM1FC_W::new(self)
    }
    ///Bit 3 - MPCWM2FC
    #[inline(always)]
    #[must_use]
    pub fn mpcwm2fc(&mut self) -> MPCWM2FC_W<3> {
        MPCWM2FC_W::new(self)
    }
    ///Bit 4 - MPCBB1FC
    #[inline(always)]
    #[must_use]
    pub fn mpcbb1fc(&mut self) -> MPCBB1FC_W<4> {
        MPCBB1FC_W::new(self)
    }
    ///Bit 5 - MPCBB1_REGFC
    #[inline(always)]
    #[must_use]
    pub fn mpcbb1_regfc(&mut self) -> MPCBB1_REGFC_W<5> {
        MPCBB1_REGFC_W::new(self)
    }
    ///Bit 6 - MPCBB2FC
    #[inline(always)]
    #[must_use]
    pub fn mpcbb2fc(&mut self) -> MPCBB2FC_W<6> {
        MPCBB2FC_W::new(self)
    }
    ///Bit 7 - MPCBB2_REGFC
    #[inline(always)]
    #[must_use]
    pub fn mpcbb2_regfc(&mut self) -> MPCBB2_REGFC_W<7> {
        MPCBB2_REGFC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZIC interrupt clear register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fcr3](index.html) module
pub struct FCR3_SPEC;
impl crate::RegisterSpec for FCR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [fcr3::R](R) reader structure
impl crate::Readable for FCR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fcr3::W](W) writer structure
impl crate::Writable for FCR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FCR3 to value 0
impl crate::Resettable for FCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
