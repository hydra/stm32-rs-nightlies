///Register `MACA3LR` reader
pub struct R(crate::R<MACA3LR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACA3LR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACA3LR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACA3LR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACA3LR` writer
pub struct W(crate::W<MACA3LR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACA3LR_SPEC>;
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
impl From<crate::W<MACA3LR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACA3LR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MACA3L` reader - MAC address3 low
pub type MACA3L_R = crate::FieldReader<u32, u32>;
///Field `MACA3L` writer - MAC address3 low
pub type MACA3L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACA3LR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - MAC address3 low
    #[inline(always)]
    pub fn maca3l(&self) -> MACA3L_R {
        MACA3L_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - MAC address3 low
    #[inline(always)]
    #[must_use]
    pub fn maca3l(&mut self) -> MACA3L_W<0> {
        MACA3L_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet MAC address 3 low register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [maca3lr](index.html) module
pub struct MACA3LR_SPEC;
impl crate::RegisterSpec for MACA3LR_SPEC {
    type Ux = u32;
}
///`read()` method returns [maca3lr::R](R) reader structure
impl crate::Readable for MACA3LR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [maca3lr::W](W) writer structure
impl crate::Writable for MACA3LR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACA3LR to value 0xffff_ffff
impl crate::Resettable for MACA3LR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
