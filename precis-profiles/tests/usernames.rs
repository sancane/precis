#[cfg(test)]
mod case_mapped {
    use precis_core::profile::Profile;
    use precis_core::{CodepointInfo, DerivedPropertyValue, Error};
    use precis_profiles::UsernameCaseMapped;
    use std::borrow::Cow;

    #[test]
    fn prepare() {
        let profile = UsernameCaseMapped::new();

        // Preparation does no apply case mapping rule
        let res = profile.prepare("XxXxX");
        assert_eq!(res, Ok(Cow::from("XxXxX")));

        // Preparation applies width mapping rule
        let res = profile.prepare("Xx\u{ff01}xX");
        assert_eq!(res, Ok(Cow::from("Xx\u{0021}xX")));

        // The "at" sign ("@") is allowed in the PRECIS `IdentifierClass`
        let res = profile.prepare("juliet@example.com");
        assert_eq!(res, Ok(Cow::from("juliet@example.com")));

        let res = profile.prepare("fussaball");
        assert_eq!(res, Ok(Cow::from("fussaball")));

        // The third character is LATIN SMALL LETTER SHARP S `U+00DF`
        let res = profile.prepare("fußball");
        assert_eq!(res, Ok(Cow::from("fußball")));

        // A `userpart` of GREEK SMALL LETTER PI `U+03C0`
        let res = profile.prepare("π");
        assert_eq!(res, Ok(Cow::from("π")));

        // A `userpart` of GREEK CAPITAL LETTER SIGMA `U+03A3`
        let res = profile.prepare("Σ");
        assert_eq!(res, Ok(Cow::from("Σ")));

        // A `userpart` of GREEK SMALL LETTER SIGMA `U+03C3`
        let res = profile.prepare("σ");
        assert_eq!(res, Ok(Cow::from("σ")));

        // A `userpart` of GREEK SMALL LETTER FINAL SIGMA `U+03C2`
        let res = profile.prepare("ς");
        assert_eq!(res, Ok(Cow::from("ς")));

        // SPACE `U+0020` is disallowed in the `userpart`
        let res = profile.prepare("foo bar");
        assert_eq!(
            res,
            Err(Error::BadCodepoint(CodepointInfo::new(
                0x0020,
                3,
                DerivedPropertyValue::SpecClassDis
            )))
        );

        // Zero-length `userpart`
        let res = profile.prepare("");
        assert_eq!(res, Err(Error::Invalid));

        // The sixth character is ROMAN NUMERAL FOUR `U+2163`
        let res = profile.prepare("henry\u{2163}");
        assert_eq!(
            res,
            Err(Error::BadCodepoint(CodepointInfo::new(
                0x2163,
                5,
                DerivedPropertyValue::SpecClassDis
            )))
        );

        // A `userpart` of INFINITY `U+221E`
        let res = profile.prepare("∞");
        assert_eq!(
            res,
            Err(Error::BadCodepoint(CodepointInfo::new(
                0x221e,
                0,
                DerivedPropertyValue::SpecClassDis
            )))
        );
    }

