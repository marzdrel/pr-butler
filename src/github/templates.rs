use regex::Regex;

pub fn gh_pull_requests(
    github_org: String,
    github_repo: String,
) -> String {
    let query_template = r#"
      { 
        repository(owner:"$GITHUB_ORG", name:"$GITHUB_REPO") {
          label(name: "conflicting") {
            id
          }
          pullRequests(last: 100, states: OPEN) {
            edges {
              node {
                id
                title
                number
                mergeable
              }
            }
          }
        }
      }
    "#;

    prepare_query(
        query_template,
        vec![("$GITHUB_ORG", github_org), ("$GITHUB_REPO", github_repo)],
    )
}

pub fn gh_add_labels(label_id: String, labelable_id: String) -> String {
    let query_template = r#"
        mutation {
          addLabelsToLabelable(input:
            {
              labelIds:"$LABEL_ID",
              labelableId:"$LABELABLE_ID"
            }
          ) 
          {
            clientMutationId
          }
        }
    "#;

    prepare_query(
        query_template,
        vec![("$LABEL_ID", label_id), ("$LABELABLE_ID", labelable_id)],
    )
}

trait Wrappable {
    fn wrap_query(self) -> String;
}

impl Wrappable for String {
    fn wrap_query(self) -> String {
        format!("{{ \"query\": \"{}\" }}", self)
    }
}

fn prepare_query(
    query_template: &str,
    vars: Vec<(&str, String)>,
) -> String {
    let mut base_scope = Regex::new(r"\s+")
        .unwrap()
        .replace_all(query_template, " ")
        .replace("\n", "")
        .replace("\"", "\\\"")
        .wrap_query();

    for (key, value) in vars {
        base_scope = base_scope.replace(key, &value);
    }

    base_scope.to_string()
}
