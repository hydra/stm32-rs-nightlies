///Register `TDH2R` reader
pub struct R(crate::R<TDH2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TDH2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TDH2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TDH2R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TDH2R` writer
pub struct W(crate::W<TDH2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TDH2R_SPEC>;
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
impl From<crate::W<TDH2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TDH2R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DATA4` reader - DATA4
pub type DATA4_R = crate::FieldReader<u8, u8>;
///Field `DATA4` writer - DATA4
pub type DATA4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TDH2R_SPEC, u8, u8, 8, O>;
///Field `DATA5` reader - DATA5
pub type DATA5_R = crate::FieldReader<u8, u8>;
///Field `DATA5` writer - DATA5
pub type DATA5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TDH2R_SPEC, u8, u8, 8, O>;
///Field `DATA6` reader - DATA6
pub type DATA6_R = crate::FieldReader<u8, u8>;
///Field `DATA6` writer - DATA6
pub type DATA6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TDH2R_SPEC, u8, u8, 8, O>;
///Field `DATA7` reader - DATA7
pub type DATA7_R = crate::FieldReader<u8, u8>;
///Field `DATA7` writer - DATA7
pub type DATA7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TDH2R_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - DATA4
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - DATA5
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - DATA6
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - DATA7
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - DATA4
    #[inline(always)]
    #[must_use]
    pub fn data4(&mut self) -> DATA4_W<0> {
        DATA4_W::new(self)
    }
    ///Bits 8:15 - DATA5
    #[inline(always)]
    #[must_use]
    pub fn data5(&mut self) -> DATA5_W<8> {
        DATA5_W::new(self)
    }
    ///Bits 16:23 - DATA6
    #[inline(always)]
    #[must_use]
    pub fn data6(&mut self) -> DATA6_W<16> {
        DATA6_W::new(self)
    }
    ///Bits 24:31 - DATA7
    #[inline(always)]
    #[must_use]
    pub fn data7(&mut self) -> DATA7_W<24> {
        DATA7_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///mailbox data high register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tdh2r](index.html) module
pub struct TDH2R_SPEC;
impl crate::RegisterSpec for TDH2R_SPEC {
    type Ux = u32;
}
///`read()` method returns [tdh2r::R](R) reader structure
impl crate::Readable for TDH2R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tdh2r::W](W) writer structure
impl crate::Writable for TDH2R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TDH2R to value 0
impl crate::Resettable for TDH2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
