///Register `CSR` reader
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSR` writer
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FUNC` reader - Function
pub type FUNC_R = crate::FieldReader<u8, FUNC_A>;
///Function
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FUNC_A {
    ///0: Cosine function
    Cosine = 0,
    ///1: Sine function
    Sine = 1,
    ///2: Phase function
    Phase = 2,
    ///3: Modulus function
    Modulus = 3,
    ///4: Arctangent function
    Arctangent = 4,
    ///5: Hyperbolic Cosine function
    HyperbolicCosine = 5,
    ///6: Hyperbolic Sine function
    HyperbolicSine = 6,
    ///7: Arctanh function
    Arctanh = 7,
    ///8: Natural Logarithm function
    NaturalLogarithm = 8,
    ///9: Square Root function
    SquareRoot = 9,
}
impl From<FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNC_A) -> Self {
        variant as _
    }
}
impl FUNC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<FUNC_A> {
        match self.bits {
            0 => Some(FUNC_A::Cosine),
            1 => Some(FUNC_A::Sine),
            2 => Some(FUNC_A::Phase),
            3 => Some(FUNC_A::Modulus),
            4 => Some(FUNC_A::Arctangent),
            5 => Some(FUNC_A::HyperbolicCosine),
            6 => Some(FUNC_A::HyperbolicSine),
            7 => Some(FUNC_A::Arctanh),
            8 => Some(FUNC_A::NaturalLogarithm),
            9 => Some(FUNC_A::SquareRoot),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Cosine`
    #[inline(always)]
    pub fn is_cosine(&self) -> bool {
        *self == FUNC_A::Cosine
    }
    ///Checks if the value of the field is `Sine`
    #[inline(always)]
    pub fn is_sine(&self) -> bool {
        *self == FUNC_A::Sine
    }
    ///Checks if the value of the field is `Phase`
    #[inline(always)]
    pub fn is_phase(&self) -> bool {
        *self == FUNC_A::Phase
    }
    ///Checks if the value of the field is `Modulus`
    #[inline(always)]
    pub fn is_modulus(&self) -> bool {
        *self == FUNC_A::Modulus
    }
    ///Checks if the value of the field is `Arctangent`
    #[inline(always)]
    pub fn is_arctangent(&self) -> bool {
        *self == FUNC_A::Arctangent
    }
    ///Checks if the value of the field is `HyperbolicCosine`
    #[inline(always)]
    pub fn is_hyperbolic_cosine(&self) -> bool {
        *self == FUNC_A::HyperbolicCosine
    }
    ///Checks if the value of the field is `HyperbolicSine`
    #[inline(always)]
    pub fn is_hyperbolic_sine(&self) -> bool {
        *self == FUNC_A::HyperbolicSine
    }
    ///Checks if the value of the field is `Arctanh`
    #[inline(always)]
    pub fn is_arctanh(&self) -> bool {
        *self == FUNC_A::Arctanh
    }
    ///Checks if the value of the field is `NaturalLogarithm`
    #[inline(always)]
    pub fn is_natural_logarithm(&self) -> bool {
        *self == FUNC_A::NaturalLogarithm
    }
    ///Checks if the value of the field is `SquareRoot`
    #[inline(always)]
    pub fn is_square_root(&self) -> bool {
        *self == FUNC_A::SquareRoot
    }
}
///Field `FUNC` writer - Function
pub type FUNC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, FUNC_A, 4, O>;
impl<'a, const O: u8> FUNC_W<'a, O> {
    ///Cosine function
    #[inline(always)]
    pub fn cosine(self) -> &'a mut W {
        self.variant(FUNC_A::Cosine)
    }
    ///Sine function
    #[inline(always)]
    pub fn sine(self) -> &'a mut W {
        self.variant(FUNC_A::Sine)
    }
    ///Phase function
    #[inline(always)]
    pub fn phase(self) -> &'a mut W {
        self.variant(FUNC_A::Phase)
    }
    ///Modulus function
    #[inline(always)]
    pub fn modulus(self) -> &'a mut W {
        self.variant(FUNC_A::Modulus)
    }
    ///Arctangent function
    #[inline(always)]
    pub fn arctangent(self) -> &'a mut W {
        self.variant(FUNC_A::Arctangent)
    }
    ///Hyperbolic Cosine function
    #[inline(always)]
    pub fn hyperbolic_cosine(self) -> &'a mut W {
        self.variant(FUNC_A::HyperbolicCosine)
    }
    ///Hyperbolic Sine function
    #[inline(always)]
    pub fn hyperbolic_sine(self) -> &'a mut W {
        self.variant(FUNC_A::HyperbolicSine)
    }
    ///Arctanh function
    #[inline(always)]
    pub fn arctanh(self) -> &'a mut W {
        self.variant(FUNC_A::Arctanh)
    }
    ///Natural Logarithm function
    #[inline(always)]
    pub fn natural_logarithm(self) -> &'a mut W {
        self.variant(FUNC_A::NaturalLogarithm)
    }
    ///Square Root function
    #[inline(always)]
    pub fn square_root(self) -> &'a mut W {
        self.variant(FUNC_A::SquareRoot)
    }
}
///Field `PRECISION` reader - Precision required (number of iterations/cycles), where PRECISION = (number of iterations/4)
pub type PRECISION_R = crate::FieldReader<u8, PRECISION_A>;
///Precision required (number of iterations/cycles), where PRECISION = (number of iterations/4)
///
///Value on reset: 5
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRECISION_A {
    ///1: 4 iterations
    Iters4 = 1,
    ///2: 8 iterations
    Iters8 = 2,
    ///3: 12 iterations
    Iters12 = 3,
    ///4: 16 iterations
    Iters16 = 4,
    ///5: 20 iterations
    Iters20 = 5,
    ///6: 24 iterations
    Iters24 = 6,
    ///7: 28 iterations
    Iters28 = 7,
    ///8: 32 iterations
    Iters32 = 8,
    ///9: 36 iterations
    Iters36 = 9,
    ///10: 40 iterations
    Iters40 = 10,
    ///11: 44 iterations
    Iters44 = 11,
    ///12: 48 iterations
    Iters48 = 12,
    ///13: 52 iterations
    Iters52 = 13,
    ///14: 56 iterations
    Iters56 = 14,
    ///15: 60 iterations
    Iters60 = 15,
}
impl From<PRECISION_A> for u8 {
    #[inline(always)]
    fn from(variant: PRECISION_A) -> Self {
        variant as _
    }
}
impl PRECISION_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PRECISION_A> {
        match self.bits {
            1 => Some(PRECISION_A::Iters4),
            2 => Some(PRECISION_A::Iters8),
            3 => Some(PRECISION_A::Iters12),
            4 => Some(PRECISION_A::Iters16),
            5 => Some(PRECISION_A::Iters20),
            6 => Some(PRECISION_A::Iters24),
            7 => Some(PRECISION_A::Iters28),
            8 => Some(PRECISION_A::Iters32),
            9 => Some(PRECISION_A::Iters36),
            10 => Some(PRECISION_A::Iters40),
            11 => Some(PRECISION_A::Iters44),
            12 => Some(PRECISION_A::Iters48),
            13 => Some(PRECISION_A::Iters52),
            14 => Some(PRECISION_A::Iters56),
            15 => Some(PRECISION_A::Iters60),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Iters4`
    #[inline(always)]
    pub fn is_iters4(&self) -> bool {
        *self == PRECISION_A::Iters4
    }
    ///Checks if the value of the field is `Iters8`
    #[inline(always)]
    pub fn is_iters8(&self) -> bool {
        *self == PRECISION_A::Iters8
    }
    ///Checks if the value of the field is `Iters12`
    #[inline(always)]
    pub fn is_iters12(&self) -> bool {
        *self == PRECISION_A::Iters12
    }
    ///Checks if the value of the field is `Iters16`
    #[inline(always)]
    pub fn is_iters16(&self) -> bool {
        *self == PRECISION_A::Iters16
    }
    ///Checks if the value of the field is `Iters20`
    #[inline(always)]
    pub fn is_iters20(&self) -> bool {
        *self == PRECISION_A::Iters20
    }
    ///Checks if the value of the field is `Iters24`
    #[inline(always)]
    pub fn is_iters24(&self) -> bool {
        *self == PRECISION_A::Iters24
    }
    ///Checks if the value of the field is `Iters28`
    #[inline(always)]
    pub fn is_iters28(&self) -> bool {
        *self == PRECISION_A::Iters28
    }
    ///Checks if the value of the field is `Iters32`
    #[inline(always)]
    pub fn is_iters32(&self) -> bool {
        *self == PRECISION_A::Iters32
    }
    ///Checks if the value of the field is `Iters36`
    #[inline(always)]
    pub fn is_iters36(&self) -> bool {
        *self == PRECISION_A::Iters36
    }
    ///Checks if the value of the field is `Iters40`
    #[inline(always)]
    pub fn is_iters40(&self) -> bool {
        *self == PRECISION_A::Iters40
    }
    ///Checks if the value of the field is `Iters44`
    #[inline(always)]
    pub fn is_iters44(&self) -> bool {
        *self == PRECISION_A::Iters44
    }
    ///Checks if the value of the field is `Iters48`
    #[inline(always)]
    pub fn is_iters48(&self) -> bool {
        *self == PRECISION_A::Iters48
    }
    ///Checks if the value of the field is `Iters52`
    #[inline(always)]
    pub fn is_iters52(&self) -> bool {
        *self == PRECISION_A::Iters52
    }
    ///Checks if the value of the field is `Iters56`
    #[inline(always)]
    pub fn is_iters56(&self) -> bool {
        *self == PRECISION_A::Iters56
    }
    ///Checks if the value of the field is `Iters60`
    #[inline(always)]
    pub fn is_iters60(&self) -> bool {
        *self == PRECISION_A::Iters60
    }
}
///Field `PRECISION` writer - Precision required (number of iterations/cycles), where PRECISION = (number of iterations/4)
pub type PRECISION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSR_SPEC, u8, PRECISION_A, 4, O>;
impl<'a, const O: u8> PRECISION_W<'a, O> {
    ///4 iterations
    #[inline(always)]
    pub fn iters4(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters4)
    }
    ///8 iterations
    #[inline(always)]
    pub fn iters8(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters8)
    }
    ///12 iterations
    #[inline(always)]
    pub fn iters12(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters12)
    }
    ///16 iterations
    #[inline(always)]
    pub fn iters16(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters16)
    }
    ///20 iterations
    #[inline(always)]
    pub fn iters20(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters20)
    }
    ///24 iterations
    #[inline(always)]
    pub fn iters24(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters24)
    }
    ///28 iterations
    #[inline(always)]
    pub fn iters28(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters28)
    }
    ///32 iterations
    #[inline(always)]
    pub fn iters32(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters32)
    }
    ///36 iterations
    #[inline(always)]
    pub fn iters36(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters36)
    }
    ///40 iterations
    #[inline(always)]
    pub fn iters40(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters40)
    }
    ///44 iterations
    #[inline(always)]
    pub fn iters44(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters44)
    }
    ///48 iterations
    #[inline(always)]
    pub fn iters48(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters48)
    }
    ///52 iterations
    #[inline(always)]
    pub fn iters52(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters52)
    }
    ///56 iterations
    #[inline(always)]
    pub fn iters56(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters56)
    }
    ///60 iterations
    #[inline(always)]
    pub fn iters60(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters60)
    }
}
///Field `SCALE` reader - Scaling factor (2^-n for arguments, 2^n for results)
pub type SCALE_R = crate::FieldReader<u8, u8>;
///Field `SCALE` writer - Scaling factor (2^-n for arguments, 2^n for results)
pub type SCALE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CSR_SPEC, u8, u8, 3, O>;
///Field `IEN` reader - Enable interrupt
pub type IEN_R = crate::BitReader<IEN_A>;
///Enable interrupt
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN_A {
    ///0: Disable interrupt request generation
    Disabled = 0,
    ///1: Enable interrupt request generation
    Enabled = 1,
}
impl From<IEN_A> for bool {
    #[inline(always)]
    fn from(variant: IEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IEN_A {
        match self.bits {
            false => IEN_A::Disabled,
            true => IEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IEN_A::Enabled
    }
}
///Field `IEN` writer - Enable interrupt
pub type IEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, IEN_A, O>;
impl<'a, const O: u8> IEN_W<'a, O> {
    ///Disable interrupt request generation
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IEN_A::Disabled)
    }
    ///Enable interrupt request generation
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IEN_A::Enabled)
    }
}
///Field `DMAREN` reader - Enable DMA wread channel
pub type DMAREN_R = crate::BitReader<DMAREN_A>;
///Enable DMA wread channel
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAREN_A {
    ///0: No DMA channel reads are generated
    Disabled = 0,
    ///1: Read requests are generated on the DMA channel when RRDY flag is set
    Enabled = 1,
}
impl From<DMAREN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAREN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAREN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAREN_A {
        match self.bits {
            false => DMAREN_A::Disabled,
            true => DMAREN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAREN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAREN_A::Enabled
    }
}
///Field `DMAREN` writer - Enable DMA wread channel
pub type DMAREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, DMAREN_A, O>;
impl<'a, const O: u8> DMAREN_W<'a, O> {
    ///No DMA channel reads are generated
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAREN_A::Disabled)
    }
    ///Read requests are generated on the DMA channel when RRDY flag is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAREN_A::Enabled)
    }
}
///Field `DMAWEN` reader - Enable DMA write channel
pub type DMAWEN_R = crate::BitReader<DMAWEN_A>;
///Enable DMA write channel
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAWEN_A {
    ///0: No DMA channel writes are generated
    Disabled = 0,
    ///1: Write requests are generated on the DMA channel when no operation is pending
    Enabled = 1,
}
impl From<DMAWEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAWEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAWEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAWEN_A {
        match self.bits {
            false => DMAWEN_A::Disabled,
            true => DMAWEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAWEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAWEN_A::Enabled
    }
}
///Field `DMAWEN` writer - Enable DMA write channel
pub type DMAWEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, DMAWEN_A, O>;
impl<'a, const O: u8> DMAWEN_W<'a, O> {
    ///No DMA channel writes are generated
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAWEN_A::Disabled)
    }
    ///Write requests are generated on the DMA channel when no operation is pending
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAWEN_A::Enabled)
    }
}
///Field `NRES` reader - Number of results in the RDATA register
pub type NRES_R = crate::BitReader<NRES_A>;
///Number of results in the RDATA register
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NRES_A {
    ///0: Only single result value will be returned. After a single read RRDY will be automatically cleared
    Num1 = 0,
    ///1: Two return reads need to be performed. After two reads RRDY will be automatically cleared
    Num2 = 1,
}
impl From<NRES_A> for bool {
    #[inline(always)]
    fn from(variant: NRES_A) -> Self {
        variant as u8 != 0
    }
}
impl NRES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NRES_A {
        match self.bits {
            false => NRES_A::Num1,
            true => NRES_A::Num2,
        }
    }
    ///Checks if the value of the field is `Num1`
    #[inline(always)]
    pub fn is_num1(&self) -> bool {
        *self == NRES_A::Num1
    }
    ///Checks if the value of the field is `Num2`
    #[inline(always)]
    pub fn is_num2(&self) -> bool {
        *self == NRES_A::Num2
    }
}
///Field `NRES` writer - Number of results in the RDATA register
pub type NRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, NRES_A, O>;
impl<'a, const O: u8> NRES_W<'a, O> {
    ///Only single result value will be returned. After a single read RRDY will be automatically cleared
    #[inline(always)]
    pub fn num1(self) -> &'a mut W {
        self.variant(NRES_A::Num1)
    }
    ///Two return reads need to be performed. After two reads RRDY will be automatically cleared
    #[inline(always)]
    pub fn num2(self) -> &'a mut W {
        self.variant(NRES_A::Num2)
    }
}
///Field `NARGS` reader - Number of arguments expected by the WDATA register
pub type NARGS_R = crate::BitReader<NARGS_A>;
///Number of arguments expected by the WDATA register
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NARGS_A {
    ///0: Only single argument write is needed for next calculation
    Num1 = 0,
    ///1: Two argument writes need to be performed for next calculation
    Num2 = 1,
}
impl From<NARGS_A> for bool {
    #[inline(always)]
    fn from(variant: NARGS_A) -> Self {
        variant as u8 != 0
    }
}
impl NARGS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NARGS_A {
        match self.bits {
            false => NARGS_A::Num1,
            true => NARGS_A::Num2,
        }
    }
    ///Checks if the value of the field is `Num1`
    #[inline(always)]
    pub fn is_num1(&self) -> bool {
        *self == NARGS_A::Num1
    }
    ///Checks if the value of the field is `Num2`
    #[inline(always)]
    pub fn is_num2(&self) -> bool {
        *self == NARGS_A::Num2
    }
}
///Field `NARGS` writer - Number of arguments expected by the WDATA register
pub type NARGS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, NARGS_A, O>;
impl<'a, const O: u8> NARGS_W<'a, O> {
    ///Only single argument write is needed for next calculation
    #[inline(always)]
    pub fn num1(self) -> &'a mut W {
        self.variant(NARGS_A::Num1)
    }
    ///Two argument writes need to be performed for next calculation
    #[inline(always)]
    pub fn num2(self) -> &'a mut W {
        self.variant(NARGS_A::Num2)
    }
}
///Field `RESSIZE` reader - Width of output data
pub type RESSIZE_R = crate::BitReader<RESSIZE_A>;
///Width of output data
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESSIZE_A {
    ///0: Use 32 bit output values
    Bits32 = 0,
    ///1: Use 16 bit output values
    Bits16 = 1,
}
impl From<RESSIZE_A> for bool {
    #[inline(always)]
    fn from(variant: RESSIZE_A) -> Self {
        variant as u8 != 0
    }
}
impl RESSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RESSIZE_A {
        match self.bits {
            false => RESSIZE_A::Bits32,
            true => RESSIZE_A::Bits16,
        }
    }
    ///Checks if the value of the field is `Bits32`
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == RESSIZE_A::Bits32
    }
    ///Checks if the value of the field is `Bits16`
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == RESSIZE_A::Bits16
    }
}
///Field `RESSIZE` writer - Width of output data
pub type RESSIZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, RESSIZE_A, O>;
impl<'a, const O: u8> RESSIZE_W<'a, O> {
    ///Use 32 bit output values
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(RESSIZE_A::Bits32)
    }
    ///Use 16 bit output values
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(RESSIZE_A::Bits16)
    }
}
///Field `ARGSIZE` reader - Width of input data
pub type ARGSIZE_R = crate::BitReader<ARGSIZE_A>;
///Width of input data
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARGSIZE_A {
    ///0: Use 32 bit input values
    Bits32 = 0,
    ///1: Use 16 bit input values
    Bits16 = 1,
}
impl From<ARGSIZE_A> for bool {
    #[inline(always)]
    fn from(variant: ARGSIZE_A) -> Self {
        variant as u8 != 0
    }
}
impl ARGSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ARGSIZE_A {
        match self.bits {
            false => ARGSIZE_A::Bits32,
            true => ARGSIZE_A::Bits16,
        }
    }
    ///Checks if the value of the field is `Bits32`
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == ARGSIZE_A::Bits32
    }
    ///Checks if the value of the field is `Bits16`
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == ARGSIZE_A::Bits16
    }
}
///Field `ARGSIZE` writer - Width of input data
pub type ARGSIZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, ARGSIZE_A, O>;
impl<'a, const O: u8> ARGSIZE_W<'a, O> {
    ///Use 32 bit input values
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(ARGSIZE_A::Bits32)
    }
    ///Use 16 bit input values
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(ARGSIZE_A::Bits16)
    }
}
///Field `RRDY` reader - Result ready flag
pub type RRDY_R = crate::BitReader<RRDYR_A>;
///Result ready flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRDYR_A {
    ///0: Results from computation are not read
    NotReady = 0,
    ///1: Results are ready, this flag will be automatically cleared once value is read
    Ready = 1,
}
impl From<RRDYR_A> for bool {
    #[inline(always)]
    fn from(variant: RRDYR_A) -> Self {
        variant as u8 != 0
    }
}
impl RRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RRDYR_A {
        match self.bits {
            false => RRDYR_A::NotReady,
            true => RRDYR_A::Ready,
        }
    }
    ///Checks if the value of the field is `NotReady`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == RRDYR_A::NotReady
    }
    ///Checks if the value of the field is `Ready`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == RRDYR_A::Ready
    }
}
///Field `RRDY` writer - Result ready flag
pub type RRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, RRDYR_A, O>;
impl<'a, const O: u8> RRDY_W<'a, O> {
    ///Results from computation are not read
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(RRDYR_A::NotReady)
    }
    ///Results are ready, this flag will be automatically cleared once value is read
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(RRDYR_A::Ready)
    }
}
impl R {
    ///Bits 0:3 - Function
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Precision required (number of iterations/cycles), where PRECISION = (number of iterations/4)
    #[inline(always)]
    pub fn precision(&self) -> PRECISION_R {
        PRECISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:10 - Scaling factor (2^-n for arguments, 2^n for results)
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 16 - Enable interrupt
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Enable DMA wread channel
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Enable DMA write channel
    #[inline(always)]
    pub fn dmawen(&self) -> DMAWEN_R {
        DMAWEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Number of results in the RDATA register
    #[inline(always)]
    pub fn nres(&self) -> NRES_R {
        NRES_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Number of arguments expected by the WDATA register
    #[inline(always)]
    pub fn nargs(&self) -> NARGS_R {
        NARGS_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Width of output data
    #[inline(always)]
    pub fn ressize(&self) -> RESSIZE_R {
        RESSIZE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Width of input data
    #[inline(always)]
    pub fn argsize(&self) -> ARGSIZE_R {
        ARGSIZE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 31 - Result ready flag
    #[inline(always)]
    pub fn rrdy(&self) -> RRDY_R {
        RRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - Function
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FUNC_W<0> {
        FUNC_W::new(self)
    }
    ///Bits 4:7 - Precision required (number of iterations/cycles), where PRECISION = (number of iterations/4)
    #[inline(always)]
    #[must_use]
    pub fn precision(&mut self) -> PRECISION_W<4> {
        PRECISION_W::new(self)
    }
    ///Bits 8:10 - Scaling factor (2^-n for arguments, 2^n for results)
    #[inline(always)]
    #[must_use]
    pub fn scale(&mut self) -> SCALE_W<8> {
        SCALE_W::new(self)
    }
    ///Bit 16 - Enable interrupt
    #[inline(always)]
    #[must_use]
    pub fn ien(&mut self) -> IEN_W<16> {
        IEN_W::new(self)
    }
    ///Bit 17 - Enable DMA wread channel
    #[inline(always)]
    #[must_use]
    pub fn dmaren(&mut self) -> DMAREN_W<17> {
        DMAREN_W::new(self)
    }
    ///Bit 18 - Enable DMA write channel
    #[inline(always)]
    #[must_use]
    pub fn dmawen(&mut self) -> DMAWEN_W<18> {
        DMAWEN_W::new(self)
    }
    ///Bit 19 - Number of results in the RDATA register
    #[inline(always)]
    #[must_use]
    pub fn nres(&mut self) -> NRES_W<19> {
        NRES_W::new(self)
    }
    ///Bit 20 - Number of arguments expected by the WDATA register
    #[inline(always)]
    #[must_use]
    pub fn nargs(&mut self) -> NARGS_W<20> {
        NARGS_W::new(self)
    }
    ///Bit 21 - Width of output data
    #[inline(always)]
    #[must_use]
    pub fn ressize(&mut self) -> RESSIZE_W<21> {
        RESSIZE_W::new(self)
    }
    ///Bit 22 - Width of input data
    #[inline(always)]
    #[must_use]
    pub fn argsize(&mut self) -> ARGSIZE_W<22> {
        ARGSIZE_W::new(self)
    }
    ///Bit 31 - Result ready flag
    #[inline(always)]
    #[must_use]
    pub fn rrdy(&mut self) -> RRDY_W<31> {
        RRDY_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Control and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr](index.html) module
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr::R](R) reader structure
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csr::W](W) writer structure
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSR to value 0x50
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x50;
}
