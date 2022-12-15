use serde_derive::{Deserialize, Serialize};


/*
SayHiRequest is the request struct that is used to send to sayhi api.
 */
#[derive(Deserialize, Serialize)]
pub struct SayHiRequest {
    pub name: String,
    pub age: u32,
}

/*
SayHiResponse is the response struct that is used to receive the response of sayhi api.
 */
#[derive(Deserialize, Serialize)]
pub struct SayHiResponse {
    pub code: u32,
    pub message: String,
}

const BASE_URL: &str = "http://localhost:8088";
const SAYHI_PATH: &str = "/sayhi";
/*
sayhi is the function that is used to send request to sayhi api.
 */
pub async fn sayhi(req: SayHiRequest) -> Result<SayHiResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!("{}{}", BASE_URL, SAYHI_PATH);
    let resp = client.post(&url).json(&req).send().await;
    return match resp {
        Ok(resp) => {
            let resp = resp.json::<SayHiResponse>().await;
            match resp {
                Ok(resp) => Ok(resp),
                Err(err) => Err(Box::new(err)),
            }
        }
        Err(err) => Err(Box::new(err)),
    };
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[tokio::test]
    async fn test_sayhi() {
        let req = SayHiRequest {
            name: "John".to_string(),
            age: 18,
        };
        let resp = sayhi(req).await.unwrap();
        assert_eq!(resp.code, 1000);
        assert_eq!(resp.message, format!("Hi, {}, you are {} years old.", "John", 18));
    }

    #[rstest]
    #[case("Louis", 18, "Hi, Louis, you are 18 years old.")]
    #[case("Tom", 20, "Hi, Tom, you are 20 years old.")]
    #[tokio::test]
    async fn test_sayhi_with_rstest(#[case] name: &str, #[case] age: u32, #[case] message: &str) {
        let req = SayHiRequest {
            name: name.to_string(),
            age,
        };
        let resp = sayhi(req).await.unwrap();
        assert_eq!(resp.code, 1000);
        assert_eq!(resp.message, message);
    }
}