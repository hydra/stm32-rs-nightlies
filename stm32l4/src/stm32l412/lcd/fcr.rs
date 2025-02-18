///Register `FCR` reader
pub struct R(crate::R<FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FCR` writer
pub struct W(crate::W<FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR_SPEC>;
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
impl From<crate::W<FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HD` reader - High drive enable
pub type HD_R = crate::BitReader<bool>;
///Field `HD` writer - High drive enable
pub type HD_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
///Field `SOFIE` reader - Start of frame interrupt enable
pub type SOFIE_R = crate::BitReader<bool>;
///Field `SOFIE` writer - Start of frame interrupt enable
pub type SOFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
///Field `UDDIE` reader - Update display done interrupt enable
pub type UDDIE_R = crate::BitReader<bool>;
///Field `UDDIE` writer - Update display done interrupt enable
pub type UDDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
///Field `PON` reader - Pulse ON duration
pub type PON_R = crate::FieldReader<u8, u8>;
///Field `PON` writer - Pulse ON duration
pub type PON_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCR_SPEC, u8, u8, 3, O>;
///Field `DEAD` reader - Dead time duration
pub type DEAD_R = crate::FieldReader<u8, u8>;
///Field `DEAD` writer - Dead time duration
pub type DEAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCR_SPEC, u8, u8, 3, O>;
///Field `CC` reader - Contrast control
pub type CC_R = crate::FieldReader<u8, u8>;
///Field `CC` writer - Contrast control
pub type CC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCR_SPEC, u8, u8, 3, O>;
///Field `BLINKF` reader - Blink frequency selection
pub type BLINKF_R = crate::FieldReader<u8, u8>;
///Field `BLINKF` writer - Blink frequency selection
pub type BLINKF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCR_SPEC, u8, u8, 3, O>;
///Field `BLINK` reader - Blink mode selection
pub type BLINK_R = crate::FieldReader<u8, u8>;
///Field `BLINK` writer - Blink mode selection
pub type BLINK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCR_SPEC, u8, u8, 2, O>;
///Field `DIV` reader - DIV clock divider
pub type DIV_R = crate::FieldReader<u8, u8>;
///Field `DIV` writer - DIV clock divider
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCR_SPEC, u8, u8, 4, O>;
///Field `PS` reader - PS 16-bit prescaler
pub type PS_R = crate::FieldReader<u8, u8>;
///Field `PS` writer - PS 16-bit prescaler
pub type PS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCR_SPEC, u8, u8, 4, O>;
impl R {
    ///Bit 0 - High drive enable
    #[inline(always)]
    pub fn hd(&self) -> HD_R {
        HD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Start of frame interrupt enable
    #[inline(always)]
    pub fn sofie(&self) -> SOFIE_R {
        SOFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Update display done interrupt enable
    #[inline(always)]
    pub fn uddie(&self) -> UDDIE_R {
        UDDIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Pulse ON duration
    #[inline(always)]
    pub fn pon(&self) -> PON_R {
        PON_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 7:9 - Dead time duration
    #[inline(always)]
    pub fn dead(&self) -> DEAD_R {
        DEAD_R::new(((self.bits >> 7) & 7) as u8)
    }
    ///Bits 10:12 - Contrast control
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 10) & 7) as u8)
    }
    ///Bits 13:15 - Blink frequency selection
    #[inline(always)]
    pub fn blinkf(&self) -> BLINKF_R {
        BLINKF_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 16:17 - Blink mode selection
    #[inline(always)]
    pub fn blink(&self) -> BLINK_R {
        BLINK_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:21 - DIV clock divider
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bits 22:25 - PS 16-bit prescaler
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - High drive enable
    #[inline(always)]
    #[must_use]
    pub fn hd(&mut self) -> HD_W<0> {
        HD_W::new(self)
    }
    ///Bit 1 - Start of frame interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn sofie(&mut self) -> SOFIE_W<1> {
        SOFIE_W::new(self)
    }
    ///Bit 3 - Update display done interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn uddie(&mut self) -> UDDIE_W<3> {
        UDDIE_W::new(self)
    }
    ///Bits 4:6 - Pulse ON duration
    #[inline(always)]
    #[must_use]
    pub fn pon(&mut self) -> PON_W<4> {
        PON_W::new(self)
    }
    ///Bits 7:9 - Dead time duration
    #[inline(always)]
    #[must_use]
    pub fn dead(&mut self) -> DEAD_W<7> {
        DEAD_W::new(self)
    }
    ///Bits 10:12 - Contrast control
    #[inline(always)]
    #[must_use]
    pub fn cc(&mut self) -> CC_W<10> {
        CC_W::new(self)
    }
    ///Bits 13:15 - Blink frequency selection
    #[inline(always)]
    #[must_use]
    pub fn blinkf(&mut self) -> BLINKF_W<13> {
        BLINKF_W::new(self)
    }
    ///Bits 16:17 - Blink mode selection
    #[inline(always)]
    #[must_use]
    pub fn blink(&mut self) -> BLINK_W<16> {
        BLINK_W::new(self)
    }
    ///Bits 18:21 - DIV clock divider
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<18> {
        DIV_W::new(self)
    }
    ///Bits 22:25 - PS 16-bit prescaler
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<22> {
        PS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///frame control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fcr](index.html) module
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [fcr::R](R) reader structure
impl crate::Readable for FCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fcr::W](W) writer structure
impl crate::Writable for FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FCR to value 0
impl crate::Resettable for FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
