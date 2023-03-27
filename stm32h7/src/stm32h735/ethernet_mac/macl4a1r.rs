///Register `MACL4A1R` reader
pub struct R(crate::R<MACL4A1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACL4A1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACL4A1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACL4A1R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACL4A1R` writer
pub struct W(crate::W<MACL4A1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACL4A1R_SPEC>;
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
impl From<crate::W<MACL4A1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACL4A1R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `L4SP1` reader - Layer 4 Source Port Number Field
pub type L4SP1_R = crate::FieldReader<u16, u16>;
///Field `L4SP1` writer - Layer 4 Source Port Number Field
pub type L4SP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACL4A1R_SPEC, u16, u16, 16, O>;
///Field `L4DP1` reader - Layer 4 Destination Port Number Field
pub type L4DP1_R = crate::FieldReader<u16, u16>;
///Field `L4DP1` writer - Layer 4 Destination Port Number Field
pub type L4DP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACL4A1R_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Layer 4 Source Port Number Field
    #[inline(always)]
    pub fn l4sp1(&self) -> L4SP1_R {
        L4SP1_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Layer 4 Destination Port Number Field
    #[inline(always)]
    pub fn l4dp1(&self) -> L4DP1_R {
        L4DP1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Layer 4 Source Port Number Field
    #[inline(always)]
    #[must_use]
    pub fn l4sp1(&mut self) -> L4SP1_W<0> {
        L4SP1_W::new(self)
    }
    ///Bits 16:31 - Layer 4 Destination Port Number Field
    #[inline(always)]
    #[must_use]
    pub fn l4dp1(&mut self) -> L4DP1_W<16> {
        L4DP1_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Layer 4 address filter 1 register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macl4a1r](index.html) module
pub struct MACL4A1R_SPEC;
impl crate::RegisterSpec for MACL4A1R_SPEC {
    type Ux = u32;
}
///`read()` method returns [macl4a1r::R](R) reader structure
impl crate::Readable for MACL4A1R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macl4a1r::W](W) writer structure
impl crate::Writable for MACL4A1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACL4A1R to value 0
impl crate::Resettable for MACL4A1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
