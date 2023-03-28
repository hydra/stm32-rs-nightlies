///Register `L2PFCR` reader
pub struct R(crate::R<L2PFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2PFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2PFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2PFCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `L2PFCR` writer
pub struct W(crate::W<L2PFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L2PFCR_SPEC>;
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
impl From<crate::W<L2PFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L2PFCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PF` reader - Pixel Format
pub type PF_R = crate::FieldReader<u8, PF_A>;
///Pixel Format
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PF_A {
    ///0: ARGB8888
    Argb8888 = 0,
    ///1: RGB888
    Rgb888 = 1,
    ///2: RGB565
    Rgb565 = 2,
    ///3: ARGB1555
    Argb1555 = 3,
    ///4: ARGB4444
    Argb4444 = 4,
    ///5: L8 (8-bit luminance)
    L8 = 5,
    ///6: AL44 (4-bit alpha, 4-bit luminance)
    Al44 = 6,
    ///7: AL88 (8-bit alpha, 8-bit luminance)
    Al88 = 7,
}
impl From<PF_A> for u8 {
    #[inline(always)]
    fn from(variant: PF_A) -> Self {
        variant as _
    }
}
impl PF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PF_A {
        match self.bits {
            0 => PF_A::Argb8888,
            1 => PF_A::Rgb888,
            2 => PF_A::Rgb565,
            3 => PF_A::Argb1555,
            4 => PF_A::Argb4444,
            5 => PF_A::L8,
            6 => PF_A::Al44,
            7 => PF_A::Al88,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Argb8888`
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == PF_A::Argb8888
    }
    ///Checks if the value of the field is `Rgb888`
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == PF_A::Rgb888
    }
    ///Checks if the value of the field is `Rgb565`
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        *self == PF_A::Rgb565
    }
    ///Checks if the value of the field is `Argb1555`
    #[inline(always)]
    pub fn is_argb1555(&self) -> bool {
        *self == PF_A::Argb1555
    }
    ///Checks if the value of the field is `Argb4444`
    #[inline(always)]
    pub fn is_argb4444(&self) -> bool {
        *self == PF_A::Argb4444
    }
    ///Checks if the value of the field is `L8`
    #[inline(always)]
    pub fn is_l8(&self) -> bool {
        *self == PF_A::L8
    }
    ///Checks if the value of the field is `Al44`
    #[inline(always)]
    pub fn is_al44(&self) -> bool {
        *self == PF_A::Al44
    }
    ///Checks if the value of the field is `Al88`
    #[inline(always)]
    pub fn is_al88(&self) -> bool {
        *self == PF_A::Al88
    }
}
///Field `PF` writer - Pixel Format
pub type PF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, L2PFCR_SPEC, u8, PF_A, 3, O>;
impl<'a, const O: u8> PF_W<'a, O> {
    ///ARGB8888
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut W {
        self.variant(PF_A::Argb8888)
    }
    ///RGB888
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(PF_A::Rgb888)
    }
    ///RGB565
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut W {
        self.variant(PF_A::Rgb565)
    }
    ///ARGB1555
    #[inline(always)]
    pub fn argb1555(self) -> &'a mut W {
        self.variant(PF_A::Argb1555)
    }
    ///ARGB4444
    #[inline(always)]
    pub fn argb4444(self) -> &'a mut W {
        self.variant(PF_A::Argb4444)
    }
    ///L8 (8-bit luminance)
    #[inline(always)]
    pub fn l8(self) -> &'a mut W {
        self.variant(PF_A::L8)
    }
    ///AL44 (4-bit alpha, 4-bit luminance)
    #[inline(always)]
    pub fn al44(self) -> &'a mut W {
        self.variant(PF_A::Al44)
    }
    ///AL88 (8-bit alpha, 8-bit luminance)
    #[inline(always)]
    pub fn al88(self) -> &'a mut W {
        self.variant(PF_A::Al88)
    }
}
impl R {
    ///Bits 0:2 - Pixel Format
    #[inline(always)]
    pub fn pf(&self) -> PF_R {
        PF_R::new((self.bits & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - Pixel Format
    #[inline(always)]
    #[must_use]
    pub fn pf(&mut self) -> PF_W<0> {
        PF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LTDC Layer Pixel Format Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l2pfcr](index.html) module
pub struct L2PFCR_SPEC;
impl crate::RegisterSpec for L2PFCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [l2pfcr::R](R) reader structure
impl crate::Readable for L2PFCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [l2pfcr::W](W) writer structure
impl crate::Writable for L2PFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets L2PFCR to value 0
impl crate::Resettable for L2PFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
