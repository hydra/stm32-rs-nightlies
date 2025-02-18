///Register `CNT` reader
pub struct R(crate::R<CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CNT` writer
pub struct W(crate::W<CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNT_SPEC>;
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
impl From<crate::W<CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CNT_bit0` reader - CNT
pub type CNT_BIT0_R = crate::FieldReader<u16, u16>;
///Field `CNT_bit0` writer - CNT
pub type CNT_BIT0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNT_SPEC, u16, u16, 16, O>;
///Field `UIFCPY` reader - UIFCPY or Res
pub type UIFCPY_R = crate::BitReader<bool>;
///Field `UIFCPY` writer - UIFCPY or Res
pub type UIFCPY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNT_SPEC, bool, O>;
impl R {
    ///Bits 0:15 - CNT
    #[inline(always)]
    pub fn cnt_bit0(&self) -> CNT_BIT0_R {
        CNT_BIT0_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 31 - UIFCPY or Res
    #[inline(always)]
    pub fn uifcpy(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:15 - CNT
    #[inline(always)]
    #[must_use]
    pub fn cnt_bit0(&mut self) -> CNT_BIT0_W<0> {
        CNT_BIT0_W::new(self)
    }
    ///Bit 31 - UIFCPY or Res
    #[inline(always)]
    #[must_use]
    pub fn uifcpy(&mut self) -> UIFCPY_W<31> {
        UIFCPY_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///counter
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cnt](index.html) module
pub struct CNT_SPEC;
impl crate::RegisterSpec for CNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [cnt::R](R) reader structure
impl crate::Readable for CNT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cnt::W](W) writer structure
impl crate::Writable for CNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CNT to value 0
impl crate::Resettable for CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
