///Register `OR1` reader
pub struct R(crate::R<OR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OR1` writer
pub struct W(crate::W<OR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR1_SPEC>;
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
impl From<crate::W<OR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TI1_RMP` reader - Input Capture 1 remap
pub type TI1_RMP_R = crate::BitReader<bool>;
///Field `TI1_RMP` writer - Input Capture 1 remap
pub type TI1_RMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR1_SPEC, bool, O>;
///Field `ENCODER_MODE` reader - Encoder mode
pub type ENCODER_MODE_R = crate::FieldReader<u8, u8>;
///Field `ENCODER_MODE` writer - Encoder mode
pub type ENCODER_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OR1_SPEC, u8, u8, 2, O>;
impl R {
    ///Bit 0 - Input Capture 1 remap
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Encoder mode
    #[inline(always)]
    pub fn encoder_mode(&self) -> ENCODER_MODE_R {
        ENCODER_MODE_R::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - Input Capture 1 remap
    #[inline(always)]
    #[must_use]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W<0> {
        TI1_RMP_W::new(self)
    }
    ///Bits 1:2 - Encoder mode
    #[inline(always)]
    #[must_use]
    pub fn encoder_mode(&mut self) -> ENCODER_MODE_W<1> {
        ENCODER_MODE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM15 option register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [or1](index.html) module
pub struct OR1_SPEC;
impl crate::RegisterSpec for OR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [or1::R](R) reader structure
impl crate::Readable for OR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [or1::W](W) writer structure
impl crate::Writable for OR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OR1 to value 0
impl crate::Resettable for OR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
