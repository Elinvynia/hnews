const URL: &str = "https://hacker-news.firebaseio.com";
const VERSION: &str = "v0";

pub(crate) struct Endpoint;

impl Endpoint {
    fn build(arg: String) -> String {
        format!("{}/{}/{}", URL, VERSION, arg)
    }

    pub(crate) fn item(id: u64) -> String {
        Endpoint::build(format!("item/{}", id))
    }

    pub(crate) fn user(name: &str) -> String {
        Endpoint::build(format!("user/{}", name))
    }

    pub(crate) fn maxitem() -> String {
        Endpoint::build("maxitem".into())
    }

    pub(crate) fn topstories() -> String {
        Endpoint::build("topstories".into())
    }

    pub(crate) fn newstories() -> String {
        Endpoint::build("newstories".into())
    }

    pub(crate) fn beststories() -> String {
        Endpoint::build("beststories".into())
    }

    pub(crate) fn askstories() -> String {
        Endpoint::build("askstories".into())
    }

    pub(crate) fn showstories() -> String {
        Endpoint::build("showstories".into())
    }

    pub(crate) fn updates() -> String {
        Endpoint::build("updates".into())
    }
}
