#[doc = "Reader of register MSR[%s]"]
pub type R = crate::R<u32, super::MSR>;
#[doc = "Writer for register MSR[%s]"]
pub type W = crate::W<u32, super::MSR>;
#[doc = "Register MSR[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::MSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SHADOWW`"]
pub type SHADOWW_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SHADOWW`"]
pub struct SHADOWW_W<'a> {
    w: &'a mut W,
}
impl<'a> SHADOWW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Timer counter match shadow value."]
    #[inline(always)]
    pub fn shadoww(&self) -> SHADOWW_R {
        SHADOWW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer counter match shadow value."]
    #[inline(always)]
    pub fn shadoww(&mut self) -> SHADOWW_W {
        SHADOWW_W { w: self }
    }
}
