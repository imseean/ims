//! # Contain some helper of the handlebars

use handlebars::{Handlebars, Helper, RenderContext, RenderError, Renderable};
use serde::Serialize;
use serde_json;
use serde_json::Value;
use std::collections::BTreeMap;
use std::collections::HashMap;
/// # Render the json Format of the object.
///
/// A helper for handlebars
pub fn json_helper(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    if let Some(param) = h.param(0) {
        let json = param.value();
        rc.writer.write(&json.to_string().into_bytes()).is_ok();
    }
    Ok(())
}

/// # Output content to file.
///
/// A helper for handlebars.
pub fn file_helper(h: &Helper, r: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let template = h.template();

    let parms: Vec<String> = h
        .params()
        .iter()
        .map(|x| {
            let mut param = String::new();
            if x.value().is_string() {
                if let Some(temp) = x.value().as_str() {
                    param = temp.to_string();
                }
            } else if x.value().is_number() {
                if let Some(temp) = x.value().as_i64() {
                    param = format!("{}", temp);
                }
            }
            return param;
        }).collect();
    let file_path = parms.join("");

    match template {
        Some(t) => {
            t.render(r, rc).unwrap();
        }
        None => {}
    }
    rc.writer
        .write_all(&file_path.to_string().into_bytes())
        .is_ok();
    Ok(())
}
#[allow(unused)]
pub fn pagination_helper(
    h: &Helper,
    r: &Handlebars,
    rc: &mut RenderContext,
) -> Result<(), RenderError> {
    let mut list: Vec<Value> = vec![];
    let mut size = 10;
    if let Some(param) = h.param(0) {
        let json = param.value();
        if json.is_array() {
            list = json.as_array().unwrap().clone();
        }
    }
    if let Some(param) = h.param(1) {
        let json = param.value();
        if json.is_u64() {
            size = json.as_u64().unwrap().clone();
        }
    }
    let size = size as usize;
    let count = (list.len() + size + 1) / size;

    for index in 0..count {
        let page: Vec<Value> = list
            .clone()
            .into_iter()
            .skip(index * size)
            .take(size)
            .collect();
        let template = h.template();
        let mut local_rc = rc.derive();

        // local_rc.set_local_var("@list".to_string(), serde_json::to_value(page).unwrap());
        local_rc.set_local_var("@size".to_string(), serde_json::to_value(size).unwrap());
        local_rc.set_local_var(
            "@index".to_string(),
            serde_json::to_value(index + 1).unwrap(),
        );
        local_rc.set_local_var("@count".to_string(), serde_json::to_value(count).unwrap());
        if let Some(block_param) = h.block_param() {
            let mut map = BTreeMap::new();
            map.insert(block_param.to_string(), serde_json::to_value(&page).unwrap());
            local_rc.push_block_context(&map).unwrap();
        } else {
            local_rc.push_block_context(&page).unwrap();
        }

        local_rc.push_block_context(&page).unwrap();
        match template {
            Some(t) => {
                t.render(r, &mut local_rc).unwrap();
            }
            None => {}
        }
        local_rc.pop_block_context();
    }

    Ok(())
}

pub trait HandlebarsExtension {
    fn render_with_file<T>(
        &mut self,
        name: &str,
        data: &T,
    ) -> Result<HashMap<String, String>, RenderError>
    where
        T: Serialize;
    fn render_template_with_file<T>(
        &mut self,
        template_string: &str,
        data: &T,
    ) -> Result<HashMap<String, String>, RenderError>
    where
        T: Serialize;
}

impl HandlebarsExtension for Handlebars {
    fn render_with_file<T>(
        &mut self,
        name: &str,
        data: &T,
    ) -> Result<HashMap<String, String>, RenderError>
    where
        T: Serialize,
    {
        if self.get_template("file") == None {
            self.register_helper("file", Box::new(file_helper));
        }

        let mut write = writer::TemplateWriter::new();
        self.render_to_write(name, data, &mut write).unwrap();
        return Ok(write.map);
    }

    fn render_template_with_file<T>(
        &mut self,
        template_string: &str,
        data: &T,
    ) -> Result<HashMap<String, String>, RenderError>
    where
        T: Serialize,
    {
        if self.get_template("file") == None {
            self.register_helper("file", Box::new(file_helper));
        }
        let mut write = writer::TemplateWriter::new();
        self.render_template_to_write(template_string, data, &mut write)
            .unwrap();
        return Ok(write.map);
    }
}

mod writer {
    use std::collections::HashMap;
    use std::io::prelude::*;
    use std::io::Result;
    pub struct TemplateWriter {
        pub map: HashMap<String, String>,
        pub buffer: Vec<u8>,
        pub indexes: Vec<usize>,
    }
    impl TemplateWriter {
        pub fn new() -> TemplateWriter {
            TemplateWriter {
                map: HashMap::new(),
                buffer: vec![],
                indexes: vec![],
            }
        }
    }

    impl Write for TemplateWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.buffer.extend_from_slice(buf);

            return Ok(buf.len());
        }

        fn flush(&mut self) -> Result<()> {
            return Ok(());
        }

        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            let mut buffer: Vec<u8> = vec![];
            buffer.extend_from_slice(buf);
            let path = String::from_utf8(buffer).unwrap_or(String::new());
            self.map.insert(
                path.clone(),
                String::from_utf8(self.buffer.clone()).unwrap_or(String::new()),
            );
            self.buffer = vec![];

            return Ok(());
        }
    }
}
