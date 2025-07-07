use surrealism::surrealism;

// use some_crate::weight;

// #[surrealism(init)]
// fn init() {
//     surrealism::ml::load_weights("mistral-7b-instruct-v0.1", []);
// }

#[surrealism]
fn can_drive(age: i64) -> bool {
    age >= 18

    // surrealism::ml::some_sys_call()
}

#[surrealism]
fn create_user((name, age): (String, i64), enabled: bool) -> String {
    let exists: bool =
        surrealism::run("fn::user_exists".to_string(), None, (name.clone(), age)).unwrap();
    if exists {
        return format!("User {name} already exists");
    }
    format!("Created user {name} of age {age}. Enabled? {enabled}")
}

#[surrealism(name = "other")]
fn can_drive_bla(age: i64) -> bool {
    age >= 18
}

#[surrealism(default)]
fn def(age: i64) -> bool {
    age >= 18
}

// pipeline = pipeline(task="automatic-speech-recognition", model="openai/whisper-large-v3")
// pipeline("https://huggingface.co/datasets/Narsil/asr_dummy/resolve/main/mlk.flac")
// {'text': ' I have a dream that one day this nation will rise up and live out the true meaning of its creed.'}

// calculate sentiment of a question (is the user happy or not)
#[surrealism]
fn js_support_agent_sentiment(question: String) -> i64 {
    let tokenizer = "mistral-7b-instruct-v0.1".to_string();
    let model = "bert-base-uncased".to_string();
    let tokenized = surrealism::ml::tokenize(tokenizer, question).unwrap(); // do we also pass weights?
    surrealism::ml::invoke_model(model, tokenized, 100).unwrap()
}

// generate a response to a question
#[surrealism]
fn js_support_agent_response(question: String) -> String {
    let prompt = format!(
        "<system>You're a support agent whos an expert in javascript, give a good response to the user so that they dont churn in our product.</system><user>{question}</user>"
    );
    let model = "mistral-7b-instruct-v0.1".to_string();
    surrealism::ml::invoke_model(model, prompt, 100).unwrap()
}
