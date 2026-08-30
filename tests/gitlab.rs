#![expect(
    missing_docs,
    clippy::missing_errors_doc,
    clippy::too_many_lines,
    reason = "Tests"
)]

use core::error::Error;

use wayfind::RouterBuilder;

#[path = "../benches/fixtures/gitlab_routes.rs"]
mod gitlab_routes;

#[test]
fn gitlab() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    for (index, route) in gitlab_routes::routes().iter().enumerate() {
        builder.insert(route, index)?;
    }

    let router = builder.build();
    insta::assert_snapshot!(router, @"
    /
    ├─ -/
    │  ├─ a
    │  │  ├─ buse_reports
    │  │  │  ╰─ /
    │  │  │     ╰─ add_category
    │  │  │        ╰─ /
    │  │  ├─ cme-challenge
    │  │  │  ╰─ /
    │  │  ╰─ utocomplete/
    │  │     ├─ award_emojis
    │  │     │  ╰─ /
    │  │     ├─ deploy_keys_with_owners
    │  │     │  ╰─ /
    │  │     ├─ group_subgroups
    │  │     │  ╰─ /
    │  │     ├─ merge_request_
    │  │     │  ├─ source_branches
    │  │     │  │  ╰─ /
    │  │     │  ╰─ target_branches
    │  │     │     ╰─ /
    │  │     ├─ namespace_routes
    │  │     │  ╰─ /
    │  │     ├─ project
    │  │     │  ├─ _
    │  │     │  │  ├─ groups
    │  │     │  │  │  ╰─ /
    │  │     │  │  ╰─ routes
    │  │     │  │     ╰─ /
    │  │     │  ╰─ s
    │  │     │     ╰─ /
    │  │     ╰─ users
    │  │        ╰─ /
    │  │           ╰─ <id>
    │  │              ╰─ /
    │  ├─ c
    │  │  ├─ haos/
    │  │  │  ├─ cpu_spin
    │  │  │  │  ╰─ /
    │  │  │  ├─ db_spin
    │  │  │  │  ╰─ /
    │  │  │  ├─ gc
    │  │  │  │  ╰─ /
    │  │  │  ├─ kill
    │  │  │  │  ╰─ /
    │  │  │  ├─ leakmem
    │  │  │  │  ╰─ /
    │  │  │  ├─ quit
    │  │  │  │  ╰─ /
    │  │  │  ╰─ sleep
    │  │  │     ╰─ /
    │  │  ├─ ountr
    │  │  │  ├─ ies
    │  │  │  │  ╰─ /
    │  │  │  ╰─ y_states
    │  │  │     ╰─ /
    │  │  ╰─ ustomers_dot/proxy/graphql
    │  │     ╰─ /
    │  ├─ ex
    │  │  ├─ periment/
    │  │  │  ╰─ <id>
    │  │  │     ╰─ /
    │  │  ╰─ ternal_redirect
    │  │     ╰─ /
    │  ├─ g
    │  │  ├─ oogle_api/auth/callback
    │  │  │  ╰─ /
    │  │  ╰─ raphql-explorer
    │  │     ╰─ /
    │  ├─ i
    │  │  ├─ de
    │  │  │  ├─ /
    │  │  │  │  ├─ oauth_redirect
    │  │  │  │  │  ╰─ /
    │  │  │  │  ├─ project
    │  │  │  │  │  ╰─ /
    │  │  │  │  │     ╰─ <project_id>
    │  │  │  │  │        ╰─ /
    │  │  │  │  │           ├─ blob
    │  │  │  │  │           │  ╰─ /
    │  │  │  │  │           │     ├─ <*branch>
    │  │  │  │  │           │     │  ╰─ /
    │  │  │  │  │           │     │     ╰─ -
    │  │  │  │  │           │     │        ╰─ /
    │  │  │  │  │           │     │           ├─ <*path>
    │  │  │  │  │           │     │           │  ╰─ /
    │  │  │  │  │           │     │           ╰─ <*path>
    │  │  │  │  │           │     ╰─ <*branch>
    │  │  │  │  │           ├─ edit
    │  │  │  │  │           │  ╰─ /
    │  │  │  │  │           │     ├─ <*branch>
    │  │  │  │  │           │     │  ╰─ /
    │  │  │  │  │           │     │     ╰─ -
    │  │  │  │  │           │     │        ╰─ /
    │  │  │  │  │           │     │           ├─ <*path>
    │  │  │  │  │           │     │           │  ╰─ /
    │  │  │  │  │           │     │           ╰─ <*path>
    │  │  │  │  │           │     ╰─ <*branch>
    │  │  │  │  │           ├─ merge_requests/
    │  │  │  │  │           │  ╰─ <merge_request_id>
    │  │  │  │  │           │     ╰─ /
    │  │  │  │  │           ╰─ tree
    │  │  │  │  │              ╰─ /
    │  │  │  │  │                 ├─ <*branch>
    │  │  │  │  │                 │  ╰─ /
    │  │  │  │  │                 │     ╰─ -
    │  │  │  │  │                 │        ╰─ /
    │  │  │  │  │                 │           ├─ <*path>
    │  │  │  │  │                 │           │  ╰─ /
    │  │  │  │  │                 │           ╰─ <*path>
    │  │  │  │  │                 ╰─ <*branch>
    │  │  │  │  ╰─ reset_oauth_application_settings
    │  │  │  │     ╰─ /
    │  │  │  ╰─ ntity_verification
    │  │  │     ╰─ /
    │  │  │        ├─ s
    │  │  │        │  ├─ end_phone_verification_code
    │  │  │        │  │  ╰─ /
    │  │  │        │  ╰─ uccess
    │  │  │        │     ╰─ /
    │  │  │        ├─ toggle_phone_exemption
    │  │  │        │  ╰─ /
    │  │  │        ╰─ verif
    │  │  │           ├─ ication_state
    │  │  │           │  ╰─ /
    │  │  │           ╰─ y_
    │  │  │              ├─ credit_card
    │  │  │              │  ├─ /
    │  │  │              │  ╰─ _captcha
    │  │  │              │     ╰─ /
    │  │  │              ╰─ phone_verification_code
    │  │  │                 ╰─ /
    │  │  ╰─ nvites/
    │  │     ╰─ <id>
    │  │        ╰─ /
    │  │           ├─ accept
    │  │           │  ╰─ /
    │  │           ╰─ decline
    │  │              ╰─ /
    │  ├─ j
    │  │  ├─ ira
    │  │  │  ├─ /
    │  │  │  │  ╰─ <*namespace_id>
    │  │  │  │     ╰─ /
    │  │  │  │        ╰─ <project_id>
    │  │  │  │           ╰─ /commit/
    │  │  │  │              ╰─ <id>
    │  │  │  │                 ├─ .
    │  │  │  │                 │  ╰─ <format>
    │  │  │  │                 │     ╰─ /
    │  │  │  │                 ╰─ /
    │  │  │  ╰─ _connect
    │  │  │     ╰─ /
    │  │  │        ├─ app_descriptor
    │  │  │        │  ╰─ /
    │  │  │        ├─ branches/
    │  │  │        │  ├─ new
    │  │  │        │  │  ╰─ /
    │  │  │        │  ╰─ route
    │  │  │        │     ╰─ /
    │  │  │        ├─ events/
    │  │  │        │  ├─ installed
    │  │  │        │  │  ╰─ /
    │  │  │        │  ╰─ uninstalled
    │  │  │        │     ╰─ /
    │  │  │        ├─ installations
    │  │  │        │  ╰─ /
    │  │  │        ├─ oauth_
    │  │  │        │  ├─ application_id
    │  │  │        │  │  ╰─ /
    │  │  │        │  ╰─ callbacks
    │  │  │        │     ╰─ /
    │  │  │        ├─ public_keys/
    │  │  │        │  ╰─ <id>
    │  │  │        │     ╰─ /
    │  │  │        ├─ repositories/
    │  │  │        │  ├─ associate
    │  │  │        │  │  ╰─ /
    │  │  │        │  ╰─ search
    │  │  │        │     ╰─ /
    │  │  │        ├─ subscriptions
    │  │  │        │  ╰─ /
    │  │  │        │     ╰─ <id>
    │  │  │        │        ╰─ /
    │  │  │        ╰─ workspaces/search
    │  │  │           ╰─ /
    │  │  ╰─ wks
    │  │     ╰─ /
    │  ├─ kubernetes
    │  │  ╰─ /
    │  │     ╰─ <agent_id>
    │  │        ╰─ /
    │  │           ├─ <*vueroute>
    │  │           │  ╰─ /
    │  │           ╰─ <*vueroute>
    │  ├─ liveness
    │  │  ╰─ /
    │  ├─ m
    │  │  ├─ a
    │  │  │  ├─ ilgun/webhooks
    │  │  │  │  ╰─ /
    │  │  │  ╰─ nifest
    │  │  │     ╰─ /
    │  │  ╰─ e
    │  │     ├─ mbers/mailgun/permanent_failures
    │  │     │  ╰─ /
    │  │     ╰─ trics
    │  │        ╰─ /
    │  │           ╰─ system
    │  │              ╰─ /
    │  ├─ o
    │  │  ├─ ffline
    │  │  │  ╰─ /
    │  │  ├─ perations
    │  │  │  ╰─ /
    │  │  │     ╰─ environments
    │  │  │        ╰─ /
    │  │  ╰─ rganizations
    │  │     ╰─ /
    │  │        ├─ new
    │  │        │  ╰─ /
    │  │        ├─ preview_markdown
    │  │        │  ╰─ /
    │  │        ╰─ <organization_path>
    │  │           ╰─ /
    │  │              ├─ activity
    │  │              │  ╰─ /
    │  │              ├─ groups
    │  │              │  ├─ /
    │  │              │  │  ├─ new
    │  │              │  │  │  ╰─ /
    │  │              │  │  ╰─ <*id>
    │  │              │  │     ╰─ /edit
    │  │              │  │        ╰─ /
    │  │              │  ╰─ _and_projects
    │  │              │     ╰─ /
    │  │              ├─ projects/
    │  │              │  ╰─ <*namespace_id>
    │  │              │     ╰─ /
    │  │              │        ╰─ <id>
    │  │              │           ╰─ /edit
    │  │              │              ╰─ /
    │  │              ├─ settings/general
    │  │              │  ╰─ /
    │  │              ╰─ users
    │  │                 ╰─ /
    │  ├─ p
    │  │  ├─ eek/results
    │  │  │  ╰─ /
    │  │  ├─ hone_verification/telesign_callback
    │  │  │  ╰─ /
    │  │  ├─ rofile/
    │  │  │  ├─ a
    │  │  │  │  ├─ ccount
    │  │  │  │  │  ╰─ /
    │  │  │  │  │     ╰─ unlink
    │  │  │  │  │        ╰─ /
    │  │  │  │  ├─ pplications
    │  │  │  │  │  ╰─ /
    │  │  │  │  ├─ udit_log
    │  │  │  │  │  ╰─ /
    │  │  │  │  ╰─ vatar
    │  │  │  │     ╰─ /
    │  │  │  ├─ billings
    │  │  │  │  ╰─ /
    │  │  │  ├─ c
    │  │  │  │  ├─ hat_names
    │  │  │  │  │  ╰─ /
    │  │  │  │  │     ├─ deny
    │  │  │  │  │     │  ╰─ /
    │  │  │  │  │     ├─ new
    │  │  │  │  │     │  ╰─ /
    │  │  │  │  │     ╰─ <id>
    │  │  │  │  │        ╰─ /
    │  │  │  │  ╰─ omment_templates
    │  │  │  │     ╰─ /
    │  │  │  │        ╰─ <id>
    │  │  │  │           ╰─ /
    │  │  │  ├─ emails
    │  │  │  │  ╰─ /
    │  │  │  │     ├─ confirmation
    │  │  │  │     │  ╰─ /
    │  │  │  │     │     ╰─ new
    │  │  │  │     │        ╰─ /
    │  │  │  │     ╰─ <id>
    │  │  │  │        ╰─ /
    │  │  │  │           ╰─ resend_confirmation_instructions
    │  │  │  │              ╰─ /
    │  │  │  ├─ groups/
    │  │  │  │  ╰─ <*id>
    │  │  │  │     ╰─ /notifications
    │  │  │  │        ├─ .
    │  │  │  │        │  ╰─ <format>
    │  │  │  │        │     ╰─ /
    │  │  │  │        ╰─ /
    │  │  │  ├─ join_early_access_program
    │  │  │  │  ╰─ /
    │  │  │  ├─ notifications
    │  │  │  │  ╰─ /
    │  │  │  ├─ preferences
    │  │  │  │  ╰─ /
    │  │  │  ├─ reset_
    │  │  │  │  ├─ feed_token
    │  │  │  │  │  ╰─ /
    │  │  │  │  ├─ incoming_email_token
    │  │  │  │  │  ╰─ /
    │  │  │  │  ╰─ static_object_token
    │  │  │  │     ╰─ /
    │  │  │  ├─ slack/
    │  │  │  │  ├─ edit
    │  │  │  │  │  ╰─ /
    │  │  │  │  ╰─ slack_link
    │  │  │  │     ╰─ /
    │  │  │  ├─ two_factor_auth
    │  │  │  │  ╰─ /
    │  │  │  │     ├─ c
    │  │  │  │     │  ├─ odes
    │  │  │  │     │  │  ╰─ /
    │  │  │  │     │  ╰─ reate_webauthn
    │  │  │  │     │     ╰─ /
    │  │  │  │     ╰─ skip
    │  │  │  │        ╰─ /
    │  │  │  ├─ u
    │  │  │  │  ├─ pdate_username
    │  │  │  │  │  ╰─ /
    │  │  │  │  ╰─ sage_quotas
    │  │  │  │     ╰─ /
    │  │  │  ╰─ webauthn_registrations/
    │  │  │     ╰─ <id>
    │  │  │        ╰─ /
    │  │  ╰─ ush_from_secondary/
    │  │     ╰─ <geo_node_id>
    │  │        ╰─ /
    │  │           ├─ <*repository_path>
    │  │           │  ╰─ /
    │  │           │     ├─ git
    │  │           │     │  ├─ -
    │  │           │     │  │  ├─ receive-pack
    │  │           │     │  │  │  ╰─ /
    │  │           │     │  │  ╰─ upload-pack
    │  │           │     │  │     ╰─ /
    │  │           │     │  ╰─ lab-lfs/objects/
    │  │           │     │     ├─ <*oid>
    │  │           │     │     │  ╰─ /
    │  │           │     │     │     ╰─ <size>
    │  │           │     │     │        ╰─ /
    │  │           │     │     │           ╰─ authorize
    │  │           │     │     │              ╰─ /
    │  │           │     │     ╰─ <*oid>
    │  │           │     ├─ info/
    │  │           │     │  ├─ lfs/
    │  │           │     │  │  ├─ locks
    │  │           │     │  │  │  ╰─ /
    │  │           │     │  │  │     ├─ new
    │  │           │     │  │  │     │  ╰─ /
    │  │           │     │  │  │     ├─ verify
    │  │           │     │  │  │     │  ╰─ /
    │  │           │     │  │  │     ╰─ <id>
    │  │           │     │  │  │        ╰─ /
    │  │           │     │  │  │           ├─ edit
    │  │           │     │  │  │           │  ╰─ /
    │  │           │     │  │  │           ╰─ unlock
    │  │           │     │  │  │              ╰─ /
    │  │           │     │  │  ╰─ objects
    │  │           │     │  │     ╰─ /
    │  │           │     │  │        ├─ batch
    │  │           │     │  │        │  ╰─ /
    │  │           │     │  │        ├─ <*oid>
    │  │           │     │  │        │  ╰─ /
    │  │           │     │  │        ╰─ <*oid>
    │  │           │     │  ╰─ refs
    │  │           │     │     ╰─ /
    │  │           │     ╰─ ssh-
    │  │           │        ├─ receive-pack
    │  │           │        │  ╰─ /
    │  │           │        ╰─ upload-pack
    │  │           │           ╰─ /
    │  │           ╰─ <*repository_path>
    │  ├─ r
    │  │  ├─ e
    │  │  │  ├─ adiness
    │  │  │  │  ╰─ /
    │  │  │  ╰─ mote_development/workspaces
    │  │  │     ├─ /
    │  │  │     │  ├─ new
    │  │  │     │  │  ╰─ /
    │  │  │     │  ├─ <workspace_id>
    │  │  │     │  │  ╰─ /workspaces
    │  │  │     │  │     ╰─ /
    │  │  │     │  │        ╰─ new
    │  │  │     │  │           ╰─ /
    │  │  │     │  ├─ <id>
    │  │  │     │  │  ╰─ /
    │  │  │     │  │     ╰─ edit
    │  │  │     │  │        ╰─ /
    │  │  │     │  ├─ <*vueroute>
    │  │  │     │  │  ╰─ /
    │  │  │     │  │     ├─ new
    │  │  │     │  │     │  ╰─ /
    │  │  │     │  │     ├─ <workspace_id>
    │  │  │     │  │     │  ╰─ /workspaces
    │  │  │     │  │     │     ╰─ /
    │  │  │     │  │     │        ╰─ new
    │  │  │     │  │     │           ╰─ /
    │  │  │     │  │     ╰─ <id>
    │  │  │     │  │        ╰─ /
    │  │  │     │  │           ╰─ edit
    │  │  │     │  │              ╰─ /
    │  │  │     │  ╰─ <*vueroute>
    │  │  │     ╰─ _feature_flag
    │  │  │        ╰─ /
    │  │  ╰─ unner_setup/platforms
    │  │     ╰─ /
    │  ├─ s
    │  │  ├─ /
    │  │  │  ╰─ <username>
    │  │  │     ╰─ /
    │  │  ├─ andbox/
    │  │  │  ├─ mermaid
    │  │  │  │  ╰─ /
    │  │  │  ╰─ swagger
    │  │  │     ╰─ /
    │  │  ├─ e
    │  │  │  ├─ curity
    │  │  │  │  ╰─ /
    │  │  │  │     ├─ dashboard
    │  │  │  │     │  ╰─ /
    │  │  │  │     │     ╰─ settings
    │  │  │  │     │        ╰─ /
    │  │  │  │     ├─ projects
    │  │  │  │     │  ╰─ /
    │  │  │  │     │     ╰─ <id>
    │  │  │  │     │        ╰─ /
    │  │  │  │     ╰─ vulnerabilities
    │  │  │  │        ╰─ /
    │  │  │  ╰─ nt_notifications/
    │  │  │     ╰─ <id>
    │  │  │        ╰─ /unsubscribe
    │  │  │           ╰─ /
    │  │  ├─ martcard/
    │  │  │  ├─ auth
    │  │  │  │  ╰─ /
    │  │  │  ├─ extract_certificate
    │  │  │  │  ╰─ /
    │  │  │  ╰─ verify_certificate
    │  │  │     ╰─ /
    │  │  ├─ nippets
    │  │  │  ╰─ /
    │  │  │     ├─ new
    │  │  │     │  ╰─ /
    │  │  │     ├─ preview_markdown
    │  │  │     │  ╰─ /
    │  │  │     ├─ <id>
    │  │  │     │  ╰─ /
    │  │  │     │     ├─ edit
    │  │  │     │     │  ╰─ /
    │  │  │     │     ├─ mark_as_spam
    │  │  │     │     │  ╰─ /
    │  │  │     │     ├─ raw
    │  │  │     │     │  ╰─ /
    │  │  │     │     ╰─ toggle_award_emoji
    │  │  │     │        ╰─ /
    │  │  │     ╰─ <snippet_id>
    │  │  │        ╰─ /
    │  │  │           ├─ notes
    │  │  │           │  ╰─ /
    │  │  │           │     ╰─ <id>
    │  │  │           │        ╰─ /
    │  │  │           │           ├─ delete_attachment
    │  │  │           │           │  ╰─ /
    │  │  │           │           ╰─ toggle_award_emoji
    │  │  │           │              ╰─ /
    │  │  │           ╰─ raw/
    │  │  │              ╰─ <ref>
    │  │  │                 ╰─ /
    │  │  │                    ├─ <*path>
    │  │  │                    │  ╰─ /
    │  │  │                    ╰─ <*path>
    │  │  ╰─ ubscriptions
    │  │     ╰─ /
    │  │        ├─ buy_
    │  │        │  ├─ minutes
    │  │        │  │  ╰─ /
    │  │        │  ╰─ storage
    │  │        │     ╰─ /
    │  │        ├─ groups
    │  │        │  ╰─ /
    │  │        │     ├─ new
    │  │        │     │  ╰─ /
    │  │        │     ╰─ <id>
    │  │        │        ╰─ /
    │  │        │           ╰─ edit
    │  │        │              ╰─ /
    │  │        ├─ hand_raise_leads
    │  │        │  ╰─ /
    │  │        ├─ new
    │  │        │  ╰─ /
    │  │        ├─ payment_
    │  │        │  ├─ form
    │  │        │  │  ╰─ /
    │  │        │  ╰─ method
    │  │        │     ╰─ /
    │  │        ╰─ validate_payment_method
    │  │           ╰─ /
    │  ├─ t
    │  │  ├─ imelogs
    │  │  │  ╰─ /
    │  │  ╰─ r
    │  │     ├─ ack_namespace_visits
    │  │     │  ╰─ /
    │  │     ╰─ ial
    │  │        ├─ _registrations
    │  │        │  ╰─ /
    │  │        │     ╰─ new
    │  │        │        ╰─ /
    │  │        ╰─ s
    │  │           ╰─ /
    │  │              ├─ duo_
    │  │              │  ├─ enterprise
    │  │              │  │  ╰─ /
    │  │              │  │     ╰─ new
    │  │              │  │        ╰─ /
    │  │              │  ╰─ pro
    │  │              │     ╰─ /
    │  │              │        ╰─ new
    │  │              │           ╰─ /
    │  │              ╰─ new
    │  │                 ╰─ /
    │  ├─ user
    │  │  ├─ _settings/
    │  │  │  ├─ a
    │  │  │  │  ├─ ctive_sessions
    │  │  │  │  │  ╰─ /
    │  │  │  │  │     ├─ saml
    │  │  │  │  │     │  ╰─ /
    │  │  │  │  │     ╰─ <id>
    │  │  │  │  │        ╰─ /
    │  │  │  │  ├─ pplications
    │  │  │  │  │  ╰─ /
    │  │  │  │  ╰─ uthentication_log
    │  │  │  │     ╰─ /
    │  │  │  ├─ gpg_keys
    │  │  │  │  ╰─ /
    │  │  │  │     ╰─ <id>
    │  │  │  │        ╰─ /
    │  │  │  │           ╰─ revoke
    │  │  │  │              ╰─ /
    │  │  │  ├─ identities
    │  │  │  │  ╰─ /
    │  │  │  │     ╰─ new
    │  │  │  │        ╰─ /
    │  │  │  ├─ p
    │  │  │  │  ├─ assword
    │  │  │  │  │  ╰─ /
    │  │  │  │  │     ├─ edit
    │  │  │  │  │     │  ╰─ /
    │  │  │  │  │     ├─ new
    │  │  │  │  │     │  ╰─ /
    │  │  │  │  │     ╰─ reset
    │  │  │  │  │        ╰─ /
    │  │  │  │  ├─ ersonal_access_tokens
    │  │  │  │  │  ╰─ /
    │  │  │  │  │     ╰─ <id>
    │  │  │  │  │        ╰─ /revoke
    │  │  │  │  │           ╰─ /
    │  │  │  │  ╰─ rofile
    │  │  │  │     ╰─ /
    │  │  │  ╰─ ssh_keys
    │  │  │     ╰─ /
    │  │  │        ╰─ <id>
    │  │  │           ╰─ /
    │  │  │              ╰─ revoke
    │  │  │                 ╰─ /
    │  │  ╰─ s/
    │  │     ├─ broadcast_message_dismissals
    │  │     │  ╰─ /
    │  │     ├─ callouts
    │  │     │  ╰─ /
    │  │     ├─ group_callouts
    │  │     │  ╰─ /
    │  │     ├─ p
    │  │     │  ├─ ins
    │  │     │  │  ╰─ /
    │  │     │  ╰─ roject_callouts
    │  │     │     ╰─ /
    │  │     ╰─ terms
    │  │        ╰─ /
    │  │           ╰─ <id>
    │  │              ╰─ /
    │  │                 ├─ accept
    │  │                 │  ╰─ /
    │  │                 ╰─ decline
    │  │                    ╰─ /
    │  ├─ whats_new
    │  │  ╰─ /
    │  ╰─ <model>
    │     ╰─ /
    │        ╰─ <model_id>
    │           ╰─ /uploads/
    │              ╰─ <secret>
    │                 ╰─ /
    │                    ╰─ <filename>
    │                       ╰─ /
    ├─ .well-known/
    │  ├─ change-password
    │  │  ╰─ /
    │  ├─ o
    │  │  ├─ auth-authorization-server
    │  │  │  ╰─ /
    │  │  ╰─ penid-configuration
    │  │     ╰─ /
    │  ├─ security.txt
    │  │  ╰─ /
    │  ├─ terraform.json
    │  │  ╰─ /
    │  ╰─ webfinger
    │     ╰─ /
    ├─ a
    │  ├─ dmin
    │  │  ╰─ /
    │  │     ├─ a
    │  │     │  ├─ buse_reports
    │  │     │  │  ╰─ /
    │  │     │  │     ╰─ <id>
    │  │     │  │        ╰─ /
    │  │     │  │           ╰─ moderate_user
    │  │     │  │              ╰─ /
    │  │     │  ├─ i/
    │  │     │  │  ├─ feature_settings
    │  │     │  │  │  ╰─ /
    │  │     │  │  │     ╰─ <id>
    │  │     │  │  │        ╰─ /
    │  │     │  │  │           ╰─ edit
    │  │     │  │  │              ╰─ /
    │  │     │  │  ╰─ self_hosted_models
    │  │     │  │     ╰─ /
    │  │     │  │        ├─ new
    │  │     │  │        │  ╰─ /
    │  │     │  │        ├─ terms_and_conditions
    │  │     │  │        │  ╰─ /
    │  │     │  │        ╰─ <id>
    │  │     │  │           ╰─ /
    │  │     │  │              ╰─ edit
    │  │     │  │                 ╰─ /
    │  │     │  ├─ pplication
    │  │     │  │  ├─ _settings
    │  │     │  │  │  ╰─ /
    │  │     │  │  │     ├─ a
    │  │     │  │  │     │  ├─ dvanced_search
    │  │     │  │  │     │  │  ╰─ /
    │  │     │  │  │     │  ├─ nalytics
    │  │     │  │  │     │  │  ╰─ /
    │  │     │  │  │     │  ╰─ ppearance
    │  │     │  │  │     │     ╰─ /
    │  │     │  │  │     │        ├─ favicon
    │  │     │  │  │     │        │  ╰─ /
    │  │     │  │  │     │        ├─ header_logos
    │  │     │  │  │     │        │  ╰─ /
    │  │     │  │  │     │        ├─ logo
    │  │     │  │  │     │        │  ╰─ /
    │  │     │  │  │     │        ╰─ p
    │  │     │  │  │     │           ├─ review_sign_in
    │  │     │  │  │     │           │  ╰─ /
    │  │     │  │  │     │           ╰─ wa_icon
    │  │     │  │  │     │              ╰─ /
    │  │     │  │  │     ├─ c
    │  │     │  │  │     │  ├─ i_cd
    │  │     │  │  │     │  │  ╰─ /
    │  │     │  │  │     │  ╰─ lear_repository_check_states
    │  │     │  │  │     │     ╰─ /
    │  │     │  │  │     ├─ ge
    │  │     │  │  │     │  ├─ neral
    │  │     │  │  │     │  │  ╰─ /
    │  │     │  │  │     │  ╰─ o
    │  │     │  │  │     │     ╰─ /
    │  │     │  │  │     ├─ integrations
    │  │     │  │  │     │  ╰─ /
    │  │     │  │  │     │     ╰─ <id>
    │  │     │  │  │     │        ╰─ /
    │  │     │  │  │     │           ├─ edit
    │  │     │  │  │     │           │  ╰─ /
    │  │     │  │  │     │           ├─ overrides
    │  │     │  │  │     │           │  ╰─ /
    │  │     │  │  │     │           ├─ reset
    │  │     │  │  │     │           │  ╰─ /
    │  │     │  │  │     │           ╰─ test
    │  │     │  │  │     │              ╰─ /
    │  │     │  │  │     ├─ lets_encrypt_terms_of_service
    │  │     │  │  │     │  ╰─ /
    │  │     │  │  │     ├─ metrics_and_profiling
    │  │     │  │  │     │  ╰─ /
    │  │     │  │  │     ├─ n
    │  │     │  │  │     │  ├─ amespace_storage
    │  │     │  │  │     │  │  ╰─ /
    │  │     │  │  │     │  ╰─ etwork
    │  │     │  │  │     │     ╰─ /
    │  │     │  │  │     ├─ preferences
    │  │     │  │  │     │  ╰─ /
    │  │     │  │  │     ├─ r
    │  │     │  │  │     │  ├─ e
    │  │     │  │  │     │  │  ├─ po
    │  │     │  │  │     │  │  │  ├─ rting
    │  │     │  │  │     │  │  │  │  ╰─ /
    │  │     │  │  │     │  │  │  ╰─ sitory
    │  │     │  │  │     │  │  │     ╰─ /
    │  │     │  │  │     │  │  ╰─ set_
    │  │     │  │  │     │  │     ├─ error_tracking_access_token
    │  │     │  │  │     │  │     │  ╰─ /
    │  │     │  │  │     │  │     ├─ health_check_token
    │  │     │  │  │     │  │     │  ╰─ /
    │  │     │  │  │     │  │     ╰─ registration_token
    │  │     │  │  │     │  │        ╰─ /
    │  │     │  │  │     │  ╰─ oles_and_permissions
    │  │     │  │  │     │     ╰─ /
    │  │     │  │  │     │        ├─ new
    │  │     │  │  │     │        │  ╰─ /
    │  │     │  │  │     │        ╰─ <id>
    │  │     │  │  │     │           ╰─ /
    │  │     │  │  │     │              ╰─ edit
    │  │     │  │  │     │                 ╰─ /
    │  │     │  │  │     ├─ s
    │  │     │  │  │     │  ├─ cim_oauth
    │  │     │  │  │     │  │  ╰─ /
    │  │     │  │  │     │  ├─ e
    │  │     │  │  │     │  │  ├─ at_link_payload
    │  │     │  │  │     │  │  │  ╰─ /
    │  │     │  │  │     │  │  ╰─ curity_and_compliance
    │  │     │  │  │     │  │     ╰─ /
    │  │     │  │  │     │  ╰─ lack
    │  │     │  │  │     │     ├─ /
    │  │     │  │  │     │     │  ╰─ slack_auth
    │  │     │  │  │     │     │     ╰─ /
    │  │     │  │  │     │     ╰─ _app_manifest_
    │  │     │  │  │     │        ├─ download
    │  │     │  │  │     │        │  ╰─ /
    │  │     │  │  │     │        ╰─ share
    │  │     │  │  │     │           ╰─ /
    │  │     │  │  │     ├─ templates
    │  │     │  │  │     │  ╰─ /
    │  │     │  │  │     ╰─ u
    │  │     │  │  │        ├─ pdate_microsoft_application
    │  │     │  │  │        │  ╰─ /
    │  │     │  │  │        ╰─ sage_data
    │  │     │  │  │           ╰─ /
    │  │     │  │  ╰─ s
    │  │     │  │     ╰─ /
    │  │     │  │        ├─ new
    │  │     │  │        │  ╰─ /
    │  │     │  │        ╰─ <id>
    │  │     │  │           ╰─ /
    │  │     │  │              ├─ edit
    │  │     │  │              │  ╰─ /
    │  │     │  │              ╰─ renew
    │  │     │  │                 ╰─ /
    │  │     │  ╰─ udit_log
    │  │     │     ├─ _reports
    │  │     │     │  ├─ .
    │  │     │     │  │  ╰─ <format>
    │  │     │     │  │     ╰─ /
    │  │     │     │  ╰─ /
    │  │     │     ╰─ s
    │  │     │        ╰─ /
    │  │     ├─ b
    │  │     │  ├─ ackground_
    │  │     │  │  ├─ jobs
    │  │     │  │  │  ╰─ /
    │  │     │  │  ╰─ migrations
    │  │     │  │     ╰─ /
    │  │     │  │        ├─ <background_migration_id>
    │  │     │  │        │  ╰─ /batched_jobs/
    │  │     │  │        │     ╰─ <id>
    │  │     │  │        │        ╰─ /
    │  │     │  │        ╰─ <id>
    │  │     │  │           ╰─ /
    │  │     │  │              ├─ pause
    │  │     │  │              │  ╰─ /
    │  │     │  │              ╰─ re
    │  │     │  │                 ├─ sume
    │  │     │  │                 │  ╰─ /
    │  │     │  │                 ╰─ try
    │  │     │  │                    ╰─ /
    │  │     │  ╰─ roadcast_messages
    │  │     │     ╰─ /
    │  │     │        ├─ preview
    │  │     │        │  ╰─ /
    │  │     │        ╰─ <id>
    │  │     │           ╰─ /
    │  │     │              ╰─ edit
    │  │     │                 ╰─ /
    │  │     ├─ c
    │  │     │  ├─ i/variables
    │  │     │  │  ╰─ /
    │  │     │  ├─ lusters
    │  │     │  │  ╰─ /
    │  │     │  │     ├─ c
    │  │     │  │     │  ├─ onnect
    │  │     │  │     │  │  ╰─ /
    │  │     │  │     │  ╰─ reate_user
    │  │     │  │     │     ╰─ /
    │  │     │  │     ├─ new_cluster_docs
    │  │     │  │     │  ╰─ /
    │  │     │  │     ├─ <cluster_id>
    │  │     │  │     │  ╰─ /integration/create_or_update
    │  │     │  │     │     ╰─ /
    │  │     │  │     ╰─ <id>
    │  │     │  │        ╰─ /
    │  │     │  │           ├─ cl
    │  │     │  │           │  ├─ ear_cache
    │  │     │  │           │  │  ╰─ /
    │  │     │  │           │  ╰─ uster_status
    │  │     │  │           │     ╰─ /
    │  │     │  │           ├─ environments
    │  │     │  │           │  ╰─ /
    │  │     │  │           ╰─ metrics
    │  │     │  │              ├─ /
    │  │     │  │              ╰─ _dashboard
    │  │     │  │                 ╰─ /
    │  │     │  ├─ o
    │  │     │  │  ├─ de_suggestions
    │  │     │  │  │  ╰─ /
    │  │     │  │  ╰─ horts
    │  │     │  │     ╰─ /
    │  │     │  ╰─ redentials
    │  │     │     ╰─ /
    │  │     │        ├─ <credential_id>
    │  │     │        │  ╰─ /resources/
    │  │     │        │     ╰─ <resource_id>
    │  │     │        │        ╰─ /revoke
    │  │     │        │           ╰─ /
    │  │     │        ╰─ <id>
    │  │     │           ╰─ /
    │  │     │              ╰─ revoke
    │  │     │                 ╰─ /
    │  │     ├─ d
    │  │     │  ├─ ashboard/stats
    │  │     │  │  ╰─ /
    │  │     │  ╰─ e
    │  │     │     ├─ ploy_keys
    │  │     │     │  ╰─ /
    │  │     │     │     ├─ new
    │  │     │     │     │  ╰─ /
    │  │     │     │     ╰─ <id>
    │  │     │     │        ╰─ /
    │  │     │     │           ╰─ edit
    │  │     │     │              ╰─ /
    │  │     │     ╰─ v_ops_report
    │  │     │        ├─ /
    │  │     │        ╰─ s
    │  │     │           ╰─ /
    │  │     ├─ e
    │  │     │  ├─ lasticsearch/
    │  │     │  │  ├─ cancel_index_deletion
    │  │     │  │  │  ╰─ /
    │  │     │  │  ├─ enqueue_index
    │  │     │  │  │  ╰─ /
    │  │     │  │  ├─ retry_migration
    │  │     │  │  │  ╰─ /
    │  │     │  │  ╰─ trigger_reindexing
    │  │     │  │     ╰─ /
    │  │     │  ╰─ mail
    │  │     │     ╰─ /
    │  │     ├─ g
    │  │     │  ├─ eo
    │  │     │  │  ╰─ /
    │  │     │  │     ├─ replication
    │  │     │  │     │  ╰─ /
    │  │     │  │     │     ╰─ <replicable_name_plural>
    │  │     │  │     │        ╰─ /
    │  │     │  │     ╰─ s
    │  │     │  │        ├─ ettings
    │  │     │  │        │  ╰─ /
    │  │     │  │        ╰─ ites
    │  │     │  │           ╰─ /
    │  │     │  │              ├─ new
    │  │     │  │              │  ╰─ /
    │  │     │  │              ╰─ <id>
    │  │     │  │                 ╰─ /
    │  │     │  │                    ├─ edit
    │  │     │  │                    │  ╰─ /
    │  │     │  │                    ╰─ replication
    │  │     │  │                       ╰─ /
    │  │     │  │                          ╰─ <replicable_name_plural>
    │  │     │  │                             ╰─ /
    │  │     │  ├─ italy_servers
    │  │     │  │  ╰─ /
    │  │     │  ╰─ roups
    │  │     │     ╰─ /
    │  │     │        ├─ new
    │  │     │        │  ╰─ /
    │  │     │        ├─ <*id>
    │  │     │        │  ├─ .
    │  │     │        │  │  ╰─ <format>
    │  │     │        │  │     ╰─ /
    │  │     │        │  ╰─ /
    │  │     │        │     ├─ edit
    │  │     │        │     │  ├─ .
    │  │     │        │     │  │  ╰─ <format>
    │  │     │        │     │  │     ╰─ /
    │  │     │        │     │  ╰─ /
    │  │     │        │     ├─ members_update
    │  │     │        │     │  ├─ .
    │  │     │        │     │  │  ╰─ <format>
    │  │     │        │     │  │     ╰─ /
    │  │     │        │     │  ╰─ /
    │  │     │        │     ╰─ reset_runners_minutes
    │  │     │        │        ├─ .
    │  │     │        │        │  ╰─ <format>
    │  │     │        │        │     ╰─ /
    │  │     │        │        ╰─ /
    │  │     │        ╰─ <*id>
    │  │     ├─ h
    │  │     │  ├─ ealth_check
    │  │     │  │  ╰─ /
    │  │     │  ╰─ ooks
    │  │     │     ╰─ /
    │  │     │        ├─ <hook_id>
    │  │     │        │  ╰─ /hook_logs/
    │  │     │        │     ╰─ <id>
    │  │     │        │        ╰─ /
    │  │     │        │           ╰─ retry
    │  │     │        │              ╰─ /
    │  │     │        ╰─ <id>
    │  │     │           ╰─ /
    │  │     │              ├─ edit
    │  │     │              │  ╰─ /
    │  │     │              ╰─ test
    │  │     │                 ╰─ /
    │  │     ├─ i
    │  │     │  ├─ mpersonation
    │  │     │  │  ╰─ /
    │  │     │  ╰─ n
    │  │     │     ├─ itial_setup
    │  │     │     │  ╰─ /
    │  │     │     │     ╰─ new
    │  │     │     │        ╰─ /
    │  │     │     ╰─ stance_review
    │  │     │        ╰─ /
    │  │     ├─ jobs
    │  │     │  ╰─ /
    │  │     │     ╰─ cancel_all
    │  │     │        ╰─ /
    │  │     ├─ l
    │  │     │  ├─ abels
    │  │     │  │  ╰─ /
    │  │     │  │     ├─ new
    │  │     │  │     │  ╰─ /
    │  │     │  │     ╰─ <id>
    │  │     │  │        ╰─ /
    │  │     │  │           ╰─ edit
    │  │     │  │              ╰─ /
    │  │     │  ╰─ icense
    │  │     │     ╰─ /
    │  │     │        ├─ download
    │  │     │        │  ╰─ /
    │  │     │        ├─ sync_seat_link
    │  │     │        │  ╰─ /
    │  │     │        ╰─ usage_export
    │  │     │           ╰─ /
    │  │     ├─ namespace_limits
    │  │     │  ╰─ /
    │  │     │     ╰─ export_usage
    │  │     │        ╰─ /
    │  │     ├─ organizations
    │  │     │  ╰─ /
    │  │     ├─ p
    │  │     │  ├─ lan_limits
    │  │     │  │  ╰─ /
    │  │     │  ├─ rojects
    │  │     │  │  ╰─ /
    │  │     │  │     ╰─ <*namespace_id>
    │  │     │  │        ╰─ /
    │  │     │  │           ├─ <id>
    │  │     │  │           │  ╰─ /
    │  │     │  │           │     ├─ edit
    │  │     │  │           │     │  ╰─ /
    │  │     │  │           │     ├─ repository_check
    │  │     │  │           │     │  ╰─ /
    │  │     │  │           │     ╰─ transfer
    │  │     │  │           │        ╰─ /
    │  │     │  │           ╰─ <project_id>
    │  │     │  │              ╰─ /runner_projects
    │  │     │  │                 ╰─ /
    │  │     │  │                    ╰─ <id>
    │  │     │  │                       ╰─ /
    │  │     │  ╰─ ush_rule
    │  │     │     ╰─ /
    │  │     ├─ r
    │  │     │  ├─ ole_promotion_requests
    │  │     │  │  ╰─ /
    │  │     │  ╰─ unners
    │  │     │     ╰─ /
    │  │     │        ├─ dashboard
    │  │     │        │  ╰─ /
    │  │     │        ├─ new
    │  │     │        │  ╰─ /
    │  │     │        ├─ runner_setup_scripts
    │  │     │        │  ╰─ /
    │  │     │        ├─ tag_list
    │  │     │        │  ╰─ /
    │  │     │        ╰─ <id>
    │  │     │           ╰─ /
    │  │     │              ├─ edit
    │  │     │              │  ╰─ /
    │  │     │              ├─ pause
    │  │     │              │  ╰─ /
    │  │     │              ╰─ re
    │  │     │                 ├─ gister
    │  │     │                 │  ╰─ /
    │  │     │                 ╰─ sume
    │  │     │                    ╰─ /
    │  │     ├─ s
    │  │     │  ├─ ession
    │  │     │  │  ╰─ /
    │  │     │  │     ├─ destroy
    │  │     │  │     │  ╰─ /
    │  │     │  │     ╰─ new
    │  │     │  │        ╰─ /
    │  │     │  ├─ idekiq
    │  │     │  │  ╰─ /
    │  │     │  ├─ pam_logs
    │  │     │  │  ╰─ /
    │  │     │  │     ╰─ <id>
    │  │     │  │        ╰─ /
    │  │     │  │           ╰─ mark_as_ham
    │  │     │  │              ╰─ /
    │  │     │  ├─ ubscription
    │  │     │  │  ╰─ /
    │  │     │  ╰─ ystem_info
    │  │     │     ╰─ /
    │  │     ├─ topics
    │  │     │  ╰─ /
    │  │     │     ├─ merge
    │  │     │     │  ╰─ /
    │  │     │     ├─ new
    │  │     │     │  ╰─ /
    │  │     │     ├─ preview_markdown
    │  │     │     │  ╰─ /
    │  │     │     ├─ <topic_id>
    │  │     │     │  ╰─ /avatar
    │  │     │     │     ╰─ /
    │  │     │     ╰─ <id>
    │  │     │        ╰─ /
    │  │     │           ╰─ edit
    │  │     │              ╰─ /
    │  │     ├─ us
    │  │     │  ├─ age_trends
    │  │     │  │  ╰─ /
    │  │     │  ╰─ er
    │  │     │     ├─ _permission_exports
    │  │     │     │  ╰─ /
    │  │     │     ╰─ s
    │  │     │        ╰─ /
    │  │     │           ├─ new
    │  │     │           │  ╰─ /
    │  │     │           ├─ <id>
    │  │     │           │  ╰─ /
    │  │     │           │     ├─ a
    │  │     │           │     │  ├─ ctivate
    │  │     │           │     │  │  ╰─ /
    │  │     │           │     │  ╰─ pprove
    │  │     │           │     │     ╰─ /
    │  │     │           │     ├─ b
    │  │     │           │     │  ├─ an
    │  │     │           │     │  │  ╰─ /
    │  │     │           │     │  ╰─ lock
    │  │     │           │     │     ╰─ /
    │  │     │           │     ├─ c
    │  │     │           │     │  ├─ ard_match
    │  │     │           │     │  │  ╰─ /
    │  │     │           │     │  ╰─ onfirm
    │  │     │           │     │     ╰─ /
    │  │     │           │     ├─ d
    │  │     │           │     │  ├─ e
    │  │     │           │     │  │  ├─ activate
    │  │     │           │     │  │  │  ╰─ /
    │  │     │           │     │  │  ╰─ stroy_identity_verification_exemption
    │  │     │           │     │  │     ╰─ /
    │  │     │           │     │  ╰─ isable_two_factor
    │  │     │           │     │     ╰─ /
    │  │     │           │     ├─ edit
    │  │     │           │     │  ╰─ /
    │  │     │           │     ├─ i
    │  │     │           │     │  ├─ dentity_verification_exemption
    │  │     │           │     │  │  ╰─ /
    │  │     │           │     │  ╰─ mpersonate
    │  │     │           │     │     ╰─ /
    │  │     │           │     ├─ keys
    │  │     │           │     │  ╰─ /
    │  │     │           │     ├─ p
    │  │     │           │     │  ├─ hone_match
    │  │     │           │     │  │  ╰─ /
    │  │     │           │     │  ╰─ rojects
    │  │     │           │     │     ╰─ /
    │  │     │           │     ├─ re
    │  │     │           │     │  ├─ ject
    │  │     │           │     │  │  ╰─ /
    │  │     │           │     │  ├─ move/
    │  │     │           │     │  │  ╰─ <email_id>
    │  │     │           │     │  │     ╰─ /
    │  │     │           │     │  ╰─ set_runners_minutes
    │  │     │           │     │     ╰─ /
    │  │     │           │     ├─ trust
    │  │     │           │     │  ╰─ /
    │  │     │           │     ╰─ un
    │  │     │           │        ├─ b
    │  │     │           │        │  ├─ an
    │  │     │           │        │  │  ╰─ /
    │  │     │           │        │  ╰─ lock
    │  │     │           │        │     ╰─ /
    │  │     │           │        ├─ lock
    │  │     │           │        │  ╰─ /
    │  │     │           │        ╰─ trust
    │  │     │           │           ╰─ /
    │  │     │           ╰─ <user_id>
    │  │     │              ╰─ /
    │  │     │                 ├─ i
    │  │     │                 │  ├─ dentities
    │  │     │                 │  │  ╰─ /
    │  │     │                 │  │     ├─ new
    │  │     │                 │  │     │  ╰─ /
    │  │     │                 │  │     ╰─ <id>
    │  │     │                 │  │        ╰─ /
    │  │     │                 │  │           ╰─ edit
    │  │     │                 │  │              ╰─ /
    │  │     │                 │  ╰─ mpersonation_tokens
    │  │     │                 │     ╰─ /
    │  │     │                 │        ╰─ <id>
    │  │     │                 │           ╰─ /revoke
    │  │     │                 │              ╰─ /
    │  │     │                 ╰─ keys/
    │  │     │                    ╰─ <id>
    │  │     │                       ╰─ /
    │  │     ╰─ version_check
    │  │        ╰─ /
    │  ╰─ pi/
    │     ├─ graphql
    │     │  ╰─ /
    │     ╰─ v4/geo/graphql
    │        ╰─ /
    ├─ dashboard
    │  ╰─ /
    │     ├─ activity
    │     │  ╰─ /
    │     ├─ groups
    │     │  ╰─ /
    │     ├─ issues
    │     │  ╰─ /
    │     ├─ labels
    │     │  ╰─ /
    │     ├─ m
    │     │  ├─ erge_requests
    │     │  │  ╰─ /
    │     │  │     ╰─ search
    │     │  │        ╰─ /
    │     │  ╰─ ilestones
    │     │     ╰─ /
    │     ├─ projects
    │     │  ╰─ /
    │     │     ├─ contributed
    │     │     │  ╰─ /
    │     │     ├─ member
    │     │     │  ╰─ /
    │     │     ├─ personal
    │     │     │  ╰─ /
    │     │     ├─ removed
    │     │     │  ╰─ /
    │     │     ╰─ starred
    │     │        ╰─ /
    │     ├─ snippets
    │     │  ╰─ /
    │     ╰─ todos
    │        ╰─ /
    │           ├─ bulk_restore
    │           │  ╰─ /
    │           ├─ destroy_all
    │           │  ╰─ /
    │           ├─ vue
    │           │  ╰─ /
    │           ╰─ <id>
    │              ╰─ /
    │                 ╰─ restore
    │                    ╰─ /
    ├─ explore
    │  ╰─ /
    │     ├─ catalog
    │     │  ╰─ /
    │     │     ├─ <*full_path>
    │     │     │  ╰─ /
    │     │     ╰─ <*full_path>
    │     ├─ dependencies
    │     │  ╰─ /
    │     ├─ groups
    │     │  ╰─ /
    │     ├─ projects
    │     │  ╰─ /
    │     │     ├─ starred
    │     │     │  ╰─ /
    │     │     ╰─ t
    │     │        ├─ opics
    │     │        │  ╰─ /
    │     │        │     ╰─ <topic_name>
    │     │        │        ├─ .
    │     │        │        │  ╰─ <format>
    │     │        │        │     ╰─ /
    │     │        │        ╰─ /
    │     │        ╰─ rending
    │     │           ╰─ /
    │     ╰─ snippets
    │        ╰─ /
    ├─ f
    │  ├─ avicon.
    │  │  ├─ ico
    │  │  │  ╰─ /
    │  │  ╰─ png
    │  │     ╰─ /
    │  ╰─ iles/note/
    │     ╰─ <id>
    │        ╰─ /
    │           ╰─ <filename>
    │              ╰─ /
    ├─ groups
    │  ╰─ /
    │     ├─ new
    │     │  ╰─ /
    │     ├─ <*group_id>
    │     │  ╰─ /-/
    │     │     ├─ a
    │     │     │  ├─ chievements
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ new
    │     │     │  │     │  ╰─ /
    │     │     │  │     ╰─ <id>
    │     │     │  │        ╰─ /edit
    │     │     │  │           ╰─ /
    │     │     │  ├─ dd_ons/discover_duo_pro
    │     │     │  │  ╰─ /
    │     │     │  ├─ nalytics
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ c
    │     │     │  │     │  ├─ i_cd
    │     │     │  │     │  │  ╰─ /
    │     │     │  │     │  ├─ overage_reports
    │     │     │  │     │  │  ╰─ /
    │     │     │  │     │  ╰─ ycle_analytics
    │     │     │  │     │     ╰─ /
    │     │     │  │     ├─ d
    │     │     │  │     │  ├─ ashboards
    │     │     │  │     │  │  ╰─ /
    │     │     │  │     │  │     ├─ <*vueroute>
    │     │     │  │     │  │     │  ╰─ /
    │     │     │  │     │  │     ╰─ <*vueroute>
    │     │     │  │     │  ╰─ evops_adoption
    │     │     │  │     │     ╰─ /
    │     │     │  │     ├─ merge_request_analytics
    │     │     │  │     │  ╰─ /
    │     │     │  │     ├─ productivity_analytics
    │     │     │  │     │  ╰─ /
    │     │     │  │     ├─ repository_analytics
    │     │     │  │     │  ╰─ /
    │     │     │  │     ├─ type_of_work/tasks_by_type
    │     │     │  │     │  ╰─ /
    │     │     │  │     │     ╰─ top_labels
    │     │     │  │     │        ╰─ /
    │     │     │  │     ╰─ value_stream_analytics
    │     │     │  │        ╰─ /
    │     │     │  │           ├─ cycle_times
    │     │     │  │           │  ╰─ /
    │     │     │  │           ├─ lead_times
    │     │     │  │           │  ╰─ /
    │     │     │  │           ├─ summary
    │     │     │  │           │  ╰─ /
    │     │     │  │           ├─ time_summary
    │     │     │  │           │  ╰─ /
    │     │     │  │           ╰─ value_streams
    │     │     │  │              ╰─ /
    │     │     │  │                 ├─ new
    │     │     │  │                 │  ╰─ /
    │     │     │  │                 ├─ <value_stream_id>
    │     │     │  │                 │  ╰─ /stages
    │     │     │  │                 │     ╰─ /
    │     │     │  │                 │        ╰─ <id>
    │     │     │  │                 │           ╰─ /
    │     │     │  │                 │              ├─ average
    │     │     │  │                 │              │  ├─ /
    │     │     │  │                 │              │  ╰─ _duration_chart
    │     │     │  │                 │              │     ╰─ /
    │     │     │  │                 │              ├─ count
    │     │     │  │                 │              │  ╰─ /
    │     │     │  │                 │              ├─ median
    │     │     │  │                 │              │  ╰─ /
    │     │     │  │                 │              ╰─ records
    │     │     │  │                 │                 ╰─ /
    │     │     │  │                 ╰─ <id>
    │     │     │  │                    ╰─ /
    │     │     │  │                       ╰─ edit
    │     │     │  │                          ╰─ /
    │     │     │  ├─ u
    │     │     │  │  ├─ dit_events
    │     │     │  │  │  ╰─ /
    │     │     │  │  ╰─ tocomplete_sources/
    │     │     │  │     ├─ commands
    │     │     │  │     │  ╰─ /
    │     │     │  │     ├─ epics
    │     │     │  │     │  ╰─ /
    │     │     │  │     ├─ i
    │     │     │  │     │  ├─ ssues
    │     │     │  │     │  │  ╰─ /
    │     │     │  │     │  ╰─ terations
    │     │     │  │     │     ╰─ /
    │     │     │  │     ├─ labels
    │     │     │  │     │  ╰─ /
    │     │     │  │     ├─ m
    │     │     │  │     │  ├─ e
    │     │     │  │     │  │  ├─ mbers
    │     │     │  │     │  │  │  ╰─ /
    │     │     │  │     │  │  ╰─ rge_requests
    │     │     │  │     │  │     ╰─ /
    │     │     │  │     │  ╰─ ilestones
    │     │     │  │     │     ╰─ /
    │     │     │  │     ├─ vulnerabilities
    │     │     │  │     │  ╰─ /
    │     │     │  │     ╰─ wikis
    │     │     │  │        ╰─ /
    │     │     │  ╰─ vatar
    │     │     │     ╰─ /
    │     │     ├─ b
    │     │     │  ├─ illings
    │     │     │  │  ╰─ /
    │     │     │  │     ╰─ refresh_seats
    │     │     │  │        ╰─ /
    │     │     │  ╰─ oards
    │     │     │     ╰─ /
    │     │     │        ╰─ <id>
    │     │     │           ╰─ /
    │     │     ├─ c
    │     │     │  ├─ adences
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ new
    │     │     │  │     │  ╰─ /
    │     │     │  │     ├─ <iteration_cadence_id>
    │     │     │  │     │  ╰─ /iterations
    │     │     │  │     │     ╰─ /
    │     │     │  │     │        ├─ new
    │     │     │  │     │        │  ╰─ /
    │     │     │  │     │        ╰─ <id>
    │     │     │  │     │           ╰─ /
    │     │     │  │     │              ╰─ edit
    │     │     │  │     │                 ╰─ /
    │     │     │  │     ├─ <id>
    │     │     │  │     │  ╰─ /
    │     │     │  │     │     ╰─ edit
    │     │     │  │     │        ╰─ /
    │     │     │  │     ├─ <*vueroute>
    │     │     │  │     │  ╰─ /
    │     │     │  │     │     ├─ new
    │     │     │  │     │     │  ╰─ /
    │     │     │  │     │     ├─ <iteration_cadence_id>
    │     │     │  │     │     │  ╰─ /iterations
    │     │     │  │     │     │     ╰─ /
    │     │     │  │     │     │        ├─ new
    │     │     │  │     │     │        │  ╰─ /
    │     │     │  │     │     │        ╰─ <id>
    │     │     │  │     │     │           ╰─ /
    │     │     │  │     │     │              ╰─ edit
    │     │     │  │     │     │                 ╰─ /
    │     │     │  │     │     ╰─ <id>
    │     │     │  │     │        ╰─ /
    │     │     │  │     │           ╰─ edit
    │     │     │  │     │              ╰─ /
    │     │     │  │     ╰─ <*vueroute>
    │     │     │  ├─ hildren
    │     │     │  │  ╰─ /
    │     │     │  ├─ lusters
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ c
    │     │     │  │     │  ├─ onnect
    │     │     │  │     │  │  ╰─ /
    │     │     │  │     │  ╰─ reate_user
    │     │     │  │     │     ╰─ /
    │     │     │  │     ├─ new_cluster_docs
    │     │     │  │     │  ╰─ /
    │     │     │  │     ├─ <cluster_id>
    │     │     │  │     │  ╰─ /integration/create_or_update
    │     │     │  │     │     ╰─ /
    │     │     │  │     ╰─ <id>
    │     │     │  │        ╰─ /
    │     │     │  │           ├─ cl
    │     │     │  │           │  ├─ ear_cache
    │     │     │  │           │  │  ╰─ /
    │     │     │  │           │  ╰─ uster_status
    │     │     │  │           │     ╰─ /
    │     │     │  │           ├─ environments
    │     │     │  │           │  ╰─ /
    │     │     │  │           ╰─ metrics
    │     │     │  │              ├─ /
    │     │     │  │              ╰─ _dashboard
    │     │     │  │                 ╰─ /
    │     │     │  ├─ o
    │     │     │  │  ├─ mment_templates
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ╰─ <id>
    │     │     │  │  │        ╰─ /
    │     │     │  │  ╰─ nt
    │     │     │  │     ├─ ainer_registries
    │     │     │  │     │  ╰─ /
    │     │     │  │     │     ╰─ <id>
    │     │     │  │     │        ╰─ /
    │     │     │  │     ╰─ ribution_analytics
    │     │     │  │        ╰─ /
    │     │     │  ├─ rm/
    │     │     │  │  ├─ contacts
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ new
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ╰─ <id>
    │     │     │  │  │        ╰─ /edit
    │     │     │  │  │           ╰─ /
    │     │     │  │  ╰─ organizations
    │     │     │  │     ╰─ /
    │     │     │  │        ├─ new
    │     │     │  │        │  ╰─ /
    │     │     │  │        ╰─ <id>
    │     │     │  │           ╰─ /edit
    │     │     │  │              ╰─ /
    │     │     │  ╰─ ustom_emoji
    │     │     │     ╰─ /
    │     │     │        ╰─ new
    │     │     │           ╰─ /
    │     │     ├─ d
    │     │     │  ├─ ep
    │     │     │  │  ├─ endenc
    │     │     │  │  │  ├─ ies
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  │     ╰─ l
    │     │     │  │  │  │        ├─ icenses
    │     │     │  │  │  │        │  ╰─ /
    │     │     │  │  │  │        ╰─ ocations
    │     │     │  │  │  │           ╰─ /
    │     │     │  │  │  ╰─ y_proxy
    │     │     │  │  │     ╰─ /
    │     │     │  │  ╰─ loy_tokens/
    │     │     │  │     ╰─ <id>
    │     │     │  │        ╰─ /revoke
    │     │     │  │           ╰─ /
    │     │     │  ╰─ iscover
    │     │     │     ╰─ /
    │     │     ├─ epic
    │     │     │  ├─ _boards
    │     │     │  │  ╰─ /
    │     │     │  │     ╰─ <id>
    │     │     │  │        ╰─ /
    │     │     │  ╰─ s
    │     │     │     ╰─ /
    │     │     │        ├─ bulk_update
    │     │     │        │  ╰─ /
    │     │     │        ├─ new
    │     │     │        │  ╰─ /
    │     │     │        ├─ <id>
    │     │     │        │  ╰─ /
    │     │     │        │     ├─ d
    │     │     │        │     │  ├─ escriptions/
    │     │     │        │     │  │  ╰─ <version_id>
    │     │     │        │     │  │     ╰─ /
    │     │     │        │     │  │        ╰─ diff
    │     │     │        │     │  │           ╰─ /
    │     │     │        │     │  ╰─ iscussions
    │     │     │        │     │     ╰─ /
    │     │     │        │     ├─ edit
    │     │     │        │     │  ╰─ /
    │     │     │        │     ├─ realtime_changes
    │     │     │        │     │  ╰─ /
    │     │     │        │     ╰─ toggle_
    │     │     │        │        ├─ award_emoji
    │     │     │        │        │  ╰─ /
    │     │     │        │        ╰─ subscription
    │     │     │        │           ╰─ /
    │     │     │        ╰─ <epic_id>
    │     │     │           ╰─ /
    │     │     │              ├─ issues
    │     │     │              │  ╰─ /
    │     │     │              │     ╰─ <id>
    │     │     │              │        ╰─ /
    │     │     │              ├─ links
    │     │     │              │  ╰─ /
    │     │     │              │     ╰─ <id>
    │     │     │              │        ╰─ /
    │     │     │              ├─ notes
    │     │     │              │  ╰─ /
    │     │     │              │     ╰─ <id>
    │     │     │              │        ╰─ /
    │     │     │              │           ╰─ toggle_award_emoji
    │     │     │              │              ╰─ /
    │     │     │              ╰─ related_epic_links
    │     │     │                 ╰─ /
    │     │     │                    ╰─ <id>
    │     │     │                       ╰─ /
    │     │     ├─ group_
    │     │     │  ├─ links/
    │     │     │  │  ╰─ <id>
    │     │     │  │     ╰─ /
    │     │     │  ╰─ members
    │     │     │     ╰─ /
    │     │     │        ├─ bulk_reassignment_file
    │     │     │        │  ╰─ /
    │     │     │        ├─ export_csv
    │     │     │        │  ╰─ /
    │     │     │        ├─ leave
    │     │     │        │  ╰─ /
    │     │     │        ├─ request_access
    │     │     │        │  ╰─ /
    │     │     │        ╰─ <id>
    │     │     │           ╰─ /
    │     │     │              ├─ approve_access_request
    │     │     │              │  ╰─ /
    │     │     │              ├─ ban
    │     │     │              │  ╰─ /
    │     │     │              ├─ override
    │     │     │              │  ╰─ /
    │     │     │              ├─ resend_invite
    │     │     │              │  ╰─ /
    │     │     │              ╰─ unban
    │     │     │                 ╰─ /
    │     │     ├─ h
    │     │     │  ├─ arbor/repositories
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ <repository_id>
    │     │     │  │     │  ╰─ /artifacts
    │     │     │  │     │     ╰─ /
    │     │     │  │     │        ╰─ <artifact_id>
    │     │     │  │     │           ╰─ /tags
    │     │     │  │     │              ╰─ /
    │     │     │  │     ╰─ <id>
    │     │     │  │        ╰─ /
    │     │     │  ╰─ ooks
    │     │     │     ╰─ /
    │     │     │        ├─ <hook_id>
    │     │     │        │  ╰─ /hook_logs/
    │     │     │        │     ╰─ <id>
    │     │     │        │        ╰─ /
    │     │     │        │           ╰─ retry
    │     │     │        │              ╰─ /
    │     │     │        ╰─ <id>
    │     │     │           ╰─ /
    │     │     │              ├─ edit
    │     │     │              │  ╰─ /
    │     │     │              ╰─ test
    │     │     │                 ╰─ /
    │     │     ├─ i
    │     │     │  ├─ mport
    │     │     │  │  ╰─ /
    │     │     │  ├─ n
    │     │     │  │  ├─ frastructure_registry
    │     │     │  │  │  ╰─ /
    │     │     │  │  ╰─ sights
    │     │     │  │     ╰─ /
    │     │     │  │        ╰─ query
    │     │     │  │           ╰─ /
    │     │     │  ├─ ssues
    │     │     │  │  ├─ /bulk_update
    │     │     │  │  │  ╰─ /
    │     │     │  │  ╰─ _analytics
    │     │     │  │     ╰─ /
    │     │     │  ╰─ terations
    │     │     │     ╰─ /
    │     │     │        ├─ new
    │     │     │        │  ╰─ /
    │     │     │        ╰─ <id>
    │     │     │           ╰─ /
    │     │     │              ╰─ edit
    │     │     │                 ╰─ /
    │     │     ├─ l
    │     │     │  ├─ abels
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ new
    │     │     │  │     │  ╰─ /
    │     │     │  │     ╰─ <id>
    │     │     │  │        ╰─ /
    │     │     │  │           ├─ edit
    │     │     │  │           │  ╰─ /
    │     │     │  │           ╰─ toggle_subscription
    │     │     │  │              ╰─ /
    │     │     │  ╰─ dap
    │     │     │     ├─ /sync
    │     │     │     │  ╰─ /
    │     │     │     ╰─ _group_links
    │     │     │        ╰─ /
    │     │     │           ╰─ <id>
    │     │     │              ╰─ /
    │     │     ├─ m
    │     │     │  ├─ erge_requests/bulk_update
    │     │     │  │  ╰─ /
    │     │     │  ╰─ ilestones
    │     │     │     ╰─ /
    │     │     │        ├─ new
    │     │     │        │  ╰─ /
    │     │     │        ╰─ <id>
    │     │     │           ╰─ /
    │     │     │              ├─ edit
    │     │     │              │  ╰─ /
    │     │     │              ├─ issues
    │     │     │              │  ╰─ /
    │     │     │              ├─ labels
    │     │     │              │  ╰─ /
    │     │     │              ├─ merge_requests
    │     │     │              │  ╰─ /
    │     │     │              ╰─ participants
    │     │     │                 ╰─ /
    │     │     ├─ notification_setting
    │     │     │  ╰─ /
    │     │     ├─ p
    │     │     │  ├─ ackages
    │     │     │  │  ╰─ /
    │     │     │  │     ╰─ <id>
    │     │     │  │        ╰─ /
    │     │     │  ├─ r
    │     │     │  │  ├─ eview_markdown
    │     │     │  │  │  ╰─ /
    │     │     │  │  ╰─ otected_
    │     │     │  │     ├─ branches
    │     │     │  │     │  ╰─ /
    │     │     │  │     │     ╰─ <id>
    │     │     │  │     │        ╰─ /
    │     │     │  │     ╰─ environments
    │     │     │  │        ╰─ /
    │     │     │  │           ╰─ <id>
    │     │     │  │              ╰─ /
    │     │     │  ╰─ ush_rules
    │     │     │     ╰─ /
    │     │     ├─ r
    │     │     │  ├─ e
    │     │     │  │  ├─ leases
    │     │     │  │  │  ╰─ /
    │     │     │  │  ╰─ store
    │     │     │  │     ╰─ /
    │     │     │  ├─ oadmap
    │     │     │  │  ╰─ /
    │     │     │  ╰─ unners
    │     │     │     ╰─ /
    │     │     │        ├─ dashboard
    │     │     │        │  ╰─ /
    │     │     │        ├─ new
    │     │     │        │  ╰─ /
    │     │     │        ╰─ <id>
    │     │     │           ╰─ /
    │     │     │              ├─ edit
    │     │     │              │  ╰─ /
    │     │     │              ├─ pause
    │     │     │              │  ╰─ /
    │     │     │              ╰─ re
    │     │     │                 ├─ gister
    │     │     │                 │  ╰─ /
    │     │     │                 ╰─ sume
    │     │     │                    ╰─ /
    │     │     ├─ s
    │     │     │  ├─ aml
    │     │     │  │  ├─ /
    │     │     │  │  │  ├─ callback
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  ├─ sso
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  ╰─ u
    │     │     │  │  │     ├─ nlink
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ╰─ pdate_microsoft_application
    │     │     │  │  │        ╰─ /
    │     │     │  │  ╰─ _group_links
    │     │     │  │     ╰─ /
    │     │     │  │        ╰─ <id>
    │     │     │  │           ╰─ /
    │     │     │  ├─ cim_oauth
    │     │     │  │  ╰─ /
    │     │     │  ├─ e
    │     │     │  │  ├─ at_usage
    │     │     │  │  │  ╰─ /
    │     │     │  │  ├─ curity/
    │     │     │  │  │  ├─ c
    │     │     │  │  │  │  ├─ ompliance_
    │     │     │  │  │  │  │  ├─ dashboard
    │     │     │  │  │  │  │  │  ╰─ /
    │     │     │  │  │  │  │  │     ├─ <*vueroute>
    │     │     │  │  │  │  │  │     │  ╰─ /
    │     │     │  │  │  │  │  │     ╰─ <*vueroute>
    │     │     │  │  │  │  │  ├─ framework_reports
    │     │     │  │  │  │  │  │  ├─ .
    │     │     │  │  │  │  │  │  │  ╰─ <format>
    │     │     │  │  │  │  │  │  │     ╰─ /
    │     │     │  │  │  │  │  │  ╰─ /
    │     │     │  │  │  │  │  ├─ project_framework_reports
    │     │     │  │  │  │  │  │  ├─ .
    │     │     │  │  │  │  │  │  │  ╰─ <format>
    │     │     │  │  │  │  │  │  │     ╰─ /
    │     │     │  │  │  │  │  │  ╰─ /
    │     │     │  │  │  │  │  ├─ standards_adherence_reports
    │     │     │  │  │  │  │  │  ├─ .
    │     │     │  │  │  │  │  │  │  ╰─ <format>
    │     │     │  │  │  │  │  │  │     ╰─ /
    │     │     │  │  │  │  │  │  ╰─ /
    │     │     │  │  │  │  │  ╰─ violation_reports
    │     │     │  │  │  │  │     ├─ .
    │     │     │  │  │  │  │     │  ╰─ <format>
    │     │     │  │  │  │  │     │     ╰─ /
    │     │     │  │  │  │  │     ╰─ /
    │     │     │  │  │  │  ╰─ redentials
    │     │     │  │  │  │     ╰─ /
    │     │     │  │  │  │        ╰─ <id>
    │     │     │  │  │  │           ╰─ /
    │     │     │  │  │  │              ╰─ revoke
    │     │     │  │  │  │                 ╰─ /
    │     │     │  │  │  ├─ d
    │     │     │  │  │  │  ├─ ashboard
    │     │     │  │  │  │  │  ╰─ /
    │     │     │  │  │  │  ╰─ iscover
    │     │     │  │  │  │     ╰─ /
    │     │     │  │  │  ├─ merge_commit_reports
    │     │     │  │  │  │  ├─ .
    │     │     │  │  │  │  │  ╰─ <format>
    │     │     │  │  │  │  │     ╰─ /
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  ├─ policies
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  │     ├─ new
    │     │     │  │  │  │     │  ╰─ /
    │     │     │  │  │  │     ├─ schema
    │     │     │  │  │  │     │  ╰─ /
    │     │     │  │  │  │     ╰─ <id>
    │     │     │  │  │  │        ╰─ /edit
    │     │     │  │  │  │           ╰─ /
    │     │     │  │  │  ╰─ vulnerabilities
    │     │     │  │  │     ╰─ /
    │     │     │  │  ├─ rvice_accounts
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ new
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ├─ <id>
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     │     ╰─ edit
    │     │     │  │  │     │        ╰─ /
    │     │     │  │  │     ├─ <*vueroute>
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     │     ├─ new
    │     │     │  │  │     │     │  ╰─ /
    │     │     │  │  │     │     ╰─ <id>
    │     │     │  │  │     │        ╰─ /
    │     │     │  │  │     │           ╰─ edit
    │     │     │  │  │     │              ╰─ /
    │     │     │  │  │     ╰─ <*vueroute>
    │     │     │  │  ╰─ ttings/
    │     │     │  │     ├─ a
    │     │     │  │     │  ├─ ccess_tokens
    │     │     │  │     │  │  ╰─ /
    │     │     │  │     │  │     ╰─ <id>
    │     │     │  │     │  │        ╰─ /revoke
    │     │     │  │     │  │           ╰─ /
    │     │     │  │     │  ├─ nalytics
    │     │     │  │     │  │  ╰─ /
    │     │     │  │     │  ╰─ pplications
    │     │     │  │     │     ╰─ /
    │     │     │  │     │        ├─ new
    │     │     │  │     │        │  ╰─ /
    │     │     │  │     │        ╰─ <id>
    │     │     │  │     │           ╰─ /
    │     │     │  │     │              ├─ edit
    │     │     │  │     │              │  ╰─ /
    │     │     │  │     │              ╰─ renew
    │     │     │  │     │                 ╰─ /
    │     │     │  │     ├─ ci_cd
    │     │     │  │     │  ╰─ /
    │     │     │  │     │     ├─ deploy_token/create
    │     │     │  │     │     │  ╰─ /
    │     │     │  │     │     ├─ r
    │     │     │  │     │     │  ├─ eset_registration_token
    │     │     │  │     │     │  │  ╰─ /
    │     │     │  │     │     │  ╰─ unner_setup_scripts
    │     │     │  │     │     │     ╰─ /
    │     │     │  │     │     ╰─ update_auto_devops
    │     │     │  │     │        ╰─ /
    │     │     │  │     ├─ domain_verification
    │     │     │  │     │  ╰─ /
    │     │     │  │     │     ├─ new
    │     │     │  │     │     │  ╰─ /
    │     │     │  │     │     ╰─ <id>
    │     │     │  │     │        ╰─ /
    │     │     │  │     │           ├─ clean_certificate
    │     │     │  │     │           │  ╰─ /
    │     │     │  │     │           ├─ retry_auto_ssl
    │     │     │  │     │           │  ╰─ /
    │     │     │  │     │           ╰─ verify
    │     │     │  │     │              ╰─ /
    │     │     │  │     ├─ gitlab_duo_usage
    │     │     │  │     │  ╰─ /
    │     │     │  │     ├─ integrations
    │     │     │  │     │  ╰─ /
    │     │     │  │     │     ╰─ <id>
    │     │     │  │     │        ╰─ /
    │     │     │  │     │           ├─ edit
    │     │     │  │     │           │  ╰─ /
    │     │     │  │     │           ├─ reset
    │     │     │  │     │           │  ╰─ /
    │     │     │  │     │           ╰─ test
    │     │     │  │     │              ╰─ /
    │     │     │  │     ├─ merge_requests
    │     │     │  │     │  ╰─ /
    │     │     │  │     ├─ packages_and_registries
    │     │     │  │     │  ╰─ /
    │     │     │  │     ├─ r
    │     │     │  │     │  ├─ epo
    │     │     │  │     │  │  ├─ rting
    │     │     │  │     │  │  │  ╰─ /
    │     │     │  │     │  │  ╰─ sitory
    │     │     │  │     │  │     ╰─ /
    │     │     │  │     │  │        ╰─ deploy_token/create
    │     │     │  │     │  │           ╰─ /
    │     │     │  │     │  ╰─ oles_and_permissions
    │     │     │  │     │     ╰─ /
    │     │     │  │     │        ├─ new
    │     │     │  │     │        │  ╰─ /
    │     │     │  │     │        ╰─ <id>
    │     │     │  │     │           ╰─ /
    │     │     │  │     │              ╰─ edit
    │     │     │  │     │                 ╰─ /
    │     │     │  │     ├─ slack
    │     │     │  │     │  ╰─ /
    │     │     │  │     │     ╰─ slack_auth
    │     │     │  │     │        ╰─ /
    │     │     │  │     ╰─ workspaces
    │     │     │  │        ╰─ /
    │     │     │  ╰─ hared_projects
    │     │     │     ╰─ /
    │     │     ├─ t
    │     │     │  ├─ erraform_module_registry
    │     │     │  │  ╰─ /
    │     │     │  ├─ odos
    │     │     │  │  ╰─ /
    │     │     │  ╰─ wo_factor_auth
    │     │     │     ╰─ /
    │     │     ├─ u
    │     │     │  ├─ ploads
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ authorize
    │     │     │  │     │  ╰─ /
    │     │     │  │     ╰─ <secret>
    │     │     │  │        ╰─ /
    │     │     │  │           ╰─ <filename>
    │     │     │  │              ├─ .
    │     │     │  │              │  ╰─ <format>
    │     │     │  │              │     ╰─ /
    │     │     │  │              ╰─ /
    │     │     │  ╰─ sage_quotas
    │     │     │     ╰─ /
    │     │     │        ├─ pending_members
    │     │     │        │  ╰─ /
    │     │     │        ╰─ subscription_history
    │     │     │           ├─ .
    │     │     │           │  ╰─ <format>
    │     │     │           │     ╰─ /
    │     │     │           ╰─ /
    │     │     ├─ variables
    │     │     │  ╰─ /
    │     │     ╰─ w
    │     │        ├─ ikis
    │     │        │  ╰─ /
    │     │        │     ├─ -/confluence
    │     │        │     │  ╰─ /
    │     │        │     ├─ git_access
    │     │        │     │  ╰─ /
    │     │        │     ├─ new
    │     │        │     │  ╰─ /
    │     │        │     ├─ pages
    │     │        │     │  ╰─ /
    │     │        │     ├─ templates
    │     │        │     │  ╰─ /
    │     │        │     ├─ <*id>
    │     │        │     │  ╰─ /
    │     │        │     │     ├─ diff
    │     │        │     │     │  ╰─ /
    │     │        │     │     ├─ edit
    │     │        │     │     │  ╰─ /
    │     │        │     │     ├─ history
    │     │        │     │     │  ╰─ /
    │     │        │     │     ├─ preview_markdown
    │     │        │     │     │  ╰─ /
    │     │        │     │     ╰─ raw
    │     │        │     │        ╰─ /
    │     │        │     ╰─ <*id>
    │     │        ╰─ ork_items
    │     │           ╰─ /
    │     │              ╰─ <iid>
    │     │                 ╰─ /
    │     │                    ╰─ descriptions/
    │     │                       ╰─ <version_id>
    │     │                          ╰─ /
    │     │                             ╰─ diff
    │     │                                ╰─ /
    │     ├─ <*id>
    │     │  ├─ .
    │     │  │  ╰─ <format>
    │     │  │     ╰─ /
    │     │  ╰─ /
    │     │     ╰─ -/
    │     │        ├─ a
    │     │        │  ├─ ctivity
    │     │        │  │  ├─ .
    │     │        │  │  │  ╰─ <format>
    │     │        │  │  │     ╰─ /
    │     │        │  │  ╰─ /
    │     │        │  ╰─ rchived
    │     │        │     ├─ .
    │     │        │     │  ╰─ <format>
    │     │        │     │     ╰─ /
    │     │        │     ╰─ /
    │     │        ├─ d
    │     │        │  ├─ etails
    │     │        │  │  ├─ .
    │     │        │  │  │  ╰─ <format>
    │     │        │  │  │     ╰─ /
    │     │        │  │  ╰─ /
    │     │        │  ╰─ ownload_export
    │     │        │     ├─ .
    │     │        │     │  ╰─ <format>
    │     │        │     │     ╰─ /
    │     │        │     ╰─ /
    │     │        ├─ e
    │     │        │  ├─ dit
    │     │        │  │  ├─ .
    │     │        │  │  │  ╰─ <format>
    │     │        │  │  │     ╰─ /
    │     │        │  │  ╰─ /
    │     │        │  ╰─ xport
    │     │        │     ├─ .
    │     │        │     │  ╰─ <format>
    │     │        │     │     ╰─ /
    │     │        │     ╰─ /
    │     │        ├─ i
    │     │        │  ├─ nactive
    │     │        │  │  ├─ .
    │     │        │  │  │  ╰─ <format>
    │     │        │  │  │     ╰─ /
    │     │        │  │  ╰─ /
    │     │        │  ╰─ ssues
    │     │        │     ├─ .
    │     │        │     │  ╰─ <format>
    │     │        │     │     ╰─ /
    │     │        │     ╰─ /
    │     │        ├─ merge_requests
    │     │        │  ├─ .
    │     │        │  │  ╰─ <format>
    │     │        │  │     ╰─ /
    │     │        │  ╰─ /
    │     │        ├─ projects
    │     │        │  ├─ .
    │     │        │  │  ╰─ <format>
    │     │        │  │     ╰─ /
    │     │        │  ╰─ /
    │     │        ├─ shared
    │     │        │  ├─ .
    │     │        │  │  ╰─ <format>
    │     │        │  │     ╰─ /
    │     │        │  ╰─ /
    │     │        ├─ transfer
    │     │        │  ├─ .
    │     │        │  │  ╰─ <format>
    │     │        │  │     ╰─ /
    │     │        │  ╰─ /
    │     │        ╰─ unfoldered_environment_names
    │     │           ├─ .
    │     │           │  ╰─ <format>
    │     │           │     ╰─ /
    │     │           ╰─ /
    │     ╰─ <*id>
    ├─ he
    │  ├─ alth_check
    │  │  ├─ .
    │  │  │  ╰─ <format>
    │  │  │     ╰─ /
    │  │  ╰─ /
    │  │     ╰─ <checks>
    │  │        ├─ .
    │  │        │  ╰─ <format>
    │  │        │     ╰─ /
    │  │        ╰─ /
    │  ╰─ lp
    │     ╰─ /
    │        ├─ d
    │        │  ├─ ocs
    │        │  │  ╰─ /
    │        │  ╰─ rawers/
    │        │     ├─ <*markdown_file>
    │        │     │  ╰─ /
    │        │     ╰─ <*markdown_file>
    │        ├─ instance_configuration
    │        │  ╰─ /
    │        ├─ shortcuts
    │        │  ╰─ /
    │        ├─ <*path>
    │        │  ╰─ /
    │        ╰─ <*path>
    ├─ import/
    │  ├─ b
    │  │  ├─ itbucket
    │  │  │  ├─ /
    │  │  │  │  ├─ callback
    │  │  │  │  │  ╰─ /
    │  │  │  │  ├─ realtime_changes
    │  │  │  │  │  ╰─ /
    │  │  │  │  ╰─ status
    │  │  │  │     ╰─ /
    │  │  │  ╰─ _server
    │  │  │     ╰─ /
    │  │  │        ├─ c
    │  │  │        │  ├─ allback
    │  │  │        │  │  ╰─ /
    │  │  │        │  ╰─ onfigure
    │  │  │        │     ╰─ /
    │  │  │        ├─ new
    │  │  │        │  ╰─ /
    │  │  │        ├─ realtime_changes
    │  │  │        │  ╰─ /
    │  │  │        ╰─ status
    │  │  │           ╰─ /
    │  │  ╰─ ulk_imports
    │  │     ╰─ /
    │  │        ├─ configure
    │  │        │  ╰─ /
    │  │        ├─ history
    │  │        │  ╰─ /
    │  │        ├─ realtime_changes
    │  │        │  ╰─ /
    │  │        ├─ status
    │  │        │  ╰─ /
    │  │        ╰─ <id>
    │  │           ╰─ /history
    │  │              ╰─ /
    │  │                 ╰─ <entity_id>
    │  │                    ╰─ /failures
    │  │                       ╰─ /
    │  ├─ fogbugz
    │  │  ╰─ /
    │  │     ├─ callback
    │  │     │  ╰─ /
    │  │     ├─ new
    │  │     │  ╰─ /
    │  │     ├─ realtime_changes
    │  │     │  ╰─ /
    │  │     ├─ status
    │  │     │  ╰─ /
    │  │     ╰─ user_map
    │  │        ╰─ /
    │  ├─ git
    │  │  ├─ ea
    │  │  │  ╰─ /
    │  │  │     ├─ new
    │  │  │     │  ╰─ /
    │  │  │     ├─ personal_access_token
    │  │  │     │  ╰─ /
    │  │  │     ├─ realtime_changes
    │  │  │     │  ╰─ /
    │  │  │     ╰─ status
    │  │  │        ╰─ /
    │  │  ├─ hub
    │  │  │  ├─ /
    │  │  │  │  ├─ c
    │  │  │  │  │  ├─ a
    │  │  │  │  │  │  ├─ llback
    │  │  │  │  │  │  │  ╰─ /
    │  │  │  │  │  │  ╰─ ncel
    │  │  │  │  │  │     ├─ /
    │  │  │  │  │  │     ╰─ _all
    │  │  │  │  │  │        ╰─ /
    │  │  │  │  │  ╰─ ounts
    │  │  │  │  │     ╰─ /
    │  │  │  │  ├─ details
    │  │  │  │  │  ╰─ /
    │  │  │  │  ├─ failures
    │  │  │  │  │  ╰─ /
    │  │  │  │  ├─ new
    │  │  │  │  │  ╰─ /
    │  │  │  │  ├─ personal_access_token
    │  │  │  │  │  ╰─ /
    │  │  │  │  ├─ realtime_changes
    │  │  │  │  │  ╰─ /
    │  │  │  │  ╰─ status
    │  │  │  │     ╰─ /
    │  │  │  ╰─ _group/status
    │  │  │     ╰─ /
    │  │  ╰─ lab_
    │  │     ├─ group
    │  │     │  ╰─ /
    │  │     │     ╰─ authorize
    │  │     │        ╰─ /
    │  │     ╰─ project
    │  │        ╰─ /
    │  │           ├─ authorize
    │  │           │  ╰─ /
    │  │           ╰─ new
    │  │              ╰─ /
    │  ├─ history
    │  │  ╰─ /
    │  ├─ manifest
    │  │  ╰─ /
    │  │     ├─ new
    │  │     │  ╰─ /
    │  │     ├─ realtime_changes
    │  │     │  ╰─ /
    │  │     ├─ status
    │  │     │  ╰─ /
    │  │     ╰─ upload
    │  │        ╰─ /
    │  ├─ source_users/
    │  │  ╰─ <id>
    │  │     ╰─ /
    │  │        ├─ accept
    │  │        │  ╰─ /
    │  │        ╰─ decline
    │  │           ╰─ /
    │  ╰─ url/validate
    │     ╰─ /
    ├─ jwt/auth
    │  ╰─ /
    ├─ oauth/
    │  ├─ a
    │  │  ├─ pplications
    │  │  │  ╰─ /
    │  │  │     ├─ new
    │  │  │     │  ╰─ /
    │  │  │     ╰─ <id>
    │  │  │        ╰─ /
    │  │  │           ├─ edit
    │  │  │           │  ╰─ /
    │  │  │           ╰─ renew
    │  │  │              ╰─ /
    │  │  ╰─ uthorize
    │  │     ├─ /
    │  │     │  ╰─ native
    │  │     │     ╰─ /
    │  │     ├─ _device
    │  │     │  ╰─ /
    │  │     ╰─ d_applications
    │  │        ╰─ /
    │  │           ╰─ <id>
    │  │              ╰─ /
    │  ├─ d
    │  │  ├─ evice
    │  │  │  ╰─ /
    │  │  │     ╰─ confirm
    │  │  │        ╰─ /
    │  │  ╰─ iscovery/keys
    │  │     ╰─ /
    │  ├─ geo/
    │  │  ├─ auth
    │  │  │  ╰─ /
    │  │  ├─ callback
    │  │  │  ╰─ /
    │  │  ╰─ logout
    │  │     ╰─ /
    │  ├─ introspect
    │  │  ╰─ /
    │  ├─ revoke
    │  │  ╰─ /
    │  ├─ token
    │  │  ╰─ /
    │  │     ╰─ info
    │  │        ╰─ /
    │  ╰─ userinfo
    │     ╰─ /
    ├─ p
    │  ├─ rojects
    │  │  ╰─ /
    │  │     ├─ new
    │  │     │  ╰─ /
    │  │     ╰─ <id>
    │  │        ╰─ /
    │  ╰─ ublic
    │     ╰─ /
    │        ╰─ projects
    │           ╰─ /
    ├─ rails/
    │  ├─ features
    │  │  ╰─ /
    │  │     ├─ definitions
    │  │     │  ╰─ /
    │  │     ╰─ <id>
    │  │        ╰─ /
    │  ├─ info
    │  │  ╰─ /
    │  │     ├─ properties
    │  │     │  ╰─ /
    │  │     ╰─ routes
    │  │        ╰─ /
    │  ├─ l
    │  │  ├─ etter_opener
    │  │  │  ╰─ /
    │  │  │     ├─ clear
    │  │  │     │  ╰─ /
    │  │  │     ╰─ <id>
    │  │  │        ╰─ /
    │  │  │           ├─ attachments/
    │  │  │           │  ╰─ <file>
    │  │  │           │     ╰─ /
    │  │  │           ├─ delete
    │  │  │           │  ╰─ /
    │  │  │           ╰─ <style>
    │  │  │              ╰─ /
    │  │  ╰─ ookbook
    │  │     ╰─ /
    │  │        ├─ cable
    │  │        │  ╰─ /
    │  │        ├─ embed
    │  │        │  ╰─ /
    │  │        │     ├─ <*path>
    │  │        │     │  ╰─ /
    │  │        │     ╰─ <*path>
    │  │        ├─ inspect/
    │  │        │  ├─ <*path>
    │  │        │  │  ╰─ /
    │  │        │  ╰─ <*path>
    │  │        ├─ p
    │  │        │  ├─ ages
    │  │        │  │  ╰─ /
    │  │        │  │     ├─ <*path>
    │  │        │  │     │  ╰─ /
    │  │        │  │     ╰─ <*path>
    │  │        │  ╰─ review
    │  │        │     ├─ /
    │  │        │     │  ├─ <*path>
    │  │        │     │  │  ╰─ /
    │  │        │     │  ╰─ <*path>
    │  │        │     ╰─ s
    │  │        │        ╰─ /
    │  │        ├─ <*path>
    │  │        │  ╰─ /
    │  │        ╰─ <*path>
    │  ╰─ mailers
    │     ╰─ /
    │        ╰─ <path>
    │           ╰─ /
    ├─ s
    │  ├─ earch
    │  │  ╰─ /
    │  │     ├─ a
    │  │     │  ├─ ggregations
    │  │     │  │  ╰─ /
    │  │     │  ╰─ utocomplete
    │  │     │     ╰─ /
    │  │     ├─ count
    │  │     │  ╰─ /
    │  │     ├─ opensearch
    │  │     │  ╰─ /
    │  │     ╰─ settings
    │  │        ╰─ /
    │  ├─ itemap
    │  │  ╰─ /
    │  ╰─ nippets
    │     ╰─ /
    │        ├─ <id>
    │        │  ╰─ /raw
    │        │     ╰─ /
    │        ├─ <*rest>
    │        │  ╰─ /
    │        ╰─ <*rest>
    ├─ u
    │  ├─ nsubscribes/
    │  │  ╰─ <email>
    │  │     ╰─ /
    │  ├─ ploads/
    │  │  ├─ -/system/
    │  │  │  ├─ temp/
    │  │  │  │  ╰─ <secret>
    │  │  │  │     ╰─ /
    │  │  │  │        ╰─ <filename>
    │  │  │  │           ╰─ /
    │  │  │  ╰─ <model>
    │  │  │     ╰─ /
    │  │  │        ╰─ <id>
    │  │  │           ╰─ /
    │  │  │              ╰─ <secret>
    │  │  │                 ╰─ /
    │  │  │                    ╰─ <filename>
    │  │  │                       ╰─ /
    │  │  ╰─ <model>
    │  │     ╰─ /
    │  │        ╰─ authorize
    │  │           ╰─ /
    │  ╰─ sers
    │     ╰─ /
    │        ├─ a
    │        │  ├─ lmost_there
    │        │  │  ╰─ /
    │        │  ╰─ uth
    │        │     ╰─ /
    │        │        ├─ geo/sign_
    │        │        │  ├─ in
    │        │        │  │  ╰─ /
    │        │        │  ╰─ out
    │        │        │     ╰─ /
    │        │        ╰─ kerberos/negotiate
    │        │           ╰─ /
    │        ├─ c
    │        │  ├─ ancel
    │        │  │  ╰─ /
    │        │  ╰─ onfirmation
    │        │     ╰─ /
    │        │        ╰─ new
    │        │           ╰─ /
    │        ├─ edit
    │        │  ╰─ /
    │        ├─ identity_verification
    │        │  ╰─ /
    │        │     ├─ arkose_labs_challenge
    │        │     │  ╰─ /
    │        │     ├─ res
    │        │     │  ├─ end_email_code
    │        │     │  │  ╰─ /
    │        │     │  ╰─ tricted
    │        │     │     ╰─ /
    │        │     ├─ s
    │        │     │  ├─ end_phone_verification_code
    │        │     │  │  ╰─ /
    │        │     │  ╰─ uccess
    │        │     │     ╰─ /
    │        │     ├─ toggle_phone_exemption
    │        │     │  ╰─ /
    │        │     ╰─ verif
    │        │        ├─ ication_state
    │        │        │  ╰─ /
    │        │        ╰─ y_
    │        │           ├─ arkose_labs_session
    │        │           │  ╰─ /
    │        │           ├─ credit_card
    │        │           │  ├─ /
    │        │           │  ╰─ _captcha
    │        │           │     ╰─ /
    │        │           ├─ email_code
    │        │           │  ╰─ /
    │        │           ╰─ phone_verification_code
    │        │              ╰─ /
    │        ├─ password
    │        │  ╰─ /
    │        │     ├─ complexity
    │        │     │  ╰─ /
    │        │     ├─ edit
    │        │     │  ╰─ /
    │        │     ╰─ new
    │        │        ╰─ /
    │        ├─ resend_verification_code
    │        │  ╰─ /
    │        ├─ s
    │        │  ├─ ign_
    │        │  │  ├─ in
    │        │  │  │  ╰─ /
    │        │  │  ├─ out
    │        │  │  │  ╰─ /
    │        │  │  ╰─ up
    │        │  │     ╰─ /
    │        │  │        ├─ company
    │        │  │        │  ╰─ /
    │        │  │        │     ╰─ new
    │        │  │        │        ╰─ /
    │        │  │        ├─ groups
    │        │  │        │  ╰─ /
    │        │  │        │     ╰─ new
    │        │  │        │        ╰─ /
    │        │  │        ╰─ welcome
    │        │  │           ╰─ /
    │        │  ╰─ uccessful_verification
    │        │     ╰─ /
    │        ├─ u
    │        │  ├─ nlock
    │        │  │  ╰─ /
    │        │  │     ╰─ new
    │        │  │        ╰─ /
    │        │  ╰─ pdate_email
    │        │     ╰─ /
    │        ╰─ <username>
    │           ╰─ /
    │              ├─ a
    │              │  ├─ ctivity
    │              │  │  ╰─ /
    │              │  ╰─ vailable_
    │              │     ├─ group_templates
    │              │     │  ╰─ /
    │              │     ╰─ project_templates
    │              │        ╰─ /
    │              ├─ c
    │              │  ├─ alendar
    │              │  │  ├─ /
    │              │  │  ╰─ _activities
    │              │  │     ╰─ /
    │              │  ╰─ ontributed
    │              │     ╰─ /
    │              ├─ exists
    │              │  ╰─ /
    │              ├─ follow
    │              │  ├─ /
    │              │  ├─ ers
    │              │  │  ╰─ /
    │              │  ╰─ ing
    │              │     ╰─ /
    │              ├─ groups
    │              │  ╰─ /
    │              ├─ projects
    │              │  ╰─ /
    │              ├─ s
    │              │  ├─ nippets
    │              │  │  ╰─ /
    │              │  ╰─ tarred
    │              │     ╰─ /
    │              ╰─ unfollow
    │                 ╰─ /
    ├─ v2
    │  ╰─ /
    │     ╰─ <*group_id>
    │        ╰─ /dependency_proxy/containers/
    │           ╰─ <*image>
    │              ╰─ /
    │                 ├─ blobs/
    │                 │  ╰─ <sha>
    │                 │     ╰─ /
    │                 │        ╰─ upload
    │                 │           ╰─ /
    │                 │              ╰─ authorize
    │                 │                 ╰─ /
    │                 ╰─ manifests/
    │                    ├─ <*tag>
    │                    │  ╰─ /
    │                    │     ╰─ upload
    │                    │        ╰─ /
    │                    │           ╰─ authorize
    │                    │              ╰─ /
    │                    ╰─ <*tag>
    ├─ <username>
    │  ├─ .
    │  │  ├─ gpg
    │  │  │  ╰─ /
    │  │  ╰─ keys
    │  │     ╰─ /
    │  ╰─ /
    ├─ <*repository_path>
    │  ╰─ /
    │     ├─ git
    │     │  ├─ -
    │     │  │  ├─ receive-pack
    │     │  │  │  ╰─ /
    │     │  │  ╰─ upload-pack
    │     │  │     ╰─ /
    │     │  ╰─ lab-lfs/objects/
    │     │     ├─ <*oid>
    │     │     │  ╰─ /
    │     │     │     ╰─ <size>
    │     │     │        ╰─ /
    │     │     │           ╰─ authorize
    │     │     │              ╰─ /
    │     │     ╰─ <*oid>
    │     ├─ info/
    │     │  ├─ lfs/
    │     │  │  ├─ locks
    │     │  │  │  ╰─ /
    │     │  │  │     ├─ new
    │     │  │  │     │  ╰─ /
    │     │  │  │     ├─ verify
    │     │  │  │     │  ╰─ /
    │     │  │  │     ╰─ <id>
    │     │  │  │        ╰─ /
    │     │  │  │           ├─ edit
    │     │  │  │           │  ╰─ /
    │     │  │  │           ╰─ unlock
    │     │  │  │              ╰─ /
    │     │  │  ╰─ objects
    │     │  │     ╰─ /
    │     │  │        ├─ batch
    │     │  │        │  ╰─ /
    │     │  │        ├─ <*oid>
    │     │  │        │  ╰─ /
    │     │  │        ╰─ <*oid>
    │     │  ╰─ refs
    │     │     ╰─ /
    │     ╰─ ssh-
    │        ├─ receive-pack
    │        │  ╰─ /
    │        ╰─ upload-pack
    │           ╰─ /
    ├─ <*id>
    │  ├─ .
    │  │  ╰─ <format>
    │  │     ╰─ /
    │  ╰─ /
    ├─ <*namespace_id>
    │  ╰─ /
    │     ├─ <project_id>
    │     │  ╰─ /
    │     │     ├─ -/
    │     │     │  ├─ a
    │     │     │  │  ├─ lert_management
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ╰─ <id>
    │     │     │  │  │        ╰─ /
    │     │     │  │  │           ╰─ details
    │     │     │  │  │              ╰─ /
    │     │     │  │  │                 ├─ <*page>
    │     │     │  │  │                 │  ╰─ /
    │     │     │  │  │                 ╰─ <*page>
    │     │     │  │  ├─ nalytics/
    │     │     │  │  │  ├─ code_reviews
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  ├─ dashboards
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  │     ├─ <*vueroute>
    │     │     │  │  │  │     │  ╰─ /
    │     │     │  │  │  │     ╰─ <*vueroute>
    │     │     │  │  │  ├─ issues_analytics
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  ├─ merge_request_analytics
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  ╰─ value_stream_analytics
    │     │     │  │  │     ╰─ /
    │     │     │  │  │        ├─ summary
    │     │     │  │  │        │  ╰─ /
    │     │     │  │  │        ├─ time_summary
    │     │     │  │  │        │  ╰─ /
    │     │     │  │  │        ╰─ value_streams
    │     │     │  │  │           ╰─ /
    │     │     │  │  │              ├─ new
    │     │     │  │  │              │  ╰─ /
    │     │     │  │  │              ├─ <value_stream_id>
    │     │     │  │  │              │  ╰─ /stages
    │     │     │  │  │              │     ╰─ /
    │     │     │  │  │              │        ╰─ <id>
    │     │     │  │  │              │           ╰─ /
    │     │     │  │  │              │              ├─ average
    │     │     │  │  │              │              │  ├─ /
    │     │     │  │  │              │              │  ╰─ _duration_chart
    │     │     │  │  │              │              │     ╰─ /
    │     │     │  │  │              │              ├─ count
    │     │     │  │  │              │              │  ╰─ /
    │     │     │  │  │              │              ├─ median
    │     │     │  │  │              │              │  ╰─ /
    │     │     │  │  │              │              ╰─ records
    │     │     │  │  │              │                 ╰─ /
    │     │     │  │  │              ╰─ <id>
    │     │     │  │  │                 ╰─ /
    │     │     │  │  │                    ╰─ edit
    │     │     │  │  │                       ╰─ /
    │     │     │  │  ├─ pprover
    │     │     │  │  │  ├─ _groups/
    │     │     │  │  │  │  ╰─ <id>
    │     │     │  │  │  │     ╰─ /
    │     │     │  │  │  ╰─ s/
    │     │     │  │  │     ╰─ <id>
    │     │     │  │  │        ╰─ /
    │     │     │  │  ├─ r
    │     │     │  │  │  ├─ chive/
    │     │     │  │  │  │  ╰─ <id>
    │     │     │  │  │  │     ╰─ .
    │     │     │  │  │  │        ╰─ <format>
    │     │     │  │  │  │           ╰─ /
    │     │     │  │  │  ╰─ tifacts
    │     │     │  │  │     ╰─ /
    │     │     │  │  │        ╰─ <id>
    │     │     │  │  │           ╰─ /
    │     │     │  │  ├─ u
    │     │     │  │  │  ├─ dit_events
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  ╰─ to
    │     │     │  │  │     ├─ complete_sources/
    │     │     │  │  │     │  ├─ co
    │     │     │  │  │     │  │  ├─ mmands
    │     │     │  │  │     │  │  │  ╰─ /
    │     │     │  │  │     │  │  ╰─ ntacts
    │     │     │  │  │     │  │     ╰─ /
    │     │     │  │  │     │  ├─ epics
    │     │     │  │  │     │  │  ╰─ /
    │     │     │  │  │     │  ├─ i
    │     │     │  │  │     │  │  ├─ ssues
    │     │     │  │  │     │  │  │  ╰─ /
    │     │     │  │  │     │  │  ╰─ terations
    │     │     │  │  │     │  │     ╰─ /
    │     │     │  │  │     │  ├─ labels
    │     │     │  │  │     │  │  ╰─ /
    │     │     │  │  │     │  ├─ m
    │     │     │  │  │     │  │  ├─ e
    │     │     │  │  │     │  │  │  ├─ mbers
    │     │     │  │  │     │  │  │  │  ╰─ /
    │     │     │  │  │     │  │  │  ╰─ rge_requests
    │     │     │  │  │     │  │  │     ╰─ /
    │     │     │  │  │     │  │  ╰─ ilestones
    │     │     │  │  │     │  │     ╰─ /
    │     │     │  │  │     │  ├─ snippets
    │     │     │  │  │     │  │  ╰─ /
    │     │     │  │  │     │  ├─ vulnerabilities
    │     │     │  │  │     │  │  ╰─ /
    │     │     │  │  │     │  ╰─ wikis
    │     │     │  │  │     │     ╰─ /
    │     │     │  │  │     ╰─ mations
    │     │     │  │  │        ╰─ /
    │     │     │  │  ├─ vatar
    │     │     │  │  │  ╰─ /
    │     │     │  │  ╰─ ws
    │     │     │  │     ╰─ /
    │     │     │  │        ╰─ configuration
    │     │     │  │           ╰─ /
    │     │     │  ├─ b
    │     │     │  │  ├─ adges/release
    │     │     │  │  │  ├─ .
    │     │     │  │  │  │  ╰─ <format>
    │     │     │  │  │  │     ╰─ /
    │     │     │  │  │  ╰─ /
    │     │     │  │  ├─ l
    │     │     │  │  │  ├─ ame
    │     │     │  │  │  │  ├─ /
    │     │     │  │  │  │  │  ├─ <*id>
    │     │     │  │  │  │  │  │  ╰─ /
    │     │     │  │  │  │  │  │     ╰─ streaming
    │     │     │  │  │  │  │  │        ╰─ /
    │     │     │  │  │  │  │  ╰─ <*id>
    │     │     │  │  │  │  ╰─ _page/
    │     │     │  │  │  │     ├─ <*id>
    │     │     │  │  │  │     │  ╰─ /
    │     │     │  │  │  │     ╰─ <*id>
    │     │     │  │  │  ╰─ ob/
    │     │     │  │  │     ├─ <*id>
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     │     ╰─ diff
    │     │     │  │  │     │        ╰─ /
    │     │     │  │  │     ╰─ <*id>
    │     │     │  │  ├─ oards
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ╰─ <id>
    │     │     │  │  │        ╰─ /
    │     │     │  │  ╰─ ranches
    │     │     │  │     ╰─ /
    │     │     │  │        ├─ diverging_commit_counts
    │     │     │  │        │  ╰─ /
    │     │     │  │        ├─ new
    │     │     │  │        │  ╰─ /
    │     │     │  │        ╰─ <id>
    │     │     │  │           ╰─ /
    │     │     │  ├─ c
    │     │     │  │  ├─ adences
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ new
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ├─ <iteration_cadence_id>
    │     │     │  │  │     │  ╰─ /iterations
    │     │     │  │  │     │     ╰─ /
    │     │     │  │  │     │        ╰─ <id>
    │     │     │  │  │     │           ╰─ /
    │     │     │  │  │     ├─ <id>
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     │     ╰─ edit
    │     │     │  │  │     │        ╰─ /
    │     │     │  │  │     ├─ <*vueroute>
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     │     ├─ new
    │     │     │  │  │     │     │  ╰─ /
    │     │     │  │  │     │     ├─ <iteration_cadence_id>
    │     │     │  │  │     │     │  ╰─ /iterations
    │     │     │  │  │     │     │     ╰─ /
    │     │     │  │  │     │     │        ╰─ <id>
    │     │     │  │  │     │     │           ╰─ /
    │     │     │  │  │     │     ╰─ <id>
    │     │     │  │  │     │        ╰─ /
    │     │     │  │  │     │           ╰─ edit
    │     │     │  │  │     │              ╰─ /
    │     │     │  │  │     ╰─ <*vueroute>
    │     │     │  │  ├─ i/
    │     │     │  │  │  ├─ daily_build_group_report_results
    │     │     │  │  │  │  ├─ .
    │     │     │  │  │  │  │  ╰─ <format>
    │     │     │  │  │  │  │     ╰─ /
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  ├─ editor
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  ├─ lint
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  ╰─ prometheus_metrics/histograms
    │     │     │  │  │     ├─ .
    │     │     │  │  │     │  ╰─ <format>
    │     │     │  │  │     │     ╰─ /
    │     │     │  │  │     ╰─ /
    │     │     │  │  ├─ luster
    │     │     │  │  │  ├─ _agents/
    │     │     │  │  │  │  ╰─ <name>
    │     │     │  │  │  │     ╰─ /
    │     │     │  │  │  ╰─ s
    │     │     │  │  │     ╰─ /
    │     │     │  │  │        ├─ c
    │     │     │  │  │        │  ├─ onnect
    │     │     │  │  │        │  │  ╰─ /
    │     │     │  │  │        │  ╰─ reate_user
    │     │     │  │  │        │     ╰─ /
    │     │     │  │  │        ├─ new_cluster_docs
    │     │     │  │  │        │  ╰─ /
    │     │     │  │  │        ├─ <cluster_id>
    │     │     │  │  │        │  ╰─ /integration/create_or_update
    │     │     │  │  │        │     ╰─ /
    │     │     │  │  │        ╰─ <id>
    │     │     │  │  │           ╰─ /
    │     │     │  │  │              ├─ cl
    │     │     │  │  │              │  ├─ ear_cache
    │     │     │  │  │              │  │  ╰─ /
    │     │     │  │  │              │  ╰─ uster_status
    │     │     │  │  │              │     ╰─ /
    │     │     │  │  │              ├─ environments
    │     │     │  │  │              │  ╰─ /
    │     │     │  │  │              ╰─ metrics
    │     │     │  │  │                 ├─ /
    │     │     │  │  │                 ╰─ _dashboard
    │     │     │  │  │                    ╰─ /
    │     │     │  │  ├─ om
    │     │     │  │  │  ├─ m
    │     │     │  │  │  │  ├─ ent_templates
    │     │     │  │  │  │  │  ╰─ /
    │     │     │  │  │  │  │     ╰─ <id>
    │     │     │  │  │  │  │        ╰─ /
    │     │     │  │  │  │  ╰─ it
    │     │     │  │  │  │     ├─ /
    │     │     │  │  │  │     │  ╰─ <id>
    │     │     │  │  │  │     │     ╰─ /
    │     │     │  │  │  │     │        ├─ branches
    │     │     │  │  │  │     │        │  ╰─ /
    │     │     │  │  │  │     │        ├─ cherry_pick
    │     │     │  │  │  │     │        │  ╰─ /
    │     │     │  │  │  │     │        ├─ diff_f
    │     │     │  │  │  │     │        │  ├─ iles
    │     │     │  │  │  │     │        │  │  ╰─ /
    │     │     │  │  │  │     │        │  ╰─ or_path
    │     │     │  │  │  │     │        │     ╰─ /
    │     │     │  │  │  │     │        ├─ merge_requests
    │     │     │  │  │  │     │        │  ╰─ /
    │     │     │  │  │  │     │        ├─ pipelines
    │     │     │  │  │  │     │        │  ╰─ /
    │     │     │  │  │  │     │        ╰─ revert
    │     │     │  │  │  │     │           ╰─ /
    │     │     │  │  │  │     ╰─ s
    │     │     │  │  │  │        ╰─ /
    │     │     │  │  │  │           ├─ <*id>
    │     │     │  │  │  │           │  ╰─ /
    │     │     │  │  │  │           │     ╰─ signatures
    │     │     │  │  │  │           │        ╰─ /
    │     │     │  │  │  │           ╰─ <*id>
    │     │     │  │  │  ╰─ pare
    │     │     │  │  │     ╰─ /
    │     │     │  │  │        ├─ diff_for_path
    │     │     │  │  │        │  ╰─ /
    │     │     │  │  │        ├─ signatures
    │     │     │  │  │        │  ╰─ /
    │     │     │  │  │        ╰─ <from>
    │     │     │  │  │           ╰─ ...
    │     │     │  │  │              ╰─ <to>
    │     │     │  │  │                 ╰─ /
    │     │     │  │  ├─ reate
    │     │     │  │  │  ├─ /
    │     │     │  │  │  │  ├─ <*id>
    │     │     │  │  │  │  │  ╰─ /
    │     │     │  │  │  │  ╰─ <*id>
    │     │     │  │  │  ╰─ _dir/
    │     │     │  │  │     ├─ <*id>
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ╰─ <*id>
    │     │     │  │  ╰─ ycle_analytics
    │     │     │  │     ╰─ /
    │     │     │  ├─ de
    │     │     │  │  ├─ p
    │     │     │  │  │  ├─ endencies
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  ╰─ loy_
    │     │     │  │  │     ├─ keys
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     │     ├─ available_p
    │     │     │  │  │     │     │  ├─ roject_keys
    │     │     │  │  │     │     │  │  ╰─ /
    │     │     │  │  │     │     │  ╰─ ublic_keys
    │     │     │  │  │     │     │     ╰─ /
    │     │     │  │  │     │     ├─ enabled_keys
    │     │     │  │  │     │     │  ╰─ /
    │     │     │  │  │     │     ├─ new
    │     │     │  │  │     │     │  ╰─ /
    │     │     │  │  │     │     ╰─ <id>
    │     │     │  │  │     │        ╰─ /
    │     │     │  │  │     │           ├─ disable
    │     │     │  │  │     │           │  ╰─ /
    │     │     │  │  │     │           ╰─ e
    │     │     │  │  │     │              ├─ dit
    │     │     │  │  │     │              │  ╰─ /
    │     │     │  │  │     │              ╰─ nable
    │     │     │  │  │     │                 ╰─ /
    │     │     │  │  │     ╰─ tokens/
    │     │     │  │  │        ╰─ <id>
    │     │     │  │  │           ╰─ /revoke
    │     │     │  │  │              ╰─ /
    │     │     │  │  ╰─ sign_management/designs/
    │     │     │  │     ╰─ <design_id>
    │     │     │  │        ╰─ /
    │     │     │  │           ├─ r
    │     │     │  │           │  ├─ aw_image
    │     │     │  │           │  │  ╰─ /
    │     │     │  │           │  ╰─ esized_image/
    │     │     │  │           │     ╰─ <id>
    │     │     │  │           │        ╰─ /
    │     │     │  │           ╰─ <sha>
    │     │     │  │              ╰─ /r
    │     │     │  │                 ├─ aw_image
    │     │     │  │                 │  ╰─ /
    │     │     │  │                 ╰─ esized_image/
    │     │     │  │                    ╰─ <id>
    │     │     │  │                       ╰─ /
    │     │     │  ├─ e
    │     │     │  │  ├─ dit/
    │     │     │  │  │  ├─ <*id>
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  ╰─ <*id>
    │     │     │  │  ├─ nvironments
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ folders/
    │     │     │  │  │     │  ├─ <*id>
    │     │     │  │  │     │  │  ├─ .
    │     │     │  │  │     │  │  │  ╰─ <format>
    │     │     │  │  │     │  │  │     ╰─ /
    │     │     │  │  │     │  │  ╰─ /
    │     │     │  │  │     │  ╰─ <*id>
    │     │     │  │  │     ├─ new
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ├─ search
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ├─ <id>
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     │     ├─ cancel_auto_stop
    │     │     │  │  │     │     │  ╰─ /
    │     │     │  │  │     │     ├─ edit
    │     │     │  │  │     │     │  ╰─ /
    │     │     │  │  │     │     ├─ k8s
    │     │     │  │  │     │     │  ╰─ /
    │     │     │  │  │     │     │     ├─ <*vueroute>
    │     │     │  │  │     │     │     │  ╰─ /
    │     │     │  │  │     │     │     ╰─ <*vueroute>
    │     │     │  │  │     │     ├─ prometheus/api/v1/
    │     │     │  │  │     │     │  ├─ <*proxy_path>
    │     │     │  │  │     │     │  │  ╰─ /
    │     │     │  │  │     │     │  ╰─ <*proxy_path>
    │     │     │  │  │     │     ├─ stop
    │     │     │  │  │     │     │  ╰─ /
    │     │     │  │  │     │     ╰─ terminal
    │     │     │  │  │     │        ├─ .ws/authorize
    │     │     │  │  │     │        │  ╰─ /
    │     │     │  │  │     │        ╰─ /
    │     │     │  │  │     ╰─ <environment_id>
    │     │     │  │  │        ╰─ /deployments
    │     │     │  │  │           ╰─ /
    │     │     │  │  │              ╰─ <id>
    │     │     │  │  │                 ╰─ /
    │     │     │  │  │                    ├─ additional_metrics
    │     │     │  │  │                    │  ╰─ /
    │     │     │  │  │                    ╰─ metrics
    │     │     │  │  │                       ╰─ /
    │     │     │  │  ├─ rror_tracking
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ projects
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ╰─ <issue_id>
    │     │     │  │  │        ╰─ /
    │     │     │  │  │           ├─ details
    │     │     │  │  │           │  ╰─ /
    │     │     │  │  │           ╰─ stack_trace
    │     │     │  │  │              ╰─ /
    │     │     │  │  ╰─ scalation_policies
    │     │     │  │     ╰─ /
    │     │     │  ├─ f
    │     │     │  │  ├─ eature_flags
    │     │     │  │  │  ├─ /
    │     │     │  │  │  │  ├─ new
    │     │     │  │  │  │  │  ╰─ /
    │     │     │  │  │  │  ├─ <feature_flag_iid>
    │     │     │  │  │  │  │  ╰─ /issues
    │     │     │  │  │  │  │     ╰─ /
    │     │     │  │  │  │  │        ╰─ <id>
    │     │     │  │  │  │  │           ╰─ /
    │     │     │  │  │  │  ╰─ <iid>
    │     │     │  │  │  │     ╰─ /
    │     │     │  │  │  │        ╰─ edit
    │     │     │  │  │  │           ╰─ /
    │     │     │  │  │  ╰─ _
    │     │     │  │  │     ├─ client/reset_token
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ╰─ user_lists
    │     │     │  │  │        ╰─ /
    │     │     │  │  │           ├─ new
    │     │     │  │  │           │  ╰─ /
    │     │     │  │  │           ╰─ <iid>
    │     │     │  │  │              ╰─ /
    │     │     │  │  │                 ╰─ edit
    │     │     │  │  │                    ╰─ /
    │     │     │  │  ├─ i
    │     │     │  │  │  ├─ les/
    │     │     │  │  │  │  ├─ <*id>
    │     │     │  │  │  │  │  ╰─ /
    │     │     │  │  │  │  ╰─ <*id>
    │     │     │  │  │  ╰─ nd_file/
    │     │     │  │  │     ├─ <*id>
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ╰─ <*id>
    │     │     │  │  ╰─ orks
    │     │     │  │     ╰─ /
    │     │     │  │        ╰─ new
    │     │     │  │           ╰─ /
    │     │     │  ├─ g
    │     │     │  │  ├─ oogle_cloud
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ artifact_registry
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     │     ╰─ projects/
    │     │     │  │  │     │        ╰─ <project>
    │     │     │  │  │     │           ╰─ /locations/
    │     │     │  │  │     │              ╰─ <location>
    │     │     │  │  │     │                 ╰─ /repositories/
    │     │     │  │  │     │                    ╰─ <repository>
    │     │     │  │  │     │                       ╰─ /dockerImages/
    │     │     │  │  │     │                          ╰─ <image>
    │     │     │  │  │     │                             ╰─ /
    │     │     │  │  │     ├─ configuration
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ├─ d
    │     │     │  │  │     │  ├─ atabases
    │     │     │  │  │     │  │  ╰─ /
    │     │     │  │  │     │  │     ╰─ new/
    │     │     │  │  │     │  │        ╰─ <product>
    │     │     │  │  │     │  │           ╰─ /
    │     │     │  │  │     │  ╰─ eployments
    │     │     │  │  │     │     ╰─ /
    │     │     │  │  │     │        ╰─ cloud_
    │     │     │  │  │     │           ├─ run
    │     │     │  │  │     │           │  ╰─ /
    │     │     │  │  │     │           ╰─ storage
    │     │     │  │  │     │              ╰─ /
    │     │     │  │  │     ├─ gcp_regions
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ├─ revoke_oauth
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ╰─ service_accounts
    │     │     │  │  │        ╰─ /
    │     │     │  │  ╰─ r
    │     │     │  │     ├─ aphs/
    │     │     │  │     │  ╰─ <id>
    │     │     │  │     │     ╰─ /
    │     │     │  │     │        ├─ c
    │     │     │  │     │        │  ├─ harts
    │     │     │  │     │        │  │  ╰─ /
    │     │     │  │     │        │  ├─ i
    │     │     │  │     │        │  │  ╰─ /
    │     │     │  │     │        │  ╰─ ommits
    │     │     │  │     │        │     ╰─ /
    │     │     │  │     │        ╰─ languages
    │     │     │  │     │           ╰─ /
    │     │     │  │     ╰─ oup_links/
    │     │     │  │        ╰─ <id>
    │     │     │  │           ╰─ /
    │     │     │  ├─ h
    │     │     │  │  ├─ arbor/repositories
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ <repository_id>
    │     │     │  │  │     │  ╰─ /artifacts
    │     │     │  │  │     │     ╰─ /
    │     │     │  │  │     │        ╰─ <artifact_id>
    │     │     │  │  │     │           ╰─ /tags
    │     │     │  │  │     │              ╰─ /
    │     │     │  │  │     ╰─ <id>
    │     │     │  │  │        ╰─ /
    │     │     │  │  ╰─ ooks
    │     │     │  │     ╰─ /
    │     │     │  │        ├─ <hook_id>
    │     │     │  │        │  ╰─ /hook_logs/
    │     │     │  │        │     ╰─ <id>
    │     │     │  │        │        ╰─ /
    │     │     │  │        │           ╰─ retry
    │     │     │  │        │              ╰─ /
    │     │     │  │        ╰─ <id>
    │     │     │  │           ╰─ /
    │     │     │  │              ├─ edit
    │     │     │  │              │  ╰─ /
    │     │     │  │              ╰─ test
    │     │     │  │                 ╰─ /
    │     │     │  ├─ i
    │     │     │  │  ├─ mport
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ jira
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ╰─ new
    │     │     │  │  │        ╰─ /
    │     │     │  │  ├─ n
    │     │     │  │  │  ├─ cident
    │     │     │  │  │  │  ├─ _management/timeline_events/preview_markdown
    │     │     │  │  │  │  │  ╰─ /
    │     │     │  │  │  │  ╰─ s
    │     │     │  │  │  │     ╰─ /
    │     │     │  │  │  │        ╰─ integrations/pagerduty
    │     │     │  │  │  │           ╰─ /
    │     │     │  │  │  ├─ frastructure_registry
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  ╰─ tegrations/
    │     │     │  │  │     ├─ jira/issues
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     │     ╰─ <id>
    │     │     │  │  │     │        ╰─ /
    │     │     │  │  │     ├─ slash_commands
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     │     ╰─ confirm
    │     │     │  │  │     │        ╰─ /
    │     │     │  │  │     ╰─ zentao/issues
    │     │     │  │  │        ╰─ /
    │     │     │  │  │           ╰─ <id>
    │     │     │  │  │              ╰─ /
    │     │     │  │  ├─ ssues
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ bulk_update
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ├─ export_csv
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ├─ i
    │     │     │  │  │     │  ├─ mport_csv
    │     │     │  │  │     │  │  ╰─ /
    │     │     │  │  │     │  ╰─ ncident/
    │     │     │  │  │     │     ╰─ <id>
    │     │     │  │  │     │        ╰─ /
    │     │     │  │  │     │           ╰─ <incident_tab>
    │     │     │  │  │     │              ╰─ /
    │     │     │  │  │     ├─ new
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ├─ service_desk
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ├─ <id>
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     │     ├─ c
    │     │     │  │  │     │     │  ├─ an_create_branch
    │     │     │  │  │     │     │  │  ╰─ /
    │     │     │  │  │     │     │  ╰─ reate_merge_request
    │     │     │  │  │     │     │     ╰─ /
    │     │     │  │  │     │     ├─ d
    │     │     │  │  │     │     │  ├─ es
    │     │     │  │  │     │     │  │  ├─ criptions/
    │     │     │  │  │     │     │  │  │  ╰─ <version_id>
    │     │     │  │  │     │     │  │  │     ╰─ /
    │     │     │  │  │     │     │  │  │        ╰─ diff
    │     │     │  │  │     │     │  │  │           ╰─ /
    │     │     │  │  │     │     │  │  ╰─ igns
    │     │     │  │  │     │     │  │     ╰─ /
    │     │     │  │  │     │     │  │        ├─ <*vueroute>
    │     │     │  │  │     │     │  │        │  ╰─ /
    │     │     │  │  │     │     │  │        ╰─ <*vueroute>
    │     │     │  │  │     │     │  ╰─ iscussions
    │     │     │  │  │     │     │     ╰─ /
    │     │     │  │  │     │     ├─ edit
    │     │     │  │  │     │     │  ╰─ /
    │     │     │  │  │     │     ├─ m
    │     │     │  │  │     │     │  ├─ ark_as_spam
    │     │     │  │  │     │     │  │  ╰─ /
    │     │     │  │  │     │     │  ╰─ ove
    │     │     │  │  │     │     │     ╰─ /
    │     │     │  │  │     │     ├─ re
    │     │     │  │  │     │     │  ├─ altime_changes
    │     │     │  │  │     │     │  │  ╰─ /
    │     │     │  │  │     │     │  ├─ lated_branches
    │     │     │  │  │     │     │  │  ╰─ /
    │     │     │  │  │     │     │  ╰─ order
    │     │     │  │  │     │     │     ╰─ /
    │     │     │  │  │     │     ├─ toggle_
    │     │     │  │  │     │     │  ├─ award_emoji
    │     │     │  │  │     │     │  │  ╰─ /
    │     │     │  │  │     │     │  ╰─ subscription
    │     │     │  │  │     │     │     ╰─ /
    │     │     │  │  │     │     ╰─ <incident_tab>
    │     │     │  │  │     │        ╰─ /
    │     │     │  │  │     ╰─ <issue_id>
    │     │     │  │  │        ╰─ /
    │     │     │  │  │           ├─ feature_flags
    │     │     │  │  │           │  ╰─ /
    │     │     │  │  │           │     ╰─ <id>
    │     │     │  │  │           │        ╰─ /
    │     │     │  │  │           ╰─ links
    │     │     │  │  │              ╰─ /
    │     │     │  │  │                 ╰─ <id>
    │     │     │  │  │                    ╰─ /
    │     │     │  │  ╰─ terations
    │     │     │  │     ╰─ /
    │     │     │  │        ╰─ <id>
    │     │     │  │           ╰─ /
    │     │     │  ├─ jobs
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ artifacts/
    │     │     │  │     │  ├─ <*ref_name_and_path>
    │     │     │  │     │  │  ╰─ /
    │     │     │  │     │  ╰─ <*ref_name_and_path>
    │     │     │  │     ├─ <job_id>
    │     │     │  │     │  ╰─ /artifacts/
    │     │     │  │     │     ├─ browse
    │     │     │  │     │     │  ╰─ /
    │     │     │  │     │     │     ├─ <*path>
    │     │     │  │     │     │     │  ╰─ /
    │     │     │  │     │     │     ╰─ <*path>
    │     │     │  │     │     ├─ download
    │     │     │  │     │     │  ╰─ /
    │     │     │  │     │     ├─ external_file/
    │     │     │  │     │     │  ├─ <*path>
    │     │     │  │     │     │  │  ╰─ /
    │     │     │  │     │     │  ╰─ <*path>
    │     │     │  │     │     ├─ file/
    │     │     │  │     │     │  ├─ <*path>
    │     │     │  │     │     │  │  ╰─ /
    │     │     │  │     │     │  ╰─ <*path>
    │     │     │  │     │     ├─ keep
    │     │     │  │     │     │  ╰─ /
    │     │     │  │     │     ╰─ raw/
    │     │     │  │     │        ├─ <*path>
    │     │     │  │     │        │  ╰─ /
    │     │     │  │     │        ╰─ <*path>
    │     │     │  │     ╰─ <id>
    │     │     │  │        ╰─ /
    │     │     │  │           ├─ cancel
    │     │     │  │           │  ╰─ /
    │     │     │  │           ├─ erase
    │     │     │  │           │  ╰─ /
    │     │     │  │           ├─ p
    │     │     │  │           │  ├─ lay
    │     │     │  │           │  │  ╰─ /
    │     │     │  │           │  ╰─ roxy
    │     │     │  │           │     ├─ .ws/authorize
    │     │     │  │           │     │  ╰─ /
    │     │     │  │           │     ╰─ /
    │     │     │  │           ├─ r
    │     │     │  │           │  ├─ aw
    │     │     │  │           │  │  ╰─ /
    │     │     │  │           │  ╰─ etry
    │     │     │  │           │     ╰─ /
    │     │     │  │           ├─ status
    │     │     │  │           │  ╰─ /
    │     │     │  │           ├─ t
    │     │     │  │           │  ├─ e
    │     │     │  │           │  │  ├─ rminal
    │     │     │  │           │  │  │  ├─ .ws/authorize
    │     │     │  │           │  │  │  │  ╰─ /
    │     │     │  │           │  │  │  ╰─ /
    │     │     │  │           │  │  ╰─ st_report_summary
    │     │     │  │           │  │     ╰─ /
    │     │     │  │           │  ╰─ race
    │     │     │  │           │     ├─ .
    │     │     │  │           │     │  ╰─ <format>
    │     │     │  │           │     │     ╰─ /
    │     │     │  │           │     ╰─ /
    │     │     │  │           ├─ unschedule
    │     │     │  │           │  ╰─ /
    │     │     │  │           ╰─ viewer
    │     │     │  │              ╰─ /
    │     │     │  ├─ l
    │     │     │  │  ├─ abels
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ generate
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ├─ new
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ├─ set_priorities
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ╰─ <id>
    │     │     │  │  │        ╰─ /
    │     │     │  │  │           ├─ edit
    │     │     │  │  │           │  ╰─ /
    │     │     │  │  │           ├─ promote
    │     │     │  │  │           │  ╰─ /
    │     │     │  │  │           ├─ remove_priority
    │     │     │  │  │           │  ╰─ /
    │     │     │  │  │           ╰─ toggle_subscription
    │     │     │  │  │              ╰─ /
    │     │     │  │  ├─ earn_gitlab
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ╰─ end_tutorial
    │     │     │  │  │        ╰─ /
    │     │     │  │  ╰─ ogs
    │     │     │  │     ╰─ /
    │     │     │  ├─ m
    │     │     │  │  ├─ attermost
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ╰─ new
    │     │     │  │  │        ╰─ /
    │     │     │  │  ├─ e
    │     │     │  │  │  ├─ rge
    │     │     │  │  │  │  ├─ _
    │     │     │  │  │  │  │  ├─ requests
    │     │     │  │  │  │  │  │  ╰─ /
    │     │     │  │  │  │  │  │     ├─ bulk_update
    │     │     │  │  │  │  │  │     │  ╰─ /
    │     │     │  │  │  │  │  │     ├─ diff_for_path
    │     │     │  │  │  │  │  │     │  ╰─ /
    │     │     │  │  │  │  │  │     ├─ export_csv
    │     │     │  │  │  │  │  │     │  ╰─ /
    │     │     │  │  │  │  │  │     ├─ new
    │     │     │  │  │  │  │  │     │  ╰─ /
    │     │     │  │  │  │  │  │     │     ├─ branch_
    │     │     │  │  │  │  │  │     │     │  ├─ from
    │     │     │  │  │  │  │  │     │     │  │  ╰─ /
    │     │     │  │  │  │  │  │     │     │  ╰─ to
    │     │     │  │  │  │  │  │     │     │     ╰─ /
    │     │     │  │  │  │  │  │     │     ├─ diff
    │     │     │  │  │  │  │  │     │     │  ├─ _for_path
    │     │     │  │  │  │  │  │     │     │  │  ╰─ /
    │     │     │  │  │  │  │  │     │     │  ╰─ s
    │     │     │  │  │  │  │  │     │     │     ╰─ /
    │     │     │  │  │  │  │  │     │     ├─ pipelines
    │     │     │  │  │  │  │  │     │     │  ╰─ /
    │     │     │  │  │  │  │  │     │     ╰─ target_projects
    │     │     │  │  │  │  │  │     │        ╰─ /
    │     │     │  │  │  │  │  │     ├─ <id>
    │     │     │  │  │  │  │  │     │  ╰─ /
    │     │     │  │  │  │  │  │     │     ├─ a
    │     │     │  │  │  │  │  │     │     │  ├─ ccessibility_reports
    │     │     │  │  │  │  │  │     │     │  │  ╰─ /
    │     │     │  │  │  │  │  │     │     │  ├─ pi_fuzzing_reports
    │     │     │  │  │  │  │  │     │     │  │  ╰─ /
    │     │     │  │  │  │  │  │     │     │  ╰─ ssign_related_issues
    │     │     │  │  │  │  │  │     │     │     ╰─ /
    │     │     │  │  │  │  │  │     │     ├─ c
    │     │     │  │  │  │  │  │     │     │  ├─ a
    │     │     │  │  │  │  │  │     │     │  │  ├─ ched_widget
    │     │     │  │  │  │  │  │     │     │  │  │  ╰─ /
    │     │     │  │  │  │  │  │     │     │  │  ╰─ ncel_auto_merge
    │     │     │  │  │  │  │  │     │     │  │     ╰─ /
    │     │     │  │  │  │  │  │     │     │  ├─ i_environments_status
    │     │     │  │  │  │  │  │     │     │  │  ╰─ /
    │     │     │  │  │  │  │  │     │     │  ╰─ o
    │     │     │  │  │  │  │  │     │     │     ├─ dequality_
    │     │     │  │  │  │  │  │     │     │     │  ├─ mr_diff_reports
    │     │     │  │  │  │  │  │     │     │     │  │  ╰─ /
    │     │     │  │  │  │  │  │     │     │     │  ╰─ reports
    │     │     │  │  │  │  │  │     │     │     │     ╰─ /
    │     │     │  │  │  │  │  │     │     │     ├─ mmit
    │     │     │  │  │  │  │  │     │     │     │  ├─ _change_content
    │     │     │  │  │  │  │  │     │     │     │  │  ╰─ /
    │     │     │  │  │  │  │  │     │     │     │  ╰─ s
    │     │     │  │  │  │  │  │     │     │     │     ╰─ /
    │     │     │  │  │  │  │  │     │     │     ├─ n
    │     │     │  │  │  │  │  │     │     │     │  ├─ flict
    │     │     │  │  │  │  │  │     │     │     │  │  ├─ _for_path
    │     │     │  │  │  │  │  │     │     │     │  │  │  ╰─ /
    │     │     │  │  │  │  │  │     │     │     │  │  ╰─ s
    │     │     │  │  │  │  │  │     │     │     │  │     ╰─ /
    │     │     │  │  │  │  │  │     │     │     │  ╰─ t
    │     │     │  │  │  │  │  │     │     │     │     ├─ ainer_scanning_reports
    │     │     │  │  │  │  │  │     │     │     │     │  ╰─ /
    │     │     │  │  │  │  │  │     │     │     │     ╰─ ext_commits
    │     │     │  │  │  │  │  │     │     │     │        ╰─ /
    │     │     │  │  │  │  │  │     │     │     ╰─ verage_
    │     │     │  │  │  │  │  │     │     │        ├─ fuzzing_reports
    │     │     │  │  │  │  │  │     │     │        │  ╰─ /
    │     │     │  │  │  │  │  │     │     │        ╰─ reports
    │     │     │  │  │  │  │  │     │     │           ╰─ /
    │     │     │  │  │  │  │  │     │     ├─ d
    │     │     │  │  │  │  │  │     │     │  ├─ ast_reports
    │     │     │  │  │  │  │  │     │     │  │  ╰─ /
    │     │     │  │  │  │  │  │     │     │  ├─ e
    │     │     │  │  │  │  │  │     │     │  │  ├─ pendency_scanning_reports
    │     │     │  │  │  │  │  │     │     │  │  │  ╰─ /
    │     │     │  │  │  │  │  │     │     │  │  ╰─ scriptions/
    │     │     │  │  │  │  │  │     │     │  │     ╰─ <version_id>
    │     │     │  │  │  │  │  │     │     │  │        ╰─ /
    │     │     │  │  │  │  │  │     │     │  │           ╰─ diff
    │     │     │  │  │  │  │  │     │     │  │              ╰─ /
    │     │     │  │  │  │  │  │     │     │  ╰─ i
    │     │     │  │  │  │  │  │     │     │     ├─ ff
    │     │     │  │  │  │  │  │     │     │     │  ├─ _
    │     │     │  │  │  │  │  │     │     │     │  │  ├─ by_file_hash/
    │     │     │  │  │  │  │  │     │     │     │  │  │  ╰─ <file_hash>
    │     │     │  │  │  │  │  │     │     │     │  │  │     ╰─ /
    │     │     │  │  │  │  │  │     │     │     │  │  ╰─ for_path
    │     │     │  │  │  │  │  │     │     │     │  │     ╰─ /
    │     │     │  │  │  │  │  │     │     │     │  ╰─ s
    │     │     │  │  │  │  │  │     │     │     │     ├─ /
    │     │     │  │  │  │  │  │     │     │     │     ╰─ _
    │     │     │  │  │  │  │  │     │     │     │        ├─ batch
    │     │     │  │  │  │  │  │     │     │     │        │  ╰─ /
    │     │     │  │  │  │  │  │     │     │     │        ├─ metadata
    │     │     │  │  │  │  │  │     │     │     │        │  ╰─ /
    │     │     │  │  │  │  │  │     │     │     │        ╰─ stream
    │     │     │  │  │  │  │  │     │     │     │           ╰─ /
    │     │     │  │  │  │  │  │     │     │     ╰─ scussions
    │     │     │  │  │  │  │  │     │     │        ╰─ /
    │     │     │  │  │  │  │  │     │     ├─ e
    │     │     │  │  │  │  │  │     │     │  ├─ dit
    │     │     │  │  │  │  │  │     │     │  │  ╰─ /
    │     │     │  │  │  │  │  │     │     │  ╰─ xposed_artifacts
    │     │     │  │  │  │  │  │     │     │     ╰─ /
    │     │     │  │  │  │  │  │     │     ├─ license_scanning_reports
    │     │     │  │  │  │  │  │     │     │  ├─ /
    │     │     │  │  │  │  │  │     │     │  ╰─ _collapsed
    │     │     │  │  │  │  │  │     │     │     ╰─ /
    │     │     │  │  │  │  │  │     │     ├─ me
    │     │     │  │  │  │  │  │     │     │  ├─ rge
    │     │     │  │  │  │  │  │     │     │  │  ╰─ /
    │     │     │  │  │  │  │  │     │     │  ╰─ trics_reports
    │     │     │  │  │  │  │  │     │     │     ╰─ /
    │     │     │  │  │  │  │  │     │     ├─ pipeline
    │     │     │  │  │  │  │  │     │     │  ├─ _status
    │     │     │  │  │  │  │  │     │     │  │  ╰─ /
    │     │     │  │  │  │  │  │     │     │  ╰─ s
    │     │     │  │  │  │  │  │     │     │     ╰─ /
    │     │     │  │  │  │  │  │     │     ├─ re
    │     │     │  │  │  │  │  │     │     │  ├─ base
    │     │     │  │  │  │  │  │     │     │  │  ╰─ /
    │     │     │  │  │  │  │  │     │     │  ├─ move_wip
    │     │     │  │  │  │  │  │     │     │  │  ╰─ /
    │     │     │  │  │  │  │  │     │     │  ├─ ports
    │     │     │  │  │  │  │  │     │     │  │  ╰─ /
    │     │     │  │  │  │  │  │     │     │  ╰─ solve_conflicts
    │     │     │  │  │  │  │  │     │     │     ╰─ /
    │     │     │  │  │  │  │  │     │     ├─ s
    │     │     │  │  │  │  │  │     │     │  ├─ a
    │     │     │  │  │  │  │  │     │     │  │  ├─ ml_approval
    │     │     │  │  │  │  │  │     │     │  │  │  ╰─ /
    │     │     │  │  │  │  │  │     │     │  │  ╰─ st_reports
    │     │     │  │  │  │  │  │     │     │  │     ╰─ /
    │     │     │  │  │  │  │  │     │     │  ╰─ ec
    │     │     │  │  │  │  │  │     │     │     ├─ ret_detection_reports
    │     │     │  │  │  │  │  │     │     │     │  ╰─ /
    │     │     │  │  │  │  │  │     │     │     ╰─ urity_reports
    │     │     │  │  │  │  │  │     │     │        ╰─ /
    │     │     │  │  │  │  │  │     │     ├─ t
    │     │     │  │  │  │  │  │     │     │  ├─ e
    │     │     │  │  │  │  │  │     │     │  │  ├─ rraform_reports
    │     │     │  │  │  │  │  │     │     │  │  │  ╰─ /
    │     │     │  │  │  │  │  │     │     │  │  ╰─ st_reports
    │     │     │  │  │  │  │  │     │     │  │     ╰─ /
    │     │     │  │  │  │  │  │     │     │  ╰─ oggle_
    │     │     │  │  │  │  │  │     │     │     ├─ award_emoji
    │     │     │  │  │  │  │  │     │     │     │  ╰─ /
    │     │     │  │  │  │  │  │     │     │     ╰─ subscription
    │     │     │  │  │  │  │  │     │     │        ╰─ /
    │     │     │  │  │  │  │  │     │     ╰─ widget
    │     │     │  │  │  │  │  │     │        ╰─ /
    │     │     │  │  │  │  │  │     ╰─ <merge_request_id>
    │     │     │  │  │  │  │  │        ╰─ /
    │     │     │  │  │  │  │  │           ├─ approver
    │     │     │  │  │  │  │  │           │  ├─ _groups/
    │     │     │  │  │  │  │  │           │  │  ╰─ <id>
    │     │     │  │  │  │  │  │           │  │     ╰─ /
    │     │     │  │  │  │  │  │           │  ╰─ s
    │     │     │  │  │  │  │  │           │     ╰─ /
    │     │     │  │  │  │  │  │           │        ╰─ <id>
    │     │     │  │  │  │  │  │           │           ╰─ /
    │     │     │  │  │  │  │  │           ╰─ drafts
    │     │     │  │  │  │  │  │              ╰─ /
    │     │     │  │  │  │  │  │                 ├─ discard
    │     │     │  │  │  │  │  │                 │  ╰─ /
    │     │     │  │  │  │  │  │                 ├─ publish
    │     │     │  │  │  │  │  │                 │  ╰─ /
    │     │     │  │  │  │  │  │                 ╰─ <id>
    │     │     │  │  │  │  │  │                    ╰─ /
    │     │     │  │  │  │  │  ╰─ trains
    │     │     │  │  │  │  │     ╰─ /
    │     │     │  │  │  │  ╰─ d_branches
    │     │     │  │  │  │     ╰─ /
    │     │     │  │  │  ╰─ trics
    │     │     │  │  │     ╰─ /
    │     │     │  │  │        ╰─ <id>
    │     │     │  │  │           ╰─ /
    │     │     │  │  ├─ i
    │     │     │  │  │  ├─ lestones
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  │     ├─ new
    │     │     │  │  │  │     │  ╰─ /
    │     │     │  │  │  │     ╰─ <id>
    │     │     │  │  │  │        ╰─ /
    │     │     │  │  │  │           ├─ edit
    │     │     │  │  │  │           │  ╰─ /
    │     │     │  │  │  │           ├─ issues
    │     │     │  │  │  │           │  ╰─ /
    │     │     │  │  │  │           ├─ labels
    │     │     │  │  │  │           │  ╰─ /
    │     │     │  │  │  │           ├─ merge_requests
    │     │     │  │  │  │           │  ╰─ /
    │     │     │  │  │  │           ╰─ p
    │     │     │  │  │  │              ├─ articipants
    │     │     │  │  │  │              │  ╰─ /
    │     │     │  │  │  │              ╰─ romote
    │     │     │  │  │  │                 ╰─ /
    │     │     │  │  │  ╰─ rror
    │     │     │  │  │     ╰─ /
    │     │     │  │  │        ├─ ssh_host_keys
    │     │     │  │  │        │  ├─ .
    │     │     │  │  │        │  │  ╰─ <format>
    │     │     │  │  │        │  │     ╰─ /
    │     │     │  │  │        │  ╰─ /
    │     │     │  │  │        ╰─ update_now
    │     │     │  │  │           ╰─ /
    │     │     │  │  ╰─ l/
    │     │     │  │     ├─ agents
    │     │     │  │     │  ╰─ /
    │     │     │  │     │     ├─ new
    │     │     │  │     │     │  ╰─ /
    │     │     │  │     │     ├─ <id>
    │     │     │  │     │     │  ╰─ /
    │     │     │  │     │     │     ╰─ edit
    │     │     │  │     │     │        ╰─ /
    │     │     │  │     │     ├─ <*vueroute>
    │     │     │  │     │     │  ╰─ /
    │     │     │  │     │     │     ├─ new
    │     │     │  │     │     │     │  ╰─ /
    │     │     │  │     │     │     ╰─ <id>
    │     │     │  │     │     │        ╰─ /
    │     │     │  │     │     │           ╰─ edit
    │     │     │  │     │     │              ╰─ /
    │     │     │  │     │     ╰─ <*vueroute>
    │     │     │  │     ├─ candidates/
    │     │     │  │     │  ╰─ <iid>
    │     │     │  │     │     ╰─ /
    │     │     │  │     ├─ experiments
    │     │     │  │     │  ╰─ /
    │     │     │  │     │     ╰─ <iid>
    │     │     │  │     │        ╰─ /
    │     │     │  │     ├─ models
    │     │     │  │     │  ╰─ /
    │     │     │  │     │     ├─ new
    │     │     │  │     │     │  ╰─ /
    │     │     │  │     │     ├─ <model_model_id>
    │     │     │  │     │     │  ╰─ /versions/
    │     │     │  │     │     │     ╰─ <model_version_id>
    │     │     │  │     │     │        ╰─ /
    │     │     │  │     │     ╰─ <model_id>
    │     │     │  │     │        ╰─ /
    │     │     │  │     ╰─ preview_markdown
    │     │     │  │        ╰─ /
    │     │     │  ├─ ne
    │     │     │  │  ├─ twork/
    │     │     │  │  │  ╰─ <id>
    │     │     │  │  │     ╰─ /
    │     │     │  │  ╰─ w/
    │     │     │  │     ├─ <*id>
    │     │     │  │     │  ╰─ /
    │     │     │  │     ╰─ <*id>
    │     │     │  ├─ on
    │     │     │  │  ├─ _demand_scans
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ new
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ╰─ <id>
    │     │     │  │  │        ╰─ /edit
    │     │     │  │  │           ╰─ /
    │     │     │  │  ╰─ call_schedules
    │     │     │  │     ╰─ /
    │     │     │  ├─ p
    │     │     │  │  ├─ ackage
    │     │     │  │  │  ├─ _files/
    │     │     │  │  │  │  ╰─ <id>
    │     │     │  │  │  │     ╰─ /download
    │     │     │  │  │  │        ╰─ /
    │     │     │  │  │  ╰─ s
    │     │     │  │  │     ╰─ /
    │     │     │  │  │        ╰─ <id>
    │     │     │  │  │           ╰─ /
    │     │     │  │  ├─ ipeline
    │     │     │  │  │  ├─ _schedules
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  │     ├─ new
    │     │     │  │  │  │     │  ╰─ /
    │     │     │  │  │  │     ╰─ <id>
    │     │     │  │  │  │        ╰─ /
    │     │     │  │  │  │           ├─ edit
    │     │     │  │  │  │           │  ╰─ /
    │     │     │  │  │  │           ├─ play
    │     │     │  │  │  │           │  ╰─ /
    │     │     │  │  │  │           ╰─ take_ownership
    │     │     │  │  │  │              ╰─ /
    │     │     │  │  │  ╰─ s
    │     │     │  │  │     ╰─ /
    │     │     │  │  │        ├─ charts
    │     │     │  │  │        │  ╰─ /
    │     │     │  │  │        ├─ latest
    │     │     │  │  │        │  ╰─ /
    │     │     │  │  │        ├─ new
    │     │     │  │  │        │  ╰─ /
    │     │     │  │  │        ├─ settings
    │     │     │  │  │        │  ╰─ /
    │     │     │  │  │        ├─ <id>
    │     │     │  │  │        │  ╰─ /
    │     │     │  │  │        │     ├─ builds
    │     │     │  │  │        │     │  ╰─ /
    │     │     │  │  │        │     ├─ c
    │     │     │  │  │        │     │  ├─ ancel
    │     │     │  │  │        │     │  │  ╰─ /
    │     │     │  │  │        │     │  ╰─ odequality_report
    │     │     │  │  │        │     │     ╰─ /
    │     │     │  │  │        │     ├─ d
    │     │     │  │  │        │     │  ├─ ag
    │     │     │  │  │        │     │  │  ╰─ /
    │     │     │  │  │        │     │  ╰─ ownloadable_artifacts
    │     │     │  │  │        │     │     ╰─ /
    │     │     │  │  │        │     ├─ failures
    │     │     │  │  │        │     │  ╰─ /
    │     │     │  │  │        │     ├─ license
    │     │     │  │  │        │     │  ├─ _count
    │     │     │  │  │        │     │  │  ╰─ /
    │     │     │  │  │        │     │  ╰─ s
    │     │     │  │  │        │     │     ╰─ /
    │     │     │  │  │        │     ├─ manual_variables
    │     │     │  │  │        │     │  ╰─ /
    │     │     │  │  │        │     ├─ retry
    │     │     │  │  │        │     │  ╰─ /
    │     │     │  │  │        │     ├─ s
    │     │     │  │  │        │     │  ├─ ecurity
    │     │     │  │  │        │     │  │  ╰─ /
    │     │     │  │  │        │     │  ╰─ ta
    │     │     │  │  │        │     │     ├─ ge
    │     │     │  │  │        │     │     │  ╰─ /
    │     │     │  │  │        │     │     ╰─ tus
    │     │     │  │  │        │     │        ╰─ /
    │     │     │  │  │        │     ╰─ test_report
    │     │     │  │  │        │        ╰─ /
    │     │     │  │  │        ├─ <pipeline_id>
    │     │     │  │  │        │  ╰─ /
    │     │     │  │  │        │     ├─ stages/
    │     │     │  │  │        │     │  ╰─ <stage_name>
    │     │     │  │  │        │     │     ╰─ /play_manual
    │     │     │  │  │        │     │        ╰─ /
    │     │     │  │  │        │     ├─ tests/
    │     │     │  │  │        │     │  ├─ summary
    │     │     │  │  │        │     │  │  ╰─ /
    │     │     │  │  │        │     │  ╰─ <suite_name>
    │     │     │  │  │        │     │     ╰─ /
    │     │     │  │  │        │     ╰─ validate_account
    │     │     │  │  │        │        ╰─ /
    │     │     │  │  │        ╰─ <*ref>
    │     │     │  │  │           ╰─ /latest
    │     │     │  │  │              ╰─ /
    │     │     │  │  ├─ r
    │     │     │  │  │  ├─ eview
    │     │     │  │  │  │  ├─ /
    │     │     │  │  │  │  │  ├─ <*id>
    │     │     │  │  │  │  │  │  ╰─ /
    │     │     │  │  │  │  │  ╰─ <*id>
    │     │     │  │  │  │  ╰─ _markdown
    │     │     │  │  │  │     ╰─ /
    │     │     │  │  │  ╰─ o
    │     │     │  │  │     ├─ ject_members
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     │     ├─ leave
    │     │     │  │  │     │     │  ╰─ /
    │     │     │  │  │     │     ├─ request_access
    │     │     │  │  │     │     │  ╰─ /
    │     │     │  │  │     │     ╰─ <id>
    │     │     │  │  │     │        ╰─ /
    │     │     │  │  │     │           ├─ approve_access_request
    │     │     │  │  │     │           │  ╰─ /
    │     │     │  │  │     │           ╰─ resend_invite
    │     │     │  │  │     │              ╰─ /
    │     │     │  │  │     ╰─ tected_
    │     │     │  │  │        ├─ branches
    │     │     │  │  │        │  ╰─ /
    │     │     │  │  │        │     ╰─ <id>
    │     │     │  │  │        │        ╰─ /
    │     │     │  │  │        ├─ environments
    │     │     │  │  │        │  ╰─ /
    │     │     │  │  │        │     ├─ search
    │     │     │  │  │        │     │  ╰─ /
    │     │     │  │  │        │     ╰─ <id>
    │     │     │  │  │        │        ╰─ /
    │     │     │  │  │        ╰─ tags
    │     │     │  │  │           ╰─ /
    │     │     │  │  │              ╰─ <id>
    │     │     │  │  │                 ╰─ /
    │     │     │  │  ╰─ ush_rules/
    │     │     │  │     ╰─ <id>
    │     │     │  │        ╰─ /
    │     │     │  ├─ quality/test_cases
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ new
    │     │     │  │     │  ╰─ /
    │     │     │  │     ╰─ <id>
    │     │     │  │        ╰─ /
    │     │     │  ├─ r
    │     │     │  │  ├─ aw/
    │     │     │  │  │  ├─ <*id>
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  ╰─ <*id>
    │     │     │  │  ├─ e
    │     │     │  │  │  ├─ fs/
    │     │     │  │  │  │  ├─ switch
    │     │     │  │  │  │  │  ╰─ /
    │     │     │  │  │  │  ╰─ <id>
    │     │     │  │  │  │     ╰─ /logs_tree
    │     │     │  │  │  │        ╰─ /
    │     │     │  │  │  │           ├─ <*path>
    │     │     │  │  │  │           │  ╰─ /
    │     │     │  │  │  │           ╰─ <*path>
    │     │     │  │  │  ├─ leases
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  │     ├─ inbox
    │     │     │  │  │  │     │  ╰─ /
    │     │     │  │  │  │     ├─ new
    │     │     │  │  │  │     │  ╰─ /
    │     │     │  │  │  │     ├─ outbox
    │     │     │  │  │  │     │  ╰─ /
    │     │     │  │  │  │     ├─ permalink/latest
    │     │     │  │  │  │     │  ╰─ /
    │     │     │  │  │  │     │     ├─ <*suffix_path>
    │     │     │  │  │  │     │     │  ╰─ /
    │     │     │  │  │  │     │     ╰─ <*suffix_path>
    │     │     │  │  │  │     ╰─ <tag>
    │     │     │  │  │  │        ╰─ /
    │     │     │  │  │  │           ├─ downloads/
    │     │     │  │  │  │           │  ├─ <*filepath>
    │     │     │  │  │  │           │  │  ╰─ /
    │     │     │  │  │  │           │  ╰─ <*filepath>
    │     │     │  │  │  │           ╰─ e
    │     │     │  │  │  │              ├─ dit
    │     │     │  │  │  │              │  ╰─ /
    │     │     │  │  │  │              ╰─ vidences/
    │     │     │  │  │  │                 ╰─ <id>
    │     │     │  │  │  │                    ╰─ /
    │     │     │  │  │  ├─ pository
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  ╰─ quirements_management/requirements
    │     │     │  │  │     ╰─ /
    │     │     │  │  │        ╰─ import_csv
    │     │     │  │  │           ╰─ /
    │     │     │  │  │              ╰─ authorize
    │     │     │  │  │                 ╰─ /
    │     │     │  │  ╰─ unners
    │     │     │  │     ╰─ /
    │     │     │  │        ├─ new
    │     │     │  │        │  ╰─ /
    │     │     │  │        ├─ toggle_
    │     │     │  │        │  ├─ group_runners
    │     │     │  │        │  │  ╰─ /
    │     │     │  │        │  ╰─ shared_runners
    │     │     │  │        │     ╰─ /
    │     │     │  │        ╰─ <id>
    │     │     │  │           ╰─ /
    │     │     │  │              ├─ edit
    │     │     │  │              │  ╰─ /
    │     │     │  │              ├─ pause
    │     │     │  │              │  ╰─ /
    │     │     │  │              ╰─ re
    │     │     │  │                 ├─ gister
    │     │     │  │                 │  ╰─ /
    │     │     │  │                 ╰─ sume
    │     │     │  │                    ╰─ /
    │     │     │  ├─ s
    │     │     │  │  ├─ chema/
    │     │     │  │  │  ╰─ <branch>
    │     │     │  │  │     ╰─ /
    │     │     │  │  │        ├─ <*filename>
    │     │     │  │  │        │  ╰─ /
    │     │     │  │  │        ╰─ <*filename>
    │     │     │  │  ├─ e
    │     │     │  │  │  ├─ c
    │     │     │  │  │  │  ├─ rets
    │     │     │  │  │  │  │  ╰─ /
    │     │     │  │  │  │  │     ├─ <*vueroute>
    │     │     │  │  │  │  │     │  ╰─ /
    │     │     │  │  │  │  │     ╰─ <*vueroute>
    │     │     │  │  │  │  ╰─ urity/
    │     │     │  │  │  │     ├─ configuration
    │     │     │  │  │  │     │  ╰─ /
    │     │     │  │  │  │     │     ├─ api_fuzzing
    │     │     │  │  │  │     │     │  ╰─ /
    │     │     │  │  │  │     │     ├─ corpus_management
    │     │     │  │  │  │     │     │  ╰─ /
    │     │     │  │  │  │     │     ├─ dast
    │     │     │  │  │  │     │     │  ╰─ /
    │     │     │  │  │  │     │     ├─ profile_library
    │     │     │  │  │  │     │     │  ╰─ /
    │     │     │  │  │  │     │     │     ╰─ dast_s
    │     │     │  │  │  │     │     │        ├─ canner_profiles/
    │     │     │  │  │  │     │     │        │  ├─ new
    │     │     │  │  │  │     │     │        │  │  ╰─ /
    │     │     │  │  │  │     │     │        │  ╰─ <id>
    │     │     │  │  │  │     │     │        │     ╰─ /edit
    │     │     │  │  │  │     │     │        │        ╰─ /
    │     │     │  │  │  │     │     │        ╰─ ite_profiles/
    │     │     │  │  │  │     │     │           ├─ new
    │     │     │  │  │  │     │     │           │  ╰─ /
    │     │     │  │  │  │     │     │           ╰─ <id>
    │     │     │  │  │  │     │     │              ╰─ /edit
    │     │     │  │  │  │     │     │                 ╰─ /
    │     │     │  │  │  │     │     ╰─ s
    │     │     │  │  │  │     │        ├─ ast
    │     │     │  │  │  │     │        │  ╰─ /
    │     │     │  │  │  │     │        ╰─ ecret_detection
    │     │     │  │  │  │     │           ╰─ /
    │     │     │  │  │  │     ├─ d
    │     │     │  │  │  │     │  ├─ ashboard
    │     │     │  │  │  │     │  │  ╰─ /
    │     │     │  │  │  │     │  ╰─ iscover
    │     │     │  │  │  │     │     ╰─ /
    │     │     │  │  │  │     ├─ policies
    │     │     │  │  │  │     │  ╰─ /
    │     │     │  │  │  │     │     ├─ new
    │     │     │  │  │  │     │     │  ╰─ /
    │     │     │  │  │  │     │     ├─ schema
    │     │     │  │  │  │     │     │  ╰─ /
    │     │     │  │  │  │     │     ╰─ <id>
    │     │     │  │  │  │     │        ╰─ /edit
    │     │     │  │  │  │     │           ╰─ /
    │     │     │  │  │  │     ├─ scanned_resources
    │     │     │  │  │  │     │  ╰─ /
    │     │     │  │  │  │     ╰─ vulnerabilit
    │     │     │  │  │  │        ├─ ies/
    │     │     │  │  │  │        │  ├─ new
    │     │     │  │  │  │        │  │  ╰─ /
    │     │     │  │  │  │        │  ├─ <id>
    │     │     │  │  │  │        │  │  ╰─ /
    │     │     │  │  │  │        │  │     ╰─ discussions
    │     │     │  │  │  │        │  │        ╰─ /
    │     │     │  │  │  │        │  ╰─ <vulnerability_id>
    │     │     │  │  │  │        │     ╰─ /notes
    │     │     │  │  │  │        │        ╰─ /
    │     │     │  │  │  │        │           ╰─ <id>
    │     │     │  │  │  │        │              ╰─ /
    │     │     │  │  │  │        │                 ╰─ toggle_award_emoji
    │     │     │  │  │  │        │                    ╰─ /
    │     │     │  │  │  │        ╰─ y_report
    │     │     │  │  │  │           ╰─ /
    │     │     │  │  │  ├─ rvice_desk/custom_email
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  ╰─ ttings/
    │     │     │  │  │     ├─ a
    │     │     │  │  │     │  ├─ ccess_tokens
    │     │     │  │  │     │  │  ╰─ /
    │     │     │  │  │     │  │     ╰─ <id>
    │     │     │  │  │     │  │        ╰─ /revoke
    │     │     │  │  │     │  │           ╰─ /
    │     │     │  │  │     │  ╰─ nalytics
    │     │     │  │  │     │     ╰─ /
    │     │     │  │  │     ├─ ci_cd
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     │     ├─ deploy_token/create
    │     │     │  │  │     │     │  ╰─ /
    │     │     │  │  │     │     ╰─ r
    │     │     │  │  │     │        ├─ eset_
    │     │     │  │  │     │        │  ├─ cache
    │     │     │  │  │     │        │  │  ╰─ /
    │     │     │  │  │     │        │  ╰─ registration_token
    │     │     │  │  │     │        │     ╰─ /
    │     │     │  │  │     │        ╰─ unner_setup_scripts
    │     │     │  │  │     │           ╰─ /
    │     │     │  │  │     ├─ integrations
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     │     ├─ <integration_id>
    │     │     │  │  │     │     │  ╰─ /hook_logs/
    │     │     │  │  │     │     │     ╰─ <id>
    │     │     │  │  │     │     │        ╰─ /
    │     │     │  │  │     │     │           ╰─ retry
    │     │     │  │  │     │     │              ╰─ /
    │     │     │  │  │     │     ╰─ <id>
    │     │     │  │  │     │        ╰─ /
    │     │     │  │  │     │           ├─ edit
    │     │     │  │  │     │           │  ╰─ /
    │     │     │  │  │     │           ╰─ test
    │     │     │  │  │     │              ╰─ /
    │     │     │  │  │     ├─ merge_requests
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ├─ operations
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     │     ╰─ reset_
    │     │     │  │  │     │        ├─ alerting_token
    │     │     │  │  │     │        │  ╰─ /
    │     │     │  │  │     │        ╰─ pagerduty_token
    │     │     │  │  │     │           ╰─ /
    │     │     │  │  │     ├─ packages_and_registries
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     │     ╰─ cleanup_image_tags
    │     │     │  │  │     │        ╰─ /
    │     │     │  │  │     ├─ repository
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     │     ├─ branch_rules
    │     │     │  │  │     │     │  ╰─ /
    │     │     │  │  │     │     ├─ cleanup
    │     │     │  │  │     │     │  ╰─ /
    │     │     │  │  │     │     ╰─ deploy_token/create
    │     │     │  │  │     │        ╰─ /
    │     │     │  │  │     ╰─ slack
    │     │     │  │  │        ╰─ /
    │     │     │  │  │           ├─ edit
    │     │     │  │  │           │  ╰─ /
    │     │     │  │  │           ╰─ slack_auth
    │     │     │  │  │              ╰─ /
    │     │     │  │  ├─ nippets
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ new
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ├─ <id>
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     │     ├─ edit
    │     │     │  │  │     │     │  ╰─ /
    │     │     │  │  │     │     ├─ mark_as_spam
    │     │     │  │  │     │     │  ╰─ /
    │     │     │  │  │     │     ├─ raw
    │     │     │  │  │     │     │  ╰─ /
    │     │     │  │  │     │     ╰─ toggle_award_emoji
    │     │     │  │  │     │        ╰─ /
    │     │     │  │  │     ╰─ <snippet_id>
    │     │     │  │  │        ╰─ /raw/
    │     │     │  │  │           ╰─ <ref>
    │     │     │  │  │              ╰─ /
    │     │     │  │  │                 ├─ <*path>
    │     │     │  │  │                 │  ╰─ /
    │     │     │  │  │                 ╰─ <*path>
    │     │     │  │  ├─ tarrers
    │     │     │  │  │  ╰─ /
    │     │     │  │  ╰─ ubscriptions
    │     │     │  │     ╰─ /
    │     │     │  │        ╰─ <id>
    │     │     │  │           ╰─ /
    │     │     │  ├─ t
    │     │     │  │  ├─ a
    │     │     │  │  │  ├─ gs
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  │     ├─ new
    │     │     │  │  │  │     │  ╰─ /
    │     │     │  │  │  │     ╰─ <id>
    │     │     │  │  │  │        ╰─ /
    │     │     │  │  │  ╰─ rget_branch_rules
    │     │     │  │  │     ╰─ /
    │     │     │  │  │        ╰─ <id>
    │     │     │  │  │           ╰─ /
    │     │     │  │  ├─ erraform
    │     │     │  │  │  ├─ /
    │     │     │  │  │  ╰─ _module_registry
    │     │     │  │  │     ╰─ /
    │     │     │  │  │        ╰─ <id>
    │     │     │  │  │           ╰─ /
    │     │     │  │  ╰─ r
    │     │     │  │     ├─ acing
    │     │     │  │     │  ╰─ /
    │     │     │  │     │     ╰─ <id>
    │     │     │  │     │        ╰─ /
    │     │     │  │     ├─ ee/
    │     │     │  │     │  ├─ <*id>
    │     │     │  │     │  │  ╰─ /
    │     │     │  │     │  ╰─ <*id>
    │     │     │  │     ╰─ iggers
    │     │     │  │        ╰─ /
    │     │     │  │           ╰─ <id>
    │     │     │  │              ╰─ /
    │     │     │  ├─ u
    │     │     │  │  ├─ pdate/
    │     │     │  │  │  ├─ <*id>
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  ╰─ <*id>
    │     │     │  │  ╰─ sage_quotas
    │     │     │  │     ╰─ /
    │     │     │  ├─ v
    │     │     │  │  ├─ a
    │     │     │  │  │  ├─ lue_stream_analytics
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  │     ╰─ events/
    │     │     │  │  │  │        ├─ code
    │     │     │  │  │  │        │  ╰─ /
    │     │     │  │  │  │        ├─ issue
    │     │     │  │  │  │        │  ╰─ /
    │     │     │  │  │  │        ├─ p
    │     │     │  │  │  │        │  ├─ lan
    │     │     │  │  │  │        │  │  ╰─ /
    │     │     │  │  │  │        │  ╰─ roduction
    │     │     │  │  │  │        │     ╰─ /
    │     │     │  │  │  │        ├─ review
    │     │     │  │  │  │        │  ╰─ /
    │     │     │  │  │  │        ├─ staging
    │     │     │  │  │  │        │  ╰─ /
    │     │     │  │  │  │        ╰─ test
    │     │     │  │  │  │           ╰─ /
    │     │     │  │  │  ╰─ riables
    │     │     │  │  │     ╰─ /
    │     │     │  │  ╰─ ulnerability_feedback
    │     │     │  │     ╰─ /
    │     │     │  │        ├─ count
    │     │     │  │        │  ╰─ /
    │     │     │  │        ╰─ <id>
    │     │     │  │           ╰─ /
    │     │     │  ├─ w
    │     │     │  │  ├─ ikis
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ -/confluence
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ├─ git_access
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ├─ new
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ├─ pages
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ├─ templates
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ├─ <*id>
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     │     ├─ diff
    │     │     │  │  │     │     │  ╰─ /
    │     │     │  │  │     │     ├─ edit
    │     │     │  │  │     │     │  ╰─ /
    │     │     │  │  │     │     ├─ history
    │     │     │  │  │     │     │  ╰─ /
    │     │     │  │  │     │     ├─ preview_markdown
    │     │     │  │  │     │     │  ╰─ /
    │     │     │  │  │     │     ╰─ raw
    │     │     │  │  │     │        ╰─ /
    │     │     │  │  │     ╰─ <*id>
    │     │     │  │  ╰─ ork_items/
    │     │     │  │     ├─ import_csv
    │     │     │  │     │  ╰─ /
    │     │     │  │     │     ╰─ authorize
    │     │     │  │     │        ╰─ /
    │     │     │  │     ╰─ <iid>
    │     │     │  │        ╰─ /
    │     │     │  │           ╰─ designs
    │     │     │  │              ╰─ /
    │     │     │  │                 ├─ <*vueroute>
    │     │     │  │                 │  ╰─ /
    │     │     │  │                 ╰─ <*vueroute>
    │     │     │  ╰─ <noteable_type>
    │     │     │     ╰─ /
    │     │     │        ╰─ <noteable_id>
    │     │     │           ╰─ /discussions/
    │     │     │              ╰─ <id>
    │     │     │                 ╰─ /
    │     │     │                    ╰─ resolve
    │     │     │                       ╰─ /
    │     │     ├─ a
    │     │     │  ├─ lert
    │     │     │  │  ├─ _management
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ <*rest>
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ╰─ <*rest>
    │     │     │  │  ╰─ s/notify
    │     │     │  │     ╰─ /
    │     │     │  │        ╰─ <name>
    │     │     │  │           ╰─ /
    │     │     │  │              ╰─ <endpoint_identifier>
    │     │     │  │                 ╰─ /
    │     │     │  ╰─ udit_events
    │     │     │     ╰─ /
    │     │     │        ├─ <*rest>
    │     │     │        │  ╰─ /
    │     │     │        ╰─ <*rest>
    │     │     ├─ b
    │     │     │  ├─ adges
    │     │     │  │  ╰─ /
    │     │     │  │     ╰─ <*ref>
    │     │     │  │        ╰─ /
    │     │     │  │           ├─ coverage
    │     │     │  │           │  ├─ .
    │     │     │  │           │  │  ╰─ <format>
    │     │     │  │           │  │     ╰─ /
    │     │     │  │           │  ╰─ /
    │     │     │  │           ╰─ pipeline
    │     │     │  │              ├─ .
    │     │     │  │              │  ╰─ <format>
    │     │     │  │              │     ╰─ /
    │     │     │  │              ╰─ /
    │     │     │  ├─ l
    │     │     │  │  ├─ ame/
    │     │     │  │  │  ├─ <*id>
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  ╰─ <*id>
    │     │     │  │  ╰─ ob/
    │     │     │  │     ├─ <*id>
    │     │     │  │     │  ╰─ /
    │     │     │  │     ╰─ <*id>
    │     │     │  ╰─ uilds
    │     │     │     ╰─ /
    │     │     │        ├─ artifacts/
    │     │     │        │  ├─ <*ref_name_and_path>
    │     │     │        │  │  ╰─ /
    │     │     │        │  ╰─ <*ref_name_and_path>
    │     │     │        ├─ <build_id>
    │     │     │        │  ╰─ /artifacts/
    │     │     │        │     ├─ browse
    │     │     │        │     │  ╰─ /
    │     │     │        │     │     ├─ <*path>
    │     │     │        │     │     │  ╰─ /
    │     │     │        │     │     ╰─ <*path>
    │     │     │        │     ├─ download
    │     │     │        │     │  ╰─ /
    │     │     │        │     ├─ file/
    │     │     │        │     │  ├─ <*path>
    │     │     │        │     │  │  ╰─ /
    │     │     │        │     │  ╰─ <*path>
    │     │     │        │     ╰─ raw/
    │     │     │        │        ├─ <*path>
    │     │     │        │        │  ╰─ /
    │     │     │        │        ╰─ <*path>
    │     │     │        ╰─ <id>
    │     │     │           ╰─ /
    │     │     │              ╰─ raw
    │     │     │                 ╰─ /
    │     │     ├─ c
    │     │     │  ├─ lusters
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ <*rest>
    │     │     │  │     │  ╰─ /
    │     │     │  │     ╰─ <*rest>
    │     │     │  ├─ o
    │     │     │  │  ├─ m
    │     │     │  │  │  ├─ mit
    │     │     │  │  │  │  ├─ /
    │     │     │  │  │  │  │  ├─ <id>
    │     │     │  │  │  │  │  │  ├─ .
    │     │     │  │  │  │  │  │  │  ╰─ <format>
    │     │     │  │  │  │  │  │  │     ╰─ /
    │     │     │  │  │  │  │  │  ╰─ /
    │     │     │  │  │  │  │  ├─ <*rest>
    │     │     │  │  │  │  │  │  ╰─ /
    │     │     │  │  │  │  │  ╰─ <*rest>
    │     │     │  │  │  │  ╰─ s
    │     │     │  │  │  │     ╰─ /
    │     │     │  │  │  │        ├─ <*rest>
    │     │     │  │  │  │        │  ╰─ /
    │     │     │  │  │  │        ╰─ <*rest>
    │     │     │  │  │  ╰─ pare
    │     │     │  │  │     ╰─ /
    │     │     │  │  │        ├─ <*rest>
    │     │     │  │  │        │  ╰─ /
    │     │     │  │  │        ╰─ <*rest>
    │     │     │  │  ╰─ ntainer_registry
    │     │     │  │     ╰─ /
    │     │     │  │        ╰─ <id>
    │     │     │  │           ╰─ /
    │     │     │  ╰─ ycle_analytics
    │     │     │     ╰─ /
    │     │     │        ├─ <*rest>
    │     │     │        │  ╰─ /
    │     │     │        ╰─ <*rest>
    │     │     ├─ de
    │     │     │  ├─ pendencies
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ <*rest>
    │     │     │  │     │  ╰─ /
    │     │     │  │     ╰─ <*rest>
    │     │     │  ╰─ scription_templates/names/
    │     │     │     ╰─ <template_type>
    │     │     │        ├─ .
    │     │     │        │  ╰─ <format>
    │     │     │        │     ╰─ /
    │     │     │        ╰─ /
    │     │     ├─ e
    │     │     │  ├─ dit/
    │     │     │  │  ├─ <*id>
    │     │     │  │  │  ╰─ /
    │     │     │  │  ╰─ <*id>
    │     │     │  ├─ nvironments
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ <*rest>
    │     │     │  │     │  ╰─ /
    │     │     │  │     ╰─ <*rest>
    │     │     │  ╰─ rror_tracking
    │     │     │     ╰─ /
    │     │     │        ├─ <*rest>
    │     │     │        │  ╰─ /
    │     │     │        ╰─ <*rest>
    │     │     ├─ fi
    │     │     │  ├─ les
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ <*rest>
    │     │     │  │     │  ╰─ /
    │     │     │  │     ╰─ <*rest>
    │     │     │  ╰─ nd_file
    │     │     │     ╰─ /
    │     │     │        ├─ <*rest>
    │     │     │        │  ╰─ /
    │     │     │        ╰─ <*rest>
    │     │     ├─ hooks
    │     │     │  ╰─ /
    │     │     │     ├─ <*rest>
    │     │     │     │  ╰─ /
    │     │     │     ╰─ <*rest>
    │     │     ├─ i
    │     │     │  ├─ de_terminals
    │     │     │  │  ├─ .
    │     │     │  │  │  ╰─ <format>
    │     │     │  │  │     ╰─ /
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ check_config
    │     │     │  │     │  ├─ .
    │     │     │  │     │  │  ╰─ <format>
    │     │     │  │     │  │     ╰─ /
    │     │     │  │     │  ╰─ /
    │     │     │  │     ╰─ <id>
    │     │     │  │        ├─ .
    │     │     │  │        │  ╰─ <format>
    │     │     │  │        │     ╰─ /
    │     │     │  │        ╰─ /
    │     │     │  │           ├─ cancel
    │     │     │  │           │  ├─ .
    │     │     │  │           │  │  ╰─ <format>
    │     │     │  │           │  │     ╰─ /
    │     │     │  │           │  ╰─ /
    │     │     │  │           ╰─ retry
    │     │     │  │              ├─ .
    │     │     │  │              │  ╰─ <format>
    │     │     │  │              │     ╰─ /
    │     │     │  │              ╰─ /
    │     │     │  ├─ nsights
    │     │     │  │  ╰─ /
    │     │     │  │     ╰─ query
    │     │     │  │        ╰─ /
    │     │     │  ╰─ ssues
    │     │     │     ╰─ /
    │     │     │        ├─ <*rest>
    │     │     │        │  ╰─ /
    │     │     │        ╰─ <*rest>
    │     │     ├─ m
    │     │     │  ├─ attermost
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ <*rest>
    │     │     │  │     │  ╰─ /
    │     │     │  │     ╰─ <*rest>
    │     │     │  ├─ erge_requests
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ <*rest>
    │     │     │  │     │  ╰─ /
    │     │     │  │     ╰─ <*rest>
    │     │     │  ╰─ irror
    │     │     │     ╰─ /
    │     │     │        ├─ <*rest>
    │     │     │        │  ╰─ /
    │     │     │        ╰─ <*rest>
    │     │     ├─ n
    │     │     │  ├─ ew/
    │     │     │  │  ├─ <*id>
    │     │     │  │  │  ╰─ /
    │     │     │  │  ╰─ <*id>
    │     │     │  ╰─ ote
    │     │     │     ├─ able/
    │     │     │     │  ╰─ <target_type>
    │     │     │     │     ╰─ /
    │     │     │     │        ╰─ <target_id>
    │     │     │     │           ╰─ /notes
    │     │     │     │              ╰─ /
    │     │     │     ╰─ s
    │     │     │        ╰─ /
    │     │     │           ╰─ <id>
    │     │     │              ╰─ /
    │     │     │                 ├─ delete_attachment
    │     │     │                 │  ╰─ /
    │     │     │                 ├─ outdated_line_change
    │     │     │                 │  ╰─ /
    │     │     │                 ├─ resolve
    │     │     │                 │  ╰─ /
    │     │     │                 ╰─ toggle_award_emoji
    │     │     │                    ╰─ /
    │     │     ├─ p
    │     │     │  ├─ a
    │     │     │  │  ├─ ges
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ domains
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     │     ├─ new
    │     │     │  │  │     │     │  ╰─ /
    │     │     │  │  │     │     ╰─ <id>
    │     │     │  │  │     │        ╰─ /
    │     │     │  │  │     │           ├─ clean_certificate
    │     │     │  │  │     │           │  ╰─ /
    │     │     │  │  │     │           ├─ edit
    │     │     │  │  │     │           │  ╰─ /
    │     │     │  │  │     │           ├─ retry_auto_ssl
    │     │     │  │  │     │           │  ╰─ /
    │     │     │  │  │     │           ╰─ verify
    │     │     │  │  │     │              ╰─ /
    │     │     │  │  │     ╰─ new
    │     │     │  │  │        ╰─ /
    │     │     │  │  ╰─ th_locks
    │     │     │  │     ╰─ /
    │     │     │  │        ├─ toggle
    │     │     │  │        │  ╰─ /
    │     │     │  │        ╰─ <id>
    │     │     │  │           ╰─ /
    │     │     │  ├─ ipeline
    │     │     │  │  ├─ _schedules
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ <*rest>
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ╰─ <*rest>
    │     │     │  │  ╰─ s
    │     │     │  │     ╰─ /
    │     │     │  │        ├─ <*rest>
    │     │     │  │        │  ╰─ /
    │     │     │  │        ╰─ <*rest>
    │     │     │  ╰─ ro
    │     │     │     ├─ metheus/
    │     │     │     │  ├─ alerts/
    │     │     │     │  │  ├─ notify
    │     │     │     │  │  │  ╰─ /
    │     │     │     │  │  ╰─ <id>
    │     │     │     │  │     ╰─ /metrics_dashboard
    │     │     │     │  │        ╰─ /
    │     │     │     │  ╰─ metrics
    │     │     │     │     ╰─ /
    │     │     │     │        ├─ active_common
    │     │     │     │        │  ╰─ /
    │     │     │     │        ├─ new
    │     │     │     │        │  ╰─ /
    │     │     │     │        ├─ validate_query
    │     │     │     │        │  ╰─ /
    │     │     │     │        ╰─ <id>
    │     │     │     │           ╰─ /
    │     │     │     │              ╰─ edit
    │     │     │     │                 ╰─ /
    │     │     │     ╰─ tected_environments
    │     │     │        ╰─ /
    │     │     │           ├─ <*rest>
    │     │     │           │  ╰─ /
    │     │     │           ╰─ <*rest>
    │     │     ├─ r
    │     │     │  ├─ aw/
    │     │     │  │  ├─ <*id>
    │     │     │  │  │  ╰─ /
    │     │     │  │  ╰─ <*id>
    │     │     │  ├─ e
    │     │     │  │  ├─ fs/
    │     │     │  │  │  ├─ switch
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  ╰─ <id>
    │     │     │  │  │     ╰─ /logs_tree
    │     │     │  │  │        ╰─ /
    │     │     │  │  │           ├─ <*path>
    │     │     │  │  │           │  ╰─ /
    │     │     │  │  │           ╰─ <*path>
    │     │     │  │  ├─ gistry/repository/
    │     │     │  │  │  ╰─ <repository_id>
    │     │     │  │  │     ╰─ /tags
    │     │     │  │  │        ╰─ /
    │     │     │  │  │           ├─ bulk_destroy
    │     │     │  │  │           │  ╰─ /
    │     │     │  │  │           ╰─ <id>
    │     │     │  │  │              ╰─ /
    │     │     │  │  ├─ pository
    │     │     │  │  │  ╰─ /
    │     │     │  │  ╰─ store
    │     │     │  │     ╰─ /
    │     │     │  ╰─ unner
    │     │     │     ├─ _projects
    │     │     │     │  ╰─ /
    │     │     │     │     ╰─ <id>
    │     │     │     │        ╰─ /
    │     │     │     ╰─ s
    │     │     │        ╰─ /
    │     │     │           ├─ <*rest>
    │     │     │           │  ╰─ /
    │     │     │           ╰─ <*rest>
    │     │     ├─ s
    │     │     │  ├─ e
    │     │     │  │  ├─ curity
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ <*rest>
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     ╰─ <*rest>
    │     │     │  │  ╰─ rv
    │     │     │  │     ├─ erless
    │     │     │  │     │  ╰─ /
    │     │     │  │     │     ├─ <*rest>
    │     │     │  │     │     │  ╰─ /
    │     │     │  │     │     ╰─ <*rest>
    │     │     │  │     ╰─ ice_
    │     │     │  │        ├─ desk
    │     │     │  │        │  ╰─ /
    │     │     │  │        ╰─ ping/web_ide_pipelines_count
    │     │     │  │           ╰─ /
    │     │     │  ╰─ nippets
    │     │     │     ╰─ /
    │     │     │        ├─ <id>
    │     │     │        │  ╰─ /raw
    │     │     │        │     ╰─ /
    │     │     │        ├─ <*rest>
    │     │     │        │  ╰─ /
    │     │     │        ╰─ <*rest>
    │     │     ├─ t
    │     │     │  ├─ ags
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ <*rest>
    │     │     │  │     │  ╰─ /
    │     │     │  │     ╰─ <*rest>
    │     │     │  ├─ emplates/
    │     │     │  │  ╰─ <template_type>
    │     │     │  │     ├─ .
    │     │     │  │     │  ╰─ <format>
    │     │     │  │     │     ╰─ /
    │     │     │  │     ╰─ /
    │     │     │  │        ╰─ <key>
    │     │     │  │           ├─ .
    │     │     │  │           │  ╰─ <format>
    │     │     │  │           │     ╰─ /
    │     │     │  │           ╰─ /
    │     │     │  ├─ odos
    │     │     │  │  ╰─ /
    │     │     │  ╰─ r
    │     │     │     ├─ ee/
    │     │     │     │  ├─ <*id>
    │     │     │     │  │  ╰─ /
    │     │     │     │  ╰─ <*id>
    │     │     │     ╰─ iggers
    │     │     │        ╰─ /
    │     │     │           ├─ <*rest>
    │     │     │           │  ╰─ /
    │     │     │           ╰─ <*rest>
    │     │     ├─ uploads
    │     │     │  ╰─ /
    │     │     │     ├─ authorize
    │     │     │     │  ╰─ /
    │     │     │     ╰─ <secret>
    │     │     │        ╰─ /
    │     │     │           ╰─ <filename>
    │     │     │              ╰─ /
    │     │     ├─ v
    │     │     │  ├─ ariables
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ <*rest>
    │     │     │  │     │  ╰─ /
    │     │     │  │     ╰─ <*rest>
    │     │     │  ╰─ ulnerability_feedback
    │     │     │     ╰─ /
    │     │     │        ├─ <*rest>
    │     │     │        │  ╰─ /
    │     │     │        ╰─ <*rest>
    │     │     ├─ wikis
    │     │     │  ╰─ /
    │     │     │     ├─ <*rest>
    │     │     │     │  ╰─ /
    │     │     │     ╰─ <*rest>
    │     │     ├─ <*all>
    │     │     │  ╰─ /
    │     │     ╰─ <*all>
    │     ╰─ <id>
    │        ╰─ /
    │           ├─ a
    │           │  ├─ ctivity
    │           │  │  ╰─ /
    │           │  ╰─ rchive
    │           │     ╰─ /
    │           ├─ download_export
    │           │  ╰─ /
    │           ├─ e
    │           │  ├─ dit
    │           │  │  ╰─ /
    │           │  ╰─ xport
    │           │     ╰─ /
    │           ├─ generate_new_export
    │           │  ╰─ /
    │           ├─ housekeeping
    │           │  ╰─ /
    │           ├─ new_issuable_address
    │           │  ╰─ /
    │           ├─ re
    │           │  ├─ fs
    │           │  │  ╰─ /
    │           │  ╰─ move_
    │           │     ├─ export
    │           │     │  ╰─ /
    │           │     ╰─ fork
    │           │        ╰─ /
    │           ├─ t
    │           │  ├─ oggle_star
    │           │  │  ╰─ /
    │           │  ╰─ ransfer
    │           │     ╰─ /
    │           ╰─ un
    │              ├─ archive
    │              │  ╰─ /
    │              ╰─ foldered_environment_names
    │                 ╰─ /
    ╰─ <*id>
    ");

    Ok(())
}
