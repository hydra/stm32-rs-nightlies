///Register `WRP2BR` reader
pub struct R(crate::R<WRP2BR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRP2BR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRP2BR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRP2BR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `WRP2BR` writer
pub struct W(crate::W<WRP2BR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRP2BR_SPEC>;
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
impl From<crate::W<WRP2BR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRP2BR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WRP2B_STRT` reader - Bank 2 WRP second area B start offset
pub type WRP2B_STRT_R = crate::FieldReader<u8, u8>;
///Field `WRP2B_STRT` writer - Bank 2 WRP second area B start offset
pub type WRP2B_STRT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRP2BR_SPEC, u8, u8, 8, O>;
///Field `WRP2B_END` reader - Bank 2 WRP second area B end offset
pub type WRP2B_END_R = crate::FieldReader<u8, u8>;
///Field `WRP2B_END` writer - Bank 2 WRP second area B end offset
pub type WRP2B_END_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRP2BR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - Bank 2 WRP second area B start offset
    #[inline(always)]
    pub fn wrp2b_strt(&self) -> WRP2B_STRT_R {
        WRP2B_STRT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - Bank 2 WRP second area B end offset
    #[inline(always)]
    pub fn wrp2b_end(&self) -> WRP2B_END_R {
        WRP2B_END_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Bank 2 WRP second area B start offset
    #[inline(always)]
    #[must_use]
    pub fn wrp2b_strt(&mut self) -> WRP2B_STRT_W<0> {
        WRP2B_STRT_W::new(self)
    }
    ///Bits 16:23 - Bank 2 WRP second area B end offset
    #[inline(always)]
    #[must_use]
    pub fn wrp2b_end(&mut self) -> WRP2B_END_W<16> {
        WRP2B_END_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flash Bank 2 WRP area B address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wrp2br](index.html) module
pub struct WRP2BR_SPEC;
impl crate::RegisterSpec for WRP2BR_SPEC {
    type Ux = u32;
}
///`read()` method returns [wrp2br::R](R) reader structure
impl crate::Readable for WRP2BR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [wrp2br::W](W) writer structure
impl crate::Writable for WRP2BR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets WRP2BR to value 0xff00_ff00
impl crate::Resettable for WRP2BR_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00_ff00;
}
