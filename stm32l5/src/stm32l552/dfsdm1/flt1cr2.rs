///Register `FLT1CR2` reader
pub struct R(crate::R<FLT1CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLT1CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLT1CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLT1CR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLT1CR2` writer
pub struct W(crate::W<FLT1CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLT1CR2_SPEC>;
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
impl From<crate::W<FLT1CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLT1CR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `JEOCIE` reader - Injected end of conversion interrupt enable
pub type JEOCIE_R = crate::BitReader<bool>;
///Field `JEOCIE` writer - Injected end of conversion interrupt enable
pub type JEOCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLT1CR2_SPEC, bool, O>;
///Field `REOCIE` reader - Regular end of conversion interrupt enable
pub type REOCIE_R = crate::BitReader<bool>;
///Field `REOCIE` writer - Regular end of conversion interrupt enable
pub type REOCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLT1CR2_SPEC, bool, O>;
///Field `JOVRIE` reader - Injected data overrun interrupt enable
pub type JOVRIE_R = crate::BitReader<bool>;
///Field `JOVRIE` writer - Injected data overrun interrupt enable
pub type JOVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLT1CR2_SPEC, bool, O>;
///Field `ROVRIE` reader - Regular data overrun interrupt enable
pub type ROVRIE_R = crate::BitReader<bool>;
///Field `ROVRIE` writer - Regular data overrun interrupt enable
pub type ROVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLT1CR2_SPEC, bool, O>;
///Field `AWDIE` reader - Analog watchdog interrupt enable
pub type AWDIE_R = crate::BitReader<bool>;
///Field `AWDIE` writer - Analog watchdog interrupt enable
pub type AWDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLT1CR2_SPEC, bool, O>;
///Field `SCDIE` reader - Short-circuit detector interrupt enable
pub type SCDIE_R = crate::BitReader<bool>;
///Field `SCDIE` writer - Short-circuit detector interrupt enable
pub type SCDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLT1CR2_SPEC, bool, O>;
///Field `CKABIE` reader - Clock absence interrupt enable
pub type CKABIE_R = crate::BitReader<bool>;
///Field `CKABIE` writer - Clock absence interrupt enable
pub type CKABIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLT1CR2_SPEC, bool, O>;
///Field `EXCH` reader - Extremes detector channel selection
pub type EXCH_R = crate::FieldReader<u8, u8>;
///Field `EXCH` writer - Extremes detector channel selection
pub type EXCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLT1CR2_SPEC, u8, u8, 8, O>;
///Field `AWDCH` reader - Analog watchdog channel selection
pub type AWDCH_R = crate::FieldReader<u8, u8>;
///Field `AWDCH` writer - Analog watchdog channel selection
pub type AWDCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLT1CR2_SPEC, u8, u8, 8, O>;
impl R {
    ///Bit 0 - Injected end of conversion interrupt enable
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Regular end of conversion interrupt enable
    #[inline(always)]
    pub fn reocie(&self) -> REOCIE_R {
        REOCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Injected data overrun interrupt enable
    #[inline(always)]
    pub fn jovrie(&self) -> JOVRIE_R {
        JOVRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Regular data overrun interrupt enable
    #[inline(always)]
    pub fn rovrie(&self) -> ROVRIE_R {
        ROVRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Analog watchdog interrupt enable
    #[inline(always)]
    pub fn awdie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Short-circuit detector interrupt enable
    #[inline(always)]
    pub fn scdie(&self) -> SCDIE_R {
        SCDIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Clock absence interrupt enable
    #[inline(always)]
    pub fn ckabie(&self) -> CKABIE_R {
        CKABIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 8:15 - Extremes detector channel selection
    #[inline(always)]
    pub fn exch(&self) -> EXCH_R {
        EXCH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Analog watchdog channel selection
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bit 0 - Injected end of conversion interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn jeocie(&mut self) -> JEOCIE_W<0> {
        JEOCIE_W::new(self)
    }
    ///Bit 1 - Regular end of conversion interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn reocie(&mut self) -> REOCIE_W<1> {
        REOCIE_W::new(self)
    }
    ///Bit 2 - Injected data overrun interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn jovrie(&mut self) -> JOVRIE_W<2> {
        JOVRIE_W::new(self)
    }
    ///Bit 3 - Regular data overrun interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rovrie(&mut self) -> ROVRIE_W<3> {
        ROVRIE_W::new(self)
    }
    ///Bit 4 - Analog watchdog interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn awdie(&mut self) -> AWDIE_W<4> {
        AWDIE_W::new(self)
    }
    ///Bit 5 - Short-circuit detector interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn scdie(&mut self) -> SCDIE_W<5> {
        SCDIE_W::new(self)
    }
    ///Bit 6 - Clock absence interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn ckabie(&mut self) -> CKABIE_W<6> {
        CKABIE_W::new(self)
    }
    ///Bits 8:15 - Extremes detector channel selection
    #[inline(always)]
    #[must_use]
    pub fn exch(&mut self) -> EXCH_W<8> {
        EXCH_W::new(self)
    }
    ///Bits 16:23 - Analog watchdog channel selection
    #[inline(always)]
    #[must_use]
    pub fn awdch(&mut self) -> AWDCH_W<16> {
        AWDCH_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flt1cr2](index.html) module
pub struct FLT1CR2_SPEC;
impl crate::RegisterSpec for FLT1CR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [flt1cr2::R](R) reader structure
impl crate::Readable for FLT1CR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [flt1cr2::W](W) writer structure
impl crate::Writable for FLT1CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLT1CR2 to value 0
impl crate::Resettable for FLT1CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
