///Register `SHIFTR` writer
pub struct W(crate::W<SHIFTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTR_SPEC>;
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
impl From<crate::W<SHIFTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SUBFS` writer - Subtract a fraction of a second
pub type SUBFS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SHIFTR_SPEC, u16, u16, 15, O>;
///Add one second
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADD1SW_AW {
    ///1: Add one second to the clock/calendar
    Add1 = 1,
}
impl From<ADD1SW_AW> for bool {
    #[inline(always)]
    fn from(variant: ADD1SW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADD1S` writer - Add one second
pub type ADD1S_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHIFTR_SPEC, ADD1SW_AW, O>;
impl<'a, const O: u8> ADD1S_W<'a, O> {
    ///Add one second to the clock/calendar
    #[inline(always)]
    pub fn add1(self) -> &'a mut W {
        self.variant(ADD1SW_AW::Add1)
    }
}
impl W {
    ///Bits 0:14 - Subtract a fraction of a second
    #[inline(always)]
    #[must_use]
    pub fn subfs(&mut self) -> SUBFS_W<0> {
        SUBFS_W::new(self)
    }
    ///Bit 31 - Add one second
    #[inline(always)]
    #[must_use]
    pub fn add1s(&mut self) -> ADD1S_W<31> {
        ADD1S_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Shift control register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [shiftr](index.html) module
pub struct SHIFTR_SPEC;
impl crate::RegisterSpec for SHIFTR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [shiftr::W](W) writer structure
impl crate::Writable for SHIFTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SHIFTR to value 0
impl crate::Resettable for SHIFTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
