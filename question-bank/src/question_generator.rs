use std::{collections::HashMap, fs};

use anyhow::{Ok, anyhow};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use indoc::indoc;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuestionGenerator {
    pub problem_name: String,
    pub function_name: String,
    pub input_fields: Vec<(String, String)>,
    pub output_fields: Vec<(String, String)>,
}

static CPP_TYPES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("bool", "bool"),
        ("char", "char"),
        ("string", "string"),
        ("int", "int"),
        ("float", "float"),
        ("list<string>", "vector<string>"),
        ("list<int>", "vector<int>"),
        ("list<float>", "vector<float>")
    ])
});

static JS_TYPES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("bool", "boolean"),
        ("string", "string"),
        ("int", "number"),
        ("float", "number"),
        ("list<string>", "string[]"),
        ("list<int>", "number[]"),
        ("list<float>", "float[]")
    ])
});

static RUST_TYPES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("bool", "bool"),
        ("string", "String"),
        ("int", "i32"),
        ("float", "f32"),
        ("list<string>", "Vec<String>"),
        ("list<int>", "Vec<i32>"),
        ("list<float>", "Vec<f32>")
    ])
});

impl QuestionGenerator {
    pub fn new(structure: String) -> anyhow::Result<Self> {
        let mut problem_name: Option<String> = None;
        let mut function_name: Option<String> = None;
        let mut input_fields: Vec<(String, String)> = Vec::new();      //Vec(field_name, field_type)
        let mut output_fields: Vec<(String, String)> = Vec::new();

        for line in structure.lines() {
            if line.starts_with("Problem Name:") {
                let value = line.split(':')
                    .nth(1).expect("Problem name is empty")
                    .trim()
                    .trim_matches('"')
                    .to_string();
                problem_name = Some(value);
            } else if line.starts_with("Function Name:") {
                let value = line.split(':')
                    .nth(1).expect("Function name is empty")
                    .trim()
                    .trim_matches('"')
                    .to_string();     
                function_name = Some(value);  
            } else if line.starts_with("Input Field:") {
                let field_str = line.split(':')
                    .nth(1).expect("Input Field is empty")
                    .trim()
                    .trim_matches('"')
                    .to_string();

                let mut field = field_str.split(" "); 

                let field_type = field.next().expect("Input Field type is empty");
                let field_name = field.next().expect("Input Field name is  empty");
                
                let tup = (field_name.to_string(), field_type.to_string());
                input_fields.push(tup);
            } else if line.starts_with("Output Field:") {
                let field_str = line.split(':')
                    .nth(1).expect("Output Field is empty")
                    .trim()
                    .trim_matches('"')
                    .to_string();

                let mut field = field_str.split(" "); 

                let field_type = field.next().expect("Output Field type is empty");
                let field_name = field.next().expect("Output Field name is  empty");
                
                let tup = (field_name.to_string(), field_type.to_string());
                output_fields.push(tup);
            }
        }

        if problem_name.is_none() || function_name.is_none() || input_fields.is_empty() || output_fields.is_empty() {
            return Err(anyhow!("Wrong structure"));
        }

        Ok(Self {
            problem_name: problem_name.unwrap(),
            function_name: function_name.unwrap(),
            input_fields: input_fields,
            output_fields: output_fields
        })
    }

