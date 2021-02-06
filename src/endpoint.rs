const URL: &str = "https://hacker-news.firebaseio.com";
const VERSION: &str = "v0";

pub(crate) struct Endpoint;

impl Endpoint {
    fn build(arg: String) -> String {
        format!("{}/{}/{}", URL, VERSION, arg)
    }

    pub(crate) fn item(id: u64) -> String {
        Endpoint::build(format!("item/{}.json", id))
    }

    pub(crate) fn user(name: &str) -> String {
        Endpoint::build(format!("user/{}.json", name))
    }

    pub(crate) fn maxitem() -> String {
        Endpoint::build("maxitem.json".into())
    }

    pub(crate) fn topstories() -> String {
        Endpoint::build("topstories.json".into())
    }

    pub(crate) fn newstories() -> String {
        Endpoint::build("newstories.json".into())
    }

    pub(crate) fn beststories() -> String {
        Endpoint::build("beststories.json".into())
    }

    pub(crate) fn askstories() -> String {
        Endpoint::build("askstories.json".into())
    }

    pub(crate) fn showstories() -> String {
        Endpoint::build("showstories.json".into())
    }

    pub(crate) fn updates() -> String {
        Endpoint::build("updates.json".into())
    }
}
