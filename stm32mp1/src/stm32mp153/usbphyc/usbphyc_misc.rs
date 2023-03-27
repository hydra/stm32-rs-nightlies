///Register `USBPHYC_MISC` reader
pub struct R(crate::R<USBPHYC_MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPHYC_MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPHYC_MISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPHYC_MISC_SPEC>) -> Self {
        R(reader)
    }
}
///Register `USBPHYC_MISC` writer
pub struct W(crate::W<USBPHYC_MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPHYC_MISC_SPEC>;
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
impl From<crate::W<USBPHYC_MISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPHYC_MISC_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SWITHOST` reader - SWITHOST
pub type SWITHOST_R = crate::BitReader<bool>;
///Field `SWITHOST` writer - SWITHOST
pub type SWITHOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_MISC_SPEC, bool, O>;
///Field `PPCKDIS` reader - PPCKDIS
pub type PPCKDIS_R = crate::FieldReader<u8, u8>;
///Field `PPCKDIS` writer - PPCKDIS
pub type PPCKDIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USBPHYC_MISC_SPEC, u8, u8, 2, O>;
impl R {
    ///Bit 0 - SWITHOST
    #[inline(always)]
    pub fn swithost(&self) -> SWITHOST_R {
        SWITHOST_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - PPCKDIS
    #[inline(always)]
    pub fn ppckdis(&self) -> PPCKDIS_R {
        PPCKDIS_R::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - SWITHOST
    #[inline(always)]
    #[must_use]
    pub fn swithost(&mut self) -> SWITHOST_W<0> {
        SWITHOST_W::new(self)
    }
    ///Bits 1:2 - PPCKDIS
    #[inline(always)]
    #[must_use]
    pub fn ppckdis(&mut self) -> PPCKDIS_W<1> {
        PPCKDIS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to control the switch between controllers for the HS PHY.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [usbphyc_misc](index.html) module
pub struct USBPHYC_MISC_SPEC;
impl crate::RegisterSpec for USBPHYC_MISC_SPEC {
    type Ux = u32;
}
///`read()` method returns [usbphyc_misc::R](R) reader structure
impl crate::Readable for USBPHYC_MISC_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [usbphyc_misc::W](W) writer structure
impl crate::Writable for USBPHYC_MISC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets USBPHYC_MISC to value 0
impl crate::Resettable for USBPHYC_MISC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
