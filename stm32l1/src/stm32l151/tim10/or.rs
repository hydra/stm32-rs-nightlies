///Register `OR` reader
pub struct R(crate::R<OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OR` writer
pub struct W(crate::W<OR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR_SPEC>;
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
impl From<crate::W<OR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TI1_RMP` reader - Timer 10 input 1 remap
pub type TI1_RMP_R = crate::FieldReader<u8, u8>;
///Field `TI1_RMP` writer - Timer 10 input 1 remap
pub type TI1_RMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OR_SPEC, u8, u8, 2, O>;
///Field `ETR_RMP` reader - Timer10 ETR remap
pub type ETR_RMP_R = crate::BitReader<bool>;
///Field `ETR_RMP` writer - Timer10 ETR remap
pub type ETR_RMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR_SPEC, bool, O>;
///Field `TI1_RMP_RI` reader - Timer10 Input 1 remap for Routing Interface (RI)
pub type TI1_RMP_RI_R = crate::BitReader<bool>;
///Field `TI1_RMP_RI` writer - Timer10 Input 1 remap for Routing Interface (RI)
pub type TI1_RMP_RI_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR_SPEC, bool, O>;
impl R {
    ///Bits 0:1 - Timer 10 input 1 remap
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Timer10 ETR remap
    #[inline(always)]
    pub fn etr_rmp(&self) -> ETR_RMP_R {
        ETR_RMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timer10 Input 1 remap for Routing Interface (RI)
    #[inline(always)]
    pub fn ti1_rmp_ri(&self) -> TI1_RMP_RI_R {
        TI1_RMP_RI_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Timer 10 input 1 remap
    #[inline(always)]
    #[must_use]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W<0> {
        TI1_RMP_W::new(self)
    }
    ///Bit 2 - Timer10 ETR remap
    #[inline(always)]
    #[must_use]
    pub fn etr_rmp(&mut self) -> ETR_RMP_W<2> {
        ETR_RMP_W::new(self)
    }
    ///Bit 3 - Timer10 Input 1 remap for Routing Interface (RI)
    #[inline(always)]
    #[must_use]
    pub fn ti1_rmp_ri(&mut self) -> TI1_RMP_RI_W<3> {
        TI1_RMP_RI_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///option register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [or](index.html) module
pub struct OR_SPEC;
impl crate::RegisterSpec for OR_SPEC {
    type Ux = u32;
}
///`read()` method returns [or::R](R) reader structure
impl crate::Readable for OR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [or::W](W) writer structure
impl crate::Writable for OR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OR to value 0
impl crate::Resettable for OR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
