///Register `FCR` reader
pub struct R(crate::R<FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FCR` writer
pub struct W(crate::W<FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR_SPEC>;
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
impl From<crate::W<FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IOSR` reader - Integrator oversampling ratio (averaging length)
pub type IOSR_R = crate::FieldReader<u8, u8>;
///Field `IOSR` writer - Integrator oversampling ratio (averaging length)
pub type IOSR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FCR_SPEC, u8, u8, 8, O>;
///Field `FOSR` reader - Sinc filter oversampling ratio (decimation rate)
pub type FOSR_R = crate::FieldReader<u16, u16>;
///Field `FOSR` writer - Sinc filter oversampling ratio (decimation rate)
pub type FOSR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FCR_SPEC, u16, u16, 10, O>;
///Field `FORD` reader - Sinc filter order
pub type FORD_R = crate::FieldReader<u8, FORD_A>;
///Sinc filter order
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FORD_A {
    ///0: FastSinc filter type
    FastSinc = 0,
    ///1: Sinc1 filter type
    Sinc1 = 1,
    ///2: Sinc2 filter type
    Sinc2 = 2,
    ///3: Sinc3 filter type
    Sinc3 = 3,
    ///4: Sinc4 filter type
    Sinc4 = 4,
    ///5: Sinc5 filter type
    Sinc5 = 5,
}
impl From<FORD_A> for u8 {
    #[inline(always)]
    fn from(variant: FORD_A) -> Self {
        variant as _
    }
}
impl FORD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<FORD_A> {
        match self.bits {
            0 => Some(FORD_A::FastSinc),
            1 => Some(FORD_A::Sinc1),
            2 => Some(FORD_A::Sinc2),
            3 => Some(FORD_A::Sinc3),
            4 => Some(FORD_A::Sinc4),
            5 => Some(FORD_A::Sinc5),
            _ => None,
        }
    }
    ///Checks if the value of the field is `FastSinc`
    #[inline(always)]
    pub fn is_fast_sinc(&self) -> bool {
        *self == FORD_A::FastSinc
    }
    ///Checks if the value of the field is `Sinc1`
    #[inline(always)]
    pub fn is_sinc1(&self) -> bool {
        *self == FORD_A::Sinc1
    }
    ///Checks if the value of the field is `Sinc2`
    #[inline(always)]
    pub fn is_sinc2(&self) -> bool {
        *self == FORD_A::Sinc2
    }
    ///Checks if the value of the field is `Sinc3`
    #[inline(always)]
    pub fn is_sinc3(&self) -> bool {
        *self == FORD_A::Sinc3
    }
    ///Checks if the value of the field is `Sinc4`
    #[inline(always)]
    pub fn is_sinc4(&self) -> bool {
        *self == FORD_A::Sinc4
    }
    ///Checks if the value of the field is `Sinc5`
    #[inline(always)]
    pub fn is_sinc5(&self) -> bool {
        *self == FORD_A::Sinc5
    }
}
///Field `FORD` writer - Sinc filter order
pub type FORD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCR_SPEC, u8, FORD_A, 3, O>;
impl<'a, const O: u8> FORD_W<'a, O> {
    ///FastSinc filter type
    #[inline(always)]
    pub fn fast_sinc(self) -> &'a mut W {
        self.variant(FORD_A::FastSinc)
    }
    ///Sinc1 filter type
    #[inline(always)]
    pub fn sinc1(self) -> &'a mut W {
        self.variant(FORD_A::Sinc1)
    }
    ///Sinc2 filter type
    #[inline(always)]
    pub fn sinc2(self) -> &'a mut W {
        self.variant(FORD_A::Sinc2)
    }
    ///Sinc3 filter type
    #[inline(always)]
    pub fn sinc3(self) -> &'a mut W {
        self.variant(FORD_A::Sinc3)
    }
    ///Sinc4 filter type
    #[inline(always)]
    pub fn sinc4(self) -> &'a mut W {
        self.variant(FORD_A::Sinc4)
    }
    ///Sinc5 filter type
    #[inline(always)]
    pub fn sinc5(self) -> &'a mut W {
        self.variant(FORD_A::Sinc5)
    }
}
impl R {
    ///Bits 0:7 - Integrator oversampling ratio (averaging length)
    #[inline(always)]
    pub fn iosr(&self) -> IOSR_R {
        IOSR_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:25 - Sinc filter oversampling ratio (decimation rate)
    #[inline(always)]
    pub fn fosr(&self) -> FOSR_R {
        FOSR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bits 29:31 - Sinc filter order
    #[inline(always)]
    pub fn ford(&self) -> FORD_R {
        FORD_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    ///Bits 0:7 - Integrator oversampling ratio (averaging length)
    #[inline(always)]
    #[must_use]
    pub fn iosr(&mut self) -> IOSR_W<0> {
        IOSR_W::new(self)
    }
    ///Bits 16:25 - Sinc filter oversampling ratio (decimation rate)
    #[inline(always)]
    #[must_use]
    pub fn fosr(&mut self) -> FOSR_W<16> {
        FOSR_W::new(self)
    }
    ///Bits 29:31 - Sinc filter order
    #[inline(always)]
    #[must_use]
    pub fn ford(&mut self) -> FORD_W<29> {
        FORD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///filter control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fcr](index.html) module
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [fcr::R](R) reader structure
impl crate::Readable for FCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fcr::W](W) writer structure
impl crate::Writable for FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FCR to value 0
impl crate::Resettable for FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
