///Register `TBR` reader
pub struct R(crate::R<TBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TBR` writer
pub struct W(crate::W<TBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBR_SPEC>;
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
impl From<crate::W<TBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TSEL` reader - Trigger selection
pub type TSEL_R = crate::FieldReader<u8, u8>;
///Field `TSEL` writer - Trigger selection
pub type TSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBR_SPEC, u8, u8, 6, O>;
///Field `SBUS` reader - Source BUS select This bit is protected and can be written only if EN is 0.
pub type SBUS_R = crate::BitReader<bool>;
///Field `SBUS` writer - Source BUS select This bit is protected and can be written only if EN is 0.
pub type SBUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBR_SPEC, bool, O>;
///Field `DBUS` reader - Destination BUS slect This bit is protected and can be written only if EN is 0.
pub type DBUS_R = crate::BitReader<bool>;
///Field `DBUS` writer - Destination BUS slect This bit is protected and can be written only if EN is 0.
pub type DBUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBR_SPEC, bool, O>;
impl R {
    ///Bits 0:5 - Trigger selection
    #[inline(always)]
    pub fn tsel(&self) -> TSEL_R {
        TSEL_R::new((self.bits & 0x3f) as u8)
    }
    ///Bit 16 - Source BUS select This bit is protected and can be written only if EN is 0.
    #[inline(always)]
    pub fn sbus(&self) -> SBUS_R {
        SBUS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Destination BUS slect This bit is protected and can be written only if EN is 0.
    #[inline(always)]
    pub fn dbus(&self) -> DBUS_R {
        DBUS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    ///Bits 0:5 - Trigger selection
    #[inline(always)]
    #[must_use]
    pub fn tsel(&mut self) -> TSEL_W<0> {
        TSEL_W::new(self)
    }
    ///Bit 16 - Source BUS select This bit is protected and can be written only if EN is 0.
    #[inline(always)]
    #[must_use]
    pub fn sbus(&mut self) -> SBUS_W<16> {
        SBUS_W::new(self)
    }
    ///Bit 17 - Destination BUS slect This bit is protected and can be written only if EN is 0.
    #[inline(always)]
    #[must_use]
    pub fn dbus(&mut self) -> DBUS_W<17> {
        DBUS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MDMA channel x Trigger and Bus selection Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tbr](index.html) module
pub struct TBR_SPEC;
impl crate::RegisterSpec for TBR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tbr::R](R) reader structure
impl crate::Readable for TBR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tbr::W](W) writer structure
impl crate::Writable for TBR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TBR to value 0
impl crate::Resettable for TBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
