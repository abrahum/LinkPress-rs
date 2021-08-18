use crate::logger::info;
use axum::prelude::{extract, response};
use axum::response::IntoResponse;
use urlencoding::decode;

fn init(
    md_string: String,
    type_: &str,
) -> (
    crate::markdown::MarkdownParserResult,
    crate::utils::Dir,
    tera::Tera,
) {
    let lp_config = crate::config::load_config();
    let cwd = std::path::PathBuf::from(".");
    (
        crate::markdown::build_pagedata(type_, md_string, &lp_config),
        crate::utils::build_dir(&cwd),
        crate::utils::get_tera(&cwd.join("themes").join(&lp_config.site.theme)),
    )
}

fn to_resp(type_: &str, context: impl serde::Serialize, tera: &tera::Tera) -> impl IntoResponse {
    response::Html(
        tera.render(type_, &tera::Context::from_serialize(&context).unwrap())
            .unwrap(),
    )
}

pub async fn index() -> impl IntoResponse {
    info(format!("get'/'"));
    let (mut context, dir_tree, tera) = init(String::new(), "index");
    context.index = Some(dir_tree.build_index("posts"));
    to_resp("index", context, &tera)
}

pub async fn tags_index() -> impl IntoResponse {
    info(format!("get'/tags'"));
    let (mut context, dir_tree, tera) = init(String::new(), "index");
    context.tags_index = crate::utils::build_tag_vec(&dir_tree);
    to_resp("tags", context, &tera)
}

pub async fn tag_pages(extract::Path(tag): extract::Path<String>) -> impl IntoResponse {
    info(format!("get'/tags/{}'", tag));
    let (mut context, dir_tree, tera) = init(String::new(), "index");
    context.index = Some(dir_tree.build_tags_index().get(&tag).unwrap().clone());
    to_resp("index", context, &tera)
}

pub async fn types_index(extract::Path(type_): extract::Path<String>) -> impl IntoResponse {
    info(format!("get'/{}'", type_));
    let (mut context, dir_tree, tera) = init(String::new(), "index");
    context.index = Some(dir_tree.build_index(&type_));
    to_resp("index", context, &tera)
}

pub async fn types_pages(
    extract::Path((type_, name)): extract::Path<(String, String)>,
) -> impl IntoResponse {
    let name = decode(&name).unwrap();
    info(format!("get'/{}/{}'", type_, name));
    let template_name = type_.clone();
    let (context, _, tera) = init(
        crate::utils::get_page(&template_name, &name).unwrap(),
        &template_name,
    );
    // if let Some(t) = &context.front_matter.template {
    //     template_name = t.clone();
    // }
    to_resp(&template_name, context, &tera)
}
