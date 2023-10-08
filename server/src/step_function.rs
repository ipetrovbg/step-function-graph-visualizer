use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LambdaFunction {
    pub handler: String,
    pub timeout: Option<i8>,
    #[serde(rename = "memorySize")]
    pub memory_size: Option<i16>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Serverless {
    #[serde(rename = "stepFunctions")]
    pub step_functions: Option<StepFunctions>,
    pub functions: Option<BTreeMap<String, LambdaFunction>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StateMachineDefinition {
    #[serde(rename = "Comment")]
    pub comment: Option<String>,
    #[serde(rename = "StartAt")]
    pub start_at: String,
    #[serde(rename = "States")]
    pub states: BTreeMap<String, Step>,
}

impl Display for StateMachineDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.comment {
            None => {}
            Some(comment) => {
                writeln!(f, "  \"Comment\": \"{}\",", comment)?;
            }
        }

        writeln!(f, "  \"StartAt\": \"{}\",", self.start_at)?;

        if !self.states.is_empty() {
            writeln!(f, "  \"States\": {{")?;
            let mut iteration = 1;

            for (key, step) in self.states.iter() {
                writeln!(f, "    \"{}\": {{", key)?;
                print!("    {}", step);

                if let Some(_) = step.end {
                    if (iteration) == self.states.iter().len() {
                        writeln!(f, "    }}")?;
                    } else {
                        writeln!(f, "    }},")?;
                    }
                } else {
                    if (iteration) == self.states.iter().len() {
                        writeln!(f, "    }}")?;
                    } else {
                        writeln!(f, "    }},")?;
                    }
                }

                iteration += 1;
            }

            writeln!(f, "  }}")?;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Type {
    Task,
    Pass,
    Choice,
    Fail,
    Succeed,
    Wait,
    Map,
    Parallel,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Task => {
                writeln!(f, "  \"Type\": \"Task\",")?;
            }
            Type::Pass => {
                writeln!(f, "  \"Type\": \"Pass\",")?;
            }
            Type::Choice => {
                writeln!(f, "  \"Type\": \"Choice\",")?;
            }
            Type::Fail => {
                writeln!(f, "  \"Type\": \"Fail\",")?;
            }
            Type::Succeed => {
                writeln!(f, "  \"Type\": \"Succeed\"")?;
            }
            Type::Wait => {
                writeln!(f, "  \"Type\": \"Wait\",")?;
            }
            Type::Map => {
                writeln!(f, "  \"Type\": \"Map\",")?;
            }
            Type::Parallel => {
                writeln!(f, "  \"Type\": \"Parallel\",")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Step {
    #[serde(rename = "Type")]
    pub step_type: Type,

    #[serde(rename = "End")]
    pub end: Option<bool>,

    #[serde(rename = "Next")]
    pub next: Option<String>,

    #[serde(rename = "Resource")]
    pub resource: Option<Value>,

    #[serde(rename = "ResultPath")]
    pub result_path: Option<String>,

    #[serde(rename = "Choices")]
    pub choices: Option<Vec<Choice>>,

    #[serde(rename = "Catch")]
    pub catch: Option<Vec<Catch>>,

    #[serde(rename = "Default")]
    pub default: Option<String>,
}

impl Display for Step {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        print!("  {}", self.step_type);
        match self.step_type {
            Type::Task => {
                match &self.resource {
                    None => {}
                    Some(resource) => match resource {
                        Value::String(resource) => {
                            writeln!(f, "        \"Resource\": \"{}\",", resource)?;
                        }
                        Value::Mapping(resource) => {
                            for resource in resource.iter() {
                                match resource.0.as_str() {
                                    None => {}
                                    Some(get_attr) => {
                                        if get_attr == "Fn::GetAtt" {
                                            let attr = resource.1.as_sequence().unwrap();
                                            let lambda_name =
                                                attr.first().unwrap().as_str().unwrap();
                                            write!(f, "        \"Resource\": \"arn:aws:states:::lambda:invoke\",")?;
                                            writeln!(f, "")?;
                                            writeln!(f, "        \"Parameters\": {{")?;
                                            write!(f,
                                                       "            \"FunctionName\": \"arn:aws:lambda:eu-central-1:00000000000:function:{}\"",
                                                       lambda_name
                                                )?;
                                            writeln!(f, "")?;
                                            writeln!(f, "        }},")?;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    },
                }

                match &self.result_path {
                    None => {}
                    Some(result_path) => {
                        writeln!(f, "        \"ResultPath\": \"{}\",", result_path)?;
                    }
                }
            }
            Type::Choice => match &self.choices {
                None => {}
                Some(choices) => {
                    let length = choices.len();
                    let mut current: usize = 1;
                    writeln!(f, "        \"Choices\": [")?;
                    for choice in choices {
                        writeln!(f, "            {{")?;
                        println!("                {}", &choice);
                        if current == length {
                            writeln!(f, "            }}")?;
                        } else {
                            writeln!(f, "            }},")?;
                        }
                        current = current + 1;
                    }
                    write!(f, "        ]")?;
                }
            },
            _ => {}
        }

        if let Some(default) = &self.default {
            writeln!(f, ",")?;
            write!(f, "        \"Default\": \"{}\"", default)?;
        }

        match self.end {
            None => {}
            Some(end) => {
                if end {
                    write!(f, "        \"End\": true")?;
                }
            }
        }

        if let Some(catches) = &self.catch {
            writeln!(f, "       \"Catch\": [")?;
            let mut iteration = 1;
            for catch in catches {
                println!("          {{");
                println!("              {}", catch);
                if iteration == catches.len() {
                    println!("          }}");
                } else {
                    println!("          }},");
                }
                iteration += 1;
            }
            println!("       ],");
        }

        match &self.next {
            None => {
                writeln!(f, "")?;
            }
            Some(next) => {
                writeln!(f, "        \"Next\": \"{}\"", next)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Catch {
    #[serde(rename = "ErrorEquals")]
    pub error_equals: Vec<String>,
    #[serde(rename = "Next")]
    pub next: String,
    #[serde(rename = "ResultPath")]
    pub result_path: String,
}
impl Display for Catch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\"ErrorEquals\": [")?;
        let mut iter = 1;
        for error_equal in &self.error_equals {
            if iter == self.error_equals.len() {
                writeln!(f, "               \"{}\"", error_equal)?;
            } else {
                writeln!(f, "               \"{}\",", error_equal)?;
            }
            iter += 1;
        }
        writeln!(f, "              ],")?;

        writeln!(
            f,
            "              \"ResultPath\": \"{}\",",
            &self.result_path
        )?;

        writeln!(f, "              \"Next\": \"{}\"", &self.next)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Choice {
    #[serde(rename = "Variable")]
    variable: String,

    #[serde(rename = "IsPresent")]
    is_present: Option<bool>,

    #[serde(rename = "BooleanEquals")]
    bool_equals: Option<bool>,

    #[serde(rename = "Next")]
    pub next: Option<String>,
}

impl Display for Choice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"Variable\": \"{}\"", self.variable)?;
        match self.is_present {
            None => {}
            Some(is_present) => {
                if is_present {
                    write!(f, ",")?;
                    writeln!(f, "")?;
                    writeln!(f, "                \"IsPresent\": true,")?;
                } else {
                    write!(f, ",")?;
                    writeln!(f, "")?;
                    writeln!(f, "                \"IsPresent\": false,")?;
                }
            }
        }

        match self.bool_equals {
            None => {}
            Some(bool_equals) => {
                if bool_equals {
                    write!(f, ",")?;
                    writeln!(f, "")?;
                    writeln!(f, "                \"BooleanEquals\": true,")?;
                } else {
                    write!(f, ",")?;
                    writeln!(f, "")?;
                    writeln!(f, "                \"BooleanEquals\": false,")?;
                }
            }
        }

        match &self.next {
            None => {}
            Some(next) => {
                write!(f, "                \"Next\": \"{}\"", next)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StateMachines {
    pub name: String,
    pub definition: StateMachineDefinition,
}

impl Display for StateMachines {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        print!("{}", self.definition);
        writeln!(f, "}}")?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StepFunctions {
    pub validate: bool,
    #[serde(rename = "stateMachines")]
    pub state_machines: BTreeMap<String, StateMachines>,
}
