use anyhow::Result;
use surrealism::surrealism;

// use some_crate::weight;

// #[surrealism(init)]
// fn init() -> Result<(), String> {
//     // let _: () = surrealism::sql(r#"
//     //     DEFINE TABLE demo_module_data;
//     //     // some fields
//     // "#).unwrap();

//     // Simulate some initialization that could fail
//     if std::env::var("FAIL_INIT").is_ok() {
//         Err("Initialization failed due to environment variable".to_string())
//     } else {
//         Ok(())
//     }
// }

#[surrealism]
fn can_drive(age: i64) -> bool {
    age >= 18

    // surrealism::ml::some_sys_call()
}

#[surrealism]
fn create_user((name, age): (String, i64), enabled: bool) -> Result<String> {
    let exists: bool =
        surrealism::run("fn::user_exists".to_string(), None, (name.clone(), age))?;
    if exists {
        return Ok(format!("User {name} already exists"));
    }
    Ok(format!("Created user {name} of age {age}. Enabled? {enabled}"))
}

#[surrealism(name = "other")]
fn can_drive_bla(age: i64) -> bool {
    age >= 18
}

#[surrealism(default)]
fn def(age: i64) -> bool {
    age >= 18
}

// Test function that returns a Result
#[surrealism]
fn safe_divide(a: i64, b: i64) -> Result<i64, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// Test function with a different error type
#[surrealism]
fn parse_number(input: String) -> Result<i64, std::num::ParseIntError> {
    input.parse::<i64>()
}

// pipeline = pipeline(task="automatic-speech-recognition", model="openai/whisper-large-v3")
// pipeline("https://huggingface.co/datasets/Narsil/asr_dummy/resolve/main/mlk.flac")
// {'text': ' I have a dream that one day this nation will rise up and live out the true meaning of its creed.'}

// calculate sentiment of a question (is the user happy or not)
#[surrealism]
fn js_support_agent_sentiment(question: String) -> Result<i64> {
    let tokenizer = "mistral-7b-instruct-v0.1";
    let model = "bert-base-uncased";
    let tokenized = surrealism::ml::tokenize(tokenizer, question)?; // do we also pass weights?
    surrealism::ml::invoke_model(model, tokenized, 100)
}

// generate a response to a question
#[surrealism]
fn js_support_agent_response(question: String) -> Result<String> {
    let prompt = format!(
        "<system>You're a support agent whos an expert in javascript, give a good response to the user so that they dont churn in our product.</system><user>{question}</user>"
    );
    let model = "mistral-7b-instruct-v0.1";
    surrealism::ml::invoke_model(model, prompt, 100)
}

#[surrealism]
fn js_support_agent_response_2(a: String) -> Result<String> {
    Ok(a)
}

#[surrealism]
fn result(should_fail: bool) -> Result<String> {
    if should_fail {
        anyhow::bail!("Failed")
    } else {
        Ok("Success".to_string())
    }
}

