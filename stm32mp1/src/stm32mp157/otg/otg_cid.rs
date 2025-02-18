///Register `OTG_CID` reader
pub struct R(crate::R<OTG_CID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_CID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_CID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_CID_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OTG_CID` writer
pub struct W(crate::W<OTG_CID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_CID_SPEC>;
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
impl From<crate::W<OTG_CID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_CID_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PRODUCT_ID` reader - PRODUCT_ID
pub type PRODUCT_ID_R = crate::FieldReader<u32, u32>;
///Field `PRODUCT_ID` writer - PRODUCT_ID
pub type PRODUCT_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_CID_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - PRODUCT_ID
    #[inline(always)]
    pub fn product_id(&self) -> PRODUCT_ID_R {
        PRODUCT_ID_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - PRODUCT_ID
    #[inline(always)]
    #[must_use]
    pub fn product_id(&mut self) -> PRODUCT_ID_W<0> {
        PRODUCT_ID_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This is a register containing the Product ID as reset value.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_cid](index.html) module
pub struct OTG_CID_SPEC;
impl crate::RegisterSpec for OTG_CID_SPEC {
    type Ux = u32;
}
///`read()` method returns [otg_cid::R](R) reader structure
impl crate::Readable for OTG_CID_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [otg_cid::W](W) writer structure
impl crate::Writable for OTG_CID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OTG_CID to value 0x4000
impl crate::Resettable for OTG_CID_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000;
}