    #[test]
    fn enforce() {
        let profile = UsernameCaseMapped::new();

        // Enforcement applies case mapping rule
        let res = profile.enforce("XxXxX");
        assert_eq!(res, Ok(Cow::from("xxxxx")));

        // Enforcement applies width mapping rule
        let res = profile.enforce("Xx\u{ff01}xX");
        assert_eq!(res, Ok(Cow::from("xx!xx")));

        // The "at" sign ("@") is allowed in the PRECIS `IdentifierClass`
        let res = profile.enforce("juliet@example.com");
        assert_eq!(res, Ok(Cow::from("juliet@example.com")));

        let res = profile.enforce("fussaball");
        assert_eq!(res, Ok(Cow::from("fussaball")));

        // The third character is LATIN SMALL LETTER SHARP S `U+00DF`
        let res = profile.enforce("fußball");
        assert_eq!(res, Ok(Cow::from("fußball")));

        // A `userpart` of GREEK SMALL LETTER PI `U+03C0`
        let res = profile.enforce("π");
        assert_eq!(res, Ok(Cow::from("π")));

        // A `userpart` of GREEK CAPITAL LETTER SIGMA `U+03A3`
        let res = profile.enforce("Σ");
        assert_eq!(res, Ok(Cow::from("σ")));

        // A `userpart` of GREEK SMALL LETTER SIGMA `U+03C3`
        let res = profile.prepare("σ");
        assert_eq!(res, Ok(Cow::from("σ")));

        // A `userpart` of GREEK SMALL LETTER FINAL SIGMA `U+03C2`
        let res = profile.prepare("ς");
        assert_eq!(res, Ok(Cow::from("ς")));

        // SPACE `U+0020` is disallowed in the `userpart`
        let res = profile.prepare("foo bar");
        assert_eq!(
            res,
            Err(Error::BadCodepoint(CodepointInfo::new(
                0x0020,
                3,
                DerivedPropertyValue::SpecClassDis
            )))
        );

        // Zero-length `userpart`
        let res = profile.prepare("");
        assert_eq!(res, Err(Error::Invalid));

        // The sixth character is ROMAN NUMERAL FOUR `U+2163`
        let res = profile.prepare("henry\u{2163}");
        assert_eq!(
            res,
            Err(Error::BadCodepoint(CodepointInfo::new(
                0x2163,
                5,
                DerivedPropertyValue::SpecClassDis
            )))
        );

        // A `userpart` of INFINITY `U+221E`
        let res = profile.prepare("∞");
        assert_eq!(
            res,
            Err(Error::BadCodepoint(CodepointInfo::new(
                0x221e,
                0,
                DerivedPropertyValue::SpecClassDis
            )))
        );
    }

    #[test]
    fn compare() {
        let profile = UsernameCaseMapped::new();

        let res = profile.compare("", "Hello");
        assert_eq!(res, Err(Error::Invalid));

        let res = profile.compare("Hello", "");
        assert_eq!(res, Err(Error::Invalid));

        let res = profile.compare("heLLo", "Hello");
        assert_eq!(res, Ok(true));

        let res = profile.compare("heLLo!", "Hello\u{ff01}");
        assert_eq!(res, Ok(true));

        let res = profile.compare("heLLo", "Bello");
        assert_eq!(res, Ok(false));

        let res = profile.compare("heLL∞", "Hell");
        assert_eq!(
            res,
            Err(Error::BadCodepoint(CodepointInfo::new(
                0x221e,
                4,
                DerivedPropertyValue::SpecClassDis
            )))
        );
    }
}

#[cfg(test)]
mod case_preserved {
    use precis_core::profile::Profile;
    use precis_core::{CodepointInfo, DerivedPropertyValue, Error};
    use precis_profiles::UsernameCasePreserved;
    use std::borrow::Cow;

    #[test]
    fn prepare() {
        let profile = UsernameCasePreserved::new();

        // Preparation does no apply case mapping rule
        let res = profile.prepare("XxXxX");
        assert_eq!(res, Ok(Cow::from("XxXxX")));

        // Preparation applies width mapping rule
        let res = profile.prepare("Xx\u{ff01}xX");
        assert_eq!(res, Ok(Cow::from("Xx\u{0021}xX")));

        // The "at" sign (`@`) is allowed in the PRECIS `IdentifierClass`
        let res = profile.prepare("juliet@example.com");
        assert_eq!(res, Ok(Cow::from("juliet@example.com")));

        let res = profile.prepare("fussaball");
        assert_eq!(res, Ok(Cow::from("fussaball")));

        // The third character is LATIN SMALL LETTER SHARP S `U+00DF`
        let res = profile.prepare("fußball");
        assert_eq!(res, Ok(Cow::from("fußball")));

        // A `userpart` of GREEK SMALL LETTER PI `U+03C0`
        let res = profile.prepare("π");
        assert_eq!(res, Ok(Cow::from("π")));

        // A `userpart` of GREEK CAPITAL LETTER SIGMA `U+03A3`
        let res = profile.prepare("Σ");
        assert_eq!(res, Ok(Cow::from("Σ")));

        // A `userpart` of GREEK SMALL LETTER SIGMA `U+03C3`
        let res = profile.prepare("σ");
        assert_eq!(res, Ok(Cow::from("σ")));

        // A `userpart` of GREEK SMALL LETTER FINAL SIGMA `U+03C2`
        let res = profile.prepare("ς");
        assert_eq!(res, Ok(Cow::from("ς")));

        // SPACE `U+0020` is disallowed in the `userpart`
        let res = profile.prepare("foo bar");
        assert_eq!(
            res,
            Err(Error::BadCodepoint(CodepointInfo::new(
                0x0020,
                3,
                DerivedPropertyValue::SpecClassDis
            )))
        );

        // Zero-length `userpart`
        let res = profile.prepare("");
        assert_eq!(res, Err(Error::Invalid));

        // The sixth character is ROMAN NUMERAL FOUR `U+2163`
        let res = profile.prepare("henry\u{2163}");
        assert_eq!(
            res,
            Err(Error::BadCodepoint(CodepointInfo::new(
                0x2163,
                5,
                DerivedPropertyValue::SpecClassDis
            )))
        );

        // A `userpart` of INFINITY `U+221E`
        let res = profile.prepare("∞");
        assert_eq!(
            res,
            Err(Error::BadCodepoint(CodepointInfo::new(
                0x221e,
                0,
                DerivedPropertyValue::SpecClassDis
            )))
        );
    }

