pub struct TitleField(String);

impl TitleField {
    pub fn parse(s: String) -> Result<TitleField, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        print!("is_empty_or_whitespace {}", is_empty_or_whitespace);
        if is_empty_or_whitespace {
            Err(format!("{} is not a valid subscriber name.", s))
        } else {
            Ok(Self(s))
        }
    }

    pub fn inner(self) -> String {
        self.0
    }

    pub fn inner_mut(&mut self) -> &mut str {
        &mut self.0
    }

    pub fn inner_ref(&self) -> &str {
        &self.0
    }
}

#[derive(serde::Serialize)]
pub struct Response {
    pub status: String,
    pub message: String
}

#[derive(serde::Serialize)]
pub struct ResponseWithData<T> {
    pub status: String,
    pub message: String,
    pub data: T
}