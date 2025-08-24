#[doc = "Register `CALBC1_1MHZ` reader"]
pub type R = crate::R<Calbc1_1mhzSpec>;
#[doc = "Register `CALBC1_1MHZ` writer"]
pub type W = crate::W<Calbc1_1mhzSpec>;
#[doc = "Field `CALBC1_1MHZ` reader - Value for the BCSCTL1 register for 1 MHz"]
pub type Calbc1_1mhzR = crate::FieldReader;
#[doc = "Field `CALBC1_1MHZ` writer - Value for the BCSCTL1 register for 1 MHz"]
pub type Calbc1_1mhzW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - Value for the BCSCTL1 register for 1 MHz"]
    #[inline(always)]
    pub fn calbc1_1mhz(&self) -> Calbc1_1mhzR {
        Calbc1_1mhzR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Value for the BCSCTL1 register for 1 MHz"]
    #[inline(always)]
    pub fn calbc1_1mhz(&mut self) -> Calbc1_1mhzW<'_, Calbc1_1mhzSpec> {
        Calbc1_1mhzW::new(self, 0)
    }
}
#[doc = "BCSCTL1 Calibration Data for 1MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`calbc1_1mhz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calbc1_1mhz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Calbc1_1mhzSpec;
impl crate::RegisterSpec for Calbc1_1mhzSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`calbc1_1mhz::R`](R) reader structure"]
impl crate::Readable for Calbc1_1mhzSpec {}
#[doc = "`write(|w| ..)` method takes [`calbc1_1mhz::W`](W) writer structure"]
impl crate::Writable for Calbc1_1mhzSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CALBC1_1MHZ to value 0"]
impl crate::Resettable for Calbc1_1mhzSpec {}
