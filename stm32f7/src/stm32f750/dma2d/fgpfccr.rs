///Register `FGPFCCR` reader
pub struct R(crate::R<FGPFCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGPFCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGPFCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGPFCCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FGPFCCR` writer
pub struct W(crate::W<FGPFCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGPFCCR_SPEC>;
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
impl From<crate::W<FGPFCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGPFCCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CM` reader - Color mode
pub type CM_R = crate::FieldReader<u8, CM_A>;
///Color mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CM_A {
    ///0: Color mode ARGB8888
    Argb8888 = 0,
    ///1: Color mode RGB888
    Rgb888 = 1,
    ///2: Color mode RGB565
    Rgb565 = 2,
    ///3: Color mode ARGB1555
    Argb1555 = 3,
    ///4: Color mode ARGB4444
    Argb4444 = 4,
    ///5: Color mode L8
    L8 = 5,
    ///6: Color mode AL44
    Al44 = 6,
    ///7: Color mode AL88
    Al88 = 7,
    ///8: Color mode L4
    L4 = 8,
    ///9: Color mode A8
    A8 = 9,
    ///10: Color mode A4
    A4 = 10,
}
impl From<CM_A> for u8 {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as _
    }
}
impl CM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CM_A> {
        match self.bits {
            0 => Some(CM_A::Argb8888),
            1 => Some(CM_A::Rgb888),
            2 => Some(CM_A::Rgb565),
            3 => Some(CM_A::Argb1555),
            4 => Some(CM_A::Argb4444),
            5 => Some(CM_A::L8),
            6 => Some(CM_A::Al44),
            7 => Some(CM_A::Al88),
            8 => Some(CM_A::L4),
            9 => Some(CM_A::A8),
            10 => Some(CM_A::A4),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Argb8888`
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == CM_A::Argb8888
    }
    ///Checks if the value of the field is `Rgb888`
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == CM_A::Rgb888
    }
    ///Checks if the value of the field is `Rgb565`
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        *self == CM_A::Rgb565
    }
    ///Checks if the value of the field is `Argb1555`
    #[inline(always)]
    pub fn is_argb1555(&self) -> bool {
        *self == CM_A::Argb1555
    }
    ///Checks if the value of the field is `Argb4444`
    #[inline(always)]
    pub fn is_argb4444(&self) -> bool {
        *self == CM_A::Argb4444
    }
    ///Checks if the value of the field is `L8`
    #[inline(always)]
    pub fn is_l8(&self) -> bool {
        *self == CM_A::L8
    }
    ///Checks if the value of the field is `Al44`
    #[inline(always)]
    pub fn is_al44(&self) -> bool {
        *self == CM_A::Al44
    }
    ///Checks if the value of the field is `Al88`
    #[inline(always)]
    pub fn is_al88(&self) -> bool {
        *self == CM_A::Al88
    }
    ///Checks if the value of the field is `L4`
    #[inline(always)]
    pub fn is_l4(&self) -> bool {
        *self == CM_A::L4
    }
    ///Checks if the value of the field is `A8`
    #[inline(always)]
    pub fn is_a8(&self) -> bool {
        *self == CM_A::A8
    }
    ///Checks if the value of the field is `A4`
    #[inline(always)]
    pub fn is_a4(&self) -> bool {
        *self == CM_A::A4
    }
}
///Field `CM` writer - Color mode
pub type CM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGPFCCR_SPEC, u8, CM_A, 4, O>;
impl<'a, const O: u8> CM_W<'a, O> {
    ///Color mode ARGB8888
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut W {
        self.variant(CM_A::Argb8888)
    }
    ///Color mode RGB888
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(CM_A::Rgb888)
    }
    ///Color mode RGB565
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut W {
        self.variant(CM_A::Rgb565)
    }
    ///Color mode ARGB1555
    #[inline(always)]
    pub fn argb1555(self) -> &'a mut W {
        self.variant(CM_A::Argb1555)
    }
    ///Color mode ARGB4444
    #[inline(always)]
    pub fn argb4444(self) -> &'a mut W {
        self.variant(CM_A::Argb4444)
    }
    ///Color mode L8
    #[inline(always)]
    pub fn l8(self) -> &'a mut W {
        self.variant(CM_A::L8)
    }
    ///Color mode AL44
    #[inline(always)]
    pub fn al44(self) -> &'a mut W {
        self.variant(CM_A::Al44)
    }
    ///Color mode AL88
    #[inline(always)]
    pub fn al88(self) -> &'a mut W {
        self.variant(CM_A::Al88)
    }
    ///Color mode L4
    #[inline(always)]
    pub fn l4(self) -> &'a mut W {
        self.variant(CM_A::L4)
    }
    ///Color mode A8
    #[inline(always)]
    pub fn a8(self) -> &'a mut W {
        self.variant(CM_A::A8)
    }
    ///Color mode A4
    #[inline(always)]
    pub fn a4(self) -> &'a mut W {
        self.variant(CM_A::A4)
    }
}
///Field `CCM` reader - CLUT color mode
pub type CCM_R = crate::BitReader<CCM_A>;
///CLUT color mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCM_A {
    ///0: CLUT color format ARGB8888
    Argb8888 = 0,
    ///1: CLUT color format RGB888
    Rgb888 = 1,
}
impl From<CCM_A> for bool {
    #[inline(always)]
    fn from(variant: CCM_A) -> Self {
        variant as u8 != 0
    }
}
impl CCM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CCM_A {
        match self.bits {
            false => CCM_A::Argb8888,
            true => CCM_A::Rgb888,
        }
    }
    ///Checks if the value of the field is `Argb8888`
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == CCM_A::Argb8888
    }
    ///Checks if the value of the field is `Rgb888`
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == CCM_A::Rgb888
    }
}
///Field `CCM` writer - CLUT color mode
pub type CCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FGPFCCR_SPEC, CCM_A, O>;
impl<'a, const O: u8> CCM_W<'a, O> {
    ///CLUT color format ARGB8888
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut W {
        self.variant(CCM_A::Argb8888)
    }
    ///CLUT color format RGB888
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(CCM_A::Rgb888)
    }
}
///Field `START` reader - Start
pub type START_R = crate::BitReader<START_A>;
///Start
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_A {
    ///1: Start the automatic loading of the CLUT
    Start = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
impl START_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<START_A> {
        match self.bits {
            true => Some(START_A::Start),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Start`
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START_A::Start
    }
}
///Field `START` writer - Start
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, FGPFCCR_SPEC, START_A, O>;
impl<'a, const O: u8> START_W<'a, O> {
    ///Start the automatic loading of the CLUT
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(START_A::Start)
    }
}
///Field `CS` reader - CLUT size
pub type CS_R = crate::FieldReader<u8, u8>;
///Field `CS` writer - CLUT size
pub type CS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FGPFCCR_SPEC, u8, u8, 8, O>;
///Field `AM` reader - Alpha mode
pub type AM_R = crate::FieldReader<u8, AM_A>;
///Alpha mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AM_A {
    ///0: No modification of alpha channel
    NoModify = 0,
    ///1: Replace with value in ALPHA\[7:0\]
    Replace = 1,
    ///2: Multiply with value in ALPHA\[7:0\]
    Multiply = 2,
}
impl From<AM_A> for u8 {
    #[inline(always)]
    fn from(variant: AM_A) -> Self {
        variant as _
    }
}
impl AM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<AM_A> {
        match self.bits {
            0 => Some(AM_A::NoModify),
            1 => Some(AM_A::Replace),
            2 => Some(AM_A::Multiply),
            _ => None,
        }
    }
    ///Checks if the value of the field is `NoModify`
    #[inline(always)]
    pub fn is_no_modify(&self) -> bool {
        *self == AM_A::NoModify
    }
    ///Checks if the value of the field is `Replace`
    #[inline(always)]
    pub fn is_replace(&self) -> bool {
        *self == AM_A::Replace
    }
    ///Checks if the value of the field is `Multiply`
    #[inline(always)]
    pub fn is_multiply(&self) -> bool {
        *self == AM_A::Multiply
    }
}
///Field `AM` writer - Alpha mode
pub type AM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGPFCCR_SPEC, u8, AM_A, 2, O>;
impl<'a, const O: u8> AM_W<'a, O> {
    ///No modification of alpha channel
    #[inline(always)]
    pub fn no_modify(self) -> &'a mut W {
        self.variant(AM_A::NoModify)
    }
    ///Replace with value in ALPHA\[7:0\]
    #[inline(always)]
    pub fn replace(self) -> &'a mut W {
        self.variant(AM_A::Replace)
    }
    ///Multiply with value in ALPHA\[7:0\]
    #[inline(always)]
    pub fn multiply(self) -> &'a mut W {
        self.variant(AM_A::Multiply)
    }
}
///Field `ALPHA` reader - Alpha value
pub type ALPHA_R = crate::FieldReader<u8, u8>;
///Field `ALPHA` writer - Alpha value
pub type ALPHA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FGPFCCR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:3 - Color mode
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - CLUT color mode
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Start
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:15 - CLUT size
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:17 - Alpha mode
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 24:31 - Alpha value
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:3 - Color mode
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<0> {
        CM_W::new(self)
    }
    ///Bit 4 - CLUT color mode
    #[inline(always)]
    #[must_use]
    pub fn ccm(&mut self) -> CCM_W<4> {
        CCM_W::new(self)
    }
    ///Bit 5 - Start
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<5> {
        START_W::new(self)
    }
    ///Bits 8:15 - CLUT size
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CS_W<8> {
        CS_W::new(self)
    }
    ///Bits 16:17 - Alpha mode
    #[inline(always)]
    #[must_use]
    pub fn am(&mut self) -> AM_W<16> {
        AM_W::new(self)
    }
    ///Bits 24:31 - Alpha value
    #[inline(always)]
    #[must_use]
    pub fn alpha(&mut self) -> ALPHA_W<24> {
        ALPHA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///foreground PFC control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fgpfccr](index.html) module
pub struct FGPFCCR_SPEC;
impl crate::RegisterSpec for FGPFCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [fgpfccr::R](R) reader structure
impl crate::Readable for FGPFCCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fgpfccr::W](W) writer structure
impl crate::Writable for FGPFCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FGPFCCR to value 0
impl crate::Resettable for FGPFCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
