///Register `MACVHTR` reader
pub struct R(crate::R<MACVHTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACVHTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACVHTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACVHTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACVHTR` writer
pub struct W(crate::W<MACVHTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACVHTR_SPEC>;
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
impl From<crate::W<MACVHTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACVHTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `VLHT` reader - VLAN Hash Table This field contains the 16-bit VLAN Hash Table.
pub type VLHT_R = crate::FieldReader<u16, u16>;
///Field `VLHT` writer - VLAN Hash Table This field contains the 16-bit VLAN Hash Table.
pub type VLHT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACVHTR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - VLAN Hash Table This field contains the 16-bit VLAN Hash Table.
    #[inline(always)]
    pub fn vlht(&self) -> VLHT_R {
        VLHT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - VLAN Hash Table This field contains the 16-bit VLAN Hash Table.
    #[inline(always)]
    #[must_use]
    pub fn vlht(&mut self) -> VLHT_W<0> {
        VLHT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///VLAN Hash table register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macvhtr](index.html) module
pub struct MACVHTR_SPEC;
impl crate::RegisterSpec for MACVHTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macvhtr::R](R) reader structure
impl crate::Readable for MACVHTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macvhtr::W](W) writer structure
impl crate::Writable for MACVHTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACVHTR to value 0
impl crate::Resettable for MACVHTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
