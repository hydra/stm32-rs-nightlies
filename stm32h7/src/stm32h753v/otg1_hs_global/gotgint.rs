///Register `GOTGINT` reader
pub struct R(crate::R<GOTGINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GOTGINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GOTGINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GOTGINT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GOTGINT` writer
pub struct W(crate::W<GOTGINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GOTGINT_SPEC>;
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
impl From<crate::W<GOTGINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GOTGINT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SEDET` reader - Session end detected
pub type SEDET_R = crate::BitReader<bool>;
///Field `SEDET` writer - Session end detected
pub type SEDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGINT_SPEC, bool, O>;
///Field `SRSSCHG` reader - Session request success status change
pub type SRSSCHG_R = crate::BitReader<bool>;
///Field `SRSSCHG` writer - Session request success status change
pub type SRSSCHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGINT_SPEC, bool, O>;
///Field `HNSSCHG` reader - Host negotiation success status change
pub type HNSSCHG_R = crate::BitReader<bool>;
///Field `HNSSCHG` writer - Host negotiation success status change
pub type HNSSCHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGINT_SPEC, bool, O>;
///Field `HNGDET` reader - Host negotiation detected
pub type HNGDET_R = crate::BitReader<bool>;
///Field `HNGDET` writer - Host negotiation detected
pub type HNGDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGINT_SPEC, bool, O>;
///Field `ADTOCHG` reader - A-device timeout change
pub type ADTOCHG_R = crate::BitReader<bool>;
///Field `ADTOCHG` writer - A-device timeout change
pub type ADTOCHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGINT_SPEC, bool, O>;
///Field `DBCDNE` reader - Debounce done
pub type DBCDNE_R = crate::BitReader<bool>;
///Field `DBCDNE` writer - Debounce done
pub type DBCDNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGINT_SPEC, bool, O>;
///Field `IDCHNG` reader - ID input pin changed
pub type IDCHNG_R = crate::BitReader<bool>;
///Field `IDCHNG` writer - ID input pin changed
pub type IDCHNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGINT_SPEC, bool, O>;
impl R {
    ///Bit 2 - Session end detected
    #[inline(always)]
    pub fn sedet(&self) -> SEDET_R {
        SEDET_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - Session request success status change
    #[inline(always)]
    pub fn srsschg(&self) -> SRSSCHG_R {
        SRSSCHG_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Host negotiation success status change
    #[inline(always)]
    pub fn hnsschg(&self) -> HNSSCHG_R {
        HNSSCHG_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 17 - Host negotiation detected
    #[inline(always)]
    pub fn hngdet(&self) -> HNGDET_R {
        HNGDET_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - A-device timeout change
    #[inline(always)]
    pub fn adtochg(&self) -> ADTOCHG_R {
        ADTOCHG_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Debounce done
    #[inline(always)]
    pub fn dbcdne(&self) -> DBCDNE_R {
        DBCDNE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - ID input pin changed
    #[inline(always)]
    pub fn idchng(&self) -> IDCHNG_R {
        IDCHNG_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - Session end detected
    #[inline(always)]
    #[must_use]
    pub fn sedet(&mut self) -> SEDET_W<2> {
        SEDET_W::new(self)
    }
    ///Bit 8 - Session request success status change
    #[inline(always)]
    #[must_use]
    pub fn srsschg(&mut self) -> SRSSCHG_W<8> {
        SRSSCHG_W::new(self)
    }
    ///Bit 9 - Host negotiation success status change
    #[inline(always)]
    #[must_use]
    pub fn hnsschg(&mut self) -> HNSSCHG_W<9> {
        HNSSCHG_W::new(self)
    }
    ///Bit 17 - Host negotiation detected
    #[inline(always)]
    #[must_use]
    pub fn hngdet(&mut self) -> HNGDET_W<17> {
        HNGDET_W::new(self)
    }
    ///Bit 18 - A-device timeout change
    #[inline(always)]
    #[must_use]
    pub fn adtochg(&mut self) -> ADTOCHG_W<18> {
        ADTOCHG_W::new(self)
    }
    ///Bit 19 - Debounce done
    #[inline(always)]
    #[must_use]
    pub fn dbcdne(&mut self) -> DBCDNE_W<19> {
        DBCDNE_W::new(self)
    }
    ///Bit 20 - ID input pin changed
    #[inline(always)]
    #[must_use]
    pub fn idchng(&mut self) -> IDCHNG_W<20> {
        IDCHNG_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTG_HS interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gotgint](index.html) module
pub struct GOTGINT_SPEC;
impl crate::RegisterSpec for GOTGINT_SPEC {
    type Ux = u32;
}
///`read()` method returns [gotgint::R](R) reader structure
impl crate::Readable for GOTGINT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gotgint::W](W) writer structure
impl crate::Writable for GOTGINT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GOTGINT to value 0
impl crate::Resettable for GOTGINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
