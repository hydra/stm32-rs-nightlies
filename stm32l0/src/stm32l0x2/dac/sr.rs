///Register `SR` reader
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SR` writer
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DMAUDR1` reader - DAC channel1 DMA underrun flag
pub type DMAUDR1_R = crate::BitReader<DMAUDR1_A>;
///DAC channel1 DMA underrun flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAUDR1_A {
    ///0: No DMA underrun error condition occurred for DAC channel X
    NoUnderrun = 0,
    ///1: DMA underrun error condition occurred for DAC channel X
    Underrun = 1,
}
impl From<DMAUDR1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAUDR1_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAUDR1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAUDR1_A {
        match self.bits {
            false => DMAUDR1_A::NoUnderrun,
            true => DMAUDR1_A::Underrun,
        }
    }
    ///Checks if the value of the field is `NoUnderrun`
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == DMAUDR1_A::NoUnderrun
    }
    ///Checks if the value of the field is `Underrun`
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == DMAUDR1_A::Underrun
    }
}
///Field `DMAUDR1` writer - DAC channel1 DMA underrun flag
pub type DMAUDR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, DMAUDR1_A, O>;
impl<'a, const O: u8> DMAUDR1_W<'a, O> {
    ///No DMA underrun error condition occurred for DAC channel X
    #[inline(always)]
    pub fn no_underrun(self) -> &'a mut W {
        self.variant(DMAUDR1_A::NoUnderrun)
    }
    ///DMA underrun error condition occurred for DAC channel X
    #[inline(always)]
    pub fn underrun(self) -> &'a mut W {
        self.variant(DMAUDR1_A::Underrun)
    }
}
impl R {
    ///Bit 13 - DAC channel1 DMA underrun flag
    #[inline(always)]
    pub fn dmaudr1(&self) -> DMAUDR1_R {
        DMAUDR1_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    ///Bit 13 - DAC channel1 DMA underrun flag
    #[inline(always)]
    #[must_use]
    pub fn dmaudr1(&mut self) -> DMAUDR1_W<13> {
        DMAUDR1_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](index.html) module
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr::R](R) reader structure
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sr::W](W) writer structure
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
