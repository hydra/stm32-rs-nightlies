///Register `MACA3HR` reader
pub struct R(crate::R<MACA3HR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACA3HR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACA3HR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACA3HR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACA3HR` writer
pub struct W(crate::W<MACA3HR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACA3HR_SPEC>;
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
impl From<crate::W<MACA3HR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACA3HR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MACA3H` reader - MACA3H
pub type MACA3H_R = crate::FieldReader<u16, u16>;
///Field `MACA3H` writer - MACA3H
pub type MACA3H_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MACA3HR_SPEC, u16, u16, 16, O>;
///Field `MBC` reader - MBC
pub type MBC_R = crate::FieldReader<u8, u8>;
///Field `MBC` writer - MBC
pub type MBC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MACA3HR_SPEC, u8, u8, 6, O>;
///Field `SA` reader - SA
pub type SA_R = crate::BitReader<SA_A>;
///SA
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SA_A {
    ///0: This address is used for comparison with DA fields of the received frame
    Destination = 0,
    ///1: This address is used for comparison with SA fields of received frames
    Source = 1,
}
impl From<SA_A> for bool {
    #[inline(always)]
    fn from(variant: SA_A) -> Self {
        variant as u8 != 0
    }
}
impl SA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SA_A {
        match self.bits {
            false => SA_A::Destination,
            true => SA_A::Source,
        }
    }
    ///Checks if the value of the field is `Destination`
    #[inline(always)]
    pub fn is_destination(&self) -> bool {
        *self == SA_A::Destination
    }
    ///Checks if the value of the field is `Source`
    #[inline(always)]
    pub fn is_source(&self) -> bool {
        *self == SA_A::Source
    }
}
///Field `SA` writer - SA
pub type SA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACA3HR_SPEC, SA_A, O>;
impl<'a, const O: u8> SA_W<'a, O> {
    ///This address is used for comparison with DA fields of the received frame
    #[inline(always)]
    pub fn destination(self) -> &'a mut W {
        self.variant(SA_A::Destination)
    }
    ///This address is used for comparison with SA fields of received frames
    #[inline(always)]
    pub fn source(self) -> &'a mut W {
        self.variant(SA_A::Source)
    }
}
///Field `AE` reader - AE
pub type AE_R = crate::BitReader<AE_A>;
///AE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AE_A {
    ///0: Address filters ignore this address
    Disabled = 0,
    ///1: Address filters use this address
    Enabled = 1,
}
impl From<AE_A> for bool {
    #[inline(always)]
    fn from(variant: AE_A) -> Self {
        variant as u8 != 0
    }
}
impl AE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AE_A {
        match self.bits {
            false => AE_A::Disabled,
            true => AE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AE_A::Enabled
    }
}
///Field `AE` writer - AE
pub type AE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACA3HR_SPEC, AE_A, O>;
impl<'a, const O: u8> AE_W<'a, O> {
    ///Address filters ignore this address
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AE_A::Disabled)
    }
    ///Address filters use this address
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AE_A::Enabled)
    }
}
impl R {
    ///Bits 0:15 - MACA3H
    #[inline(always)]
    pub fn maca3h(&self) -> MACA3H_R {
        MACA3H_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 24:29 - MBC
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    ///Bit 30 - SA
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - AE
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:15 - MACA3H
    #[inline(always)]
    #[must_use]
    pub fn maca3h(&mut self) -> MACA3H_W<0> {
        MACA3H_W::new(self)
    }
    ///Bits 24:29 - MBC
    #[inline(always)]
    #[must_use]
    pub fn mbc(&mut self) -> MBC_W<24> {
        MBC_W::new(self)
    }
    ///Bit 30 - SA
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<30> {
        SA_W::new(self)
    }
    ///Bit 31 - AE
    #[inline(always)]
    #[must_use]
    pub fn ae(&mut self) -> AE_W<31> {
        AE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet MAC address 3 high register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [maca3hr](index.html) module
pub struct MACA3HR_SPEC;
impl crate::RegisterSpec for MACA3HR_SPEC {
    type Ux = u32;
}
///`read()` method returns [maca3hr::R](R) reader structure
impl crate::Readable for MACA3HR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [maca3hr::W](W) writer structure
impl crate::Writable for MACA3HR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACA3HR to value 0xffff
impl crate::Resettable for MACA3HR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
