///Register `APB3LPENR` reader
pub struct R(crate::R<APB3LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB3LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB3LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB3LPENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APB3LPENR` writer
pub struct W(crate::W<APB3LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB3LPENR_SPEC>;
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
impl From<crate::W<APB3LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB3LPENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LTDCLPEN` reader - LTDC peripheral clock enable during CSleep mode
pub type LTDCLPEN_R = crate::BitReader<LTDCLPEN_A>;
///LTDC peripheral clock enable during CSleep mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTDCLPEN_A {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<LTDCLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: LTDCLPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LTDCLPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LTDCLPEN_A {
        match self.bits {
            false => LTDCLPEN_A::Disabled,
            true => LTDCLPEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LTDCLPEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LTDCLPEN_A::Enabled
    }
}
///Field `LTDCLPEN` writer - LTDC peripheral clock enable during CSleep mode
pub type LTDCLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3LPENR_SPEC, LTDCLPEN_A, O>;
impl<'a, const O: u8> LTDCLPEN_W<'a, O> {
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LTDCLPEN_A::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LTDCLPEN_A::Enabled)
    }
}
///Field `DSILPEN` reader - DSI Peripheral Clock Enable During CSleep Mode
pub use LTDCLPEN_R as DSILPEN_R;
///Field `WWDG1LPEN` reader - WWDG1 Clock Enable During CSleep Mode
pub use LTDCLPEN_R as WWDG1LPEN_R;
///Field `DSILPEN` writer - DSI Peripheral Clock Enable During CSleep Mode
pub use LTDCLPEN_W as DSILPEN_W;
///Field `WWDG1LPEN` writer - WWDG1 Clock Enable During CSleep Mode
pub use LTDCLPEN_W as WWDG1LPEN_W;
impl R {
    ///Bit 3 - LTDC peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn ltdclpen(&self) -> LTDCLPEN_R {
        LTDCLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - DSI Peripheral Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn dsilpen(&self) -> DSILPEN_R {
        DSILPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - WWDG1 Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn wwdg1lpen(&self) -> WWDG1LPEN_R {
        WWDG1LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 3 - LTDC peripheral clock enable during CSleep mode
    #[inline(always)]
    #[must_use]
    pub fn ltdclpen(&mut self) -> LTDCLPEN_W<3> {
        LTDCLPEN_W::new(self)
    }
    ///Bit 4 - DSI Peripheral Clock Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn dsilpen(&mut self) -> DSILPEN_W<4> {
        DSILPEN_W::new(self)
    }
    ///Bit 6 - WWDG1 Clock Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn wwdg1lpen(&mut self) -> WWDG1LPEN_W<6> {
        WWDG1LPEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC APB3 Sleep Clock Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb3lpenr](index.html) module
pub struct APB3LPENR_SPEC;
impl crate::RegisterSpec for APB3LPENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [apb3lpenr::R](R) reader structure
impl crate::Readable for APB3LPENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apb3lpenr::W](W) writer structure
impl crate::Writable for APB3LPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets APB3LPENR to value 0
impl crate::Resettable for APB3LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
