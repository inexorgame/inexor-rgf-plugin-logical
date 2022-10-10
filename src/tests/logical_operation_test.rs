use std::collections::HashMap;
use std::sync::Arc;

use serde_json::json;
use uuid::Uuid;

use crate::behaviour::entity::operation::LogicalOperation;
use crate::behaviour::entity::operation::LogicalOperationProperties;
use crate::behaviour::entity::operation::LOGICAL_OPERATIONS;
use crate::model::DataType;
use crate::model::EntityInstance;
use crate::model::EntityType;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::PropertyType;
use crate::model::ReactiveEntityInstance;
use crate::model::SocketType;
use crate::reactive::Operation;

const LHS: LogicalOperationProperties = LogicalOperationProperties::LHS;
const RESULT: LogicalOperationProperties = LogicalOperationProperties::RESULT;

const NAMESPACE: &str = "logical";
const COMPONENT_NAME_LOGICAL_OPERATION: &str = "logical_operation";
const TYPE_NAME_NOT: &str = "not";

#[test]
fn behaviour_function_should_exist() {
    assert!(LOGICAL_OPERATIONS.contains_key(TYPE_NAME_NOT));
    assert!(LOGICAL_OPERATIONS.get(TYPE_NAME_NOT).is_some());
}

#[test]
fn not_operation_test() {
    let property_types = vec![
        PropertyType::new_with_socket(LHS, DataType::Number, SocketType::Input),
        PropertyType::new_with_socket(RESULT, DataType::Number, SocketType::Output),
    ];
    let not_type = EntityType::new(NAMESPACE, TYPE_NAME_NOT, "", vec![String::from(COMPONENT_NAME_LOGICAL_OPERATION)], property_types, Vec::new());
    let not_function = LOGICAL_OPERATIONS.get(TYPE_NAME_NOT).unwrap();
    let mut properties = HashMap::new();
    properties.insert(LHS.into(), json!(LHS.default_value()));
    properties.insert(RESULT.into(), json!(RESULT.default_value()));
    let not_entity = EntityInstance::new(NAMESPACE, &not_type.name, Uuid::new_v4(), properties);
    let not_reactive_entity = Arc::new(ReactiveEntityInstance::from(not_entity));
    let not_behaviour = LogicalOperation::new(not_reactive_entity.clone(), *not_function);
    assert!(not_behaviour.is_ok());
    let not_behaviour = not_behaviour.unwrap();
    assert_eq!(TYPE_NAME_NOT, not_behaviour.type_name().as_str());

    // === Reactive Entity API ===

    not_reactive_entity.set(LHS, json!(true));
    assert_eq!(false, not_reactive_entity.as_bool(RESULT).unwrap());
    not_reactive_entity.set(LHS, json!(false));
    assert_eq!(true, not_reactive_entity.as_bool(RESULT).unwrap());

    // === Behaviour API ===

    not_behaviour.lhs(json!(true));
    assert_eq!(false, not_behaviour.result().as_bool().unwrap());
    not_behaviour.lhs(json!(false));
    assert_eq!(true, not_behaviour.result().as_bool().unwrap());
}

#[test]
fn incomplete_not_operation_test() {
    let property_types = vec![
        PropertyType::new_with_socket(LHS, DataType::Number, SocketType::Input),
        PropertyType::new_with_socket(RESULT, DataType::Number, SocketType::Output),
    ];
    let not_type = EntityType::new(NAMESPACE, TYPE_NAME_NOT, "", vec![String::from(COMPONENT_NAME_LOGICAL_OPERATION)], property_types, Vec::new());
    let not_function = LOGICAL_OPERATIONS.get(TYPE_NAME_NOT).unwrap();
    // No-properties
    let not_entity = EntityInstance::new(NAMESPACE, &not_type.name, Uuid::new_v4(), HashMap::new());
    let not_reactive_entity = Arc::new(ReactiveEntityInstance::from(not_entity));
    let not_behaviour = LogicalOperation::new(not_reactive_entity.clone(), *not_function);
    assert!(not_behaviour.is_err());
}
