use std::fmt::Formatter;
use std::rc::Rc;

use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum Severity {
    Danger,
    Warning,
    Minor,
}

impl Default for Severity {
    fn default() -> Self {
        Severity::Minor
    }
}

type BackgroundClass = &'static str;
type TextClass = &'static str;

impl Severity {
    fn get_classes(&self) -> (BackgroundClass, TextClass) {
        match self {
            Severity::Danger => ("bg-danger", "text-light"),
            Severity::Warning => ("bg-warning", "text-dark"),
            Severity::Minor => ("bg-secondary", "text-light"),
        }
    }
    fn get_icon(&self) -> Html {
        match self {
            Severity::Danger => html! {
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-x-octagon-fill flex-shrink-0 me-2" viewBox="0 0 16 16">
                    <path d="M11.46.146A.5.5 0 0 0 11.107 0H4.893a.5.5 0 0 0-.353.146L.146 4.54A.5.5 0 0 0 0 4.893v6.214a.5.5 0 0 0 .146.353l4.394 4.394a.5.5 0 0 0 .353.146h6.214a.5.5 0 0 0 .353-.146l4.394-4.394a.5.5 0 0 0 .146-.353V4.893a.5.5 0 0 0-.146-.353L11.46.146zm-6.106 4.5L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 1 1 .708-.708z"/>
                </svg>
            },
            Severity::Warning => html! {
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-exclamation-triangle-fill flex-shrink-0 me-2" viewBox="0 0 16 16">
                    <path d="M8.982 1.566a1.13 1.13 0 0 0-1.96 0L.165 13.233c-.457.778.091 1.767.98 1.767h13.713c.889 0 1.438-.99.98-1.767L8.982 1.566zM8 5c.535 0 .954.462.9.995l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 5.995A.905.905 0 0 1 8 5zm.002 6a1 1 0 1 1 0 2 1 1 0 0 1 0-2z"/>
                </svg>
            },
            Severity::Minor => html! {
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-info-circle-fill flex-shrink-0 me-2" viewBox="0 0 16 16">
                    <path d="M8 16A8 8 0 1 0 8 0a8 8 0 0 0 0 16zm.93-9.412-1 4.705c-.07.34.029.533.304.533.194 0 .487-.07.686-.246l-.088.416c-.287.346-.92.598-1.465.598-.703 0-1.002-.422-.808-1.319l.738-3.468c.064-.293.006-.399-.287-.47l-.451-.081.082-.381 2.29-.287zM8 5.5a1 1 0 1 1 0-2 1 1 0 0 1 0 2z"/>
                </svg>
            },
        }
    }
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = match self {
            Severity::Danger => "Danger",
            Severity::Warning => "Warning",
            Severity::Minor => "Info",
        };
        write!(f, "{w}")
    }
}

#[derive(Properties, Clone)]
pub struct ErrorProp {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub severity: Severity,
    pub error: Rc<dyn std::error::Error>,
}

impl PartialEq for ErrorProp {
    fn eq(&self, other: &Self) -> bool {
        self.children == other.children
            && self.severity == other.severity
            && std::ptr::eq(self.error.as_ref(), other.error.as_ref())
    }
}
/// Displays a error banner
///
/// ```rust
/// use std::rc::Rc;
/// use yew::prelude::*;
/// use rquote_component::error::*;
/// use std::error::Error;
///
/// #[function_component(App)]
/// fn app()->Html {
///     let severity = Severity::Danger;
///     let error: Rc<dyn Error> = Rc::new("foo".parse::<u32>().unwrap_err());
///     html!{
///         <ErrorComponent {error} {severity}>
///             {"More information"}
///         </ErrorComponent>
///     }
/// }
/// ```
#[function_component(ErrorComponent)]
pub fn error(props: &ErrorProp) -> Html {
    let severity = &props.severity;
    let (bg, txt) = severity.get_classes();
    let icon = severity.get_icon();
    let err = props.error.as_ref();
    html! {
        <div class={classes!(bg,txt,"alert","alert-primary","d-flex","align-items-center","m-3")}>
            {icon}
            //<strong class={classes!("p-1")}>{format!("{severity}:")}</strong>
            <div class={classes!("p-1")}>{err.to_string()}</div>
            {for props.children.iter()}
        </div>
    }
}
