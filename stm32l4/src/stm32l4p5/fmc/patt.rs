///Register `PATT` reader
pub struct R(crate::R<PATT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PATT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PATT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PATT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PATT` writer
pub struct W(crate::W<PATT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PATT_SPEC>;
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
impl From<crate::W<PATT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PATT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ATTSET` reader - ATTSETx
pub type ATTSET_R = crate::FieldReader<u8, u8>;
///Field `ATTSET` writer - ATTSETx
pub type ATTSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PATT_SPEC, u8, u8, 8, O>;
///Field `ATTWAIT` reader - ATTWAITx
pub type ATTWAIT_R = crate::FieldReader<u8, u8>;
///Field `ATTWAIT` writer - ATTWAITx
pub type ATTWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PATT_SPEC, u8, u8, 8, O>;
///Field `ATTHOLD` reader - ATTHOLDx
pub type ATTHOLD_R = crate::FieldReader<u8, u8>;
///Field `ATTHOLD` writer - ATTHOLDx
pub type ATTHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PATT_SPEC, u8, u8, 8, O>;
///Field `ATTHIZ` reader - ATTHIZx
pub type ATTHIZ_R = crate::FieldReader<u8, u8>;
///Field `ATTHIZ` writer - ATTHIZx
pub type ATTHIZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PATT_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - ATTSETx
    #[inline(always)]
    pub fn attset(&self) -> ATTSET_R {
        ATTSET_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - ATTWAITx
    #[inline(always)]
    pub fn attwait(&self) -> ATTWAIT_R {
        ATTWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - ATTHOLDx
    #[inline(always)]
    pub fn atthold(&self) -> ATTHOLD_R {
        ATTHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - ATTHIZx
    #[inline(always)]
    pub fn atthiz(&self) -> ATTHIZ_R {
        ATTHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - ATTSETx
    #[inline(always)]
    #[must_use]
    pub fn attset(&mut self) -> ATTSET_W<0> {
        ATTSET_W::new(self)
    }
    ///Bits 8:15 - ATTWAITx
    #[inline(always)]
    #[must_use]
    pub fn attwait(&mut self) -> ATTWAIT_W<8> {
        ATTWAIT_W::new(self)
    }
    ///Bits 16:23 - ATTHOLDx
    #[inline(always)]
    #[must_use]
    pub fn atthold(&mut self) -> ATTHOLD_W<16> {
        ATTHOLD_W::new(self)
    }
    ///Bits 24:31 - ATTHIZx
    #[inline(always)]
    #[must_use]
    pub fn atthiz(&mut self) -> ATTHIZ_W<24> {
        ATTHIZ_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Attribute memory space timing register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [patt](index.html) module
pub struct PATT_SPEC;
impl crate::RegisterSpec for PATT_SPEC {
    type Ux = u32;
}
///`read()` method returns [patt::R](R) reader structure
impl crate::Readable for PATT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [patt::W](W) writer structure
impl crate::Writable for PATT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PATT to value 0xfcfc_fcfc
impl crate::Resettable for PATT_SPEC {
    const RESET_VALUE: Self::Ux = 0xfcfc_fcfc;
}
