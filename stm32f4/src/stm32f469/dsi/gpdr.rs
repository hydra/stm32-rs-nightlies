///Register `GPDR` reader
pub struct R(crate::R<GPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GPDR` writer
pub struct W(crate::W<GPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPDR_SPEC>;
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
impl From<crate::W<GPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DATA1` reader - Payload Byte 1
pub type DATA1_R = crate::FieldReader<u8, u8>;
///Field `DATA1` writer - Payload Byte 1
pub type DATA1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPDR_SPEC, u8, u8, 8, O>;
///Field `DATA2` reader - Payload Byte 2
pub type DATA2_R = crate::FieldReader<u8, u8>;
///Field `DATA2` writer - Payload Byte 2
pub type DATA2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPDR_SPEC, u8, u8, 8, O>;
///Field `DATA3` reader - Payload Byte 3
pub type DATA3_R = crate::FieldReader<u8, u8>;
///Field `DATA3` writer - Payload Byte 3
pub type DATA3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPDR_SPEC, u8, u8, 8, O>;
///Field `DATA4` reader - Payload Byte 4
pub type DATA4_R = crate::FieldReader<u8, u8>;
///Field `DATA4` writer - Payload Byte 4
pub type DATA4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPDR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - Payload Byte 1
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Payload Byte 2
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Payload Byte 3
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Payload Byte 4
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Payload Byte 1
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> DATA1_W<0> {
        DATA1_W::new(self)
    }
    ///Bits 8:15 - Payload Byte 2
    #[inline(always)]
    #[must_use]
    pub fn data2(&mut self) -> DATA2_W<8> {
        DATA2_W::new(self)
    }
    ///Bits 16:23 - Payload Byte 3
    #[inline(always)]
    #[must_use]
    pub fn data3(&mut self) -> DATA3_W<16> {
        DATA3_W::new(self)
    }
    ///Bits 24:31 - Payload Byte 4
    #[inline(always)]
    #[must_use]
    pub fn data4(&mut self) -> DATA4_W<24> {
        DATA4_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host Generic Payload Data Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gpdr](index.html) module
pub struct GPDR_SPEC;
impl crate::RegisterSpec for GPDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpdr::R](R) reader structure
impl crate::Readable for GPDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gpdr::W](W) writer structure
impl crate::Writable for GPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GPDR to value 0
impl crate::Resettable for GPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
