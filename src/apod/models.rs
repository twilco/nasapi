/// Model of a single Astronomy Pic of the Day (APOD)
#[derive(Debug, Deserialize)]
pub struct Pic {
    /// Copyright may or may not be there.  "...an optional return parameter copyright is returned
    /// if the image is not public domain." - https://api.nasa.gov/api.html#apod
    copyright: Option<String>,
    date: String,
    explanation: String,
    /// The URL for any high-resolution image for that day. Returned regardless of 'hd' param setting
    /// but will be omitted in the response if it does not exist originally at APOD. https://github.com/nasa/apod-api
    hdurl: Option<String>,
    media_type: String,
    service_version: String,
    title: String,
    url: String
}