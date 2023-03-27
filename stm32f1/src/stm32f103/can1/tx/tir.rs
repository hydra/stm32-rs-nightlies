///Register `TIR` reader
pub struct R(crate::R<TIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIR` writer
pub struct W(crate::W<TIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIR_SPEC>;
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
impl From<crate::W<TIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TXRQ` reader - TXRQ
pub type TXRQ_R = crate::BitReader<bool>;
///Field `TXRQ` writer - TXRQ
pub type TXRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIR_SPEC, bool, O>;
///Field `RTR` reader - RTR
pub type RTR_R = crate::BitReader<RTR_A>;
///RTR
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTR_A {
    ///0: Data frame
    Data = 0,
    ///1: Remote frame
    Remote = 1,
}
impl From<RTR_A> for bool {
    #[inline(always)]
    fn from(variant: RTR_A) -> Self {
        variant as u8 != 0
    }
}
impl RTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RTR_A {
        match self.bits {
            false => RTR_A::Data,
            true => RTR_A::Remote,
        }
    }
    ///Checks if the value of the field is `Data`
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == RTR_A::Data
    }
    ///Checks if the value of the field is `Remote`
    #[inline(always)]
    pub fn is_remote(&self) -> bool {
        *self == RTR_A::Remote
    }
}
///Field `RTR` writer - RTR
pub type RTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIR_SPEC, RTR_A, O>;
impl<'a, const O: u8> RTR_W<'a, O> {
    ///Data frame
    #[inline(always)]
    pub fn data(self) -> &'a mut W {
        self.variant(RTR_A::Data)
    }
    ///Remote frame
    #[inline(always)]
    pub fn remote(self) -> &'a mut W {
        self.variant(RTR_A::Remote)
    }
}
///Field `IDE` reader - IDE
pub type IDE_R = crate::BitReader<IDE_A>;
///IDE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDE_A {
    ///0: Standard identifier
    Standard = 0,
    ///1: Extended identifier
    Extended = 1,
}
impl From<IDE_A> for bool {
    #[inline(always)]
    fn from(variant: IDE_A) -> Self {
        variant as u8 != 0
    }
}
impl IDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IDE_A {
        match self.bits {
            false => IDE_A::Standard,
            true => IDE_A::Extended,
        }
    }
    ///Checks if the value of the field is `Standard`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == IDE_A::Standard
    }
    ///Checks if the value of the field is `Extended`
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == IDE_A::Extended
    }
}
///Field `IDE` writer - IDE
pub type IDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIR_SPEC, IDE_A, O>;
impl<'a, const O: u8> IDE_W<'a, O> {
    ///Standard identifier
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(IDE_A::Standard)
    }
    ///Extended identifier
    #[inline(always)]
    pub fn extended(self) -> &'a mut W {
        self.variant(IDE_A::Extended)
    }
}
///Field `EXID` reader - EXID
pub type EXID_R = crate::FieldReader<u32, u32>;
///Field `EXID` writer - EXID
pub type EXID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIR_SPEC, u32, u32, 18, O>;
///Field `STID` reader - STID
pub type STID_R = crate::FieldReader<u16, u16>;
///Field `STID` writer - STID
pub type STID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIR_SPEC, u16, u16, 11, O>;
impl R {
    ///Bit 0 - TXRQ
    #[inline(always)]
    pub fn txrq(&self) -> TXRQ_R {
        TXRQ_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RTR
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IDE
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:20 - EXID
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new((self.bits >> 3) & 0x0003_ffff)
    }
    ///Bits 21:31 - STID
    #[inline(always)]
    pub fn stid(&self) -> STID_R {
        STID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    ///Bit 0 - TXRQ
    #[inline(always)]
    #[must_use]
    pub fn txrq(&mut self) -> TXRQ_W<0> {
        TXRQ_W::new(self)
    }
    ///Bit 1 - RTR
    #[inline(always)]
    #[must_use]
    pub fn rtr(&mut self) -> RTR_W<1> {
        RTR_W::new(self)
    }
    ///Bit 2 - IDE
    #[inline(always)]
    #[must_use]
    pub fn ide(&mut self) -> IDE_W<2> {
        IDE_W::new(self)
    }
    ///Bits 3:20 - EXID
    #[inline(always)]
    #[must_use]
    pub fn exid(&mut self) -> EXID_W<3> {
        EXID_W::new(self)
    }
    ///Bits 21:31 - STID
    #[inline(always)]
    #[must_use]
    pub fn stid(&mut self) -> STID_W<21> {
        STID_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CAN_TI0R
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tir](index.html) module
pub struct TIR_SPEC;
impl crate::RegisterSpec for TIR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tir::R](R) reader structure
impl crate::Readable for TIR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tir::W](W) writer structure
impl crate::Writable for TIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIR to value 0
impl crate::Resettable for TIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
