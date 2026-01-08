use actix_web::{Error, error::ErrorUnprocessableEntity, web};
use once_cell::sync::Lazy;
use regex::Regex;
use validator::{Validate, ValidationError, ValidationErrors};

pub static PHONE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(0|\+33 )[1-9]([\-. ]?[0-9]{2} ){3}([\-. ]?[0-9]{2})|([0-9]{8})$").unwrap());
pub static ZIPCODE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(?:0[1-9]|[1-8]\d|9[0-8])\d{3}$").unwrap());
pub static SIRET_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d{14}|((\d{3}[ ]\d{3}[ ]\d{3})|\d{9})[ ]\d{5})$").unwrap());

pub fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    if PHONE_REGEX.is_match(phone) {
        Ok(())
    } else {
        let mut error = ValidationError::new("invalid_phone");
        error.message = Some("Phone number must be in the french format".into());
        Err(error)
    }
}

pub fn validate_zipcode(zip: &str) -> Result<(), ValidationError> {
    if ZIPCODE_REGEX.is_match(zip) {
        Ok(())
    } else {
        let mut error = ValidationError::new("invalid_zipcode");
        error.message = Some("Zip Code must be a valid french department".into());
        Err(error)
    }
}

pub fn validate_siret(siret: &str) -> Result<(), ValidationError> {
    if SIRET_REGEX.is_match(siret) {
        Ok(())
    } else {
        let mut error = ValidationError::new("invalid_siret");
        error.message = Some("Your company SIRET number must be in the french format".into());
        Err(error)
    }
}

pub fn process_validation_errors<T: Validate>(item: &T) -> Result<(), Error> {
    if let Err(validation_errors) = item.validate() {
        let error_messages = format_validation_errors(validation_errors);
        return Err(ErrorUnprocessableEntity(error_messages));
    }
    Ok(())
}

pub fn process_json_validation<T: Validate>(json: &web::Json<T>) -> Result<(), Error> {
    process_validation_errors(&json.0)
}

pub fn format_validation_errors(validation_errors: ValidationErrors) -> String {
    let error_messages: Vec<String> = validation_errors
        .field_errors()
        .into_iter()
        .map(|(field, errors)| {
            let error_messages: Vec<String> = errors
                .iter()
                .filter_map(|error| error.message.clone())
                .map(|message| message.to_string())
                .collect();

            format!("{}: {}", field, error_messages.join(", "))
        })
        .collect();

    error_messages.join("; ")
}