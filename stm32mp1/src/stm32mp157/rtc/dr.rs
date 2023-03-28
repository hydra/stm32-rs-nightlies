///Register `DR` reader
pub struct R(crate::R<DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DR` writer
pub struct W(crate::W<DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR_SPEC>;
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
impl From<crate::W<DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DU` reader - DU
pub type DU_R = crate::FieldReader<u8, u8>;
///Field `DU` writer - DU
pub type DU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR_SPEC, u8, u8, 4, O>;
///Field `DT` reader - DT
pub type DT_R = crate::FieldReader<u8, u8>;
///Field `DT` writer - DT
pub type DT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR_SPEC, u8, u8, 2, O>;
///Field `MU` reader - MU
pub type MU_R = crate::FieldReader<u8, u8>;
///Field `MU` writer - MU
pub type MU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR_SPEC, u8, u8, 4, O>;
///Field `MT` reader - MT
pub type MT_R = crate::BitReader<bool>;
///Field `MT` writer - MT
pub type MT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR_SPEC, bool, O>;
///Field `WDU` reader - WDU
pub type WDU_R = crate::FieldReader<u8, u8>;
///Field `WDU` writer - WDU
pub type WDU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR_SPEC, u8, u8, 3, O>;
///Field `YU` reader - YU
pub type YU_R = crate::FieldReader<u8, u8>;
///Field `YU` writer - YU
pub type YU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR_SPEC, u8, u8, 4, O>;
///Field `YT` reader - YT
pub type YT_R = crate::FieldReader<u8, u8>;
///Field `YT` writer - YT
pub type YT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:3 - DU
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - DT
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:11 - MU
    #[inline(always)]
    pub fn mu(&self) -> MU_R {
        MU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - MT
    #[inline(always)]
    pub fn mt(&self) -> MT_R {
        MT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:15 - WDU
    #[inline(always)]
    pub fn wdu(&self) -> WDU_R {
        WDU_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 16:19 - YU
    #[inline(always)]
    pub fn yu(&self) -> YU_R {
        YU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - YT
    #[inline(always)]
    pub fn yt(&self) -> YT_R {
        YT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - DU
    #[inline(always)]
    #[must_use]
    pub fn du(&mut self) -> DU_W<0> {
        DU_W::new(self)
    }
    ///Bits 4:5 - DT
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<4> {
        DT_W::new(self)
    }
    ///Bits 8:11 - MU
    #[inline(always)]
    #[must_use]
    pub fn mu(&mut self) -> MU_W<8> {
        MU_W::new(self)
    }
    ///Bit 12 - MT
    #[inline(always)]
    #[must_use]
    pub fn mt(&mut self) -> MT_W<12> {
        MT_W::new(self)
    }
    ///Bits 13:15 - WDU
    #[inline(always)]
    #[must_use]
    pub fn wdu(&mut self) -> WDU_W<13> {
        WDU_W::new(self)
    }
    ///Bits 16:19 - YU
    #[inline(always)]
    #[must_use]
    pub fn yu(&mut self) -> YU_W<16> {
        YU_W::new(self)
    }
    ///Bits 20:23 - YT
    #[inline(always)]
    #[must_use]
    pub fn yt(&mut self) -> YT_W<20> {
        YT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dr](index.html) module
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dr::R](R) reader structure
impl crate::Readable for DR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dr::W](W) writer structure
impl crate::Writable for DR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DR to value 0x2101
impl crate::Resettable for DR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2101;
}
