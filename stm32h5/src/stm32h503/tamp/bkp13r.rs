///Register `BKP13R` reader
pub struct R(crate::R<BKP13R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BKP13R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BKP13R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BKP13R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BKP13R` writer
pub struct W(crate::W<BKP13R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BKP13R_SPEC>;
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
impl From<crate::W<BKP13R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BKP13R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BKP` reader - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set.
pub type BKP_R = crate::FieldReader<u32, u32>;
///Field `BKP` writer - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set.
pub type BKP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BKP13R_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set.
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set.
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<0> {
        BKP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TAMP backup 13 register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bkp13r](index.html) module
pub struct BKP13R_SPEC;
impl crate::RegisterSpec for BKP13R_SPEC {
    type Ux = u32;
}
///`read()` method returns [bkp13r::R](R) reader structure
impl crate::Readable for BKP13R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bkp13r::W](W) writer structure
impl crate::Writable for BKP13R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BKP13R to value 0
impl crate::Resettable for BKP13R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
