///Register `PWR_CR3` reader
pub struct R(crate::R<PWR_CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PWR_CR3` writer
pub struct W(crate::W<PWR_CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_CR3_SPEC>;
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
impl From<crate::W<PWR_CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_CR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `REGSEL` reader - Regulator selection Note: REGSEL is reserved and must be kept at reset value in packages without SMPS.
pub type REGSEL_R = crate::BitReader<bool>;
///Field `REGSEL` writer - Regulator selection Note: REGSEL is reserved and must be kept at reset value in packages without SMPS.
pub type REGSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR3_SPEC, bool, O>;
///Field `FSTEN` reader - Fast soft start
pub type FSTEN_R = crate::BitReader<bool>;
///Field `FSTEN` writer - Fast soft start
pub type FSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR3_SPEC, bool, O>;
impl R {
    ///Bit 1 - Regulator selection Note: REGSEL is reserved and must be kept at reset value in packages without SMPS.
    #[inline(always)]
    pub fn regsel(&self) -> REGSEL_R {
        REGSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Fast soft start
    #[inline(always)]
    pub fn fsten(&self) -> FSTEN_R {
        FSTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - Regulator selection Note: REGSEL is reserved and must be kept at reset value in packages without SMPS.
    #[inline(always)]
    #[must_use]
    pub fn regsel(&mut self) -> REGSEL_W<1> {
        REGSEL_W::new(self)
    }
    ///Bit 2 - Fast soft start
    #[inline(always)]
    #[must_use]
    pub fn fsten(&mut self) -> FSTEN_W<2> {
        FSTEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PWR control register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pwr_cr3](index.html) module
pub struct PWR_CR3_SPEC;
impl crate::RegisterSpec for PWR_CR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [pwr_cr3::R](R) reader structure
impl crate::Readable for PWR_CR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pwr_cr3::W](W) writer structure
impl crate::Writable for PWR_CR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PWR_CR3 to value 0
impl crate::Resettable for PWR_CR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
