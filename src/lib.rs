
use fluvio_smartmodule::dataplane::smartmodule::{SmartModuleExtraParams};
use fluvio_smartmodule::{smartmodule, Result, Record, RecordData};
use rss::Channel;
use serde_json::json;
use serde_json::value::Value;

// Recursively remove keys with following values: null, {}, or []
fn remove_empty_keys(value: &mut Value) {
    match value {
        Value::Object(ref mut map) => {
            let mut to_remove = Vec::new();

            for (key, value) in map.into_iter() {
                if value.is_null() ||
                    value.is_object() &&value.as_object().unwrap().len() == 0 ||
                    value.is_array() && value.as_array().unwrap().len() == 0 {
                    to_remove.push(key.clone());
                } else {
                    remove_empty_keys(value);
                }
            }

            for key in to_remove {
                map.remove(&key);
            }
        }
        Value::Array(ref mut array) => {
            for value in array.into_iter() {
                remove_empty_keys(value);
            }
        }
        _ => {}
    }
}

#[smartmodule(map)]
pub fn map(record: &Record) -> Result<(Option<RecordData>, RecordData)> {
    let key = record.key.clone();

    let channel = Channel::read_from(record.value.as_ref()).unwrap();
    let mut json = json!(channel);
    remove_empty_keys(&mut json);

    let serialized_output = serde_json::to_vec(&json)?;

    Ok((key, RecordData::from(serialized_output)))
}


#[smartmodule(init)]
fn init(_params: SmartModuleExtraParams) -> Result<()> {
    // You can refer to the example SmartModules in Fluvio's GitHub Repository
    // https://github.com/infinyon/fluvio/tree/master/smartmodule
    Ok(())
}
