///Register `AIM` reader
pub struct R(crate::R<AIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AIM_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AIM` writer
pub struct W(crate::W<AIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AIM_SPEC>;
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
impl From<crate::W<AIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AIM_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OVRUDRIE` reader - Overrun/underrun interrupt enable
pub type OVRUDRIE_R = crate::BitReader<bool>;
///Field `OVRUDRIE` writer - Overrun/underrun interrupt enable
pub type OVRUDRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AIM_SPEC, bool, O>;
///Field `MUTEDETIE` reader - Mute detection interrupt enable
pub type MUTEDETIE_R = crate::BitReader<bool>;
///Field `MUTEDETIE` writer - Mute detection interrupt enable
pub type MUTEDETIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AIM_SPEC, bool, O>;
///Field `WCKCFGIE` reader - Wrong clock configuration interrupt enable
pub type WCKCFGIE_R = crate::BitReader<bool>;
///Field `WCKCFGIE` writer - Wrong clock configuration interrupt enable
pub type WCKCFGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AIM_SPEC, bool, O>;
///Field `FREQIE` reader - FIFO request interrupt enable
pub type FREQIE_R = crate::BitReader<bool>;
///Field `FREQIE` writer - FIFO request interrupt enable
pub type FREQIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AIM_SPEC, bool, O>;
///Field `CNRDYIE` reader - Codec not ready interrupt enable
pub type CNRDYIE_R = crate::BitReader<bool>;
///Field `CNRDYIE` writer - Codec not ready interrupt enable
pub type CNRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AIM_SPEC, bool, O>;
///Field `AFSDETIE` reader - Anticipated frame synchronization detection interrupt enable
pub type AFSDETIE_R = crate::BitReader<bool>;
///Field `AFSDETIE` writer - Anticipated frame synchronization detection interrupt enable
pub type AFSDETIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AIM_SPEC, bool, O>;
///Field `LFSDETIE` reader - Late frame synchronization detection interrupt enable
pub type LFSDETIE_R = crate::BitReader<bool>;
///Field `LFSDETIE` writer - Late frame synchronization detection interrupt enable
pub type LFSDETIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AIM_SPEC, bool, O>;
impl R {
    ///Bit 0 - Overrun/underrun interrupt enable
    #[inline(always)]
    pub fn ovrudrie(&self) -> OVRUDRIE_R {
        OVRUDRIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Mute detection interrupt enable
    #[inline(always)]
    pub fn mutedetie(&self) -> MUTEDETIE_R {
        MUTEDETIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wrong clock configuration interrupt enable
    #[inline(always)]
    pub fn wckcfgie(&self) -> WCKCFGIE_R {
        WCKCFGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - FIFO request interrupt enable
    #[inline(always)]
    pub fn freqie(&self) -> FREQIE_R {
        FREQIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Codec not ready interrupt enable
    #[inline(always)]
    pub fn cnrdyie(&self) -> CNRDYIE_R {
        CNRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Anticipated frame synchronization detection interrupt enable
    #[inline(always)]
    pub fn afsdetie(&self) -> AFSDETIE_R {
        AFSDETIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Late frame synchronization detection interrupt enable
    #[inline(always)]
    pub fn lfsdetie(&self) -> LFSDETIE_R {
        LFSDETIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Overrun/underrun interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn ovrudrie(&mut self) -> OVRUDRIE_W<0> {
        OVRUDRIE_W::new(self)
    }
    ///Bit 1 - Mute detection interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn mutedetie(&mut self) -> MUTEDETIE_W<1> {
        MUTEDETIE_W::new(self)
    }
    ///Bit 2 - Wrong clock configuration interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn wckcfgie(&mut self) -> WCKCFGIE_W<2> {
        WCKCFGIE_W::new(self)
    }
    ///Bit 3 - FIFO request interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn freqie(&mut self) -> FREQIE_W<3> {
        FREQIE_W::new(self)
    }
    ///Bit 4 - Codec not ready interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn cnrdyie(&mut self) -> CNRDYIE_W<4> {
        CNRDYIE_W::new(self)
    }
    ///Bit 5 - Anticipated frame synchronization detection interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn afsdetie(&mut self) -> AFSDETIE_W<5> {
        AFSDETIE_W::new(self)
    }
    ///Bit 6 - Late frame synchronization detection interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn lfsdetie(&mut self) -> LFSDETIE_W<6> {
        LFSDETIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///A Interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [aim](index.html) module
pub struct AIM_SPEC;
impl crate::RegisterSpec for AIM_SPEC {
    type Ux = u32;
}
///`read()` method returns [aim::R](R) reader structure
impl crate::Readable for AIM_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [aim::W](W) writer structure
impl crate::Writable for AIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AIM to value 0
impl crate::Resettable for AIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