    #[test]
    fn enforce() {
        let profile = UsernameCasePreserved::new();

        // Enforcement does not apply case mapping rule
        let res = profile.enforce("XxXxX");
        assert_eq!(res, Ok(Cow::from("XxXxX")));

        // Enforcement applies width mapping rule
        let res = profile.enforce("Xx\u{ff01}xX");
        assert_eq!(res, Ok(Cow::from("Xx!xX")));

        // The "at" sign (`@`) is allowed in the PRECIS `IdentifierClass`
        let res = profile.enforce("juliet@example.com");
        assert_eq!(res, Ok(Cow::from("juliet@example.com")));

        let res = profile.enforce("fussaball");
        assert_eq!(res, Ok(Cow::from("fussaball")));

        // The third character is LATIN SMALL LETTER SHARP S `U+00DF`
        let res = profile.enforce("fußball");
        assert_eq!(res, Ok(Cow::from("fußball")));

        // A `userpart` of GREEK SMALL LETTER PI `U+03C0`
        let res = profile.enforce("π");
        assert_eq!(res, Ok(Cow::from("π")));

        // A `userpart` of GREEK CAPITAL LETTER SIGMA `U+03A3`
        let res = profile.enforce("Σ");
        assert_eq!(res, Ok(Cow::from("Σ")));

        // A `userpart` of GREEK SMALL LETTER SIGMA `U+03C3`
        let res = profile.prepare("σ");
        assert_eq!(res, Ok(Cow::from("σ")));

        // A `userpart` of GREEK SMALL LETTER FINAL SIGMA `U+03C2`
        let res = profile.prepare("ς");
        assert_eq!(res, Ok(Cow::from("ς")));

        // SPACE `U+0020` is disallowed in the `userpart`
        let res = profile.prepare("foo bar");
        assert_eq!(
            res,
            Err(Error::BadCodepoint(CodepointInfo::new(
                0x0020,
                3,
                DerivedPropertyValue::SpecClassDis
            )))
        );

        // Zero-length `userpart`
        let res = profile.prepare("");
        assert_eq!(res, Err(Error::Invalid));

        // The sixth character is ROMAN NUMERAL FOUR `U+2163`
        let res = profile.prepare("henry\u{2163}");
        assert_eq!(
            res,
            Err(Error::BadCodepoint(CodepointInfo::new(
                0x2163,
                5,
                DerivedPropertyValue::SpecClassDis
            )))
        );

        // A `userpart` of INFINITY `U+221E`
        let res = profile.prepare("∞");
        assert_eq!(
            res,
            Err(Error::BadCodepoint(CodepointInfo::new(
                0x221e,
                0,
                DerivedPropertyValue::SpecClassDis
            )))
        );
    }

    #[test]
    fn compare() {
        let profile = UsernameCasePreserved::new();

        let res = profile.compare("", "Hello");
        assert_eq!(res, Err(Error::Invalid));

        let res = profile.compare("Hello", "");
        assert_eq!(res, Err(Error::Invalid));

        let res = profile.compare("heLLo", "Hello");
        assert_eq!(res, Ok(false));

        let res = profile.compare("Hello!", "Hello\u{ff01}");
        assert_eq!(res, Ok(true));

        let res = profile.compare("heLLo", "Bello");
        assert_eq!(res, Ok(false));

        let res = profile.compare("heLL∞", "Hell");
        assert_eq!(
            res,
            Err(Error::BadCodepoint(CodepointInfo::new(
                0x221e,
                4,
                DerivedPropertyValue::SpecClassDis
            )))
        );
    }
}
