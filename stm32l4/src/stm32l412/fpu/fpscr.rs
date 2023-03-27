///Register `FPSCR` reader
pub struct R(crate::R<FPSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPSCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FPSCR` writer
pub struct W(crate::W<FPSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPSCR_SPEC>;
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
impl From<crate::W<FPSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPSCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IOC` reader - Invalid operation cumulative exception bit
pub type IOC_R = crate::BitReader<bool>;
///Field `IOC` writer - Invalid operation cumulative exception bit
pub type IOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
///Field `DZC` reader - Division by zero cumulative exception bit.
pub type DZC_R = crate::BitReader<bool>;
///Field `DZC` writer - Division by zero cumulative exception bit.
pub type DZC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
///Field `OFC` reader - Overflow cumulative exception bit
pub type OFC_R = crate::BitReader<bool>;
///Field `OFC` writer - Overflow cumulative exception bit
pub type OFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
///Field `UFC` reader - Underflow cumulative exception bit
pub type UFC_R = crate::BitReader<bool>;
///Field `UFC` writer - Underflow cumulative exception bit
pub type UFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
///Field `IXC` reader - Inexact cumulative exception bit
pub type IXC_R = crate::BitReader<bool>;
///Field `IXC` writer - Inexact cumulative exception bit
pub type IXC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
///Field `IDC` reader - Input denormal cumulative exception bit.
pub type IDC_R = crate::BitReader<bool>;
///Field `IDC` writer - Input denormal cumulative exception bit.
pub type IDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
///Field `RMode` reader - Rounding Mode control field
pub type RMODE_R = crate::FieldReader<u8, u8>;
///Field `RMode` writer - Rounding Mode control field
pub type RMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FPSCR_SPEC, u8, u8, 2, O>;
///Field `FZ` reader - Flush-to-zero mode control bit:
pub type FZ_R = crate::BitReader<bool>;
///Field `FZ` writer - Flush-to-zero mode control bit:
pub type FZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
///Field `DN` reader - Default NaN mode control bit
pub type DN_R = crate::BitReader<bool>;
///Field `DN` writer - Default NaN mode control bit
pub type DN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
///Field `AHP` reader - Alternative half-precision control bit
pub type AHP_R = crate::BitReader<bool>;
///Field `AHP` writer - Alternative half-precision control bit
pub type AHP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
///Field `V` reader - Overflow condition code flag
pub type V_R = crate::BitReader<bool>;
///Field `V` writer - Overflow condition code flag
pub type V_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
///Field `C` reader - Carry condition code flag
pub type C_R = crate::BitReader<bool>;
///Field `C` writer - Carry condition code flag
pub type C_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
///Field `Z` reader - Zero condition code flag
pub type Z_R = crate::BitReader<bool>;
///Field `Z` writer - Zero condition code flag
pub type Z_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
///Field `N` reader - Negative condition code flag
pub type N_R = crate::BitReader<bool>;
///Field `N` writer - Negative condition code flag
pub type N_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Invalid operation cumulative exception bit
    #[inline(always)]
    pub fn ioc(&self) -> IOC_R {
        IOC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Division by zero cumulative exception bit.
    #[inline(always)]
    pub fn dzc(&self) -> DZC_R {
        DZC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Overflow cumulative exception bit
    #[inline(always)]
    pub fn ofc(&self) -> OFC_R {
        OFC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Underflow cumulative exception bit
    #[inline(always)]
    pub fn ufc(&self) -> UFC_R {
        UFC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Inexact cumulative exception bit
    #[inline(always)]
    pub fn ixc(&self) -> IXC_R {
        IXC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Input denormal cumulative exception bit.
    #[inline(always)]
    pub fn idc(&self) -> IDC_R {
        IDC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 22:23 - Rounding Mode control field
    #[inline(always)]
    pub fn rmode(&self) -> RMODE_R {
        RMODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bit 24 - Flush-to-zero mode control bit:
    #[inline(always)]
    pub fn fz(&self) -> FZ_R {
        FZ_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Default NaN mode control bit
    #[inline(always)]
    pub fn dn(&self) -> DN_R {
        DN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Alternative half-precision control bit
    #[inline(always)]
    pub fn ahp(&self) -> AHP_R {
        AHP_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - Overflow condition code flag
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Carry condition code flag
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Zero condition code flag
    #[inline(always)]
    pub fn z(&self) -> Z_R {
        Z_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Negative condition code flag
    #[inline(always)]
    pub fn n(&self) -> N_R {
        N_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Invalid operation cumulative exception bit
    #[inline(always)]
    #[must_use]
    pub fn ioc(&mut self) -> IOC_W<0> {
        IOC_W::new(self)
    }
    ///Bit 1 - Division by zero cumulative exception bit.
    #[inline(always)]
    #[must_use]
    pub fn dzc(&mut self) -> DZC_W<1> {
        DZC_W::new(self)
    }
    ///Bit 2 - Overflow cumulative exception bit
    #[inline(always)]
    #[must_use]
    pub fn ofc(&mut self) -> OFC_W<2> {
        OFC_W::new(self)
    }
    ///Bit 3 - Underflow cumulative exception bit
    #[inline(always)]
    #[must_use]
    pub fn ufc(&mut self) -> UFC_W<3> {
        UFC_W::new(self)
    }
    ///Bit 4 - Inexact cumulative exception bit
    #[inline(always)]
    #[must_use]
    pub fn ixc(&mut self) -> IXC_W<4> {
        IXC_W::new(self)
    }
    ///Bit 7 - Input denormal cumulative exception bit.
    #[inline(always)]
    #[must_use]
    pub fn idc(&mut self) -> IDC_W<7> {
        IDC_W::new(self)
    }
    ///Bits 22:23 - Rounding Mode control field
    #[inline(always)]
    #[must_use]
    pub fn rmode(&mut self) -> RMODE_W<22> {
        RMODE_W::new(self)
    }
    ///Bit 24 - Flush-to-zero mode control bit:
    #[inline(always)]
    #[must_use]
    pub fn fz(&mut self) -> FZ_W<24> {
        FZ_W::new(self)
    }
    ///Bit 25 - Default NaN mode control bit
    #[inline(always)]
    #[must_use]
    pub fn dn(&mut self) -> DN_W<25> {
        DN_W::new(self)
    }
    ///Bit 26 - Alternative half-precision control bit
    #[inline(always)]
    #[must_use]
    pub fn ahp(&mut self) -> AHP_W<26> {
        AHP_W::new(self)
    }
    ///Bit 28 - Overflow condition code flag
    #[inline(always)]
    #[must_use]
    pub fn v(&mut self) -> V_W<28> {
        V_W::new(self)
    }
    ///Bit 29 - Carry condition code flag
    #[inline(always)]
    #[must_use]
    pub fn c(&mut self) -> C_W<29> {
        C_W::new(self)
    }
    ///Bit 30 - Zero condition code flag
    #[inline(always)]
    #[must_use]
    pub fn z(&mut self) -> Z_W<30> {
        Z_W::new(self)
    }
    ///Bit 31 - Negative condition code flag
    #[inline(always)]
    #[must_use]
    pub fn n(&mut self) -> N_W<31> {
        N_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Floating-point status control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fpscr](index.html) module
pub struct FPSCR_SPEC;
impl crate::RegisterSpec for FPSCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [fpscr::R](R) reader structure
impl crate::Readable for FPSCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fpscr::W](W) writer structure
impl crate::Writable for FPSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FPSCR to value 0
impl crate::Resettable for FPSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
