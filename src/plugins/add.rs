use nu::{
    serve_plugin, CallInfo, ColumnPath, Plugin, ReturnSuccess, ReturnValue, ShellError,
    ShellTypeName, Signature, SpannedItem, SyntaxShape, Tagged, Value,
};

struct Add {
    field: Option<ColumnPath>,
    value: Option<Tagged<Value>>,
}
impl Add {
    fn new() -> Add {
        Add {
            field: None,
            value: None,
        }
    }

    fn add(&self, value: Tagged<Value>) -> Result<Tagged<Value>, ShellError> {
        let value_tag = value.tag();

        match (&value, &self.value, &self.field) {
            (
                obj @ Tagged {
                    item: Value::Row(_),
                    ..
                },
                Some(v),
                Some(field),
            ) => obj.clone().insert_data_at_column_path(field, v.clone()),
            (value, ..) => Err(ShellError::type_error(
                "row",
                value.type_name().spanned(value_tag),
            )),
        }
    }
}

impl Plugin for Add {
    fn config(&mut self) -> Result<Signature, ShellError> {
        Ok(Signature::build("add")
            .desc("Add a new column to the table.")
            .required("column", SyntaxShape::ColumnPath, "the column name to add")
            .required(
                "value",
                SyntaxShape::String,
                "the value to give the cell(s)",
            )
            .filter())
    }

    fn begin_filter(&mut self, call_info: CallInfo) -> Result<Vec<ReturnValue>, ShellError> {
        if let Some(args) = call_info.args.positional {
            match &args[0] {
                table @ Tagged {
                    item: Value::Table(_),
                    ..
                } => {
                    self.field = Some(table.as_column_path()?.item);
                }

                value => {
                    return Err(ShellError::type_error(
                        "table",
                        value.type_name().spanned(value.span()),
                    ))
                }
            }
            match &args[1] {
                v @ Tagged { .. } => {
                    self.value = Some(v.clone());
                }
            }
        }

        Ok(vec![])
    }

    fn filter(&mut self, input: Tagged<Value>) -> Result<Vec<ReturnValue>, ShellError> {
        Ok(vec![ReturnSuccess::value(self.add(input)?)])
    }
}

fn main() {
    serve_plugin(&mut Add::new());
}