    pub fn generate_cpp_code_partial(&self, output_path: String) -> anyhow::Result<()> {
        let tup = self.output_fields[0].clone();
        let key = tup.1;

        let return_type = CPP_TYPES.get(key.as_str());

        let mut input_args_str: String = String::from("");

        let input_len = self.input_fields.len();

        for (index, (field_name, field_type)) in self.input_fields.iter().enumerate() {
            let mapped_type = CPP_TYPES.get(field_type.as_str());
            if mapped_type.is_none() {
                return Err(anyhow!("Unsupported c++ input arg type"));
            }
            
            let args_str: String;

            if mapped_type.unwrap().starts_with("vector<") {
                args_str = format!("{}& {}", mapped_type.unwrap(), field_name);
            } else {
                args_str = format!("{} {}", mapped_type.unwrap(), field_name);
            }

            input_args_str.push_str(&args_str);
            if index != input_len - 1 {
                input_args_str.push_str(", ");
            }
        }

        let code = format!(
            indoc!{
                "#include <iostream>
                #include <vector>
                #include <map>
                #include <string>

                using std::cin;
                using std::cout;
                using std::vector;
                using std::string;

                {} {}({}) {{
                    //code goes here
                }}"
            },
            return_type.unwrap(), self.function_name, input_args_str
        );

        fs::write(output_path, code).unwrap();

        Ok(())
    }

    pub fn generate_cpp_code_full(&self, output_path: String) -> anyhow::Result<()> {
        let mut input_args = Vec::<String>::new();
        let mut code: String = String::from("");

        for (index, (field_name, field_type)) in self.input_fields.iter().enumerate() {
            input_args.push(field_name.to_string());
            let mapped_type = CPP_TYPES.get(field_type.as_str()).unwrap();
            
            let tmp_code;
            if field_type.starts_with("list<") {
                tmp_code =  format!(
                    indoc! {
                        "
                        int size_arr{index};
                        cin>>size_arr{index};
                        {mapped_type} {field_name}(size_arr{index});
                        for (int i=0; i< size_arr{index}; i++) {{
                            cin>>{field_name}[i];
                        }}
                        "
                    }
                    , 
                    index = index, 
                    mapped_type = mapped_type,
                    field_name = field_name
                );
                
            } else {
                tmp_code =  format!(
                    indoc! {
                        "
                        {mapped_type} {field_name};
                        cin>>{field_name};
                        "
                    }
                    , 
                    mapped_type = mapped_type,
                    field_name = field_name
                );
            }

            code.push_str(&tmp_code);
        }

        let tup = self.output_fields[0].clone();
        let field_name = tup.0;
        let field_type = tup.1;
        let mapped_type = CPP_TYPES.get(field_type.as_str()).unwrap();

        let tmp_code = format!(
            indoc! {
                "
                {mapped_type} {field_name};
                {field_name} = {function_name}({input_args});
                cout<<{field_name};
                "
            }
            ,
            field_name = field_name,
            mapped_type = mapped_type,
            function_name = self.function_name,
            input_args = input_args.join(", ")
        );

        code.push_str(&tmp_code);

        let full_code = format!(
            indoc!{
                "
                ##CODE_HERE##

                int main() {{
                    {code}

                    return 0;
                }}
                "
            },
            code = code
        );


        fs::write(output_path, full_code).unwrap();

        Ok(())
    }


    pub fn generate_js_code_partial(&self, output_path: String) -> anyhow::Result<()> {
        let tup = self.output_fields[0].clone();
        let key = tup.1;

        let return_type = JS_TYPES.get(key.as_str());

        let mut input_args_str: String = String::from("");

        let input_len = self.input_fields.len();

        for (index, (field_name, field_type)) in self.input_fields.iter().enumerate() {
            let mapped_type = JS_TYPES.get(field_type.as_str());
            if mapped_type.is_none() {
                return Err(anyhow!("Unsupported js input arg type"));
            }

            let args_str = format!("{}", field_name);

            input_args_str.push_str(&args_str);
            if index != input_len - 1 {
                input_args_str.push_str(", ");
            }
        }

        let code = format!(
            indoc!{
                "
                const fs = require('fs');

                function {}({}) {{
                    //code goes here
                }}"
            },
            self.function_name, input_args_str
        );

        fs::write(output_path, code).unwrap();

        Ok(())
    }    

