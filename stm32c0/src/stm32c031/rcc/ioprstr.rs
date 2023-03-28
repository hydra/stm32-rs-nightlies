///Register `IOPRSTR` reader
pub struct R(crate::R<IOPRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOPRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOPRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOPRSTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IOPRSTR` writer
pub struct W(crate::W<IOPRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOPRSTR_SPEC>;
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
impl From<crate::W<IOPRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOPRSTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GPIOARST` reader - I/O port A reset This bit is set and cleared by software.
pub type GPIOARST_R = crate::BitReader<bool>;
///Field `GPIOARST` writer - I/O port A reset This bit is set and cleared by software.
pub type GPIOARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPRSTR_SPEC, bool, O>;
///Field `GPIOBRST` reader - I/O port B reset This bit is set and cleared by software.
pub type GPIOBRST_R = crate::BitReader<bool>;
///Field `GPIOBRST` writer - I/O port B reset This bit is set and cleared by software.
pub type GPIOBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPRSTR_SPEC, bool, O>;
///Field `GPIOCRST` reader - I/O port C reset This bit is set and cleared by software.
pub type GPIOCRST_R = crate::BitReader<bool>;
///Field `GPIOCRST` writer - I/O port C reset This bit is set and cleared by software.
pub type GPIOCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPRSTR_SPEC, bool, O>;
///Field `GPIODRST` reader - I/O port D reset This bit is set and cleared by software.
pub type GPIODRST_R = crate::BitReader<bool>;
///Field `GPIODRST` writer - I/O port D reset This bit is set and cleared by software.
pub type GPIODRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPRSTR_SPEC, bool, O>;
///Field `GPIOFRST` reader - I/O port F reset This bit is set and cleared by software.
pub type GPIOFRST_R = crate::BitReader<bool>;
///Field `GPIOFRST` writer - I/O port F reset This bit is set and cleared by software.
pub type GPIOFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPRSTR_SPEC, bool, O>;
impl R {
    ///Bit 0 - I/O port A reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I/O port B reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I/O port C reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I/O port D reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - I/O port F reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - I/O port A reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GPIOARST_W<0> {
        GPIOARST_W::new(self)
    }
    ///Bit 1 - I/O port B reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<1> {
        GPIOBRST_W::new(self)
    }
    ///Bit 2 - I/O port C reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<2> {
        GPIOCRST_W::new(self)
    }
    ///Bit 3 - I/O port D reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<3> {
        GPIODRST_W::new(self)
    }
    ///Bit 5 - I/O port F reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<5> {
        GPIOFRST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC I/O port reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ioprstr](index.html) module
pub struct IOPRSTR_SPEC;
impl crate::RegisterSpec for IOPRSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ioprstr::R](R) reader structure
impl crate::Readable for IOPRSTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ioprstr::W](W) writer structure
impl crate::Writable for IOPRSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IOPRSTR to value 0
impl crate::Resettable for IOPRSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
