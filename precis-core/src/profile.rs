use crate::Error;
use std::borrow::Cow;

/// Rules that any profile of a PRECIS string class MUST define
/// to proper manage the handling of right-to-left code points as
/// well as various mapping operations such as case preservation
/// or lowercasing, Unicode normalization, mapping of certain code
/// points to other code points or to nothing, and mapping of fullwidth
/// and halfwidth code points.
pub trait Rules {
    /// Applies the width mapping rule of a profile to an input string.
    /// # Arguments:
    /// * `s`: String value
    /// # Returns
    /// The same string if no modifications were required or a new allocated
    /// string if `s` was modified as a result of applying this rule
    fn width_mapping_rule<'a, T>(&self, _s: T) -> Result<Cow<'a, str>, Error>
    where
        T: Into<Cow<'a, str>>,
    {
        Err(Error::NotApplicable)
    }

    /// Appies the additional mapping rule of a profile to an input string.
    /// # Arguments:
    /// * `s`: String value
    /// # Returns
    /// The same string if no modifications were required or a new allocated
    /// string if `s` was modified as a result of applying this rule
    fn additional_mapping_rule<'a, T>(&self, _s: T) -> Result<Cow<'a, str>, Error>
    where
        T: Into<Cow<'a, str>>,
    {
        Err(Error::NotApplicable)
    }

    /// Applies the case mapping rule of a profile to an input string
    /// # Arguments:
    /// * `s`: String value
    /// # Returns
    /// The same string if no modifications were required or a new allocated
    /// string if `s` was modified as a result of applying this rule
    fn case_mapping_rule<'a, T>(&self, _s: T) -> Result<Cow<'a, str>, Error>
    where
        T: Into<Cow<'a, str>>,
    {
        Err(Error::NotApplicable)
    }

    /// Applies the normalization rule of a profile to an input string
    /// # Arguments:
    /// * `s`: String value
    /// # Returns
    /// The same string if no modifications were required or a new allocated
    /// string if `s` was modified as a result of applying this rule
    fn normalization_rule<'a, T>(&self, _s: T) -> Result<Cow<'a, str>, Error>
    where
        T: Into<Cow<'a, str>>,
    {
        Err(Error::NotApplicable)
    }

    /// Applies the directionality rule of a profile to an input string
    /// # Arguments:
    /// * `s`: String value
    /// # Returns
    /// The same string if no modifications were required or a new allocated
    /// string if `s` was modified as a result of applying this rule
    fn directionality_rule<'a, T>(&self, _s: T) -> Result<Cow<'a, str>, Error>
    where
        T: Into<Cow<'a, str>>,
    {
        Err(Error::NotApplicable)
    }
}

/// Profile enables application protocols to apply the string classes in ways that
/// are appropriate for common constructs.
pub trait Profile {
    /// Ensures that the code points in a single input string are allowed
    /// by the underlying PRECIS string class, and sometimes also entails
    /// applying one or more of the rules specified for a particular string
    /// class or profile thereof.
    /// # Arguments:
    /// * `s`: String value
    /// # Returns
    /// The same string if no modification were required or a new allocated
    /// string if `s` needed further modifications as a result of applying the
    /// rules defined by this profile to prepare the string
    fn prepare<'a>(&self, s: &'a str) -> Result<Cow<'a, str>, Error>;

    /// Applies all of the rules specified for a particular string class,
    /// or profile thereof, to a single input string, for the purpose of
    /// checking whether the string conforms to all of the rules and thus
    /// determining if the string can be used in a given protocol slot.
    /// # Arguments:
    /// * `s`: String value
    /// # Returns
    /// The same string if no modification were required or a new allocated
    /// string if `s` needed further modifications as a result of enforcing
    /// the string according to the rules defined by this profile.
    fn enforce<'a>(&self, s: &'a str) -> Result<Cow<'a, str>, Error>;

    /// Comparison entails applying all of the rules specified for a
    /// particular string class, or profile thereof, to two separate input
    /// strings, for the purpose of determining if the two strings are
    /// equivalent.
    fn compare(&self, s1: &str, s2: &str) -> Result<bool, Error>;
}

/// Fast invokation trait that allows profiles to be used without providing
/// a specific instance. This is usually achieved by using a static instance
/// allocated with [lazy_static](https://docs.rs/lazy_static/1.4.0/lazy_static)
pub trait PrecisFastInvocation {
    fn prepare(s: &str) -> Result<Cow<'_, str>, Error>;
    fn enforce(s: &str) -> Result<Cow<'_, str>, Error>;
    fn compare(s1: &str, s2: &str) -> Result<bool, Error>;
}

/// Apply rules until the string is estable. Some profiles, especially those
/// that the result of applying these rules does not result in an idempotent
/// operation for all code points SHOULD apply the rules repeatedly until
/// the output string is stable.
/// # Arguments:
/// * `s`: String value
/// * `f`: Callback to invoke to apply the rules to `s`
/// # Returns
/// The stable string after applying the rules; if the output string
/// does not stabilize after reapplying the rules three (3) additional times
/// after the first application, the string is rejected as invalid.
pub fn stabilize<'a, F>(s: &'a str, f: F) -> Result<Cow<'a, str>, Error>
where
    F: for<'b> Fn(&'b str) -> Result<Cow<'b, str>, Error>,
{
    let mut c = Cow::from(s);
    for _i in 0..=2 {
        let tmp = f(&c)?;
        if tmp == c {
            return Ok(c);
        }

        // Strings are not equal, so we have an owned copy.
        // We move the owned string without copying it for
        // the next iteration
        c = Cow::from(tmp.into_owned());
    }

    // The string did not stabilized after applying the rules three times.
    Err(Error::Disallowed)
}