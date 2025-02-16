use dioxus::{html::h1,  prelude::*};
use dioxus_router::prelude::*;

const SOCIAL_ASSETS: Asset = asset!("/assets/main.css");
const PROJECTS_ASSETS: Asset = asset!("assets/projects.css");
const GITHUB_ICON: Asset = asset!("/assets/github.svg"); 
const LINKED_IN_ICON: Asset = asset!("assets/linkedin.svg");
const HUGGING_FACE_ICON: Asset = asset!("assets/Hugging_Face_idJ6-I79C__0.svg");
const KAGGLE_ICON: Asset = asset!("assets/Kaggle_idAheRAizH_1.svg");
const TWITTER_ICON:Asset = asset!("assets/twitter-x.svg");
const PROJECTS_HEAD_ICON: Asset = asset!("assets/codeing_projects.svg");
const CV_ASSET: Asset = asset!("assets/my_cv/Abdelrahman Mostafa_250201_181502.pdf");
const CV_ICON: Asset = asset!("assets/my_cv/iconmonstr-cv-3.svg");
#[derive(Props,Clone,PartialEq)]
struct ProjectProps {
    name_of_project: &'static str,
    link_of_project: &'static str,
    image_of_project: &'static str,
}
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/projects")]
    Projects {},
    #[route("/cv")]
    Cv{}
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {
            config: Callback::new(|_| RouterConfig::default()),
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        head {
            meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
            document::Stylesheet { href: SOCIAL_ASSETS }
        }
        Social_media_toolbar {}
        typing_Name{}
        MY_avatar{}
    }
}

#[component]
fn Projects() -> Element {
    let projects_ = vec![
        ProjectProps {
            name_of_project: "Chatacter", 
            link_of_project: "https://github.com/AlphaSphereDotAI/chatacter", 
            image_of_project: "", 
        },
        ProjectProps {
            name_of_project: "Data-Analyst Multi-Agent (DataVerse)", 
            link_of_project: "https://github.com/Eng-Abdelrahman-Mostafa-Mohamed/Agents", 
            image_of_project: "", 
        },

        ProjectProps {
            name_of_project: "Transformer From Scratch", 
            link_of_project: "https://github.com/Eng-Abdelrahman-Mostafa-Mohamed/Transformer-From-Scratch-", 
            image_of_project: "", 
        },
        ProjectProps {
            name_of_project: "For More Projects See My Kaggle and GitHub", 
            link_of_project: "", 
            image_of_project: "", 
        },

    ];

    rsx! {
        div {
            class: "projects-page",
            document::Stylesheet { href: PROJECTS_ASSETS }
            document::Stylesheet { href: SOCIAL_ASSETS }

            Social_media_toolbar {}
            div {
                class: "projects-grid",
                    {projects_.into_iter().map(|project| {      
                        rsx! {
                            projects_pannel { ..project } // Spread syntax is fine
                        }
                    }
                    
                )}
            }
        }
    }
}


#[component]
pub fn Social_media_toolbar() -> Element {
    rsx! {
        div {
            id: "social_media",
            div {
                id: "links",
                div { // Changed to a div container
                    a {
                        href: "https://github.com/Eng-Abdelrahman-Mostafa-Mohamed",
                        button { 
                            img { src: GITHUB_ICON, width: "30", height: "30"}
                        }
                    }
                    a {
                        href: "https://www.linkedin.com/in/abdelrahman-mostafa-mohamed/",
                        button { // Use a <button> element
                            img { src: LINKED_IN_ICON, width: "30", height: "30"}
                        }
                    }
                    a {
                        href: "https://huggingface.co/Abdelrahman-Mostafa",
                        button {
                            img { src:HUGGING_FACE_ICON, width: "30", height: "30", alt: "Hugging Face" }
                        }
                    }
                    a {
                        href: "https://www.kaggle.com/abdelrahmanm2003",
                        button {
                            img { src: KAGGLE_ICON, width: "30", height: "30", alt: "Kaggle" }
                        }
                    }
                    a {
                        href: "https://x.com/abdelra01699238",
                        button {
                            img { src: TWITTER_ICON, width: "30", height: "30", alt: "X (formerly Twitter)" }
                        }
                    }
                    a {
                        href: "./projects",
                        class: "project-link",
                        button {
                            img { src: PROJECTS_HEAD_ICON, width: "10", height: "10"}

                        }
                    }
                    a {
                        href: "./cv",
                        class: "MY_CV",
                        button {
                            img { src: CV_ICON, width: "10", height: "10"}

                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn projects_pannel(props: ProjectProps) -> Element {
    rsx! {
        head {
            meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
            // document::Stylesheet { href: SOCIAL_ASSETS }
        }
        div {
            class: "project-panel",
            a {
                href: "{props.link_of_project}",
                class: "project-link",
                div {
                    class: "project-content",
                    h2 {
                        class: "project-title",
                        "{props.name_of_project}"
                    }
                }
            }
        }
    }
}


#[component]
pub fn typing_Name()->Element{
    rsx!{
        head {
            meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
            // document::Stylesheet { href: SOCIAL_ASSETS }
        }
        div {   
            id:"typing_name",
            h1{"Abdelrahman Mostafa Mohamed"}
            h3{"AI Engineer - Graduated from FCAI, Cairo University"}
        }
       
    }
}



#[component]
pub fn MY_avatar() -> Element {
    rsx!(
        div {
            id: "my_avatar",
            style: "background-image: url({asset!(\"./assets/me.jpg\")});" 
        }
    )
}

#[component]
pub fn Cv() -> Element {
    rsx! {
        div {
            class: "cv-page", // Add a class for styling
            document::Stylesheet { href: SOCIAL_ASSETS }

            Social_media_toolbar {}
            iframe { 
                id: "cv",
                src: CV_ASSET, 
                width: "90%", 
                height: "800px", 


            }
        }
    }
}