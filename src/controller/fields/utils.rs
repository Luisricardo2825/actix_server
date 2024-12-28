use crate::routes::utils::reponses::ReturnError;

use super::{field_controller::FieldController, structs::CreateField};

pub fn validate_fields(fields: &Vec<CreateField>) -> Result<(), ReturnError> {
    let mut has_pk = false;
    let mut field_with_error = None;
    for field in fields.iter() {
        let validtad_result = field.validate();
        if validtad_result.is_err() {
            return Err(ReturnError::without_value(
                validtad_result.unwrap_err().to_string(),
            ));
        }
        if field.is_pk() {
            has_pk = true;
            field_with_error = Some(serde_json::to_value(field.to_owned()).unwrap());
        }
    }

    if !has_pk {
        return Err(ReturnError {
            error_msg: "Table must have a primary key".to_string(),
            values: field_with_error,
        }
        .into());
    }

    return Ok(());
}

pub fn set_table_for_vec(table_id: i32, fields: &mut Vec<CreateField>) -> Result<(), ReturnError> {
    for field in fields.iter_mut() {
        field.set_table(table_id);
        let field_found = FieldController::find_field_by_table_id_and_name(table_id, &field.name);
        if field_found.is_ok() {
            return Err(ReturnError {
                error_msg: format!("Field \"{}\" already exists", field.name),
                values: None,
            }
            .into());
        }
    }
    return Ok(());
}
