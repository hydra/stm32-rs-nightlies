///Register `CFGR` reader
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR` writer
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SEL` reader - Select the phase for the Output clock
pub type SEL_R = crate::FieldReader<u8, u8>;
///Field `SEL` writer - Select the phase for the Output clock
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 4, O>;
///Field `UNIT` reader - Delay Defines the delay of a Unit delay cell
pub type UNIT_R = crate::FieldReader<u8, u8>;
///Field `UNIT` writer - Delay Defines the delay of a Unit delay cell
pub type UNIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 7, O>;
///Field `LNG` reader - Delay line length value
pub type LNG_R = crate::FieldReader<u16, u16>;
///Field `LNG` writer - Delay line length value
pub type LNG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u16, u16, 12, O>;
///Field `LNGF` reader - Length valid flag
pub type LNGF_R = crate::BitReader<bool>;
///Field `LNGF` writer - Length valid flag
pub type LNGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
impl R {
    ///Bits 0:3 - Select the phase for the Output clock
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:14 - Delay Defines the delay of a Unit delay cell
    #[inline(always)]
    pub fn unit(&self) -> UNIT_R {
        UNIT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bits 16:27 - Delay line length value
    #[inline(always)]
    pub fn lng(&self) -> LNG_R {
        LNG_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bit 31 - Length valid flag
    #[inline(always)]
    pub fn lngf(&self) -> LNGF_R {
        LNGF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - Select the phase for the Output clock
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<0> {
        SEL_W::new(self)
    }
    ///Bits 8:14 - Delay Defines the delay of a Unit delay cell
    #[inline(always)]
    #[must_use]
    pub fn unit(&mut self) -> UNIT_W<8> {
        UNIT_W::new(self)
    }
    ///Bits 16:27 - Delay line length value
    #[inline(always)]
    #[must_use]
    pub fn lng(&mut self) -> LNG_W<16> {
        LNG_W::new(self)
    }
    ///Bit 31 - Length valid flag
    #[inline(always)]
    #[must_use]
    pub fn lngf(&mut self) -> LNGF_W<31> {
        LNGF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DLYB configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr](index.html) module
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr::R](R) reader structure
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr::W](W) writer structure
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
