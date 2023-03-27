///Register `SMPR` reader
pub struct R(crate::R<SMPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SMPR` writer
pub struct W(crate::W<SMPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR_SPEC>;
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
impl From<crate::W<SMPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SMP` reader - Sampling time selection
pub type SMP_R = crate::FieldReader<u8, SMP_A>;
///Sampling time selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP_A {
    ///0: 1.5 cycles
    Cycles15 = 0,
    ///1: 7.5 cycles
    Cycles75 = 1,
    ///2: 13.5 cycles
    Cycles135 = 2,
    ///3: 28.5 cycles
    Cycles285 = 3,
    ///4: 41.5 cycles
    Cycles415 = 4,
    ///5: 55.5 cycles
    Cycles555 = 5,
    ///6: 71.5 cycles
    Cycles715 = 6,
    ///7: 239.5 cycles
    Cycles2395 = 7,
}
impl From<SMP_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP_A) -> Self {
        variant as _
    }
}
impl SMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SMP_A {
        match self.bits {
            0 => SMP_A::Cycles15,
            1 => SMP_A::Cycles75,
            2 => SMP_A::Cycles135,
            3 => SMP_A::Cycles285,
            4 => SMP_A::Cycles415,
            5 => SMP_A::Cycles555,
            6 => SMP_A::Cycles715,
            7 => SMP_A::Cycles2395,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Cycles15`
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SMP_A::Cycles15
    }
    ///Checks if the value of the field is `Cycles75`
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == SMP_A::Cycles75
    }
    ///Checks if the value of the field is `Cycles135`
    #[inline(always)]
    pub fn is_cycles13_5(&self) -> bool {
        *self == SMP_A::Cycles135
    }
    ///Checks if the value of the field is `Cycles285`
    #[inline(always)]
    pub fn is_cycles28_5(&self) -> bool {
        *self == SMP_A::Cycles285
    }
    ///Checks if the value of the field is `Cycles415`
    #[inline(always)]
    pub fn is_cycles41_5(&self) -> bool {
        *self == SMP_A::Cycles415
    }
    ///Checks if the value of the field is `Cycles555`
    #[inline(always)]
    pub fn is_cycles55_5(&self) -> bool {
        *self == SMP_A::Cycles555
    }
    ///Checks if the value of the field is `Cycles715`
    #[inline(always)]
    pub fn is_cycles71_5(&self) -> bool {
        *self == SMP_A::Cycles715
    }
    ///Checks if the value of the field is `Cycles2395`
    #[inline(always)]
    pub fn is_cycles239_5(&self) -> bool {
        *self == SMP_A::Cycles2395
    }
}
///Field `SMP` writer - Sampling time selection
pub type SMP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SMPR_SPEC, u8, SMP_A, 3, O>;
impl<'a, const O: u8> SMP_W<'a, O> {
    ///1.5 cycles
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP_A::Cycles15)
    }
    ///7.5 cycles
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut W {
        self.variant(SMP_A::Cycles75)
    }
    ///13.5 cycles
    #[inline(always)]
    pub fn cycles13_5(self) -> &'a mut W {
        self.variant(SMP_A::Cycles135)
    }
    ///28.5 cycles
    #[inline(always)]
    pub fn cycles28_5(self) -> &'a mut W {
        self.variant(SMP_A::Cycles285)
    }
    ///41.5 cycles
    #[inline(always)]
    pub fn cycles41_5(self) -> &'a mut W {
        self.variant(SMP_A::Cycles415)
    }
    ///55.5 cycles
    #[inline(always)]
    pub fn cycles55_5(self) -> &'a mut W {
        self.variant(SMP_A::Cycles555)
    }
    ///71.5 cycles
    #[inline(always)]
    pub fn cycles71_5(self) -> &'a mut W {
        self.variant(SMP_A::Cycles715)
    }
    ///239.5 cycles
    #[inline(always)]
    pub fn cycles239_5(self) -> &'a mut W {
        self.variant(SMP_A::Cycles2395)
    }
}
impl R {
    ///Bits 0:2 - Sampling time selection
    #[inline(always)]
    pub fn smp(&self) -> SMP_R {
        SMP_R::new((self.bits & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - Sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp(&mut self) -> SMP_W<0> {
        SMP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///sampling time register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [smpr](index.html) module
pub struct SMPR_SPEC;
impl crate::RegisterSpec for SMPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [smpr::R](R) reader structure
impl crate::Readable for SMPR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [smpr::W](W) writer structure
impl crate::Writable for SMPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SMPR to value 0
impl crate::Resettable for SMPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