    pub fn generate_js_code_full(&self, output_path: String) -> anyhow::Result<()> {
        let mut input_args = Vec::<String>::new();
        
        let mut code: String = String::from("");
        code.push_str(&format!(
            indoc! {
                r"
                let input = fs.readFileSync('/dev/stdin', 'utf8').trim().split('\n').join(' ').split(' ');
                "
            }
        ));

        for (index, (field_name, field_type)) in self.input_fields.iter().enumerate() {
            input_args.push(field_name.to_string());
            
            let mut tmp_code: Option<String> = None;
            if field_type.starts_with("list<") {
                if field_type == "list<string>" {
                    tmp_code =  Some(format!(
                        indoc! {
                            "
                            let size_arr{index} = parseInt(input.shift());
                            let {field_name} = input.splice(0, size_arr{index});
                            "
                        }
                        , 
                        index = index, 
                        field_name = field_name
                    ));
                } else if field_type == "list<int>" {
                    tmp_code =  Some(format!(
                        indoc! {
                            "
                            let size_arr{index} = parseInt(input.shift());
                            let {field_name} = input.splice(0, size_arr{index}).map(num => parseInt(num));
                            "
                        }
                        , 
                        index = index, 
                        field_name = field_name
                    ));

                } else if field_type == "list<float>" {
                    tmp_code =  Some(format!(
                        indoc! {
                            "
                            let size_arr{index} = parseInt(input.shift());
                            let {field_name} = input.splice(0, size_arr{index}).map(num => parseFloat(num));
                            "
                        }
                        , 
                        index = index, 
                        field_name = field_name
                    ));
                }
            } else {
                if field_type == "string" {
                    tmp_code =  Some(format!(
                        indoc! {
                            "
                            let {field_name} = input.shift();
                            "
                        }
                        , 
                        field_name = field_name
                    ));
                } else if field_type == "int" {
                    tmp_code =  Some(format!(
                        indoc! {
                            "
                            let {field_name} = parseInt(input.shift());
                            "
                        }
                        , 
                        field_name = field_name
                    ));
                } else if field_type == "float" {
                    tmp_code =  Some(format!(
                        indoc! {
                            "
                            let {field_name} = parseFloat(input.shift());
                            "
                        }
                        , 
                        field_name = field_name
                    ));
                } else if field_type == "bool" {
                    tmp_code =  Some(format!(
                        indoc! {
                            "
                            let {field_name} = Boolean(input.shift());
                            "
                        }
                        , 
                        field_name = field_name
                    ));
                }
            }

            if tmp_code.is_none()  {
                return Err(anyhow!("tmp_code is not initialised in generate_js_code_full"));
            }

            code.push_str(&tmp_code.unwrap());
        }

        let field_name = self.output_fields[0].clone().0;

        let tmp_code = format!(
            indoc! {
                "
                let {field_name};
                {field_name} = {function_name}({input_args});
                console.log({field_name});
                "
            }
            ,
            field_name = field_name,
            function_name = self.function_name,
            input_args = input_args.join(", ")
        );

        code.push_str(&tmp_code);

        let full_code = format!(
            indoc!{
                "
                ##CODE_HERE##

                {code}
                "
            },
            code = code
        );


        fs::write(output_path, full_code).unwrap();

        Ok(())
    }


    pub fn generate_rust_code_partial(&self, output_path: String) -> anyhow::Result<()> {
        let tup = self.output_fields[0].clone();
        let key = tup.1;

        let return_type = RUST_TYPES.get(key.as_str());

        let mut input_args_str: String = String::from("");

        let input_len = self.input_fields.len();

        for (index, (field_name, field_type)) in self.input_fields.iter().enumerate() {
            let mapped_type = RUST_TYPES.get(field_type.as_str());
            if mapped_type.is_none() {
                return Err(anyhow!("Unsupported rust input arg type"));
            }
            
            let args_str = format!("{}: {}", field_name, mapped_type.unwrap());

            input_args_str.push_str(&args_str);
            if index != input_len - 1 {
                input_args_str.push_str(", ");
            }
        }

        let code = format!(
            indoc!{
                "use std::io::{{self, Read}};

                fn {}({}) -> {} {{
                    //code goes here
                }}"
            },
            self.function_name, input_args_str, return_type.unwrap(), 
        );

        fs::write(output_path, code).unwrap();

        Ok(())
    }

