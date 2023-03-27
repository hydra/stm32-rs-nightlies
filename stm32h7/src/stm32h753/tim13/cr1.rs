///Register `CR1` reader
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR1` writer
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CEN` reader - Counter enable
pub type CEN_R = crate::BitReader<CEN_A>;
///Counter enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEN_A {
    ///0: Counter disabled
    Disabled = 0,
    ///1: Counter enabled
    Enabled = 1,
}
impl From<CEN_A> for bool {
    #[inline(always)]
    fn from(variant: CEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CEN_A {
        match self.bits {
            false => CEN_A::Disabled,
            true => CEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CEN_A::Enabled
    }
}
///Field `CEN` writer - Counter enable
pub type CEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, CEN_A, O>;
impl<'a, const O: u8> CEN_W<'a, O> {
    ///Counter disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CEN_A::Disabled)
    }
    ///Counter enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CEN_A::Enabled)
    }
}
///Field `UDIS` reader - Update disable
pub type UDIS_R = crate::BitReader<UDIS_A>;
///Update disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDIS_A {
    ///0: Update event enabled
    Enabled = 0,
    ///1: Update event disabled
    Disabled = 1,
}
impl From<UDIS_A> for bool {
    #[inline(always)]
    fn from(variant: UDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl UDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UDIS_A {
        match self.bits {
            false => UDIS_A::Enabled,
            true => UDIS_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UDIS_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UDIS_A::Disabled
    }
}
///Field `UDIS` writer - Update disable
pub type UDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, UDIS_A, O>;
impl<'a, const O: u8> UDIS_W<'a, O> {
    ///Update event enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UDIS_A::Enabled)
    }
    ///Update event disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UDIS_A::Disabled)
    }
}
///Field `URS` reader - Update request source
pub type URS_R = crate::BitReader<URS_A>;
///Update request source
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum URS_A {
    ///0: Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request
    AnyEvent = 0,
    ///1: Only counter overflow/underflow generates an update interrupt or DMA request
    CounterOnly = 1,
}
impl From<URS_A> for bool {
    #[inline(always)]
    fn from(variant: URS_A) -> Self {
        variant as u8 != 0
    }
}
impl URS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> URS_A {
        match self.bits {
            false => URS_A::AnyEvent,
            true => URS_A::CounterOnly,
        }
    }
    ///Checks if the value of the field is `AnyEvent`
    #[inline(always)]
    pub fn is_any_event(&self) -> bool {
        *self == URS_A::AnyEvent
    }
    ///Checks if the value of the field is `CounterOnly`
    #[inline(always)]
    pub fn is_counter_only(&self) -> bool {
        *self == URS_A::CounterOnly
    }
}
///Field `URS` writer - Update request source
pub type URS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, URS_A, O>;
impl<'a, const O: u8> URS_W<'a, O> {
    ///Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request
    #[inline(always)]
    pub fn any_event(self) -> &'a mut W {
        self.variant(URS_A::AnyEvent)
    }
    ///Only counter overflow/underflow generates an update interrupt or DMA request
    #[inline(always)]
    pub fn counter_only(self) -> &'a mut W {
        self.variant(URS_A::CounterOnly)
    }
}
///Field `OPM` reader - One-pulse mode
pub type OPM_R = crate::BitReader<OPM_A>;
///One-pulse mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPM_A {
    ///0: Counter is not stopped at update event
    Disabled = 0,
    ///1: Counter stops counting at the next update event (clearing the CEN bit)
    Enabled = 1,
}
impl From<OPM_A> for bool {
    #[inline(always)]
    fn from(variant: OPM_A) -> Self {
        variant as u8 != 0
    }
}
impl OPM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OPM_A {
        match self.bits {
            false => OPM_A::Disabled,
            true => OPM_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OPM_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OPM_A::Enabled
    }
}
///Field `OPM` writer - One-pulse mode
pub type OPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, OPM_A, O>;
impl<'a, const O: u8> OPM_W<'a, O> {
    ///Counter is not stopped at update event
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OPM_A::Disabled)
    }
    ///Counter stops counting at the next update event (clearing the CEN bit)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OPM_A::Enabled)
    }
}
///Field `DIR` reader - Direction
pub type DIR_R = crate::BitReader<bool>;
///Field `DIR` writer - Direction
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `CMS` reader - Center-aligned mode selection
pub type CMS_R = crate::FieldReader<u8, u8>;
///Field `CMS` writer - Center-aligned mode selection
pub type CMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 2, O>;
///Field `ARPE` reader - Auto-reload preload enable
pub type ARPE_R = crate::BitReader<ARPE_A>;
///Auto-reload preload enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARPE_A {
    ///0: TIMx_APRR register is not buffered
    Disabled = 0,
    ///1: TIMx_APRR register is buffered
    Enabled = 1,
}
impl From<ARPE_A> for bool {
    #[inline(always)]
    fn from(variant: ARPE_A) -> Self {
        variant as u8 != 0
    }
}
impl ARPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ARPE_A {
        match self.bits {
            false => ARPE_A::Disabled,
            true => ARPE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARPE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARPE_A::Enabled
    }
}
///Field `ARPE` writer - Auto-reload preload enable
pub type ARPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, ARPE_A, O>;
impl<'a, const O: u8> ARPE_W<'a, O> {
    ///TIMx_APRR register is not buffered
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ARPE_A::Disabled)
    }
    ///TIMx_APRR register is buffered
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ARPE_A::Enabled)
    }
}
///Field `CKD` reader - Clock division
pub type CKD_R = crate::FieldReader<u8, CKD_A>;
///Clock division
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKD_A {
    ///0: t_DTS = t_CK_INT
    Div1 = 0,
    ///1: t_DTS = 2 × t_CK_INT
    Div2 = 1,
    ///2: t_DTS = 4 × t_CK_INT
    Div4 = 2,
}
impl From<CKD_A> for u8 {
    #[inline(always)]
    fn from(variant: CKD_A) -> Self {
        variant as _
    }
}
impl CKD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CKD_A> {
        match self.bits {
            0 => Some(CKD_A::Div1),
            1 => Some(CKD_A::Div2),
            2 => Some(CKD_A::Div4),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CKD_A::Div1
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CKD_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CKD_A::Div4
    }
}
///Field `CKD` writer - Clock division
pub type CKD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, CKD_A, 2, O>;
impl<'a, const O: u8> CKD_W<'a, O> {
    ///t_DTS = t_CK_INT
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CKD_A::Div1)
    }
    ///t_DTS = 2 × t_CK_INT
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CKD_A::Div2)
    }
    ///t_DTS = 4 × t_CK_INT
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CKD_A::Div4)
    }
}
///Field `UIFREMAP` reader - UIF status bit remapping
pub type UIFREMAP_R = crate::BitReader<bool>;
///Field `UIFREMAP` writer - UIF status bit remapping
pub type UIFREMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - Counter enable
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Update disable
    #[inline(always)]
    pub fn udis(&self) -> UDIS_R {
        UDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Update request source
    #[inline(always)]
    pub fn urs(&self) -> URS_R {
        URS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - One-pulse mode
    #[inline(always)]
    pub fn opm(&self) -> OPM_R {
        OPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Direction
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - Center-aligned mode selection
    #[inline(always)]
    pub fn cms(&self) -> CMS_R {
        CMS_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - Auto-reload preload enable
    #[inline(always)]
    pub fn arpe(&self) -> ARPE_R {
        ARPE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Clock division
    #[inline(always)]
    pub fn ckd(&self) -> CKD_R {
        CKD_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 11 - UIF status bit remapping
    #[inline(always)]
    pub fn uifremap(&self) -> UIFREMAP_R {
        UIFREMAP_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Counter enable
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<0> {
        CEN_W::new(self)
    }
    ///Bit 1 - Update disable
    #[inline(always)]
    #[must_use]
    pub fn udis(&mut self) -> UDIS_W<1> {
        UDIS_W::new(self)
    }
    ///Bit 2 - Update request source
    #[inline(always)]
    #[must_use]
    pub fn urs(&mut self) -> URS_W<2> {
        URS_W::new(self)
    }
    ///Bit 3 - One-pulse mode
    #[inline(always)]
    #[must_use]
    pub fn opm(&mut self) -> OPM_W<3> {
        OPM_W::new(self)
    }
    ///Bit 4 - Direction
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<4> {
        DIR_W::new(self)
    }
    ///Bits 5:6 - Center-aligned mode selection
    #[inline(always)]
    #[must_use]
    pub fn cms(&mut self) -> CMS_W<5> {
        CMS_W::new(self)
    }
    ///Bit 7 - Auto-reload preload enable
    #[inline(always)]
    #[must_use]
    pub fn arpe(&mut self) -> ARPE_W<7> {
        ARPE_W::new(self)
    }
    ///Bits 8:9 - Clock division
    #[inline(always)]
    #[must_use]
    pub fn ckd(&mut self) -> CKD_W<8> {
        CKD_W::new(self)
    }
    ///Bit 11 - UIF status bit remapping
    #[inline(always)]
    #[must_use]
    pub fn uifremap(&mut self) -> UIFREMAP_W<11> {
        UIFREMAP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr1](index.html) module
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr1::R](R) reader structure
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr1::W](W) writer structure
impl crate::Writable for CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