    pub fn generate_rust_code_full(&self, output_path: String) -> anyhow::Result<()> {
        let mut input_args = Vec::<String>::new();

        let mut code: String = String::from("");
        code.push_str(&format!(
            indoc! {"
                let mut input = String::new();
                io::stdin().read_to_string(&mut input).unwrap();

                let mut it = input.split_whitespace();
            "}
        ));

        for (index, (field_name, field_type)) in self.input_fields.iter().enumerate() {
            input_args.push(field_name.to_string());
            let mapped_type = RUST_TYPES.get(field_type.as_str()).unwrap();
            
            let mut tmp_code: Option<String> = None;
            if field_type.starts_with("list<") {
                if field_type == "list<string>" {
                    tmp_code =  Some(format!(
                        indoc! {
                            "
                            let size_arr{index}: usize = it.next().unwrap().parse().unwrap();
                            
                            let {field_name}:{mapped_type} = Vec::new();
                            for i in 0..size_arr{index} {{
                                {field_name}.push(it.next().unwrap());
                            }}
                            "
                        }
                        , 
                        index = index, 
                        mapped_type = mapped_type,
                        field_name = field_name
                    ));
                } else if field_type == "list<int>" {
                    tmp_code =  Some(format!(
                        indoc! {
                            "
                            let size_arr{index}: usize = it.next().unwrap().parse().unwrap();
                            
                            let {field_name}:{mapped_type} = Vec::new();
                            for i in 0..size_arr{index} {{
                                {field_name}.push(it.next().unwrap().parse::<i32>().unwrap());
                            }}
                            "
                        }
                        , 
                        index = index, 
                        mapped_type = mapped_type,
                        field_name = field_name
                    ));
                } else if field_type == "list<float>" {
                    tmp_code =  Some(format!(
                        indoc! {
                            "
                            let size_arr{index}: usize = it.next().unwrap().parse().unwrap();
                            
                            let mut {field_name}:{mapped_type} = Vec::new();
                            for _i in 0..size_arr{index} {{
                                {field_name}.push(it.next().unwrap().parse::<f32>().unwrap());
                            }}
                            "
                        }
                        , 
                        index = index, 
                        mapped_type = mapped_type,
                        field_name = field_name
                    ));
                }
            } else {
                tmp_code =  Some(format!(
                    indoc! {
                        "
                        let {field_name} = it.next().unwrap().parse::<{mapped_type}>().unwrap();
                        "
                    }
                    , 
                    mapped_type = mapped_type,
                    field_name = field_name
                ));
            }

            if tmp_code.is_none()  {
                return Err(anyhow!("tmp_code is not initialised in generate_js_code_full"));
            }

            code.push_str(&tmp_code.unwrap());
        }

        let tup = self.output_fields[0].clone();
        let field_name = tup.0;
        let field_type = tup.1;
        let mapped_type = RUST_TYPES.get(field_type.as_str()).unwrap();

        let tmp_code = format!(
            indoc! {
                "
                let {field_name}: {mapped_type} ;
                {field_name} = {function_name}({input_args});
                println!(\"{{}}\", {field_name});
                "
            }
            ,
            field_name = field_name,
            mapped_type = mapped_type,
            function_name = self.function_name,
            input_args = input_args.join(", ")
        );

        code.push_str(&tmp_code);

        let full_code = format!(
            indoc!{
                "
                ##CODE_HERE##

                pub fn main() {{
                    {code}
                }}
                "
            },
            code = code
        );


        fs::write(output_path, full_code).unwrap();

        Ok(())
    }
}